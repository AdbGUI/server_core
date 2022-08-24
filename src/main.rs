use std::{env, fs::create_dir_all};

#[macro_use]
extern crate log;

use dotenv::dotenv;
use salvo::prelude::*;

mod model;
mod routes;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("trace"));

    let port: u16 = match env::var("HTTP_PLATFORM_PORT") {
        Ok(val) => val.parse().unwrap(),
        Err(_) => "7878".parse().unwrap(),
    };

    info!("Listening on http://127.0.0.1:{}", port);
    Server::new(TcpListener::bind(format!("127.0.0.1:{}", port).as_str()))
        .serve(routes::routes())
        .await;
}

pub fn config_path() -> String {
    let mut path = env::current_dir().unwrap();
    path.push("config");
    info!("Getting config path: {:?}", &path);
    if !path.exists() {
        create_dir_all(&path).unwrap();
    }
    path.to_str().unwrap_or(".").to_string()
}
