use std::pin::Pin;

use actix_web::{http::header, Error, FromRequest, HttpRequest};
use futures_util::Future;
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};

use crate::{jwt::validate_token, AppState};

use super::user::Role;

#[derive(Deserialize)]
pub struct InputLogin {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: Role,
    pub exp: i64,
}

impl FromRequest for Claims {
    type Error = Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let req_headers = req.headers();
            let state = req
                .app_data::<AppState>()
                .expect("App state not configured properly");
            let bearer_token = req_headers
                .get(header::AUTHORIZATION)
                .ok_or(actix_web::error::ErrorUnauthorized("Unauthorized"))?
                .to_str()
                .map_err(actix_web::error::ErrorInternalServerError)?
                .split(' ')
                .collect::<Vec<&str>>();
            if bearer_token.len() != 2 {
                return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
            }
            let claims = validate_token(bearer_token[1], state.secret.expose_secret())
                .map_err(actix_web::error::ErrorInternalServerError)?;
            Ok(claims)
        })
    }
}
