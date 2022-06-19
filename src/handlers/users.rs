use crate::{db::*, handlers::auth};
use actix_web::{delete, get, post, put, services, web, Error, HttpResponse};
use actix_web_httpauth::middleware::HttpAuthentication;
use serde::Deserialize;

#[derive(Deserialize)]
struct InputUser {
    full_name: String,
    email: String,
    password: String,
}

#[get("/users", wrap = "HttpAuthentication::bearer(auth::validator)")]
async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let users = web::block(move || users::get_all(db))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(users))
}

#[get("/users/{id}", wrap = "HttpAuthentication::bearer(auth::validator)")]
async fn get_user(path: web::Path<uuid::Uuid>, db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let uid = path.into_inner();
    let user = web::block(move || users::find(uid, db))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(match user {
        Some(u) => HttpResponse::Ok().json(u),
        None => HttpResponse::NotFound().body("User not found"),
    })
}

#[post("/users")]
async fn create_user(
    req: web::Json<InputUser>,
    db: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || users::add(&req.full_name, &req.email, &req.password, db))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

#[put("/users/{id}", wrap = "HttpAuthentication::bearer(auth::validator)")]
async fn update_user(
    path: web::Path<uuid::Uuid>,
    req: web::Json<InputUser>,
    db: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let uid = path.into_inner();
    let user =
        web::block(move || users::update(uid, &req.full_name, &req.email, &req.password, db))
            .await?
            .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(match user {
        Some(u) => HttpResponse::Ok().json(u),
        None => HttpResponse::NotFound().body("User not found"),
    })
}

#[delete("/users/{id}", wrap = "HttpAuthentication::bearer(auth::validator)")]
async fn delete_user(
    path: web::Path<uuid::Uuid>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let uid = path.into_inner();
    let result = web::block(move || users::delete(uid, pool))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(services![
        get_users,
        get_user,
        create_user,
        update_user,
        delete_user
    ]);
}
