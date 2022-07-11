use actix_files as fs;
use actix_web::web;

mod apk;
pub mod logcat;

pub fn routes(cfg: &mut web::ServiceConfig) {
    // static files or website
    cfg.service(fs::Files::new("/", "./web").index_file("index.html"))
        // api service
        .service(
            web::scope("/api")
                .service(
                    web::scope("/apk")
                        .route("", web::get().to(apk::apk_get_data))
                        .route("", web::post().to(apk::apk_upload)),
                )
                .service(
                    web::scope("/logcat")
                        .service(web::resource("/ws/").route(web::get().to(logcat::ws_index))),
                ),
        );
}
