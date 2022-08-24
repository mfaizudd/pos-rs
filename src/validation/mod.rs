use std::{error::Error, fmt::Display};

pub trait Validate {
    type OkResult;
    fn validate(&self) -> Result<Self::OkResult, ValidationError>;
}

#[derive(Debug)]
pub struct ValidationError {
    errors: Vec<String>,
}

impl ValidationError {
    pub fn new() -> Self {
        ValidationError { errors: vec![] }
    }

    pub fn push<F>(&mut self, err: &str, f: F)
    where
        F: FnOnce() -> bool 
    {
        if f() {
            self.add_message(err);
        }
    }

    pub fn add_message(&mut self, err: &str) {
        self.errors.push(String::from(err));
    }

    pub fn len(&self) -> usize {
        self.errors.len()
    }

    pub fn get_result<T>(self, result: T) -> Result<T, Self> {
        if self.len() > 0 {
            return Err(self)
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
        size: usize
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
                return Err(err)
            }
            Ok(())
        }
    }

    struct PushStruct {
        size: usize
    }

    impl Validate for PushStruct {
        type OkResult = ();
        fn validate(&self) -> Result<(), super::ValidationError> {
            let mut err = ValidationError::new();
            err.push("1 error", || self.size >= 1);
            err.push("2 error", || self.size >= 2);
            err.push("3 error", || self.size >= 3);
            err.get_result(())
        }
    }

    #[test]
    fn return_correct_number_of_errors() {
        let mock = AddMessageStruct { size: 2 };
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
}
