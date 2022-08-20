extern crate pos_rs;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use env_logger::Env;
use secrecy::Secret;
use std::env;

use pos_rs::handlers;
use pos_rs::{db, AppState};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let pool = db::create_connection_pool().await;

    HttpServer::new(move || {
        let secret = env::var("SECRET").expect("Secret must be set");
        let state = AppState {
            secret: Secret::new(secret),
        };
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(state))
            .wrap(Logger::default())
            .configure(handlers::configuration)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
