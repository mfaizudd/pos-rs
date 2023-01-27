use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::validation::{validators::NotEmpty, Validate, ValidationError};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub role: Role,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
#[derive(Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub full_name: String,
    pub email: String,
    pub role: Role,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "role", rename_all = "lowercase")]
pub enum Role {
    Admin,
    User,
}

#[derive(Deserialize)]
pub struct InputUser {
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub role: Option<Role>,
}

impl UserResponse {
    pub fn from(user: &User) -> Self {
        UserResponse {
            id: user.id,
            full_name: user.full_name.to_string(),
            email: user.email.to_string(),
            role: user.role,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

impl Validate for InputUser {
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

#[cfg(test)]
mod tests {
    use crate::validation::Validate;

    use super::{InputUser, Role};

    fn generate_user() -> InputUser {
        InputUser {
            full_name: "Test User".into(),
            email: "test@test.com".into(),
            password: "12345678".into(),
            role: Some(Role::Admin),
        }
    }

    #[test]
    fn empty_full_name_should_not_pass() {
        let mut user = generate_user();
        user.full_name = "".into();
        assert_eq!(
            user.validate().err().unwrap().get_message(0),
            "Full name required"
        )
    }

    #[test]
    fn empty_email_should_not_pass() {
        let mut user = generate_user();
        user.email = "".into();
        assert_eq!(
            user.validate().err().unwrap().get_message(0),
            "Email required"
        )
    }

    #[test]
    fn invalid_password_should_not_pass() {
        let mut user = generate_user();
        user.password = "".into();
        assert_eq!(
            user.validate().err().unwrap().get_message(0),
            "Password must be at least 8 characters"
        )
    }

    #[test]
    fn valid_user_should_pass() {
        let user = generate_user();
        assert!(user.validate().is_ok())
    }
}
