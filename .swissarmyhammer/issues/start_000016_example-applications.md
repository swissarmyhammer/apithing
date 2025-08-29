# Example Applications

Refer to /Users/wballard/github/apithing/ideas/start.md

## Goal
Create example applications that demonstrate practical usage of the API framework.

## Tasks
- Create `examples/` directory
- Create `basic_usage.rs`:
  - Simple example showing direct `ApiOperation` usage
  - User creation, finding, and updating
  - Basic error handling
- Create `executor_pattern.rs`:
  - Demonstrate `ApiExecutor` for stateful operations
  - Multi-family API usage (users and products)
  - Show context sharing benefits
- Create `advanced_patterns.rs`:
  - Complex workflows combining multiple operations
  - Custom context implementation
  - Error recovery patterns
- Add `[[example]]` entries to Cargo.toml
- Ensure examples follow Rust example conventions

## Success Criteria
- Examples run with `cargo run --example <name>`
- Examples demonstrate key framework features
- Examples include helpful comments explaining patterns
- Examples progress from simple to advanced usage
- Examples serve as templates for real applications

## Example Structure
```rust
// examples/basic_usage.rs
use apithing::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut context = DatabaseContext::new("example_db".to_string());
    
    // Create a user
    let user = CreateUser::execute(&mut context, &CreateUserProps {
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    })?;
    
    println!("Created user: {:?}", user);
    Ok(())
}
```

Examples should be practical and demonstrate real-world usage patterns of the framework.