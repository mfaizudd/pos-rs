use actix_web::{delete, get, post, put, services, web, Error, HttpResponse};

use crate::{
    db::{self, DbPool},
    models::user::InputUser,
};

#[get("/users")]
async fn get_users(db: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let users = db::users::get_all(db)
        .await?;

    Ok(HttpResponse::Ok().json(users))
}

#[get("/users/{id}")]
async fn get_user(
    path: web::Path<uuid::Uuid>,
    db: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let uid = path.into_inner();
    let user = db::users::find(uid, db)
        .await?;

    Ok(match user {
        Some(u) => HttpResponse::Ok().json(u),
        None => HttpResponse::NotFound().body("User not found"),
    })
}

#[post("/users")]
async fn create_user(
    req: web::Json<InputUser>,
    db: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let InputUser {
        full_name,
        email,
        password,
        role,
    } = req.into_inner();
    let user = db::users::add(&full_name, &email, &password, role, db)
        .await?;

    Ok(HttpResponse::Ok().json(user))
}

#[put("/users/{id}")]
async fn update_user(
    path: web::Path<uuid::Uuid>,
    req: web::Json<InputUser>,
    db: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let uid = path.into_inner();
    let InputUser {
        full_name,
        email,
        password,
        role,
    } = req.into_inner();
    let user = db::users::update(uid, &full_name, &email, &password, role, db)
        .await?;

    Ok(HttpResponse::Ok().json(user))
}

#[delete("/users/{id}")]
async fn delete_user(
    path: web::Path<uuid::Uuid>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let uid = path.into_inner();
    let result = db::users::delete(uid, pool)
        .await?;

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
