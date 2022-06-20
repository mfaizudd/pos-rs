use actix_web::web::ServiceConfig;
use actix_web::{get, services, web, Error, HttpResponse};
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::db::{self, Pool};
use crate::handlers::auth::validator;

#[get("/products", wrap = "HttpAuthentication::bearer(validator)")]
async fn get_products(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let products = web::block(move || db::products::get_all(pool))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(products))
}

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(services![get_products]);
}
