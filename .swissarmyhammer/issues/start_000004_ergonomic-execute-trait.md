# Ergonomic Execute Trait

Refer to /Users/wballard/github/apithing/ideas/start.md

## Goal
Add the `Execute` trait for more ergonomic usage of API operations.

## Tasks
- Define `Execute<C, P>` trait with:
  - Associated types: `Output`, `Error`
  - Method: `fn execute_on(self, context: &mut C, props: &P) -> Result<Self::Output, Self::Error>`
- Implement blanket implementation for all `ApiOperation` implementors
- Add rustdoc documentation with usage examples
- Show how this enables fluent API usage

## Success Criteria
- Trait compiles and works with existing `ApiOperation` implementations
- Documentation clearly explains the ergonomic benefits
- Code demonstrates both direct `ApiOperation::execute` and fluent `execute_on` usage

## Implementation Notes
```rust
pub trait Execute<C, P> {
    type Output;
    type Error;
    fn execute_on(self, context: &mut C, props: &P) -> Result<Self::Output, Self::Error>;
}

impl<T, C, P> Execute<C, P> for T
where
    T: ApiOperation<C, P>
{
    type Output = T::Output;
    type Error = T::Error;
    
    fn execute_on(self, context: &mut C, props: &P) -> Result<Self::Output, Self::Error> {
        T::execute(context, props)
    }
}
```

This provides a more natural method call syntax while maintaining the same underlying functionality.