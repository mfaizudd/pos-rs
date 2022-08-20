use std::env;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub mod products;
pub mod transactions;
pub mod users;

pub type DbPool = Pool<Postgres>;

pub async fn create_connection_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("Database url must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect(&format!("Error connecting to {}", database_url));
    pool
}
