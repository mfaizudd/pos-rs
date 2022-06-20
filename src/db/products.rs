use actix_web::web;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::models::Product;
use crate::schema::products::dsl;

use super::{DbError, Pool};

pub fn get_all(pool: web::Data<Pool>) -> Result<Vec<Product>, DbError> {
    use dsl::*;
    let conn = pool.get()?;
    let items = products.load::<Product>(&conn)?;
    Ok(items)
}

pub fn get(uid: Uuid, pool: web::Data<Pool>) -> Result<Option<Product>, DbError> {
    use dsl::*;
    let conn = pool.get()?;
    let item = products
        .filter(id.eq_all(uid))
        .first::<Product>(&conn)
        .optional()?;

    Ok(item)
}

