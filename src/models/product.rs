use bigdecimal::{BigDecimal, FromPrimitive};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

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

#[derive(Deserialize, Validate)]
pub struct InputProduct {
    #[validate(length(min = 1))]
    pub name: String,
    pub barcode: Option<String>,
    #[validate(custom = "validate_price")]
    pub price: BigDecimal,
    #[validate(range(min = 0))]
    pub stock: i32,
}

fn validate_price(value: &BigDecimal) -> Result<(), ValidationError> {
    if *value < BigDecimal::from_i32(0).unwrap() {
        return Err(ValidationError::new("invalid_price"));
    }
    Ok(())
}
