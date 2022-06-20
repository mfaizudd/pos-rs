use actix_web::web::ServiceConfig;

pub mod auth;
pub mod products;
pub mod users;

pub fn configuration(cfg: &mut ServiceConfig) {
    cfg.configure(users::routes)
        .configure(auth::routes)
        .configure(products::routes);
}
