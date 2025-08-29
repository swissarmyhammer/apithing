# API Executor Pattern

Refer to /Users/wballard/github/apithing/ideas/start.md

## Goal
Implement the `ApiExecutor` struct for stateful, fluent API usage.

## Tasks
- Create `ApiExecutor<C>` struct that wraps a context
- Implement methods:
  - `new(context: C) -> Self`
  - `execute<P, Op>(&mut self, _op: Op, props: &P) -> Result<Op::Output, Op::Error>`
  - `context(&self) -> &C`
  - `context_mut(&mut self) -> &mut C`
- Add comprehensive rustdoc documentation
- Include usage examples in documentation

## Success Criteria
- Executor compiles and provides fluent API access
- Context can be shared across multiple operations
- Documentation shows clear usage patterns
- Integration with previous traits works seamlessly

## Implementation Notes
```rust
pub struct ApiExecutor<C> {
    context: C,
}

impl<C> ApiExecutor<C> {
    pub fn new(context: C) -> Self {
        Self { context }
    }
    
    pub fn execute<P, Op>(&mut self, _op: Op, props: &P) -> Result<Op::Output, Op::Error>
    where
        Op: ApiOperation<C, P>
    {
        Op::execute(&mut self.context, props)
    }
}
```

This pattern enables users to create an executor once and use it for multiple API calls while maintaining shared state.