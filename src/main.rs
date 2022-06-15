extern crate pos_rs;
extern crate diesel;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use env_logger::Env;
use pos_rs::handlers;
use pos_rs::db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let pool = db::create_connection_pool();
    
    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .wrap(Logger::default())
            .configure(handlers::configuration)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
