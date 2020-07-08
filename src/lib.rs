//! A simple no-cost wrapper for the web_sys crate to get auto-completes from IDE
//!
//! # Examples
//!
//! ```
//! use web_butter;
//!
//! # fn run() {
//! println!("{}", web_butter::add(2, 3));
//! # }
//! # run();
//! ```

pub use self::error::Error;

mod error;

/// This function adds two numbers.
pub fn add(a: i32, b: i32) -> i32 {
    a + b + 10000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
}
