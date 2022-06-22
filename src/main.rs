extern crate diesel;
extern crate pos_rs;

use std::env;
use std::sync::Arc;

use actix_session::SessionMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_web::cookie::Key;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use secrecy::{ExposeSecret, Secret};
use pos_rs::{db, AppState};
use pos_rs::handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    HttpServer::new(move || {
        let secret = env::var("SECRET").expect("Secret must be set");
        let pool = db::create_connection_pool();
        let state = AppState {
            secret: Secret::new(secret)
        };
        let store = CookieSessionStore::default();
        let key = Key::from(state.secret.expose_secret().as_bytes());
        let session = SessionMiddleware::builder(store, key)
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
