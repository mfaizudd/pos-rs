use super::DbPool;
use crate::{
    errors::ServiceError,
    models::user::{Role, User, UserResponse},
};
use actix_web::web;
use argon2::{self, Config};
use uuid::Uuid;

pub async fn get_all(pool: web::Data<DbPool>) -> Result<Vec<UserResponse>, ServiceError> {
    let mut pool = pool.acquire().await?;
    let users = sqlx::query_as!(
        UserResponse,
        r#"
    select 
        id, 
        full_name, 
        email, 
        role as "role: _", 
        created_at, 
        updated_at 
    from users"#
    )
    .fetch_all(&mut pool)
    .await?;
    Ok(users)
}

pub async fn find(
    uid: Uuid,
    pool: &web::Data<DbPool>,
) -> Result<Option<UserResponse>, ServiceError> {
    let mut pool = pool.acquire().await?;
    let user = sqlx::query_as!(
        UserResponse,
        r#"
    select 
        id, 
        full_name, 
        email, 
        role as "role: _", 
        created_at, 
        updated_at 
    from users
    where id = $1"#,
        uid
    )
    .fetch_one(&mut pool)
    .await
    .ok();
    Ok(user)
}

pub async fn find_by_email(
    email: &str,
    pool: &web::Data<DbPool>,
) -> Result<UserResponse, ServiceError> {
    let mut pool = pool.acquire().await?;
    let user = sqlx::query_as!(
        UserResponse,
        r#"
    select 
        id, 
        full_name, 
        email, 
        role as "role: _", 
        created_at, 
        updated_at 
    from users
    where email = $1"#,
        email
    )
    .fetch_one(&mut pool)
    .await?;
    Ok(user)
}

pub async fn add(
    full_name: &str,
    email: &str,
    password: &str,
    role: Option<Role>,
    pool: &web::Data<DbPool>,
) -> Result<UserResponse, ServiceError> {
    let mut pool = pool.acquire().await?;
    let salt = Uuid::new_v4();
    let config = Config::default();
    let password = argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config)?;
    let now = chrono::Local::now().naive_utc();
    let user = sqlx::query_as!(
        UserResponse,
        r#"
    insert into users(
        full_name, 
        email, 
        password, 
        role, 
        created_at, 
        updated_at 
    )
    values(
        $1, $2, $3, $4, $5, $6
    )
    returning 
        id,
        full_name, 
        email, 
        role as "role: _",
        created_at, 
        updated_at
    "#,
        full_name,
        email,
        password,
        role as _,
        now,
        now
    )
    .fetch_one(&mut pool)
    .await?;
    Ok(user)
}

pub async fn update(
    uid: Uuid,
    full_name: &str,
    email: &str,
    password: &str,
    role: Option<Role>,
    pool: web::Data<DbPool>,
) -> Result<User, ServiceError> {
    let mut pool = pool.acquire().await?;
    let salt = Uuid::new_v4();
    let config = Config::default();
    let password = argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config)?;
    let now = chrono::Local::now().naive_utc();
    println!("{uid}");
    let user = sqlx::query_as!(
        User,
        r#"
    update users set
        full_name = $1, 
        email = $2, 
        password = $3, 
        role = $4, 
        updated_at = $5
    where id = $6
    returning 
        id,
        full_name, 
        email, 
        password, 
        role as "role: _",
        created_at,
        updated_at
    "#,
        full_name,
        email,
        password,
        role as _,
        now,
        uid
    )
    .fetch_one(&mut pool)
    .await?;
    Ok(user)
}

pub async fn delete(uid: Uuid, pool: web::Data<DbPool>) -> Result<String, ServiceError> {
    let mut pool = pool.acquire().await?;
    let result = sqlx::query_as!(User, r#"delete from users where id = $1"#, uid)
        .execute(&mut pool)
        .await?;
    Ok(format!("User deleted: {}", result.rows_affected()))
}

pub async fn login(
    email: &str,
    password: &str,
    pool: web::Data<DbPool>,
) -> Result<Option<User>, ServiceError> {
    let mut pool = pool.acquire().await?;
    let user = sqlx::query_as!(
        User,
        r#"
    select 
        id, 
        full_name, 
        email, 
        password, 
        role as "role: _", 
        created_at, 
        updated_at 
    from users
    where email = $1"#,
        email
    )
    .fetch_one(&mut pool)
    .await?;
    let success = argon2::verify_encoded(&user.password, password.as_bytes())?;
    Ok(if success { Some(user) } else { None })
}
