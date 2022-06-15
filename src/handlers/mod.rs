use actix_web::web::ServiceConfig;

pub mod users;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.configure(users::routes);
}