# Update User Operation

Refer to /Users/wballard/github/apithing/ideas/start.md

## Goal
Implement the `UpdateUser` operation that modifies existing users.

## Tasks
- Create `UpdateUser` struct (zero-sized type)
- Implement `ApiOperation<DatabaseContext, UpdateUserProps>` for `UpdateUser`
- Business logic:
  - Use `FindUser::execute` to retrieve existing user
  - Apply optional updates (name and/or email)
  - Validate email if provided (contains '@')
  - Update cache with new values
  - Increment transaction counter
  - Return updated `User` struct
- Handle all error cases (user not found, invalid email)
- Add comprehensive rustdoc documentation

## Success Criteria
- Implementation reuses existing `FindUser` operation
- Optional field updates work correctly
- Email validation applies only when email is updated
- Cache is updated with new values
- Transaction counter increments on successful update
- Error propagation works correctly

## Implementation Details
```rust
pub struct UpdateUser;

impl ApiOperation<DatabaseContext, UpdateUserProps> for UpdateUser {
    type Output = User;
    type Error = UserError;

    fn execute(context: &mut DatabaseContext, props: &UpdateUserProps) -> Result<Self::Output, Self::Error> {
        // Find existing user first
        let find_props = FindUserProps { user_id: props.user_id };
        let mut user = FindUser::execute(context, &find_props)?;
        
        // Apply updates...
        // Validate email if provided...
        // Update cache and increment transaction...
    }
}
```

This demonstrates composition of operations and partial updates with validation.