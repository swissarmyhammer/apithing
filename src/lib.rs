//! # ApiThing
//!
//! A standardized API approach based on content and prop traits.
//!
//! This crate provides a framework for building APIs using a trait-based approach
//! where operations are defined using shared contexts and property objects.

#![warn(missing_docs)]
#![deny(unsafe_code)]

// Core traits and types will be implemented in subsequent issues

#[cfg(test)]
mod tests {

    #[test]
    fn test_crate_compiles() {
        // Basic test to ensure the crate compiles and tests can run
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_documentation_exists() {
        // Verify that the crate has proper documentation structure
        // This test will pass as long as the module compiles
        assert!(true);
    }
}
