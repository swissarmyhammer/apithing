Make a new crate that provides a standardized API approach based on content and prop traits.


This code captures the idea with an example implementation. Create examples in the standard rust way.

Create full rust docs, github action ci, and a readme.


```
use std::collections::HashMap;
use std::marker::PhantomData;

// Core trait that all API operations implement
pub trait ApiOperation<C, P> {
    type Output;
    type Error;

    fn execute(context: &mut C, props: &P) -> Result<Self::Output, Self::Error>;
}

// Shared context that can be used across multiple API families
#[derive(Debug)]
pub struct DatabaseContext {
    pub connection_pool: String, // Simplified for demo
    pub transaction_count: u32,
    pub cache: HashMap<String, String>,
}

impl DatabaseContext {
    pub fn new(connection: String) -> Self {
        Self {
            connection_pool: connection,
            transaction_count: 0,
            cache: HashMap::new(),
        }
    }

    pub fn increment_transaction(&mut self) {
        self.transaction_count += 1;
    }
}

// Example API family: User operations
pub mod user_api {
    use super::*;

    // Props for creating a user
    #[derive(Debug, Clone)]
    pub struct CreateUserProps {
        pub name: String,
        pub email: String,
    }

    // Props for finding a user
    #[derive(Debug, Clone)]
    pub struct FindUserProps {
        pub user_id: u64,
    }

    // Props for updating a user
    #[derive(Debug, Clone)]
    pub struct UpdateUserProps {
        pub user_id: u64,
        pub name: Option<String>,
        pub email: Option<String>,
    }

    #[derive(Debug, Clone)]
    pub struct User {
        pub id: u64,
        pub name: String,
        pub email: String,
    }

    #[derive(Debug)]
    pub enum UserError {
        NotFound,
        InvalidEmail,
        DatabaseError,
    }

    // Implementations for each operation
    pub struct CreateUser;
    impl ApiOperation<DatabaseContext, CreateUserProps> for CreateUser {
        type Output = User;
        type Error = UserError;

        fn execute(context: &mut DatabaseContext, props: &CreateUserProps) -> Result<Self::Output, Self::Error> {
            if !props.email.contains('@') {
                return Err(UserError::InvalidEmail);
            }

            context.increment_transaction();

            // Simulate creating user
            let user = User {
                id: context.transaction_count as u64,
                name: props.name.clone(),
                email: props.email.clone(),
            };

            // Cache the user
            context.cache.insert(format!("user_{}", user.id), format!("{}:{}", user.name, user.email));

            Ok(user)
        }
    }

    pub struct FindUser;
    impl ApiOperation<DatabaseContext, FindUserProps> for FindUser {
        type Output = User;
        type Error = UserError;

        fn execute(context: &mut DatabaseContext, props: &FindUserProps) -> Result<Self::Output, Self::Error> {
            // Try cache first
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

    pub struct UpdateUser;
    impl ApiOperation<DatabaseContext, UpdateUserProps> for UpdateUser {
        type Output = User;
        type Error = UserError;

        fn execute(context: &mut DatabaseContext, props: &UpdateUserProps) -> Result<Self::Output, Self::Error> {
            // Find existing user first
            let find_props = FindUserProps { user_id: props.user_id };
            let mut user = FindUser::execute(context, &find_props)?;

            // Apply updates
            if let Some(ref name) = props.name {
                user.name = name.clone();
            }
            if let Some(ref email) = props.email {
                if !email.contains('@') {
                    return Err(UserError::InvalidEmail);
                }
                user.email = email.clone();
            }

            // Update cache
            context.cache.insert(
                format!("user_{}", user.id),
                format!("{}:{}", user.name, user.email)
            );

            context.increment_transaction();

            Ok(user)
        }
    }
}

// Another API family: Product operations (showing reuse of same context)
pub mod product_api {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct CreateProductProps {
        pub name: String,
        pub price: f64,
        pub category: String,
    }

    #[derive(Debug, Clone)]
    pub struct FindProductProps {
        pub product_id: u64,
    }

    #[derive(Debug, Clone)]
    pub struct Product {
        pub id: u64,
        pub name: String,
        pub price: f64,
        pub category: String,
    }

    #[derive(Debug)]
    pub enum ProductError {
        NotFound,
        InvalidPrice,
        DatabaseError,
    }

    pub struct CreateProduct;
    impl ApiOperation<DatabaseContext, CreateProductProps> for CreateProduct {
        type Output = Product;
        type Error = ProductError;

        fn execute(context: &mut DatabaseContext, props: &CreateProductProps) -> Result<Self::Output, Self::Error> {
            if props.price < 0.0 {
                return Err(ProductError::InvalidPrice);
            }

            context.increment_transaction();

            let product = Product {
                id: context.transaction_count as u64,
                name: props.name.clone(),
                price: props.price,
                category: props.category.clone(),
            };

            context.cache.insert(
                format!("product_{}", product.id),
                format!("{}:{}:{}", product.name, product.price, product.category)
            );

            Ok(product)
        }
    }

    pub struct FindProduct;
    impl ApiOperation<DatabaseContext, FindProductProps> for FindProduct {
        type Output = Product;
        type Error = ProductError;

        fn execute(context: &mut DatabaseContext, props: &FindProductProps) -> Result<Self::Output, Self::Error> {
            if let Some(cached) = context.cache.get(&format!("product_{}", props.product_id)) {
                let parts: Vec<&str> = cached.split(':').collect();
                if parts.len() == 3 {
                    if let Ok(price) = parts[1].parse::<f64>() {
                        return Ok(Product {
                            id: props.product_id,
                            name: parts[0].to_string(),
                            price,
                            category: parts[2].to_string(),
                        });
                    }
                }
            }

            Err(ProductError::NotFound)
        }
    }
}

// Convenience trait for ergonomic usage
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

// Optional: Generic executor for even more ergonomics
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

    pub fn context(&self) -> &C {
        &self.context
    }

    pub fn context_mut(&mut self) -> &mut C {
        &mut self.context
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use user_api::*;
    use product_api::*;

    #[test]
    fn test_user_operations() {
        let mut ctx = DatabaseContext::new("test_db".to_string());

        // Create user
        let create_props = CreateUserProps {
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        };
        let user = CreateUser::execute(&mut ctx, &create_props).unwrap();
        assert_eq!(user.name, "Alice");

        // Find user
        let find_props = FindUserProps { user_id: user.id };
        let found_user = FindUser::execute(&mut ctx, &find_props).unwrap();
        assert_eq!(found_user.name, "Alice");

        // Update user
        let update_props = UpdateUserProps {
            user_id: user.id,
            name: Some("Alice Smith".to_string()),
            email: None,
        };
        let updated_user = UpdateUser::execute(&mut ctx, &update_props).unwrap();
        assert_eq!(updated_user.name, "Alice Smith");
    }

    #[test]
    fn test_executor() {
        let mut executor = ApiExecutor::new(DatabaseContext::new("test_db".to_string()));

        // Create user using executor
        let user = executor.execute(
            CreateUser,
            &CreateUserProps {
                name: "Bob".to_string(),
                email: "bob@example.com".to_string(),
            }
        ).unwrap();

        // Create product using same executor (shared context)
        let product = executor.execute(
            CreateProduct,
            &CreateProductProps {
                name: "Widget".to_string(),
                price: 29.99,
                category: "Electronics".to_string(),
            }
        ).unwrap();

        // Both operations used the same context
        assert!(executor.context().transaction_count >= 2);
    }
}
```
