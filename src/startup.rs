use std::{net::TcpListener, sync::Arc};

use actix_cors::Cors;
use actix_web::{dev::Server, http, middleware::Logger, web, App, HttpServer};

use crate::{
    db::{DbPool, RedisPool},
    handlers, AppState,
};

pub fn run(
    listener: TcpListener,
    pool: DbPool,
    redis_pool: RedisPool,
    state: AppState,
    allowed_origin: String,
) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
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
            .app_data(web::Data::new(Arc::new(pool.clone())))
            .app_data(web::Data::new(Arc::new(redis_pool.clone())))
            .app_data(web::Data::new(Arc::new(state.clone())))
            .wrap(Logger::default())
            .wrap(cors)
            .service(web::scope("/api").configure(handlers::configuration))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
