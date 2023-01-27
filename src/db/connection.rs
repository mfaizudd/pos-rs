use deadpool_redis::{Config, Runtime};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub type DbPool = Pool<Postgres>;
pub type RedisPool = deadpool_redis::Pool;

pub async fn create_connection_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("Database url must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}

pub async fn create_redis_pool() -> RedisPool {
    let redis_url = env::var("REDIS_URL").expect("Redis url must be set");
    let cfg = Config::from_url(redis_url);
    cfg.create_pool(Some(Runtime::Tokio1))
        .expect("Failed to create redis pool")
}
