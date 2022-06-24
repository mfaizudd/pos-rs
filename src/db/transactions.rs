use actix_web::web;
use crate::db::{DbError, Pool};
use crate::models::Transaction;

fn new_transaction(pool: web::Data<Pool>) -> Result<Option<Transaction>, DbError> {
    todo!("Implement new transaction")
}