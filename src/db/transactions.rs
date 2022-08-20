use actix_web::web;
use uuid::Uuid;

use crate::{
    db::DbPool,
    errors::ServiceError,
    models::transaction::{InputTransactionProduct, Transaction, TransactionProduct},
};

pub async fn new_transaction(
    uid: Uuid,
    products: Vec<InputTransactionProduct>,
    pool: web::Data<DbPool>,
) -> Result<Transaction, ServiceError> {
    let mut pool = pool.begin().await?;
    let now = chrono::Local::now().naive_utc();
    let transaction = sqlx::query_as!(
        Transaction,
        "insert into transactions(user_id, created_at) values($1, $2) returning *",
        uid,
        now
    )
    .fetch_one(&mut pool)
    .await?;

    for product in products.iter() {
        let _transaction_product = sqlx::query_as!(
            TransactionProduct,
            "insert into transaction_products(
                transaction_id,
                product_id,
                price,
                quantity
            )
            values(
                $1, $2, $3, $4
            ) returning *",
            transaction.id,
            product.product_id,
            product.price,
            product.quantity
        )
        .fetch_one(&mut pool)
        .await?;
    }
    Ok(transaction)
}
