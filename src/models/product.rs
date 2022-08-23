use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
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

#[derive(Deserialize)]
pub struct InputProduct {
    pub name: String,
    pub barcode: Option<String>,
    pub price: BigDecimal,
    pub stock: i32,
}
