# Core Trait Definitions

Refer to /Users/wballard/github/apithing/ideas/start.md

## Goal
Implement the core `ApiOperation` trait that all API operations will implement.

## Tasks
- Define the `ApiOperation<C, P>` trait with:
  - Associated types: `Output`, `Error`
  - Method: `fn execute(context: &mut C, props: &P) -> Result<Self::Output, Self::Error>`
- Add comprehensive rustdoc documentation explaining:
  - The trait's purpose and design philosophy
  - Generic parameters `C` (context) and `P` (props)
  - Usage patterns and examples
- Create module structure in lib.rs

## Success Criteria
- Trait compiles without errors
- Documentation is clear and comprehensive
- `cargo doc` generates proper documentation

## Implementation Notes
```rust
pub trait ApiOperation<C, P> {
    type Output;
    type Error;
    fn execute(context: &mut C, props: &P) -> Result<Self::Output, Self::Error>;
}
```

This trait forms the foundation for all API operations, enabling consistent patterns across different API families.

## Proposed Solution

Based on the ideas/start.md reference and current project structure, I will implement the core `ApiOperation` trait in a phased approach:

### Implementation Steps:
1. Create the core module structure in src/lib.rs
2. Define the `ApiOperation<C, P>` trait with comprehensive documentation
3. Add associated types `Output` and `Error`
4. Implement the `execute` method signature
5. Add comprehensive rustdoc with examples and design philosophy
6. Write basic compilation tests
7. Generate and verify documentation with `cargo doc`

### Technical Details:
- The trait will be generic over Context (C) and Props (P) types
- Using associated types for Output and Error to maintain type safety
- The execute method will take `&mut C` for context mutation and `&P` for immutable props
- Documentation will explain the trait's role as foundation for all API operations
- Will include usage examples showing the pattern with different context/prop combinations

### Success Verification:
- `cargo build` compiles without errors
- `cargo test` passes all tests
- `cargo doc` generates proper documentation
- Trait design matches the pattern shown in ideas/start.md
## Implementation Complete

Successfully implemented the core `ApiOperation` trait with comprehensive documentation and testing.

### What Was Implemented:
1. **Core Trait Definition**: `ApiOperation<C, P>` trait with generic parameters for Context and Properties
2. **Associated Types**: `Output` and `Error` types for type-safe operation results
3. **Execute Method**: `fn execute(context: &mut C, props: &P) -> Result<Self::Output, Self::Error>`

### Documentation Features:
- Comprehensive rustdoc with design philosophy explanation
- Clear description of generic parameters and their roles
- Two detailed usage examples showing basic and composition patterns
- Inline code examples that compile and run as doctests

### Testing Coverage:
- Basic compilation test to verify trait implementation works
- Error handling test to verify Result pattern functions correctly
- Complex context test with HashMap cache to demonstrate real-world usage
- All doctests pass, verifying documentation examples work

### Verification Results:
- ✅ `cargo build` compiles without errors
- ✅ `cargo test` passes all 5 unit tests + 2 doctests
- ✅ `cargo doc --no-deps` generates proper documentation at target/doc/apithing/index.html

### Design Decisions:
- Used mutable context reference to allow state modification and resource sharing
- Used immutable props reference to enforce input parameter immutability
- Followed Rust Result pattern for error handling
- Comprehensive examples showing practical usage patterns matching ideas/start.md

The trait is now ready to serve as the foundation for all API operations in the ApiThing framework.