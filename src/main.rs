use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get)
            .service(post)
            .service(json)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
