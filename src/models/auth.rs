use std::{ops::Deref, pin::Pin};

use actix_web::{http::header, web, FromRequest, HttpRequest};
use futures_util::Future;
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};

use crate::{
    errors::ServiceError,
    jwt::validate_token,
    validation::{Validate, ValidationError, validators::NotEmpty},
    AppState,
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
        err.push("Email is required", || self.email.len() <= 0);
        err.push("Password is required", || self.password.len() <= 0);
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

pub struct AdminClaims(Claims);

fn extract_token(req: &HttpRequest) -> Result<Vec<&str>, ServiceError> {
    let req_headers = req.headers();
    let bearer_token = req_headers
        .get(header::AUTHORIZATION)
        .ok_or(ServiceError::AuthError("Unauthorized".into()))?
        .to_str()?
        .split(' ')
        .collect::<Vec<&str>>();
    Ok(bearer_token)
}

fn extract_claims(token: Vec<&str>, state: &web::Data<AppState>) -> Result<Claims, ServiceError> {
    if token.len() != 2 {
        return Err(ServiceError::AuthError("Invalid bearer token".into()));
    }
    let claims = validate_token(token[1], state.secret.expose_secret())?;
    Ok(claims)
}

fn get_state(req: &HttpRequest) -> &web::Data<AppState> {
    req
        .app_data::<web::Data<AppState>>()
        .expect("App state not configured properly")
}

impl FromRequest for Claims {
    type Error = ServiceError;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let state = get_state(&req);
            let bearer_token = extract_token(&req)?;
            extract_claims(bearer_token, state)
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
            let state = get_state(&req);
            let bearer_token = extract_token(&req)?;
            let claims = extract_claims(bearer_token, state)?;
            let claims = AdminClaims(claims);
            match claims.role {
                Role::Admin => Ok(claims),
                _ => Err(ServiceError::AuthError("Unauthorized".into()))
            }
        })
    }
}
