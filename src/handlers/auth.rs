use actix_web::{
    get, post, services,
    web::{self, ServiceConfig},
    Error, HttpResponse,
};

use jsonwebtoken::{encode, EncodingKey, Header};
use secrecy::ExposeSecret;

use crate::{
    db::{users, DbPool},
    models::auth::{Claims, InputLogin},
    AppState,
};

#[post("/auth/login")]
async fn login(
    req: web::Json<InputLogin>,
    pool: web::Data<DbPool>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let user = users::login(&req.email, &req.password, pool)
        .await?;
    match user {
        Some(u) => {
            let key = state.secret.expose_secret();
            let key = &EncodingKey::from_secret(key.as_bytes());
            let duration = chrono::Utc::now() + chrono::Duration::days(1);
            let claims = Claims {
                sub: u.email,
                role: u.role,
                exp: duration.timestamp(),
            };
            let token = encode(&Header::default(), &claims, key);
            match token {
                Ok(t) => Ok(HttpResponse::Ok().json(t)),
                Err(_) => Ok(HttpResponse::InternalServerError().body("Token creation failed")),
            }
        }
        None => Ok(HttpResponse::Unauthorized().json("Username/Password not found")),
    }
}

#[get("/auth/status")]
async fn status(user: Option<Claims>) -> Result<HttpResponse, Error> {
    Ok(match user {
        Some(u) => HttpResponse::Ok().json(u),
        None => HttpResponse::Unauthorized().body("Not logged in"),
    })
}

#[post("/auth/logout")]
async fn logout(_user: Claims) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Logged out"))
}

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(services![login, login, status, logout]);
}
