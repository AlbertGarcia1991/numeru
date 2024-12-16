use std::fmt;

pub struct ArrayStatic {
    pub capacity: u32,
}

impl fmt::Display for ArrayStatic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "capacity: {}", self.capacity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let a: ArrayStatic = ArrayStatic { capacity: 4};
        assert_eq!(a.capacity, 4);
    }
}