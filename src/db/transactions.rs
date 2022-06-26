use actix_web::web;
use diesel::RunQueryDsl;
use uuid::Uuid;

use crate::db::{DbError, Pool};
use crate::models::{NewTransaction, Transaction, TransactionProduct};
use crate::schema;
use crate::schema::transactions::dsl;

fn new_transaction(uid: Uuid, products: Vec<TransactionProduct>, pool: web::Data<Pool>) -> Result<Transaction, DbError> {
    use schema::transaction_products;
    let conn = pool.get()?;
    let new_transaction = NewTransaction {
        user_id: uid,
        created_at: chrono::Local::now().naive_utc()
    };
    let transaction = diesel::insert_into(dsl::transactions)
        .values(new_transaction)
        .get_result::<Transaction>(&conn)?;
    diesel::insert_into(transaction_products::table)
        .values(products)
        .execute(&conn)?;
    Ok(transaction)
}