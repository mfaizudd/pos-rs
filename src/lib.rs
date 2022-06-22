#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate bcrypt;

use secrecy::Secret;

pub mod schema;
pub mod models;
pub mod handlers;
pub mod db;

#[derive(Clone)]
pub struct AppState {
    pub secret: Secret<String>
}

impl Default for AppState {
    fn default() -> Self {
        AppState { secret: Secret::new(String::new()) }
    }
}