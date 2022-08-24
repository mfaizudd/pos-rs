use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::validation::{Validate, ValidationError};

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

impl Validate for InputProduct {
    type OkResult = ();

    fn validate(&self) -> Result<Self::OkResult, ValidationError> {
        let mut err = ValidationError::new();
        err.push("Name is required", || self.name.len() <= 0);
        err.push("Invalid barcode", || {
            let barcode = self.barcode.clone().unwrap_or_default();
            barcode.len() <= 0 || barcode.chars().all(char::is_numeric)
        });
        // TODO: Validate price and stock
        err.to_result(())
    }
}
