mod env;
mod error;
mod macros;
mod model;
mod proxy;
mod schema;
mod utils;

// src/lib.rs

/// This function adds two numbers together.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// This function multiplies two numbers together.
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2, 3), 6);
    }
}
