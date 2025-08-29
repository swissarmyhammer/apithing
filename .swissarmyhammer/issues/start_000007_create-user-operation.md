# Create User Operation

Refer to /Users/wballard/github/apithing/ideas/start.md

## Goal
Implement the `CreateUser` operation that creates new users with validation.

## Tasks
- Create `CreateUser` struct (zero-sized type)
- Implement `ApiOperation<DatabaseContext, CreateUserProps>` for `CreateUser`
- Business logic:
  - Validate email contains '@' symbol
  - Increment transaction counter
  - Generate user ID from transaction count
  - Store user in cache using format "user_{id}" -> "name:email"
  - Return created `User` struct
- Handle error cases (invalid email)
- Add comprehensive rustdoc documentation with examples

## Success Criteria
- Implementation compiles and integrates with existing traits
- Email validation works correctly
- User creation updates context state appropriately
- Cache storage follows expected format
- Error handling covers all edge cases
- Documentation includes usage examples

## Implementation Details
```rust
pub struct CreateUser;

impl ApiOperation<DatabaseContext, CreateUserProps> for CreateUser {
    type Output = User;
    type Error = UserError;

    fn execute(context: &mut DatabaseContext, props: &CreateUserProps) -> Result<Self::Output, Self::Error> {
        // Validate email
        if !props.email.contains('@') {
            return Err(UserError::InvalidEmail);
        }
        
        // Create user logic...
    }
}
```

This operation demonstrates input validation, state management, and caching within the API framework.