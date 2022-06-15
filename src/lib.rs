#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate bcrypt;

pub mod schema;
pub mod models;
pub mod handlers;
pub mod db;