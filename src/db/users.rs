use actix_web::web;

use super::Pool;

pub fn load_all(pool: web::Data<Pool>) {
    let conn = pool.get().unwrap();
    todo!("fetch users")
}