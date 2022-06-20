use actix_web::web;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::models::{Product, NewProduct};
use crate::schema::products::dsl;

use super::{DbError, Pool};

pub fn get_all(pool: web::Data<Pool>) -> Result<Vec<Product>, DbError> {
    use dsl::*;
    let conn = pool.get()?;
    let items = products.load::<Product>(&conn)?;
    Ok(items)
}

pub fn get(pid: Uuid, pool: web::Data<Pool>) -> Result<Option<Product>, DbError> {
    use dsl::*;
    let conn = pool.get()?;
    let item = products
        .filter(id.eq_all(pid))
        .first::<Product>(&conn)
        .optional()?;

    Ok(item)
}

pub fn add(
    name: &str,
    barcode: Option<String>,
    price: BigDecimal,
    stock: i32,
    pool: web::Data<Pool>,
) -> Result<Product, DbError> {
    let conn = pool.get()?;
    let product = NewProduct {
        name,
        barcode: barcode.as_deref(),
        price,
        stock,
        created_at: chrono::Local::now().naive_utc(),
        updated_at: chrono::Local::now().naive_utc()
    };
    let product = diesel::insert_into(dsl::products)
        .values(product)
        .get_result(&conn)?;
    Ok(product)
}

pub fn update(
    pid: Uuid,
    name: &str,
    barcode: Option<String>,
    price: BigDecimal,
    stock: i32,
    pool: web::Data<Pool>
) -> Result<Option<Product>, DbError> {
    let conn = pool.get()?;
    let product_query = dsl::products.find(pid);
    let product: Product = product_query.first::<Product>(&conn)?;
    let updated_product = NewProduct {
        name,
        barcode: barcode.as_deref(),
        price,
        stock,
        created_at: product.created_at,
        updated_at: chrono::Local::now().naive_utc()
    };
    let product = diesel::update(product_query)
        .set(updated_product)
        .get_result::<Product>(&conn)
        .optional()?;
    Ok(product)
}

pub fn delete(pid: Uuid, pool: web::Data<Pool>) -> Result<String, DbError> {
    let conn = pool.get()?;
    let product = dsl::products.find(pid);
    let num_deleted = diesel::delete(product).execute(&conn)?;
    let response = format!("Deleted {} product(s)", num_deleted);
    Ok(response)
}
