use actix_web::web::ServiceConfig;
use actix_web::{get, post, services, web, Error, HttpResponse};
use actix_web_httpauth::middleware::HttpAuthentication;
use uuid::Uuid;

use crate::db::{self, Pool};
use crate::handlers::auth::validator;
use crate::models::InputProduct;

#[get("/products", wrap = "HttpAuthentication::bearer(validator)")]
async fn get_products(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let products = web::block(move || db::products::get_all(pool))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(products))
}

#[get("/products/{id}", wrap = "HttpAuthentication::bearer(validator)")]
async fn get_product(path: web::Path<Uuid>, pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let pid = path.into_inner();
    let product = web::block(move || db::products::get(pid, pool))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;
    match product {
        Some(p) => Ok(HttpResponse::Ok().json(p)),
        None => Ok(HttpResponse::NotFound().json("Product not found")),
    }
}

#[post("/products", wrap = "HttpAuthentication::bearer(validator)")]
async fn create_product(
    req: web::Json<InputProduct>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let InputProduct {
        name,
        barcode,
        price,
        stock,
    } = req.into_inner();
    let product = web::block(move || db::products::add(&name, barcode, price, stock, pool))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(product))
}

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(services![get_products, get_product, create_product]);
}
