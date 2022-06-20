use actix_web::web;
use diesel::RunQueryDsl;

use crate::models::Product;
use crate::schema::products::dsl;

use super::{DbError, Pool};

pub fn get_all(pool: web::Data<Pool>) -> Result<Vec<Product>, DbError> {
    use dsl::*;
    let conn = pool.get()?;
    let items = products.load::<Product>(&conn)?;
    Ok(items)
}
