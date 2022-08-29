use deadpool_redis::{Config, Runtime};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub mod products;
pub mod transactions;
pub mod users;

pub type DbPool = Pool<Postgres>;
pub type RedisPool = deadpool_redis::Pool;

pub async fn create_connection_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("Database url must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect(&format!("Error connecting to {}", database_url));
    pool
}

pub async fn create_redis_pool() -> RedisPool {
    let redis_url = env::var("REDIS_URL").expect("Redis url must be set");
    let cfg = Config::from_url(redis_url);
    let pool = cfg
        .create_pool(Some(Runtime::Tokio1))
        .expect("Failed to create redis pool");
    pool
}
