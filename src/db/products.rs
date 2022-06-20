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
