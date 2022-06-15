use super::Pool;
use crate::models::*;
use actix_web::web;
use diesel::prelude::*;
use std::error::Error;
use uuid::Uuid;

type DbError = Box<dyn Error + Sync + Send>;

pub fn get_all(pool: web::Data<Pool>) -> Result<Vec<User>, DbError> {
    use crate::schema::users::dsl::*;
    let conn = pool.get()?;
    let items = users.load::<User>(&conn)?;
    Ok(items)
}

pub fn find(uid: Uuid, pool: web::Data<Pool>) -> Result<Option<User>, DbError> {
    use crate::schema::users::dsl::*;
    let conn = pool.get()?;
    let user = users
        .filter(id.eq_all(uid))
        .first::<User>(&conn)
        .optional()?;

    Ok(user)
}

pub fn add(
    full_name: &str,
    email: &str,
    password: &str,
    pool: web::Data<Pool>,
) -> Result<User, DbError> {
    use crate::schema::users::dsl;
    let conn = pool.get()?;
    let new_user = NewUser {
        full_name,
        email,
        password,
        created_at: chrono::Local::now().naive_utc(),
        updated_at: chrono::Local::now().naive_utc(),
    };
    let user: User = diesel::insert_into(dsl::users)
        .values(&new_user)
        .get_result(&conn)?;
    Ok(user)
}
