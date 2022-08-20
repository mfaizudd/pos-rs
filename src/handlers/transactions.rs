use crate::models::auth::Claims;
use crate::models::transaction::InputTransactionProduct;
use crate::db;
use crate::db::DbPool;
use actix_web::{post, services, web, Error, HttpResponse, get};
use uuid::Uuid;

#[post("/transactions")]
pub async fn new_transaction(
    claims: Claims,
    products: web::Json<Vec<InputTransactionProduct>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let user = db::users::find_by_email(claims.sub.clone(), pool.clone())
        .await?;
    let products = products.into_inner();
    let transaction = db::transactions::new_transaction(user.id, products, pool.clone())
        .await?;
    Ok(HttpResponse::Ok().json(transaction))
}

#[get("/transactions/{id}")]
pub async fn get(uid: web::Path<Uuid>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let uid = uid.into_inner();
    let transaction = db::transactions::get(uid, pool)
        .await?;
    Ok(HttpResponse::Ok().json(transaction))
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(services![new_transaction]);
}
