use actix_web::web;
use bigdecimal::BigDecimal;
use uuid::Uuid;

use crate::{models::product::Product, errors::ServiceError};

use super::DbPool;

pub async fn get_all(pool: web::Data<DbPool>) -> Result<Vec<Product>, ServiceError> {
    let mut pool = pool.acquire().await?;
    let products = sqlx::query_as!(Product, "select * from products")
        .fetch_all(&mut pool)
        .await?;
    Ok(products)
}

pub async fn get(pid: Uuid, pool: web::Data<DbPool>) -> Result<Product, ServiceError> {
    let mut pool = pool.acquire().await?;
    let product = sqlx::query_as!(Product, "select * from products where id = $1", pid)
        .fetch_one(&mut pool)
        .await?;
    Ok(product)
}

pub async fn add(
    name: &str,
    barcode: Option<String>,
    price: BigDecimal,
    stock: i32,
    pool: web::Data<DbPool>,
) -> Result<Product, ServiceError> {
    let mut pool = pool.acquire().await?;
    let now = chrono::Local::now().naive_utc();
    let product = sqlx::query_as!(
        Product,
        "insert into products(name, barcode, price, stock, created_at, updated_at)
         values($1, $2, $3, $4, $5, $6) returning *",
        name,
        barcode,
        price,
        stock,
        now,
        now
    )
    .fetch_one(&mut pool)
    .await?;
    Ok(product)
}

pub async fn update(
    pid: Uuid,
    name: &str,
    barcode: Option<String>,
    price: BigDecimal,
    stock: i32,
    pool: web::Data<DbPool>,
) -> Result<Product, ServiceError> {
    let mut pool = pool.acquire().await?;
    let now = chrono::Local::now().naive_utc();
    let product = sqlx::query_as!(
        Product,
        "update products
         set name = $1,
            barcode = $2,
            price = $3,
            stock = $4,
            updated_at = $5
         where id = $6
         returning *",
        name,
        barcode,
        price,
        stock,
        now,
        pid
    )
    .fetch_one(&mut pool)
    .await?;
    Ok(product)
}

pub async fn delete(pid: Uuid, pool: web::Data<DbPool>) -> Result<String, ServiceError> {
    let mut pool = pool.acquire().await?;
    let result = sqlx::query_as!(Product, "delete from products where id = $1", pid)
        .execute(&mut pool)
        .await?;
    Ok(format!("Deleted products: {}", result.rows_affected()))
}
