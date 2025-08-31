//! # ApiThing
//!
//! A standardized API approach based on content and parameter traits.
//!
//! This crate provides a framework for building APIs using a trait-based approach
//! where operations are defined using shared contexts and parameter objects.
//! The framework enables consistent patterns across different API families while
//! allowing for flexible context sharing and type-safe operation execution.
//!
//! ## Quick Start
//!
//! ```rust
//! use apithing::{ApiOperation, ApiExecutor};
//!
//! // Define your operation parameters
//! #[derive(Debug, Clone)]
//! struct CreateEntityParameters {
//!     name: String,
//!     data: String,
//! }
//!
//! // Define your data types
//! #[derive(Debug, Clone)]
//! struct Entity {
//!     id: u64,
//!     name: String,
//!     data: String,
//! }
//!
//! // Define your errors
//! #[derive(Debug)]
//! enum EntityError {
//!     ValidationFailed,
//! }
//!
//! // Implement your operation
//! struct CreateEntity;
//! // Define a custom context for your application
//! #[derive(Debug)]
//! struct MyAppContext {
//!     connection: String,
//!     counter: u64,
//! }
//!
//! impl MyAppContext {
//!     fn new(connection: String) -> Self {
//!         Self { connection, counter: 0 }
//!     }
//!     fn next_id(&mut self) -> u64 {
//!         self.counter += 1;
//!         self.counter
//!     }
//! }
//!
//! impl ApiOperation<MyAppContext, CreateEntityParameters> for CreateEntity {
//!     type Output = Entity;
//!     type Error = EntityError;
//!
//!     fn execute(context: &mut MyAppContext, parameters: &CreateEntityParameters) -> Result<Entity, EntityError> {
//!         if parameters.name.is_empty() {
//!             return Err(EntityError::ValidationFailed);
//!         }
//!
//!         let entity = Entity {
//!             id: context.next_id(),
//!             name: parameters.name.clone(),
//!             data: parameters.data.clone(),
//!         };
//!         Ok(entity)
//!     }
//! }
//!
//! // Usage
//! let mut context = MyAppContext::new("db".to_string());
//! let parameters = CreateEntityParameters {
//!     name: "Example".to_string(),
//!     data: "example@data.com".to_string(),
//! };
//! let entity = CreateEntity::execute(&mut context, &parameters).unwrap();
//! ```
//!
//! ## Core Architecture
//!
//! The ApiThing framework is built around several key concepts:
//!
//! ```mermaid
//! graph TB
//!     Context["Context (C)<br/>• Shared state<br/>• Resources<br/>• Connections"]
//!     Operation["ApiOperation&lt;C,P&gt;<br/>fn execute()<br/>→ Output<br/>→ Error"]
//!     Parameters["Parameters (P)<br/>• Input params<br/>• Validation<br/>• Type safety"]
//!
//!     Execute["Execute&lt;C,P&gt;<br/>execute_on()<br/>(ergonomic API)"]
//!     Props["Operation Parameters<br/>Entity Props<br/>Domain Props<br/>..."]
//!
//!     ApiExecutor["ApiExecutor&lt;C&gt;<br/>• Stateful<br/>• Context mgmt<br/>• Multi-ops"]
//!
//!     Context --> Operation
//!     Parameters --> Operation
//!     Operation --> Execute
//!     Parameters --> Props
//!     Execute --> ApiExecutor
//! ```
//!
//!
//! This design enables:
//! - **Context sharing**: Operations across families share resources efficiently
//! - **Type safety**: Each family has its own types but shares infrastructure
//! - **Composability**: Operations can call operations from other families
//! - **Consistency**: All families follow the same patterns and conventions

#![warn(missing_docs)]
#![deny(unsafe_code)]

/// Core trait that all API operations implement.
pub trait ApiOperation<C, P> {
    /// The type returned by a successful operation execution.
    type Output;

    /// The error type returned when an operation fails.
    type Error;

    /// Execute the API operation with the given context and properties.
    fn execute(context: &mut C, parameters: &P) -> Result<Self::Output, Self::Error>;
}

/// A trait providing ergonomic method-style execution for API operations.
pub trait Execute<C, P> {
    /// The type returned by a successful operation execution.
    type Output;

    /// The error type returned when an operation fails.
    type Error;

    /// Execute the API operation on the given context with the specified properties.
    fn execute_on(self, context: &mut C, parameters: &P) -> Result<Self::Output, Self::Error>;
}

/// Blanket implementation of `Execute` for all `ApiOperation` implementors.
impl<T, C, P> Execute<C, P> for T
where
    T: ApiOperation<C, P>,
{
    type Output = T::Output;
    type Error = T::Error;

    fn execute_on(self, context: &mut C, parameters: &P) -> Result<Self::Output, Self::Error> {
        T::execute(context, parameters)
    }
}

/// A stateful executor for API operations that maintains context across multiple calls.
#[derive(Debug, Clone)]
pub struct ApiExecutor<C> {
    /// The context instance owned by this executor.
    context: C,
}

impl<C> ApiExecutor<C> {
    /// Creates a new `ApiExecutor` that owns the provided context.
    pub fn new(context: C) -> Self {
        Self { context }
    }

    /// Executes an API operation using this executor's context.
    pub fn execute<P, Op>(&mut self, _op: Op, parameters: &P) -> Result<Op::Output, Op::Error>
    where
        Op: ApiOperation<C, P>,
    {
        Op::execute(&mut self.context, parameters)
    }

    /// Returns an immutable reference to the executor's context.
    pub fn context(&self) -> &C {
        &self.context
    }

    /// Returns a mutable reference to the executor's context.
    pub fn context_mut(&mut self) -> &mut C {
        &mut self.context
    }
}

#[cfg(test)]
/// Testing utilities and example implementations for the ApiThing framework.
///
/// This module contains test-only utilities including `DatabaseContext`, which serves as
/// an example context implementation for testing and demonstrating framework patterns.
/// These utilities are not part of the public API and should not be used in production code.
///
/// The `DatabaseContext` struct demonstrates how to implement a shared context that can
/// be used across multiple API operation families while maintaining state and caching.
mod tests {
    use super::*;

    /// A database context implementation used for testing the framework.
    /// This demonstrates shared context usage across API families but is not part of the public API.
    #[derive(Debug, Clone)]
    pub struct DatabaseContext {
        /// Connection pool identifier (simplified for demonstration).
        connection_pool: String,

        /// Counter tracking the number of transactions executed.
        transaction_count: u32,

        /// General-purpose cache for storing operation results.
        cache: std::collections::HashMap<String, String>,
    }

    impl DatabaseContext {
        /// Creates a new `DatabaseContext` with the specified connection string.
        pub fn new(connection: String) -> Self {
            Self {
                connection_pool: connection,
                transaction_count: 0,
                cache: std::collections::HashMap::new(),
            }
        }

        /// Increments the transaction counter by 1.
        pub fn increment_transaction(&mut self) {
            self.transaction_count += 1;
        }

        /// Returns the current transaction count.
        pub fn transaction_count(&self) -> u32 {
            self.transaction_count
        }

        /// Returns an immutable reference to the connection pool identifier.
        pub fn connection_pool(&self) -> &str {
            &self.connection_pool
        }

        /// Returns an immutable reference to the cache.
        pub fn cache(&self) -> &std::collections::HashMap<String, String> {
            &self.cache
        }

        /// Returns a mutable reference to the cache.
        pub fn cache_mut(&mut self) -> &mut std::collections::HashMap<String, String> {
            &mut self.cache
        }
    }

    #[test]
    fn test_crate_compiles() {
        // Basic test to verify the crate compiles and runs
        // If this test runs, the crate compiled successfully
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
                parameters: &TestProps,
            ) -> Result<TestOutput, TestError> {
                if parameters.value.is_empty() {
                    return Err(TestError::EmptyValue);
                }
                context.counter += 1;
                Ok(TestOutput {
                    result: parameters.value.clone(),
                    count: context.counter,
                })
            }
        }

        // Test direct execution
        let mut context = TestContext { counter: 0 };
        let parameters = TestProps {
            value: "test".to_string(),
        };
        let result = TestOperation::execute(&mut context, &parameters).unwrap();
        assert_eq!(result.result, "test");
        assert_eq!(result.count, 1);
        assert_eq!(context.counter, 1);
    }

    #[test]
    fn test_execute_trait() {
        #[derive(Debug)]
        struct SimpleContext {
            data: String,
        }

        #[derive(Debug)]
        struct SimpleProps {
            input: String,
        }

        struct SimpleOperation;

        impl ApiOperation<SimpleContext, SimpleProps> for SimpleOperation {
            type Output = String;
            type Error = ();

            fn execute(
                context: &mut SimpleContext,
                parameters: &SimpleProps,
            ) -> Result<String, ()> {
                context.data = parameters.input.clone();
                Ok(format!("Processed: {}", parameters.input))
            }
        }

        let mut context = SimpleContext {
            data: String::new(),
        };
        let parameters = SimpleProps {
            input: "test input".to_string(),
        };

        // Test the Execute trait method
        let result = SimpleOperation
            .execute_on(&mut context, &parameters)
            .unwrap();
        assert_eq!(result, "Processed: test input");
        assert_eq!(context.data, "test input");
    }

    #[test]
    fn test_database_context() {
        let mut context = DatabaseContext::new("test_connection".to_string());

        // Test initial state
        assert_eq!(context.connection_pool(), "test_connection");
        assert_eq!(context.transaction_count(), 0);
        assert!(context.cache().is_empty());

        // Test transaction increment
        context.increment_transaction();
        assert_eq!(context.transaction_count(), 1);

        // Test cache operations
        context
            .cache_mut()
            .insert("key1".to_string(), "value1".to_string());
        assert_eq!(context.cache().len(), 1);
        assert_eq!(context.cache().get("key1"), Some(&"value1".to_string()));
    }

    #[test]
    fn test_api_executor() {
        #[derive(Debug)]
        struct CounterProps {
            increment: u32,
        }

        struct IncrementOperation;

        impl ApiOperation<DatabaseContext, CounterProps> for IncrementOperation {
            type Output = u32;
            type Error = ();

            fn execute(
                context: &mut DatabaseContext,
                parameters: &CounterProps,
            ) -> Result<u32, ()> {
                for _ in 0..parameters.increment {
                    context.increment_transaction();
                }
                Ok(context.transaction_count())
            }
        }

        let mut executor = ApiExecutor::new(DatabaseContext::new("test".to_string()));

        // Test initial state
        assert_eq!(executor.context().transaction_count(), 0);

        // Execute operation
        let parameters = CounterProps { increment: 3 };
        let result = executor.execute(IncrementOperation, &parameters).unwrap();
        assert_eq!(result, 3);
        assert_eq!(executor.context().transaction_count(), 3);

        // Execute another operation on same context
        let parameters2 = CounterProps { increment: 2 };
        let result2 = executor.execute(IncrementOperation, &parameters2).unwrap();
        assert_eq!(result2, 5);
        assert_eq!(executor.context().transaction_count(), 5);
    }

    #[test]
    fn test_examples_compile() {
        // This test ensures that the examples can be compiled and their main functions work
        // We test the core functionality without running the actual main() functions

        // Test basic_usage example concepts
        use std::collections::HashMap;

        #[derive(Debug)]
        struct ExampleAppContext {
            transaction_count: u32,
            cache: HashMap<String, String>,
        }

        impl ExampleAppContext {
            fn new(_connection: String) -> Self {
                Self {
                    transaction_count: 0,
                    cache: HashMap::new(),
                }
            }

            fn increment_transaction(&mut self) {
                self.transaction_count += 1;
            }

            fn transaction_count(&self) -> u32 {
                self.transaction_count
            }

            fn cache_mut(&mut self) -> &mut HashMap<String, String> {
                &mut self.cache
            }
        }

        #[derive(Debug, Clone)]
        struct ExampleCreateUserProps {
            name: String,
            email: String,
        }

        #[derive(Debug, Clone)]
        struct ExampleUser {
            id: u64,
            name: String,
            email: String,
        }

        #[derive(Debug)]
        enum ExampleUserError {
            InvalidEmail,
        }

        struct ExampleCreateUser;

        impl ApiOperation<ExampleAppContext, ExampleCreateUserProps> for ExampleCreateUser {
            type Output = ExampleUser;
            type Error = ExampleUserError;

            fn execute(
                context: &mut ExampleAppContext,
                parameters: &ExampleCreateUserProps,
            ) -> Result<ExampleUser, ExampleUserError> {
                if !parameters.email.contains('@') {
                    return Err(ExampleUserError::InvalidEmail);
                }

                context.increment_transaction();
                let user = ExampleUser {
                    id: context.transaction_count() as u64,
                    name: parameters.name.clone(),
                    email: parameters.email.clone(),
                };

                let cache_key = format!("user_{}", user.id);
                let cache_value = format!("{}:{}", user.name, user.email);
                context.cache_mut().insert(cache_key, cache_value);

                Ok(user)
            }
        }

        // Test that the example pattern works
        let mut context = ExampleAppContext::new("test_db".to_string());
        let parameters = ExampleCreateUserProps {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        };

        let result = ExampleCreateUser::execute(&mut context, &parameters);
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.name, "Test User");
        assert_eq!(user.email, "test@example.com");
        assert_eq!(context.transaction_count(), 1);
    }

    #[test]
    fn test_executor_pattern_example() {
        // Test that ApiExecutor works with custom contexts like in executor_pattern example
        use std::collections::HashMap;

        #[derive(Debug)]
        struct ExecutorExampleContext {
            transaction_count: u32,
            cache: HashMap<String, String>,
        }

        impl ExecutorExampleContext {
            fn new(_connection: String) -> Self {
                Self {
                    transaction_count: 0,
                    cache: HashMap::new(),
                }
            }

            fn increment_transaction(&mut self) {
                self.transaction_count += 1;
            }

            fn transaction_count(&self) -> u32 {
                self.transaction_count
            }

            fn cache_mut(&mut self) -> &mut HashMap<String, String> {
                &mut self.cache
            }
        }

        #[derive(Debug, Clone)]
        struct ExecutorCreateUserProps {
            name: String,
            email: String,
        }

        #[derive(Debug, Clone)]
        struct ExecutorUser {
            id: u64,
            name: String,
            email: String,
        }

        #[derive(Debug)]
        enum ExecutorUserError {
            InvalidEmail,
        }

        struct ExecutorCreateUser;

        impl ApiOperation<ExecutorExampleContext, ExecutorCreateUserProps> for ExecutorCreateUser {
            type Output = ExecutorUser;
            type Error = ExecutorUserError;

            fn execute(
                context: &mut ExecutorExampleContext,
                parameters: &ExecutorCreateUserProps,
            ) -> Result<ExecutorUser, ExecutorUserError> {
                if !parameters.email.contains('@') {
                    return Err(ExecutorUserError::InvalidEmail);
                }

                context.increment_transaction();
                let user = ExecutorUser {
                    id: context.transaction_count() as u64,
                    name: parameters.name.clone(),
                    email: parameters.email.clone(),
                };

                let cache_key = format!("user_{}", user.id);
                let cache_value = format!("{}:{}", user.name, user.email);
                context.cache_mut().insert(cache_key, cache_value);

                Ok(user)
            }
        }

        // Test ApiExecutor with custom context
        let mut executor =
            ApiExecutor::new(ExecutorExampleContext::new("executor_test_db".to_string()));

        let parameters = ExecutorCreateUserProps {
            name: "Executor User".to_string(),
            email: "executor@example.com".to_string(),
        };

        let result = executor.execute(ExecutorCreateUser, &parameters);
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.name, "Executor User");
        assert_eq!(user.email, "executor@example.com");
        assert_eq!(executor.context().transaction_count(), 1);
    }

    #[test]
    fn test_context_sharing() {
        #[derive(Debug)]
        struct StoreProps {
            key: String,
            value: String,
        }

        #[derive(Debug)]
        struct RetrieveProps {
            key: String,
        }

        struct StoreOperation;
        struct RetrieveOperation;

        impl ApiOperation<DatabaseContext, StoreProps> for StoreOperation {
            type Output = ();
            type Error = ();

            fn execute(context: &mut DatabaseContext, parameters: &StoreProps) -> Result<(), ()> {
                context
                    .cache_mut()
                    .insert(parameters.key.clone(), parameters.value.clone());
                context.increment_transaction();
                Ok(())
            }
        }

        impl ApiOperation<DatabaseContext, RetrieveProps> for RetrieveOperation {
            type Output = Option<String>;
            type Error = ();

            fn execute(
                context: &mut DatabaseContext,
                parameters: &RetrieveProps,
            ) -> Result<Option<String>, ()> {
                Ok(context.cache().get(&parameters.key).cloned())
            }
        }

        let mut executor = ApiExecutor::new(DatabaseContext::new("shared".to_string()));

        // Store data
        let store_parameters = StoreProps {
            key: "test_key".to_string(),
            value: "test_value".to_string(),
        };
        executor.execute(StoreOperation, &store_parameters).unwrap();

        // Retrieve data using shared context
        let retrieve_parameters = RetrieveProps {
            key: "test_key".to_string(),
        };
        let retrieved = executor
            .execute(RetrieveOperation, &retrieve_parameters)
            .unwrap();
        assert_eq!(retrieved, Some("test_value".to_string()));
        assert_eq!(executor.context().transaction_count(), 1);
    }
}
