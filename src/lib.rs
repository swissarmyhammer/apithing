//! # ApiThing
//!
//! A standardized API approach based on content and prop traits.
//!
//! This crate provides a framework for building APIs using a trait-based approach
//! where operations are defined using shared contexts and property objects.

#![warn(missing_docs)]
#![deny(unsafe_code)]

/// Core trait that all API operations implement.
///
/// The `ApiOperation` trait provides a standardized interface for all API operations
/// within the ApiThing framework. This trait enables consistent patterns across
/// different API families while allowing for flexible context sharing and type-safe
/// operation execution.
///
/// # Design Philosophy
///
/// This trait follows the Command pattern, where each operation is encapsulated
/// as a type that implements the trait. Operations receive:
/// - A mutable context (`C`) that can be shared across operations
/// - Immutable properties (`P`) that define the operation's input parameters
///
/// # Generic Parameters
///
/// * `C` - The context type that provides shared state, resources, and services
///   that operations may need (e.g., database connections, caches, configuration)
/// * `P` - The properties type that contains the input parameters specific to
///   this operation (e.g., user ID, search criteria, update fields)
///
/// # Associated Types
///
/// * `Output` - The successful result type returned by the operation
/// * `Error` - The error type that can be returned if the operation fails
///
/// # Usage Patterns
///
/// ## Basic Usage
///
/// ```rust
/// use apithing::ApiOperation;
/// use std::collections::HashMap;
///
/// // Define a context type
/// #[derive(Debug)]
/// struct AppContext {
///     cache: HashMap<String, String>,
///     request_count: u32,
/// }
///
/// // Define properties for an operation
/// #[derive(Debug)]
/// struct GetUserProps {
///     user_id: u64,
/// }
///
/// // Define the operation result
/// #[derive(Debug)]
/// struct User {
///     id: u64,
///     name: String,
/// }
///
/// // Define an error type
/// #[derive(Debug)]
/// enum UserError {
///     NotFound,
///     DatabaseError,
/// }
///
/// // Implement the operation
/// struct GetUser;
///
/// impl ApiOperation<AppContext, GetUserProps> for GetUser {
///     type Output = User;
///     type Error = UserError;
///
///     fn execute(context: &mut AppContext, props: &GetUserProps) -> Result<Self::Output, Self::Error> {
///         context.request_count += 1;
///         
///         // Check cache first
///         if let Some(cached_name) = context.cache.get(&props.user_id.to_string()) {
///             return Ok(User {
///                 id: props.user_id,
///                 name: cached_name.clone(),
///             });
///         }
///         
///         Err(UserError::NotFound)
///     }
/// }
///
/// // Usage
/// let mut context = AppContext {
///     cache: HashMap::new(),
///     request_count: 0,
/// };
/// let props = GetUserProps { user_id: 123 };
/// let result = GetUser::execute(&mut context, &props);
/// ```
///
/// ## Composing Operations
///
/// Operations can be composed by calling other operations within their implementations:
///
/// ```rust
/// # use apithing::ApiOperation;
/// # use std::collections::HashMap;
/// # #[derive(Debug)] struct AppContext { cache: HashMap<String, String>, request_count: u32 }
/// # #[derive(Debug)] struct User { id: u64, name: String }
/// # #[derive(Debug)] enum UserError { NotFound, ValidationError }
/// # struct GetUser;
/// # impl ApiOperation<AppContext, GetUserProps> for GetUser {
/// #     type Output = User;
/// #     type Error = UserError;
/// #     fn execute(context: &mut AppContext, props: &GetUserProps) -> Result<Self::Output, Self::Error> {
/// #         Ok(User { id: props.user_id, name: "Test".to_string() })
/// #     }
/// # }
/// # #[derive(Debug)] struct GetUserProps { user_id: u64 }
/// #[derive(Debug)]
/// struct UpdateUserProps {
///     user_id: u64,
///     new_name: String,
/// }
///
/// struct UpdateUser;
///
/// impl ApiOperation<AppContext, UpdateUserProps> for UpdateUser {
///     type Output = User;
///     type Error = UserError;
///
///     fn execute(context: &mut AppContext, props: &UpdateUserProps) -> Result<Self::Output, Self::Error> {
///         // First get the existing user
///         let get_props = GetUserProps { user_id: props.user_id };
///         let mut user = GetUser::execute(context, &get_props)?;
///         
///         // Validate the new name
///         if props.new_name.is_empty() {
///             return Err(UserError::ValidationError);
///         }
///         
///         // Update the user
///         user.name = props.new_name.clone();
///         context.cache.insert(props.user_id.to_string(), props.new_name.clone());
///         
///         Ok(user)
///     }
/// }
/// ```
pub trait ApiOperation<C, P> {
    /// The type returned by a successful operation execution.
    type Output;

    /// The error type returned when an operation fails.
    type Error;

    /// Execute the API operation with the given context and properties.
    ///
    /// # Parameters
    ///
    /// * `context` - A mutable reference to the shared context that provides
    ///   resources and state that the operation may need or modify
    /// * `props` - An immutable reference to the operation-specific properties
    ///   that define the input parameters for this execution
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either the successful `Output` or an `Error`
    /// if the operation fails.
    ///
    /// # Design Notes
    ///
    /// The context is mutable to allow operations to:
    /// - Update shared caches or state
    /// - Increment counters or metrics
    /// - Manage transactions or connections
    /// - Store results for subsequent operations
    ///
    /// The properties are immutable to enforce that operations should not
    /// modify their input parameters, promoting predictable behavior.
    fn execute(context: &mut C, props: &P) -> Result<Self::Output, Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_crate_compiles() {
        // Basic test to verify the crate compiles and runs
        assert!(true);
    }

    #[test]
    fn test_documentation_is_accessible() {
        // Verify crate level documentation is accessible
        // This test ensures the lib.rs structure is valid
        assert_eq!(env!("CARGO_PKG_NAME"), "apithing");
        assert_eq!(env!("CARGO_PKG_VERSION"), "0.1.0");
    }

    #[test]
    fn test_api_operation_trait_compiles() {
        // Test types that implement the trait compile correctly
        #[derive(Debug)]
        struct TestContext {
            counter: u32,
        }

        #[derive(Debug)]
        struct TestProps {
            value: String,
        }

        #[derive(Debug, PartialEq)]
        struct TestOutput {
            result: String,
            count: u32,
        }

        #[derive(Debug, PartialEq)]
        enum TestError {
            EmptyValue,
        }

        struct TestOperation;

        impl ApiOperation<TestContext, TestProps> for TestOperation {
            type Output = TestOutput;
            type Error = TestError;

            fn execute(
                context: &mut TestContext,
                props: &TestProps,
            ) -> Result<Self::Output, Self::Error> {
                if props.value.is_empty() {
                    return Err(TestError::EmptyValue);
                }

                context.counter += 1;
                Ok(TestOutput {
                    result: props.value.clone(),
                    count: context.counter,
                })
            }
        }

        let mut context = TestContext { counter: 0 };
        let props = TestProps {
            value: "test".to_string(),
        };

        let result = TestOperation::execute(&mut context, &props);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.result, "test");
        assert_eq!(output.count, 1);
        assert_eq!(context.counter, 1);
    }

    #[test]
    fn test_api_operation_error_handling() {
        // Test that error cases work correctly
        #[derive(Debug)]
        struct TestContext;

        #[derive(Debug)]
        struct TestProps {
            should_fail: bool,
        }

        #[derive(Debug, PartialEq)]
        enum TestError {
            Failure,
        }

        struct FailingOperation;

        impl ApiOperation<TestContext, TestProps> for FailingOperation {
            type Output = ();
            type Error = TestError;

            fn execute(
                _context: &mut TestContext,
                props: &TestProps,
            ) -> Result<Self::Output, Self::Error> {
                if props.should_fail {
                    Err(TestError::Failure)
                } else {
                    Ok(())
                }
            }
        }

        let mut context = TestContext;
        let props = TestProps { should_fail: true };

        let result = FailingOperation::execute(&mut context, &props);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), TestError::Failure);
    }

    #[test]
    fn test_api_operation_with_complex_context() {
        // Test with a more complex context similar to the documentation examples
        #[derive(Debug)]
        struct AppContext {
            cache: HashMap<String, String>,
            request_count: u32,
        }

        #[derive(Debug)]
        struct CacheProps {
            key: String,
            value: String,
        }

        #[derive(Debug, PartialEq)]
        struct CacheResult {
            cached: bool,
            previous_value: Option<String>,
        }

        #[derive(Debug, PartialEq)]
        enum CacheError {
            InvalidKey,
        }

        struct CacheOperation;

        impl ApiOperation<AppContext, CacheProps> for CacheOperation {
            type Output = CacheResult;
            type Error = CacheError;

            fn execute(
                context: &mut AppContext,
                props: &CacheProps,
            ) -> Result<Self::Output, Self::Error> {
                if props.key.is_empty() {
                    return Err(CacheError::InvalidKey);
                }

                context.request_count += 1;
                let previous = context.cache.insert(props.key.clone(), props.value.clone());

                Ok(CacheResult {
                    cached: true,
                    previous_value: previous,
                })
            }
        }

        let mut context = AppContext {
            cache: HashMap::new(),
            request_count: 0,
        };
        let props = CacheProps {
            key: "test_key".to_string(),
            value: "test_value".to_string(),
        };

        let result = CacheOperation::execute(&mut context, &props);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.cached);
        assert_eq!(output.previous_value, None);
        assert_eq!(context.request_count, 1);
        assert_eq!(
            context.cache.get("test_key"),
            Some(&"test_value".to_string())
        );
    }
}
