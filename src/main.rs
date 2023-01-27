use dotenvy::dotenv;
use env_logger::Env;
use futures_util::join;
use secrecy::Secret;
use std::env;
use std::net::TcpListener;

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
    let port = env::var("PORT").unwrap_or_else(|_| String::from("8080"));
    let port = port.parse::<u16>().expect("Invalid port");
    let allowed_origin = env::var("ALLOWED_ORIGIN").expect("Allowed origin must be set");
    sqlx::migrate!().run(&pool).await.unwrap();
    let address = TcpListener::bind(format!("0.0.0.0:{port}"))?;
    pos_rs::run(address, pool, redis_pool, state, allowed_origin)?.await
}
