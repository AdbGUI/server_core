use std::env::{self, current_exe};

#[macro_use]
extern crate log;

use actix::Actor;
use actix_cors::Cors;
use actix_web::{http, middleware::Logger, App, HttpServer};
use dotenv::dotenv;

mod errors;
mod model;
mod routes;

#[cfg(test)]
mod tests;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    let server = routes::logcat::Server::new().start();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&env::var("CLIENT_HOST").unwrap())
            .allow_any_method()
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(server.clone())
            .configure(routes::routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

pub fn config_path() -> String {
    let mut path = current_exe().unwrap_or_default();
    path.push("config");
    path.to_str().unwrap_or(".").to_string()
}
