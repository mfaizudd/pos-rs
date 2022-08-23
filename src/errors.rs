use std::error::Error;

use actix_web::{error::ResponseError, HttpResponse};
use bcrypt::BcryptError;
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError(Box<dyn Error + Sync + Send>),

    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),
    DatabaseError(sqlx::Error),
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError(err) => {
                HttpResponse::InternalServerError().json(err.to_string())
            }
            ServiceError::BadRequest(msg) => HttpResponse::BadRequest().json(msg),
            ServiceError::DatabaseError(err) => map_database_error(err),
        }
    }
}

fn map_database_error(err: &sqlx::Error) -> HttpResponse {
    match err {
        sqlx::error::Error::RowNotFound => HttpResponse::NotFound().json("Records not found"),
        other => HttpResponse::InternalServerError().json(other.to_string()),
    }
}

impl From<BcryptError> for ServiceError {
    fn from(err: BcryptError) -> Self {
        ServiceError::InternalServerError(Box::new(err))
    }
}

impl From<sqlx::Error> for ServiceError {
    fn from(err: sqlx::Error) -> Self {
        ServiceError::DatabaseError(err)
    }
}
