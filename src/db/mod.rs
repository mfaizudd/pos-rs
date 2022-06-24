use std::env;
use std::error::Error;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub mod users;
pub mod products;
pub mod transactions;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbError = Box<dyn Error + Sync + Send>;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("Database url must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_connection_pool() -> Pool {
    let database_url = env::var("DATABASE_URL").expect("Database url must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    pool
}
