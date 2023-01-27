use std::{error::Error, num::TryFromIntError};

use actix_web::{error::ResponseError, http::header::ToStrError, HttpResponse};
use deadpool_redis::PoolError;
use derive_more::Display;
use redis::RedisError;

use crate::validation::ValidationError;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError(Box<dyn Error + Sync + Send>),

    #[display(fmt = "Bad Request: {_0}")]
    BadRequest(String),
    DatabaseError(sqlx::Error),
    ValidationError(ValidationError),
    AuthError(String),
}

#[derive(Debug, Display)]
struct InternalError(String);
impl Error for InternalError {}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError(err) => {
                HttpResponse::InternalServerError().json(err.to_string())
            }
            ServiceError::BadRequest(msg) => HttpResponse::BadRequest().json(msg),
            ServiceError::DatabaseError(err) => map_database_error(err),
            ServiceError::ValidationError(err) => HttpResponse::BadRequest().json(err),
            ServiceError::AuthError(msg) => HttpResponse::Unauthorized().json(msg),
        }
    }
}

fn map_database_error(err: &sqlx::Error) -> HttpResponse {
    match err {
        sqlx::error::Error::RowNotFound => HttpResponse::NotFound().json("Records not found"),
        other => HttpResponse::InternalServerError().json(other.to_string()),
    }
}

impl From<sqlx::Error> for ServiceError {
    fn from(err: sqlx::Error) -> Self {
        ServiceError::DatabaseError(err)
    }
}

impl From<ValidationError> for ServiceError {
    fn from(err: ValidationError) -> Self {
        ServiceError::ValidationError(err)
    }
}

impl From<ToStrError> for ServiceError {
    fn from(err: ToStrError) -> Self {
        ServiceError::InternalServerError(Box::new(err))
    }
}

impl From<PoolError> for ServiceError {
    fn from(err: PoolError) -> Self {
        ServiceError::InternalServerError(Box::new(err))
    }
}

impl From<RedisError> for ServiceError {
    fn from(err: RedisError) -> Self {
        ServiceError::InternalServerError(Box::new(err))
    }
}

impl From<&str> for ServiceError {
    fn from(err: &str) -> Self {
        ServiceError::InternalServerError(Box::new(InternalError(err.into())))
    }
}

impl From<TryFromIntError> for ServiceError {
    fn from(err: TryFromIntError) -> Self {
        ServiceError::InternalServerError(Box::new(err))
    }
}

impl From<serde_json::Error> for ServiceError {
    fn from(err: serde_json::Error) -> Self {
        ServiceError::InternalServerError(Box::new(err))
    }
}

impl From<argon2::Error> for ServiceError {
    fn from(err: argon2::Error) -> Self {
        ServiceError::InternalServerError(Box::new(err))
    }
}
