use actix_web::web;
use uuid::Uuid;

use crate::{
    db::DbPool,
    errors::ServiceError,
    models::transaction::{InputTransaction, Transaction, TransactionProduct, TransactionResponse},
};

pub async fn new_transaction(
    uid: Uuid,
    input: InputTransaction,
    pool: web::Data<DbPool>,
) -> Result<Transaction, ServiceError> {
    let mut pool = pool.begin().await?;
    let now = chrono::Local::now().naive_utc();
    let transaction = sqlx::query_as!(
        Transaction,
        "insert into transactions(notes, total_paid, user_id, created_at) values($1, $2, $3, $4) returning *",
        input.notes,
        input.total_paid,
        uid,
        now
    )
    .fetch_one(&mut pool)
    .await?;

    for product in input.products.iter() {
        sqlx::query_as!(
            TransactionProduct,
            "insert into transaction_products(
                transaction_id,
                product_id,
                price,
                quantity
            )
            values(
                $1, $2, (select price from products where id = $2), $3
            )",
            transaction.id,
            product.product_id,
            product.quantity
        )
        .execute(&mut pool)
        .await?;
    }
    pool.commit().await?;
    Ok(transaction)
}

pub async fn get(tid: Uuid, pool: web::Data<DbPool>) -> Result<TransactionResponse, ServiceError> {
    let mut pool = pool.acquire().await?;
    let transaction = sqlx::query_as!(Transaction, "select * from transactions where id = $1", tid)
        .fetch_one(&mut pool)
        .await?;
    let products = sqlx::query_as!(
        TransactionProduct,
        "select * from transaction_products where transaction_id = $1",
        tid
    )
    .fetch_all(&mut pool)
    .await?;
    Ok(TransactionResponse::new(transaction, products))
}

pub async fn get_all(pool: web::Data<DbPool>) -> Result<Vec<Transaction>, ServiceError> {
    let mut pool = pool.acquire().await?;
    let transactions = sqlx::query_as!(Transaction, "select * from transactions")
        .fetch_all(&mut pool)
        .await?;
    Ok(transactions)
}
