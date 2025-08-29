# Comprehensive Unit Tests

Refer to /Users/wballard/github/apithing/ideas/start.md

## Goal
Create comprehensive unit tests covering all API operations and error cases.

## Tasks
- Create `tests` module in lib.rs
- Test user operations:
  - `CreateUser` with valid and invalid emails
  - `FindUser` with existing and non-existent users
  - `UpdateUser` with various field combinations
  - Error cases for all operations
- Test product operations:
  - `CreateProduct` with valid and invalid prices
  - `FindProduct` with existing and non-existent products
  - Error cases for all operations
- Test context sharing:
  - Verify transaction counter increments across API families
  - Verify cache isolation between users and products
- Test ergonomic traits:
  - `Execute` trait usage
  - `ApiExecutor` usage patterns

## Success Criteria
- All tests pass with `cargo test`
- Tests cover happy path and error cases
- Tests verify context state changes
- Tests demonstrate multi-family API usage
- Code coverage is comprehensive
- Tests serve as usage examples

## Test Structure
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use user_api::*;
    use product_api::*;

    #[test]
    fn test_user_operations() {
        let mut ctx = DatabaseContext::new("test_db".to_string());
        // Test CreateUser, FindUser, UpdateUser...
    }

    #[test] 
    fn test_executor() {
        let mut executor = ApiExecutor::new(DatabaseContext::new("test_db".to_string()));
        // Test executor pattern...
    }
}
```

Tests should follow TDD principles and serve as living documentation of the API.