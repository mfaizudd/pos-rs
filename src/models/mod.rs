use serde::Serialize;

pub mod auth;
pub mod product;
pub mod transaction;
pub mod user;

#[derive(Debug, Serialize)]
pub struct Response<T: Serialize> {
    pub status: u16,
    pub data: T,
}
