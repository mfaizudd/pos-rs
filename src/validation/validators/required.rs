pub trait NotEmpty {
    fn not_empty(&self) -> bool;
}

pub trait Required {
    fn required(&self) -> bool;
}

impl NotEmpty for Option<String> {
    /// Checks for empty string
    fn not_empty(&self) -> bool {
        if self.is_none() {
            return true;
        }
        let the_string = self.as_ref().unwrap();
        !the_string.is_empty()
    }
}

impl NotEmpty for String {
    fn not_empty(&self) -> bool {
        !self.is_empty()
    }
}

impl<T> Required for Option<T> {
    fn required(&self) -> bool {
        self.is_some()
    }
}
