use crate::db;
use crate::db::DbPool;
use crate::errors::ServiceError;
use crate::models::{auth::Claims, transaction::InputTransaction};
use crate::validation::Validate;
use actix_web::{get, post, services, web, Error, HttpResponse};
use uuid::Uuid;

#[post("/transactions")]
pub async fn new_transaction(
    claims: Claims,
    input: web::Json<InputTransaction>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServiceError> {
    let user = db::users::find_by_email(claims.sub.clone(), pool.clone()).await?;
    let input = input.into_inner();
    input.validate()?;
    let transaction = db::transactions::new_transaction(user.id, input, pool.clone()).await?;
    Ok(HttpResponse::Ok().json(transaction))
}

#[get("/transactions/{id}")]
pub async fn get(uid: web::Path<Uuid>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let uid = uid.into_inner();
    let transaction = db::transactions::get(uid, pool).await?;
    Ok(HttpResponse::Ok().json(transaction))
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(services![new_transaction]);
}
