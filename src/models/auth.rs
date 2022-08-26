use std::pin::Pin;

use actix_web::{http::header, web, FromRequest, HttpRequest};
use futures_util::Future;
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};

use crate::{
    errors::ServiceError,
    jwt::validate_token,
    validation::{Validate, ValidationError},
    AppState,
};

use super::user::Role;

#[derive(Deserialize)]
pub struct InputLogin {
    pub email: String,
    pub password: String,
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
    pub sub: String,
    pub role: Role,
    pub exp: i64,
}

impl FromRequest for Claims {
    type Error = ServiceError;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let req_headers = req.headers();
            let state = req
                .app_data::<web::Data<AppState>>()
                .expect("App state not configured properly");
            let bearer_token = req_headers
                .get(header::AUTHORIZATION)
                .ok_or(ServiceError::AuthError("Unauthorized".into()))?
                .to_str()?
                .split(' ')
                .collect::<Vec<&str>>();
            if bearer_token.len() != 2 {
                return Err(ServiceError::AuthError("Invalid bearer token".into()));
            }
            let claims = validate_token(bearer_token[1], state.secret.expose_secret())?;
            Ok(claims)
        })
    }
}
