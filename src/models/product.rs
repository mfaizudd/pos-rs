use bigdecimal::{BigDecimal, FromPrimitive};
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
            if self.barcode.is_none() {
                return false;
            }
            let barcode = self.barcode.clone().unwrap_or_default();
            barcode.len() <= 0 || !barcode.chars().all(char::is_numeric)
        });
        err.push("Invalid price", || self.price < BigDecimal::from_i32(0).unwrap());
        err.push("Invalid stock", || self.stock < 0);
        err.to_result(())
    }
}

#[cfg(test)]
mod tests {
    use bigdecimal::{BigDecimal, FromPrimitive};

    use crate::validation::Validate;

    use super::InputProduct;

    fn generate_product() -> InputProduct {
        InputProduct {
            barcode: Some("12345".into()),
            price: BigDecimal::from_i32(100).unwrap(),
            stock: 4,
            name: "Valid".into()
        }
    }

    #[test]
    fn name_required_validated() {
        let mut product = generate_product();
        product.name = "".into();
        let result = product.validate();
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(err.len(), 1);
        assert_eq!(err.get_message(0), "Name is required")
    }

    #[test]
    fn barcode_numeric_validated() {
        let mut product = generate_product();
        product.barcode = Some("12345a".into());
        let result = product.validate();
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(err.len(), 1);
        assert_eq!(err.get_message(0), "Invalid barcode")
    }

    #[test]
    fn barcode_empty_validated() {
        let mut product = generate_product();
        product.barcode = Some("".into());
        let result = product.validate();
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(err.len(), 1);
        assert_eq!(err.get_message(0), "Invalid barcode")
    }

    #[test]
    fn price_validated() {
        let mut product = generate_product();
        product.price = BigDecimal::from_i32(-1).unwrap();
        let result = product.validate();
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(err.len(), 1);
        assert_eq!(err.get_message(0), "Invalid price")
    }

    #[test]
    fn stock_validated() {
        let mut product = generate_product();
        product.stock = -1;
        let result = product.validate();
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(err.len(), 1);
        assert_eq!(err.get_message(0), "Invalid stock")
    }

    #[test]
    fn barcode_none_pass() {
        let mut product = generate_product();
        product.barcode = None;
        let result = product.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn valid_product_pass() {
        let product = generate_product();
        let result = product.validate();
        assert!(result.is_ok());
    }
}
