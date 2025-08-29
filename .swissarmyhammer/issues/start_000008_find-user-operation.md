# Find User Operation

Refer to /Users/wballard/github/apithing/ideas/start.md

## Goal
Implement the `FindUser` operation that retrieves users from cache.

## Tasks
- Create `FindUser` struct (zero-sized type)
- Implement `ApiOperation<DatabaseContext, FindUserProps>` for `FindUser`
- Business logic:
  - Check cache for user using key format "user_{id}"
  - Parse cached data format "name:email"
  - Construct and return `User` struct
  - Return `UserError::NotFound` if not in cache or parse fails
- Add comprehensive rustdoc documentation
- Include error handling examples

## Success Criteria
- Implementation integrates with existing cache format from `CreateUser`
- Cache lookup works correctly
- Parsing handles malformed cache entries gracefully
- Error cases return appropriate `UserError::NotFound`
- Documentation explains cache interaction patterns

## Implementation Details
```rust
pub struct FindUser;

impl ApiOperation<DatabaseContext, FindUserProps> for FindUser {
    type Output = User;
    type Error = UserError;

    fn execute(context: &mut DatabaseContext, props: &FindUserProps) -> Result<Self::Output, Self::Error> {
        if let Some(cached) = context.cache.get(&format!("user_{}", props.user_id)) {
            let parts: Vec<&str> = cached.split(':').collect();
            if parts.len() == 2 {
                return Ok(User {
                    id: props.user_id,
                    name: parts[0].to_string(),
                    email: parts[1].to_string(),
                });
            }
        }
        Err(UserError::NotFound)
    }
}
```

This operation demonstrates cache-based data retrieval with proper error handling.