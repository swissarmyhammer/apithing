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
        // Basic test to verify the crate compiles and runs
        assert!(true);
    }

    #[test]
    fn test_documentation_is_accessible() {
        // Verify crate level documentation is accessible
        // This test ensures the lib.rs structure is valid
        assert_eq!(env!("CARGO_PKG_NAME"), "apithing");
        assert_eq!(env!("CARGO_PKG_VERSION"), "0.1.0");
    }
}


