# Integration Tests

Refer to /Users/wballard/github/apithing/ideas/start.md

## Goal
Create integration tests that demonstrate real-world usage patterns of the API framework.

## Tasks
- Create `tests/` directory for integration tests
- Create `integration_test.rs` with comprehensive scenarios:
  - Multi-operation workflows using both API families
  - Context state persistence across operations
  - Error recovery scenarios
  - Mixed usage of direct execution and executor patterns
- Create `examples_test.rs` that validates example code works
- Test performance characteristics (basic timing, not benchmarks)
- Verify memory usage patterns with multiple operations

## Success Criteria
- Integration tests pass with `cargo test`
- Tests demonstrate realistic usage scenarios
- Tests validate that examples in documentation work
- Tests show proper resource management
- Tests verify API composability

## Test Scenarios
```rust
#[test]
fn test_multi_family_workflow() {
    // Create users and products using same context
    // Verify transaction counter increments correctly
    // Test cache isolation
}

#[test]
fn test_error_recovery() {
    // Test graceful handling of various error conditions
    // Verify context state remains consistent after errors
}

#[test]
fn test_executor_patterns() {
    // Demonstrate various executor usage patterns
    // Show fluent API usage
}
```

Integration tests should focus on how different parts work together and validate the overall API design.