use actix_web::{
    get, post, services,
    web::{self, ServiceConfig},
    Error, HttpResponse,
};

use jsonwebtoken::{encode, EncodingKey, Header};
use secrecy::ExposeSecret;

use crate::{
    db::{self, users, DbPool},
    errors::ServiceError,
    models::{
        auth::{Claims, InputLogin, InputRegister},
        user::Role,
    },
    AppState,
    validation::Validate
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
    let user = db::users::add(&full_name, &email, &password, Some(Role::User), db).await?;

    Ok(HttpResponse::Ok().json(user))
}

#[post("/auth/login")]
async fn login(
    req: web::Json<InputLogin>,
    pool: web::Data<DbPool>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let user = users::login(&req.email, &req.password, pool).await?;
    match user {
        Some(u) => {
            let key = state.secret.expose_secret();
            let key = &EncodingKey::from_secret(key.as_bytes());
            let duration = chrono::Utc::now() + chrono::Duration::days(3);
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
    // TODO: revoke the key
    Ok(HttpResponse::Ok().body("Logged out"))
}

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(services![login, login, status, logout, register]);
}
