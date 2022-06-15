use actix_web::web::ServiceConfig;
use actix_web::{get, post, put, delete, HttpResponse, Responder, services};

use diesel::prelude::*;
use crate::models::*;
use crate::db::*;
use crate::schema;

#[get("/users")]
async fn get_users() -> impl Responder {
    use schema::users::dsl::*;
    let connection = establish_connection();
    let results = users
        .load::<User>(&connection)
        .expect("Error loading users");
    HttpResponse::Ok().json(results)
}

#[get("/users/{id}")]
async fn get_user() -> impl Responder {
    use schema::users::dsl::*;
    let connection = establish_connection();
    let results = users
        .load::<User>(&connection)
        .expect("Error loading users");
    HttpResponse::Ok().json(results)
}

#[post("/users")]
async fn create_user() -> impl Responder {
    use schema::users::dsl::*;
    let connection = establish_connection();
    let new_user = NewUser {
        full_name: "Fafa",
        email: "mfaizudd@gmail.com",
        password: "password"
    };
    let user: User = diesel::insert_into(users)
        .values(&new_user)
        .get_result(&connection)
        .expect("Error saving new post");
    
    HttpResponse::Ok().json(user)
}

#[put("/users/{id}")]
async fn update_user() -> impl Responder {
    HttpResponse::Ok()
}

#[delete("/users/{id}")]
async fn delete_user() -> impl Responder {
    HttpResponse::Ok()
}

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(services![
        get_users,
        get_user,
        create_user,
        update_user,
        delete_user
    ]);
}