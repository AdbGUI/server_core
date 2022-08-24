use salvo::extra::logging::LogHandler;
use salvo::extra::serve_static::{DirHandler, Options};
use salvo::Router;

mod apk;
pub mod logcat;

pub fn routes() -> Router {
    // static files or website
    Router::new()
        .hoop(LogHandler)
        .push(
            Router::with_path("api").push(
                Router::with_path("apk")
                    .get(apk::apk_get_data)
                    .post(apk::apk_upload),
            )
            .push(Router::with_path("logcat").handle(logcat::user_connected))
        )
        .push(Router::with_path("<**path>").get(DirHandler::width_options(
            vec!["web"],
            Options {
                dot_files: false,
                listing: false,
                defaults: vec!["index.html".to_owned()],
            },
        )))
}
