use actix_web::web::ServiceConfig;

pub mod users;

pub fn configuration(cfg: &mut ServiceConfig) {
    cfg.configure(users::routes);
}