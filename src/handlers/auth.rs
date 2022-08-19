use actix_identity::Identity;
use actix_web::{dev::ServiceRequest, Error, get, HttpMessage, HttpRequest, HttpResponse, post, services, web::{self, ServiceConfig}};
use actix_web_httpauth::extractors::{
    AuthenticationError,
    bearer::{BearerAuth, Config},
};
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};

use crate::{AppState, db::{Pool, users}, models::InputLogin};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: i64,
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);
    let state = req
        .app_data::<AppState>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);
    match validate_token(credentials.token(), state.secret.expose_secret()) {
        Ok(res) => res
            .then(|| req)
            .ok_or(AuthenticationError::from(config).into()),
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}

fn validate_token(token: &str, key: &str) -> Result<bool, jsonwebtoken::errors::Error> {
    let key = DecodingKey::from_secret(key.as_ref());
    let validation = Validation::new(Algorithm::HS256);
    decode::<Claims>(token, &key, &validation)?;
    Ok(true)
}

#[post("/auth/jwt/issue")]
async fn issue_jwt(req: web::Json<InputLogin>, pool: web::Data<Pool>, state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let user = web::block(move || users::login(&req.email, &req.password, pool))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;
    match user {
        Some(u) => {
            let key = state.secret.expose_secret();
            let key = &EncodingKey::from_secret(key.as_bytes());
            let duration = chrono::Utc::now() + chrono::Duration::days(1);
            let claims = Claims {
                sub: u.email,
                company: String::new(),
                exp: duration.timestamp().into(),
            };
            let token = encode(&Header::default(), &claims, key);
            match token {
                Ok(t) => Ok(HttpResponse::Ok().json(t)),
                Err(_) => Ok(HttpResponse::InternalServerError().body("Token creation failed"))
            }
        }
        None => Ok(HttpResponse::Unauthorized().json("Username/Password not found")),
    }
}

#[post("/auth/login")]
async fn login(req: web::Json<InputLogin>, pool: web::Data<Pool>, request: HttpRequest) -> Result<HttpResponse, Error> {
    let user = web::block(move || users::login(&req.email, &req.password, pool))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;
    match user {
        Some(u) => {
            Identity::login(&request.extensions(), "User1".into())
                .map_err(actix_web::error::ErrorInternalServerError)?;
            Ok(HttpResponse::Ok().body("Logged In"))
        }
        None => Ok(HttpResponse::Unauthorized().json("Username/Password not found")),
    }
}

#[get("/auth/status")]
async fn status(user: Option<Identity>) -> Result<HttpResponse, Error> {
    Ok(match user {
        Some(u) => HttpResponse::Ok().json(u.id().unwrap()),
        None => HttpResponse::Unauthorized().body("Not logged in")
    })
}

#[post("/auth/logout")]
async fn logout(user: Identity) -> Result<HttpResponse, Error> {
    user.logout();
    Ok(HttpResponse::Ok().body("Logged out"))
}

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(services![issue_jwt, login, status, logout]);
}
