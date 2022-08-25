use std::{error::Error, fmt::Display};

use serde::{Deserialize, Serialize};

pub trait Validate {
    type OkResult;
    fn validate(&self) -> Result<Self::OkResult, ValidationError>;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ValidationError {
    errors: Vec<String>,
}

impl ValidationError {
    pub fn new() -> Self {
        ValidationError { errors: vec![] }
    }

    pub fn push<F>(&mut self, message: &str, f: F)
    where
        F: FnOnce() -> bool,
    {
        if f() {
            self.add_message(message);
        }
    }

    pub fn add_message(&mut self, message: &str) {
        self.errors.push(String::from(message));
    }

    pub fn get_message(&self, index: usize) -> &str {
        &self.errors[index]
    }

    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// Convert the error into result, returns ok when there are no error
    pub fn to_result<T>(self, result: T) -> Result<T, Self> {
        if self.len() > 0 {
            return Err(self);
        }
        Ok(result)
    }
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let messages = self.errors.join("\n");
        write!(f, "\nerrors:\n{}\n", messages)
    }
}

impl Error for ValidationError {}

#[cfg(test)]
mod tests {
    use super::{Validate, ValidationError};

    struct AddMessageStruct {
        size: usize,
    }

    impl Validate for AddMessageStruct {
        type OkResult = ();
        fn validate(&self) -> Result<(), super::ValidationError> {
            let mut err = ValidationError::new();
            if self.size >= 1 {
                err.add_message("1 error");
            }
            if self.size >= 2 {
                err.add_message("2 error");
            }
            if self.size >= 3 {
                err.add_message("3 error");
            }
            if err.len() > 0 {
                return Err(err);
            }
            Ok(())
        }
    }

    struct PushStruct {
        size: usize,
    }

    impl Validate for PushStruct {
        type OkResult = ();
        fn validate(&self) -> Result<(), super::ValidationError> {
            let mut err = ValidationError::new();
            err.push("1 error", || self.size >= 1);
            err.push("2 error", || self.size >= 2);
            err.push("3 error", || self.size >= 3);
            err.to_result(())
        }
    }

    #[test]
    fn add_message_return_correct_number_of_errors() {
        let mock = AddMessageStruct { size: 2 };
        match mock.validate() {
            Ok(_) => panic!("Validator doesn't return any error"),
            Err(err) => assert_eq!(err.len(), 2),
        }
    }

    #[test]
    fn push_return_correct_number_of_errors() {
        let mock = PushStruct { size: 2 };
        match mock.validate() {
            Ok(_) => panic!("Validator doesn't return any error"),
            Err(err) => assert_eq!(err.len(), 2),
        }
    }

    #[test]
    fn return_ok_on_success() {
        let mock = AddMessageStruct { size: 0 };
        assert!(mock.validate().is_ok(), "Validation result wasn't ok")
    }

    #[test]
    fn push_return_ok_on_success() {
        let push_mock = PushStruct { size: 0 };
        assert!(
            push_mock.validate().is_ok(),
            "Validation result using push wasn't ok"
        )
    }
}
