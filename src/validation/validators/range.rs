use bigdecimal::{BigDecimal, FromPrimitive};

pub trait InRange<T: Ord> {
    fn in_range(&self, min: T, max: T) -> bool;
}

impl InRange<i32> for BigDecimal {
    fn in_range(&self, min: i32, max: i32) -> bool {
        let min = &BigDecimal::from_i32(min).unwrap();
        let max = &BigDecimal::from_i32(max).unwrap();
        self >= min && self <= max
    }
}

pub trait Min<T: Ord> {
    fn minimum(&self, min: T) -> bool;
}

impl<T> Min<T> for T where T: Ord {
    fn minimum(&self, min: T) -> bool {
        self >= &min
    }
}

impl Min<i32> for BigDecimal {
    fn minimum(&self, min: i32) -> bool {
        self >= &BigDecimal::from_i32(min).unwrap()
    }
}

impl<T> Min<usize> for Vec<T> {
    fn minimum(&self, min: usize) -> bool {
        self.len() >= min
    }
}
