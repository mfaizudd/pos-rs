use std::{error::Error, fmt::Display};

pub trait Validate {
    fn validate(&self) -> Result<(), ValidationError>;
}

#[derive(Debug)]
pub struct ValidationError {
    errors: Vec<String>,
}

impl ValidationError {
    pub fn new() -> Self {
        ValidationError { errors: vec![] }
    }

    pub fn push(&mut self, err: &str) {
        self.errors.push(String::from(err));
    }

    pub fn len(&self) -> usize {
        self.errors.len()
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

    struct MockStruct {
        size: usize
    }

    impl Validate for MockStruct {
        fn validate(&self) -> Result<(), super::ValidationError> {
            let mut err = ValidationError::new();
            if self.size >= 1 {
                err.push("1 error");
            }
            if self.size >= 2 {
                err.push("2 error");
            }
            if self.size >= 3 {
                err.push("3 error");
            }
            if err.len() > 0 {
                return Err(err)
            }
            Ok(())
        }
    }

    #[test]
    fn return_correct_number_of_errors() {
        let mock = MockStruct { size: 2 };
        match mock.validate() {
            Ok(_) => panic!("Validator doesn't return any error"),
            Err(err) => assert_eq!(err.len(), 2),
        }
    }

    #[test]
    fn return_ok_on_success() {
        let mock = MockStruct { size: 0 };
        assert!(mock.validate().is_ok(), "Validation result wasn't ok")
    }
}
