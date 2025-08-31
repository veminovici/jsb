//! JSB Standard Library
//!
//! This crate provides standard functionality for JSB applications.

pub mod prelude;
pub mod scale_pattern;

/// Version of the JSB standard library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the JSB standard library
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Add initialization logic
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_init() {
        assert!(init().is_ok());
    }
}
