use secrecy::Secret;

pub mod db;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod validation;

#[derive(Clone)]
pub struct AppState {
    pub secret: Secret<String>,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            secret: Secret::new(String::new()),
        }
    }
}
