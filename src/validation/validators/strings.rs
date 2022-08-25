pub trait IsNumeric {
    fn is_numeric(&self) -> bool;
}

impl IsNumeric for String {
    fn is_numeric(&self) -> bool {
        self.chars().all(char::is_numeric)
    }
}