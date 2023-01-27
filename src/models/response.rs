use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Response<T: Serialize> {
    pub status: u16,
    pub data: T,
}
