use actix_web::web::ServiceConfig;
use actix_web::{delete, get, post, put, services, web, Error, HttpResponse};
use uuid::Uuid;

use crate::db::{self, DbPool};
use crate::errors::ServiceError;
use crate::models::product::InputProduct;

#[get("/products")]
async fn get_products(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let products = db::products::get_all(pool)
        .await?;
    Ok(HttpResponse::Ok().json(products))
}

#[get("/products/{id}")]
async fn get_product(
    path: web::Path<Uuid>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let pid = path.into_inner();
    let product = db::products::get(pid, pool)
        .await?;
    Ok(HttpResponse::Ok().json(product))
}

#[post("/products")]
async fn create_product(
    req: web::Json<InputProduct>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServiceError> {
    let InputProduct {
        name,
        barcode,
        price,
        stock,
    } = req.into_inner();
    let product = db::products::add(&name, barcode, price, stock, pool)
        .await?;
    Ok(HttpResponse::Ok().json(product))
}

#[put("/products/{id}")]
async fn update_product(
    path: web::Path<Uuid>,
    req: web::Json<InputProduct>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let pid = path.into_inner();
    let InputProduct {
        name,
        barcode,
        price,
        stock,
    } = req.into_inner();
    let product = db::products::update(pid, &name, barcode, price, stock, pool)
        .await?;

    Ok(HttpResponse::Ok().json(product))
}

#[delete("/products/{id}")]
async fn delete_product(
    path: web::Path<Uuid>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let pid = path.into_inner();
    let response = db::products::delete(pid, pool)
        .await?;

    Ok(HttpResponse::Ok().json(response))
}

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(services![
        get_products,
        get_product,
        create_product,
        update_product,
        delete_product
    ]);
}
