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