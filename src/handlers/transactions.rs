use crate::models::auth::Claims;
use crate::models::transaction::InputTransactionProduct;
use crate::db;
use crate::db::DbPool;
use actix_web::{post, services, web, Error, HttpResponse};

#[post("/transactions")]
pub async fn new_transaction(
    claims: web::Data<Claims>,
    products: web::Json<Vec<InputTransactionProduct>>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let claims = claims.into_inner();
    let user = db::users::find_by_email(claims.sub.clone(), pool.clone())
        .await?;
    let products = products.into_inner();
    let transaction = db::transactions::new_transaction(user.id, products, pool.clone())
        .await?;
    Ok(HttpResponse::Ok().json(transaction))
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(services![new_transaction]);
}
