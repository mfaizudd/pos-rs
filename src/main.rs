extern crate diesel;
extern crate pos_rs;

use std::env;

use actix_session::SessionMiddleware;
use actix_session::storage::RedisSessionStore;
use actix_web::{App, HttpServer, web};
use actix_web::cookie::Key;
use actix_web::middleware::Logger;
use dotenv::dotenv;
use env_logger::Env;
use secrecy::{ExposeSecret, Secret};

use pos_rs::{AppState, db};
use pos_rs::handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let redis_url = env::var("REDIS_URL").expect("Redis url must be set");
    let store = RedisSessionStore::new(redis_url).await.expect("Cannot connect to redis");

    HttpServer::new(move || {
        let secret = env::var("SECRET").expect("Secret must be set");
        let pool = db::create_connection_pool();
        let state = AppState {
            secret: Secret::new(secret)
        };
        let key = Key::from(state.secret.expose_secret().as_bytes());
        let session = SessionMiddleware::builder(store.clone(), key)
            .cookie_secure(false)
            .build();
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(state.clone()))
            .wrap(Logger::default())
            .wrap(session)
            .configure(handlers::configuration)
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
