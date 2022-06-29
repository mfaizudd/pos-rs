use actix_web::web;
use diesel::RunQueryDsl;
use uuid::Uuid;

use crate::db::{DbError, Pool};
use crate::models::{InputTransactionProduct, NewTransaction, Transaction, TransactionProduct};
use crate::schema;
use crate::schema::transactions::dsl;

pub fn new_transaction(uid: Uuid, products: Vec<InputTransactionProduct>, pool: web::Data<Pool>) -> Result<Transaction, DbError> {
    use schema::transaction_products;
    let conn = pool.get()?;
    let new_transaction = NewTransaction {
        user_id: uid,
        created_at: chrono::Local::now().naive_utc(),
    };
    let transaction = diesel::insert_into(dsl::transactions)
        .values(new_transaction)
        .get_result::<Transaction>(&conn)?;
    let products: Vec<TransactionProduct> = products.iter().map(|p| TransactionProduct {
        transaction_id: transaction.id,
        product_id: p.product_id,
        price: p.price.clone(),
        quantity: p.quantity,
    }).collect();
    diesel::insert_into(transaction_products::table)
        .values(products)
        .execute(&conn)?;
    Ok(transaction)
}