use std::{ops::Deref, pin::Pin};

use actix_web::{http::header, web, FromRequest, HttpRequest};
use futures_util::Future;
use redis::cmd;
use serde::{Deserialize, Serialize};

use crate::{
    db::RedisPool,
    errors::ServiceError,
    validation::{validators::NotEmpty, Validate, ValidationError},
};

use super::user::Role;

#[derive(Deserialize)]
pub struct InputLogin {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct InputRegister {
    pub full_name: String,
    pub email: String,
    pub password: String,
}

impl Validate for InputRegister {
    type OkResult = ();

    fn validate(&self) -> Result<Self::OkResult, ValidationError> {
        let mut err = ValidationError::new();
        err.push("Full name required", || self.full_name.not_empty());
        err.push("Email required", || self.email.not_empty());
        err.push("Password must be at least 8 characters", || {
            self.password.len() >= 8
        });
        err.to_result(())
    }
}

impl Validate for InputLogin {
    type OkResult = ();

    fn validate(&self) -> Result<Self::OkResult, crate::validation::ValidationError> {
        let mut err = ValidationError::new();
        err.push("Email is required", || self.email.is_empty());
        err.push("Password is required", || self.password.is_empty());
        err.to_result(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Email
    pub sub: String,
    pub role: Role,
    pub exp: i64,
}

pub struct ServiceToken(String);

pub struct AdminClaims(Claims);

async fn extract_bearer_token(req: &HttpRequest) -> Result<String, ServiceError> {
    let req_headers = req.headers();
    let bearer_token = req_headers
        .get(header::AUTHORIZATION)
        .ok_or_else(|| ServiceError::AuthError("Unauthorized".into()))?
        .to_str()?
        .split(' ')
        .collect::<Vec<&str>>();
    if bearer_token.len() != 2 {
        return Err(ServiceError::AuthError("Invalid bearer token".into()));
    }
    Ok(bearer_token[1].into())
}

async fn extract_claims(req: &HttpRequest) -> Result<Claims, ServiceError> {
    let bearer_token = extract_bearer_token(req).await?;
    let redis_pool = req
        .app_data::<web::Data<RedisPool>>()
        .ok_or("Failed to get redis connection")?;
    let mut redis_conn = redis_pool.get().await?;
    let token: String = cmd("GET")
        .arg(&[bearer_token])
        .query_async(&mut redis_conn)
        .await
        .map_err(|_| ServiceError::AuthError("Failed to get token".into()))?;
    let claims: Claims = serde_json::from_str(&token)?;
    Ok(claims)
}

impl FromRequest for Claims {
    type Error = ServiceError;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let claims = extract_claims(&req).await?;
            Ok(claims)
        })
    }
}

impl Deref for AdminClaims {
    type Target = Claims;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRequest for AdminClaims {
    type Error = ServiceError;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let claims = extract_claims(&req).await?;
            let claims = AdminClaims(claims);
            match claims.role {
                Role::Admin => Ok(claims),
                _ => Err(ServiceError::AuthError("Unauthorized".into())),
            }
        })
    }
}

impl FromRequest for ServiceToken {
    type Error = ServiceError;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let token = extract_bearer_token(&req).await?;
            Ok(ServiceToken(token))
        })
    }
}

impl Deref for ServiceToken {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
