use actix_web::web::ServiceConfig;

use super::{users, auth, products, transactions};

pub fn configuration(cfg: &mut ServiceConfig) {
    cfg.configure(users::routes)
        .configure(auth::routes)
        .configure(products::routes)
        .configure(transactions::routes);
}
