use std::{env, ops::Deref};

use actix_web::{
    get, post, services,
    web::{self, ServiceConfig},
    Error, HttpResponse,
};

use redis::AsyncCommands;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::{
    db::{self, users, DbPool, RedisPool},
    errors::ServiceError,
    models::{
        auth::{Claims, InputLogin, InputRegister, ServiceToken},
        user::Role,
    },
    validation::Validate,
};

#[post("/auth/register")]
async fn register(
    req: web::Json<InputRegister>,
    db: web::Data<DbPool>,
) -> Result<HttpResponse, ServiceError> {
    let input_user = req.into_inner();
    input_user.validate()?;
    let InputRegister {
        full_name,
        email,
        password,
    } = input_user;
    let existing_user = db::users::find_by_email(&email, &db).await.ok();
    if let Some(_) = existing_user {
        return Err(ServiceError::BadRequest("User already exists".into()));
    }
    let admin_email = env::var("ADMIN_EMAIL").ok();
    let role = admin_email.map(|e| if e == email { Role::Admin } else { Role::User });
    let user = db::users::add(&full_name, &email, &password, role, &db).await?;

    Ok(HttpResponse::Ok().json(user))
}

#[post("/auth/login")]
async fn login(
    req: web::Json<InputLogin>,
    pool: web::Data<DbPool>,
    redis_pool: web::Data<RedisPool>,
) -> Result<HttpResponse, ServiceError> {
    let mut redis_conn = redis_pool.get().await?;
    let user = users::login(&req.email, &req.password, pool).await?;
    match user {
        Some(u) => {
            let uuid = Uuid::new_v4();
            let token_key = format!("{}{}", uuid, u.email);
            let expiration = chrono::Utc::now() + chrono::Duration::days(3);
            let claims = Claims {
                sub: u.email,
                role: u.role,
                exp: expiration.timestamp(),
            };
            let token = serde_json::to_string(&claims)?;
            let mut hasher = Sha256::new();
            hasher.update(token_key);
            let token_key = format!("{:x}", hasher.finalize());
            redis_conn.set(&token_key, &token).await?;
            redis_conn
                .expire_at(&token_key, expiration.timestamp().try_into()?)
                .await?;
            Ok(HttpResponse::Ok().json(&token_key))
        }
        None => Ok(HttpResponse::Unauthorized().json("Username/Password not found")),
    }
}

#[get("/auth/status")]
async fn status(user: Option<Claims>) -> Result<HttpResponse, Error> {
    Ok(match user {
        Some(u) => HttpResponse::Ok().json(u),
        None => HttpResponse::Unauthorized().json("Not logged in"),
    })
}

#[post("/auth/logout")]
async fn logout(
    token: ServiceToken,
    redis_pool: web::Data<RedisPool>,
) -> Result<HttpResponse, ServiceError> {
    let mut redis_conn = redis_pool.get().await?;
    redis_conn.del(token.deref()).await?;
    Ok(HttpResponse::Ok().json("Logged out"))
}

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(services![login, status, logout, register]);
}
