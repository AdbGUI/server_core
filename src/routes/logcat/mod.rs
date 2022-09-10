// Usings
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use futures_util::{FutureExt, StreamExt};
use once_cell::sync::Lazy;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;

use salvo::extra::ws::{Message, WebSocket, WebSocketUpgrade};
use salvo::prelude::*;

type TxData = mpsc::UnboundedSender<Result<Message, salvo::Error>>;

struct UserWs {
    pub tx: TxData,
    pub origin: String,
}

type Users = RwLock<HashMap<usize, UserWs>>;

static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);
static ONLINE_USERS: Lazy<Users> = Lazy::new(Users::default);

/// If the origin:
///
/// desktop => this send the logcat, is a desktop application
/// web     => is client, this only read the logcat sended by desktop
///

#[handler]
pub async fn user_connected(req: &mut Request, res: &mut Response) -> Result<(), StatusError> {
    let origin = req.query_or_form::<String>("origin").await.unwrap_or("desktop".to_string());
    WebSocketUpgrade::new()
        .handle(req, res, |ws| handle_socket(origin, ws))
        .await
}

async fn handle_socket(origin: String, ws: WebSocket) {
    // Use a counter to assign a new unique ID for this user.
    let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);

    info!("new chat user: {}", my_id);

    // Split the socket into a sender and receive of messages.
    let (user_ws_tx, mut user_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the websocket...
    let (tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);
    let fut = rx.forward(user_ws_tx).map(|result| {
        if let Err(e) = result {
            error!("websocket send error: {e}");
        }
    });
    tokio::task::spawn(fut);
    let fut = async move {
        ONLINE_USERS.write().await.insert(
            my_id,
            UserWs {
                tx,
                origin: origin.to_string(),
            },
        );

        while let Some(result) = user_ws_rx.next().await {
            let msg = match result {
                Ok(msg) => msg,
                Err(e) => {
                    eprintln!("websocket error(uid={}): {}", my_id, e);
                    break;
                }
            };
            user_message(my_id, msg).await;
        }

        user_disconnected(my_id).await;
    };
    tokio::task::spawn(fut);
}

async fn user_message(my_id: usize, msg: Message) {
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    let new_msg = format!("<User#{}>: {}", my_id, msg);
    println!("Message: {}", &new_msg);

    let users = ONLINE_USERS.read().await;
    let users = users
        .iter()
        .filter(|(&uid, user)| my_id != uid && user.origin != "desktop");

    // New message from this user, send it to everyone else (except same uid)...
    for (&uid, user) in users {
        if let Err(_disconnected) = user.tx.send(Ok(Message::text(new_msg.clone()))) {
            // The tx is disconnected, our `user_disconnected` code
            // should be happening in another task, nothing more to
            // do here.
            user_disconnected(uid).await;
        }
    }
}

async fn user_disconnected(my_id: usize) {
    eprintln!("good bye user: {}", my_id);
    // Stream closed up, so remove from the user list
    ONLINE_USERS.write().await.remove(&my_id);
}
