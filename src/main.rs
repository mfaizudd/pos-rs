use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer, http};
use dotenvy::dotenv;
use env_logger::Env;
use futures_util::join;
use secrecy::Secret;
use std::env;

use pos_rs::handlers;
use pos_rs::{db, AppState};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let pool = db::create_connection_pool();
    let redis_pool = db::create_redis_pool();
    let (pool, redis_pool) = join!(pool, redis_pool);
    let secret = env::var("SECRET").expect("Secret must be set");
    let state = AppState {
        secret: Secret::new(secret),
    };
    let port = env::var("PORT").unwrap_or(String::from("8080"));
    let port = port.parse::<u16>().expect("Invalid port");
    let allowed_origin = env::var("ALLOWED_ORIGIN").expect("Allowed origin must be set");
    sqlx::migrate!().run(&pool).await.unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&allowed_origin)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(redis_pool.clone()))
            .app_data(web::Data::new(state.clone()))
            .wrap(Logger::default())
            .wrap(cors)
            .service(web::scope("/api").configure(handlers::configuration))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
