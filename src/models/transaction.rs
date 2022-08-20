use bigdecimal::BigDecimal;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

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
    pub products: Vec<TransactionProduct>
}

impl TransactionResponse {
    pub fn new(transaction: Transaction, products: Vec<TransactionProduct>) -> Self {
        TransactionResponse {
            id: transaction.id,
            user_id: transaction.user_id,
            notes: transaction.notes,
            total_paid: transaction.total_paid,
            created_at: transaction.created_at,
            products
        }
    }
}

#[derive(Deserialize)]
pub struct InputTransactionProduct {
    pub product_id: Uuid,
    pub quantity: i32,
    pub price: BigDecimal,
}