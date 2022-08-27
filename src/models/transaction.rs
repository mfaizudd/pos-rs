use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::validation::{
    validators::{Min, NotEmpty},
    Validate, ValidationError,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub notes: Option<String>,
    pub total_paid: BigDecimal,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionProduct {
    pub transaction_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub price: BigDecimal,
}

#[derive(Serialize)]
pub struct TransactionResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub notes: Option<String>,
    pub total_paid: BigDecimal,
    pub created_at: chrono::NaiveDateTime,
    pub products: Vec<TransactionProduct>,
}

impl TransactionResponse {
    pub fn new(transaction: Transaction, products: Vec<TransactionProduct>) -> Self {
        TransactionResponse {
            id: transaction.id,
            user_id: transaction.user_id,
            notes: transaction.notes,
            total_paid: transaction.total_paid,
            created_at: transaction.created_at,
            products,
        }
    }
}

#[derive(Deserialize)]
pub struct InputTransaction {
    pub notes: Option<String>,
    pub total_paid: BigDecimal,
    pub products: Vec<InputTransactionProduct>,
}

#[derive(Deserialize)]
pub struct InputTransactionProduct {
    pub product_id: Uuid,
    pub quantity: i32,
}

impl Validate for InputTransactionProduct {
    type OkResult = ();

    fn validate(&self) -> Result<Self::OkResult, ValidationError> {
        let mut err = ValidationError::new();
        err.push("Invalid quantity, must be higher than 0", || {
            self.quantity.minimum(1)
        });
        err.to_result(())
    }
}

impl Validate for InputTransaction {
    type OkResult = ();

    fn validate(&self) -> Result<Self::OkResult, ValidationError> {
        let mut err = ValidationError::new();
        err.push("Invalid notes", || self.notes.not_empty());
        err.push("Invalid total paids", || self.total_paid.minimum(0));
        err.push("Product must be at least 1", || self.products.minimum(1));
        err.to_result(())
    }
}

#[cfg(test)]
mod tests {
    use bigdecimal::{BigDecimal, FromPrimitive};
    use uuid::Uuid;

    use crate::validation::Validate;

    use super::{InputTransaction, InputTransactionProduct};

    /// Generates a valid transaction
    fn generate_transaction() -> InputTransaction {
        let price = BigDecimal::from_i32(10000).unwrap();
        InputTransaction {
            notes: Some("Test".to_owned()),
            total_paid: price.clone(),
            products: vec![InputTransactionProduct {
                product_id: Uuid::new_v4(),
                quantity: 4,
            }],
        }
    }

    #[test]
    fn notes_validated_on_some() {
        let mut transaction = generate_transaction();
        transaction.notes = Some("".to_owned());
        let result = transaction.validate().err().unwrap();
        assert_eq!(result.get_message(0), "Invalid notes")
    }

    #[test]
    fn total_paid_validated() {
        let mut transaction = generate_transaction();
        transaction.total_paid = BigDecimal::from_i32(-1).unwrap();
        let result = transaction.validate().err().unwrap();
        assert_eq!(result.get_message(0), "Invalid total paids")
    }

    #[test]
    fn min_number_of_product_validated() {
        let mut transaction = generate_transaction();
        transaction.products = vec![];
        let result = transaction.validate().err().unwrap();
        assert_eq!(result.get_message(0), "Product must be at least 1")
    }

    #[test]
    fn notes_none_should_pass() {
        let mut transaction = generate_transaction();
        transaction.notes = None;
        assert!(transaction.validate().is_ok())
    }

    #[test]
    fn valid_transaction_should_pass() {
        let transaction = generate_transaction();
        assert!(transaction.validate().is_ok())
    }
}
