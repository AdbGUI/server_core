use std::io::Write;

use actix_multipart::Multipart;
use actix_web::web::{self, Json};
use futures::{StreamExt, TryStreamExt};
use uuid::Uuid;

use crate::{config_path, errors::Error, model::reply::GenericReply};

use super::{get_state, APK_FOLDER};

pub async fn apk_upload(mut payload: Multipart) -> Result<Json<GenericReply<String>>, Error> {
    let file_uuid = Uuid::new_v4().to_string();
    let file_name = format!("{}.apk", file_uuid);
    let file_path = format!("{}/{}/{}", config_path(), APK_FOLDER, file_name);
    let state = get_state();

    // iterate over multipart stream
    while let Some(mut field) = payload.try_next().await.unwrap() {
        let file_path = file_path.clone();
        let file_path_last_name = file_path.clone();
        // A multipart/form-data stream has to contain `content_disposition`
        let field_name = field.content_disposition().get_name().unwrap();
        match field_name {
            "file" => {
                // File::create is blocking operation, use threadpool
                let mut f = web::block(|| std::fs::File::create(file_path))
                    .await
                    .unwrap()
                    .unwrap();

                // Field in turn is stream of *Bytes* object
                while let Some(chunk) = field.try_next().await.unwrap() {
                    // filesystem operations are blocking, we have to use threadpool
                    f = web::block(move || f.write_all(&chunk).map(|_| f))
                        .await
                        .unwrap()
                        .unwrap();
                }
                (*state.lock().unwrap()).set_last_file(file_path_last_name);
            }
            "name" => {
                if let Some(chunk) = field.next().await {
                    let data = chunk.expect("Get bytes from chunk of name error");
                    (*state.lock().unwrap()).set_app_name(serde_json::from_slice(&data).unwrap());
                }
            }
            "version" => {
                if let Some(chunk) = field.next().await {
                    let data = chunk.expect("Get bytes from chunk of name error");
                    (*state.lock().unwrap()).set_app_name(serde_json::from_slice(&data).unwrap());
                }
            }
            _ => {}
        }
    }

    Ok(web::Json(GenericReply::err_internal(
        "No apk upload".to_string(),
        "".to_string(),
    )))
}
