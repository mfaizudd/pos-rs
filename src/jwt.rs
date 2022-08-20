use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

use crate::models::auth::Claims;

pub fn validate_token(token: &str, key: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let key = DecodingKey::from_secret(key.as_ref());
    let validation = Validation::new(Algorithm::HS256);
    let claims = decode::<Claims>(token, &key, &validation)?.claims;
    Ok(claims)
}