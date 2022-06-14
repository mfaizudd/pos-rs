extern crate pos_rs;
extern crate diesel;

use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use serde::Serialize;

use self::pos_rs::*;
use self::models::*;
use self::diesel::prelude::*;

#[derive(Serialize)]
pub struct Book {
    name: String,
    author: Author
}

#[derive(Serialize)]
pub struct Author {
    name: String,
    age: u16
}

#[get("/json")]
async fn json() -> impl Responder {
    let person = Author { name: String::from("Eiichiro Oda"), age: 16 };
    let book = Book {
        name: String::from("One Piece"),
        author: person
    };
    HttpResponse::Ok().json(book)
}

#[get("/create_user")]
async fn create_user() -> impl Responder {
    use pos_rs::schema::users::dsl::*;
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

#[get("/get_user")]
async fn get_user() -> impl Responder {
    use pos_rs::schema::users::dsl::*;
    let connection = establish_connection();
    let results = users
        .load::<User>(&connection)
        .expect("Error loading users");
    println!("Displaying {} users", results.len());
    HttpResponse::Ok().json(results)
}

#[get("/")]
async fn get() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[post("/echo")]
async fn post(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

fn main_services(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get)
        .service(post)
        .service(json);
}

fn user_services(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_user)
        .service(create_user);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(main_services)
            .configure(user_services)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
