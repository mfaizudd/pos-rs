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

pub struct NewTransaction {
    pub user_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Deserialize)]
pub struct InputTransactionProduct {
    pub product_id: Uuid,
    pub quantity: i32,
    pub price: BigDecimal,
}