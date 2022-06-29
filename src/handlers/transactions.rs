use actix_session::Session;
use actix_web::{Error, HttpResponse, post, services, web};
use uuid::Uuid;
use crate::models::InputTransactionProduct;
use crate::db;
use crate::db::Pool;

#[post("/transactions")]
pub async fn new_transaction(products: web::Json<Vec<InputTransactionProduct>>, session: Session, pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let uid = match session.get::<Uuid>("session_id")? {
        Some(uid) => uid,
        None => return Ok(HttpResponse::Unauthorized().body("Not logged in"))
    };
    let products = products.into_inner();
    let transaction = web::block(move || {
        db::transactions::new_transaction(uid, products, pool)
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(transaction))
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(services![new_transaction]);
}