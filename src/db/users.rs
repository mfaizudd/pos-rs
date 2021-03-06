use super::{DbError, Pool};
use crate::models::*;
use crate::schema::users::dsl;
use actix_web::web;
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use uuid::Uuid;

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
    role: Option<Role>,
    pool: web::Data<Pool>,
) -> Result<User, DbError> {
    let conn = pool.get()?;
    let password = &hash(password, DEFAULT_COST)?;
    let new_user = NewUser {
        full_name,
        email,
        password,
        role,
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
    role: Option<Role>,
    pool: web::Data<Pool>,
) -> Result<Option<User>, DbError> {
    let conn = pool.get()?;
    let user: Option<User> = dsl::users.find(uid).first::<User>(&conn).optional()?;
    let user = match user {
        Some(u) => u,
        None => return Ok(None),
    };
    let password = &hash(password, DEFAULT_COST)?;
    let updated_user = NewUser {
        full_name,
        email,
        password,
        role,
        created_at: user.created_at,
        updated_at: chrono::Local::now().naive_utc(),
    };
    let user = dsl::users.find(uid);
    let user = diesel::update(user)
        .set(&updated_user)
        .get_result::<User>(&conn)
        .optional()?;
    Ok(user)
}

pub fn delete(uid: Uuid, pool: web::Data<Pool>) -> Result<String, DbError> {
    let conn = pool.get()?;
    let user = dsl::users.find(uid);
    let num_deleted = diesel::delete(user).execute(&conn)?;
    let response = format!("Deleted {} user(s)", num_deleted);
    Ok(response)
}

pub fn login(email: &str, password: &str, pool: web::Data<Pool>) -> Result<Option<User>, DbError> {
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
