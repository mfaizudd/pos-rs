use bigdecimal::BigDecimal;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub barcode: Option<String>,
    pub price: BigDecimal,
    pub stock: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

pub struct NewProduct<'a> {
    pub name: &'a str,
    pub barcode: Option<&'a str>,
    pub price: BigDecimal,
    pub stock: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Deserialize)]
pub struct InputProduct {
    pub name: String,
    pub barcode: Option<String>,
    pub price: BigDecimal,
    pub stock: i32,
}
