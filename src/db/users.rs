use super::Pool;
use crate::models::*;
use actix_web::web;
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use std::error::Error;
use uuid::Uuid;
use crate::schema::users::dsl;

type DbError = Box<dyn Error + Sync + Send>;

pub fn get_all(pool: web::Data<Pool>) -> Result<Vec<User>, DbError> {
    use dsl::*;
    let conn = pool.get()?;
    let items = users.load::<User>(&conn)?;
    Ok(items)
}

pub fn find(uid: Uuid, pool: web::Data<Pool>) -> Result<Option<User>, DbError> {
    use dsl::*;
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
    let conn = pool.get()?;
    let password = &hash(password, DEFAULT_COST)?;
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

pub fn update(
    uid: Uuid,
    full_name: &str,
    email: &str,
    password: &str,
    pool: web::Data<Pool>,
) -> Result<Option<User>, DbError> {
    let conn = pool.get()?;
    let password = &hash(password, DEFAULT_COST)?;
    let updated_user = NewUser {
        full_name,
        email,
        password,
        created_at: chrono::Local::now().naive_utc(),
        updated_at: chrono::Local::now().naive_utc(),
    };
    let user = diesel::update(dsl::users.filter(dsl::id.eq_all(uid)))
        .set(&updated_user)
        .get_result::<User>(&conn)
        .optional()?;
    Ok(user)
}

pub fn delete(uid: Uuid, pool: web::Data<Pool>) -> Result<String, DbError> {
    let conn = pool.get()?;
    let num_deleted = diesel::delete(dsl::users.filter(dsl::id.eq_all(uid)))
        .execute(&conn)?;
    let data = format!("Deleted {} user(s)", num_deleted);
    Ok(data)
}

pub fn login(
    email: &str,
    password: &str,
    pool: web::Data<Pool>
) -> Result<Option<User>, DbError> {
    let conn = pool.get()?;
    let user: Option<User> = dsl::users
        .filter(dsl::email.eq_all(email))
        .first::<User>(&conn)
        .optional()?;
    let user = match user {
        Some(u) => verify(password, &u.password)?.then(|| u),
        None => None,
    };
    Ok(user)
}