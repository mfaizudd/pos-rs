use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use env_logger::Env;
use futures_util::join;
use secrecy::Secret;
use std::env;

use pos_rs::handlers;
use pos_rs::{db, AppState};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let pool = db::create_connection_pool();
    let redis_pool = db::create_redis_pool();
    let (pool, redis_pool) = join!(pool, redis_pool);
    let secret = env::var("SECRET").expect("Secret must be set");
    let state = AppState {
        secret: Secret::new(secret),
    };
    sqlx::migrate!();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(redis_pool.clone()))
            .app_data(web::Data::new(state.clone()))
            .wrap(Logger::default())
            .configure(handlers::configuration)
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
