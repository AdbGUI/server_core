use salvo::extra::{logging::Logger, serve_static::{StaticDir, Options}};
use salvo::Router;

mod apk;
pub mod logcat;

pub fn routes() -> Router {
    // static files or website
    Router::new()
        .hoop(Logger)
        .push(
            Router::with_path("api").push(
                Router::with_path("apk")
                    .get(apk::apk_get_data)
                    .post(apk::apk_upload),
            )
            .push(Router::with_path("logcat").handle(logcat::user_connected))
        )
        .push(Router::with_path("<**path>").get(StaticDir::width_options(
            vec!["web"],
            Options {
                dot_files: false,
                listing: true,
                defaults: vec!["index.html".to_owned()],
            },
        )))
}
