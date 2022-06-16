use std::env;

use crate::{db::{Pool, users}, models::InputLogin};
use actix_web::{post, web::{self, ServiceConfig}, Error, HttpResponse, services};
use dotenv::dotenv;
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: i64,
}

#[post("/auth/login")]
async fn login(req: web::Json<InputLogin>, pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        users::login(&req.email, &req.password, pool)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    match user {
        Some(u) => {
            dotenv().ok();
            let key = env::var("JWT_SECRET")
                .expect("No secret set");
            let key = EncodingKey::from_secret(key.as_ref());
            let duration = chrono::Utc::now()+chrono::Duration::days(1);
            let claims = Claims {
                sub: u.email,
                company: String::new(),
                exp: duration.timestamp().into()
            };
            let token = encode(&Header::default(), &claims, &key)
                .expect("Token creation failed");
            Ok(HttpResponse::Ok().json(token))
        }
        None => Ok(HttpResponse::Unauthorized().json("Username/Password not found"))
    }
}

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(services![
        login
    ]);
}