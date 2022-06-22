extern crate bcrypt;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use secrecy::Secret;

pub mod schema;
pub mod models;
pub mod handlers;
pub mod db;
pub mod errors;

#[derive(Clone)]
pub struct AppState {
    pub secret: Secret<String>,
}

impl Default for AppState {
    fn default() -> Self {
        AppState { secret: Secret::new(String::new()) }
    }
}