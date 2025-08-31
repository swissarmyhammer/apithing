//! Basic usage example demonstrating direct ApiOperation calls.
//!
//! This example shows the fundamental patterns for using the ApiThing framework:
//! - Creating a context
//! - Executing operations directly
//! - Handling errors
//! - Working with results
//!
//! Run with: `cargo run --example basic_usage`

use apithing::ApiOperation;
use std::collections::HashMap;

// Simple User API implementation for this example
#[derive(Debug, Clone)]
struct CreateUserProps {
    name: String,
    email: String,
}

#[derive(Debug, Clone)]
struct FindUserProps {
    user_id: u64,
}

#[derive(Debug, Clone)]
struct UpdateUserProps {
    user_id: u64,
    name: Option<String>,
    email: Option<String>,
}

#[derive(Debug, Clone)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[derive(Debug)]
enum UserError {
    InvalidEmail,
    NotFound,
}

// Custom context for this example
#[derive(Debug)]
struct AppContext {
    connection_pool: String,
    transaction_count: u32,
    cache: HashMap<String, String>,
}

impl AppContext {
    fn new(connection: String) -> Self {
        Self {
            connection_pool: connection,
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

    fn connection_pool(&self) -> &str {
        &self.connection_pool
    }

    fn cache(&self) -> &HashMap<String, String> {
        &self.cache
    }

    fn cache_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.cache
    }
}

struct CreateUser;
struct FindUser;
struct UpdateUser;

impl ApiOperation<AppContext, CreateUserProps> for CreateUser {
    type Output = User;
    type Error = UserError;

    fn execute(context: &mut AppContext, parameters: &CreateUserProps) -> Result<User, UserError> {
        if !parameters.email.contains('@') {
            return Err(UserError::InvalidEmail);
        }

        context.increment_transaction();
        let user = User {
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

impl ApiOperation<AppContext, FindUserProps> for FindUser {
    type Output = User;
    type Error = UserError;

    fn execute(context: &mut AppContext, parameters: &FindUserProps) -> Result<User, UserError> {
        let cache_key = format!("user_{}", parameters.user_id);

        if let Some(cached_data) = context.cache().get(&cache_key) {
            let parts: Vec<&str> = cached_data.split(':').collect();
            if parts.len() == 2 {
                return Ok(User {
                    id: parameters.user_id,
                    name: parts[0].to_string(),
                    email: parts[1].to_string(),
                });
            }
        }

        Err(UserError::NotFound)
    }
}

impl ApiOperation<AppContext, UpdateUserProps> for UpdateUser {
    type Output = User;
    type Error = UserError;

    fn execute(context: &mut AppContext, parameters: &UpdateUserProps) -> Result<User, UserError> {
        let find_parameters = FindUserProps {
            user_id: parameters.user_id,
        };
        let mut user = FindUser::execute(context, &find_parameters)?;

        if let Some(name) = &parameters.name {
            user.name = name.clone();
        }
        if let Some(email) = &parameters.email {
            if !email.contains('@') {
                return Err(UserError::InvalidEmail);
            }
            user.email = email.clone();
        }

        context.increment_transaction();
        let cache_key = format!("user_{}", user.id);
        let cache_value = format!("{}:{}", user.name, user.email);
        context.cache_mut().insert(cache_key, cache_value);

        Ok(user)
    }
}

fn main() {
    println!("🚀 ApiThing Basic Usage Example");
    println!("================================\n");

    // Create an application context - this manages shared state across operations
    let mut context = AppContext::new("example_database".to_string());
    println!(
        "📊 Created context with connection: {}",
        context.connection_pool()
    );
    println!(
        "🔢 Initial transaction count: {}\n",
        context.transaction_count()
    );

    // 1. Create a user using direct ApiOperation::execute call
    println!("👤 Creating a user...");
    let create_parameters = CreateUserProps {
        name: "Alice Johnson".to_string(),
        email: "alice@example.com".to_string(),
    };

    let user = match CreateUser::execute(&mut context, &create_parameters) {
        Ok(user) => {
            println!(
                "✅ Created user: {} (ID: {}, Email: {})",
                user.name, user.id, user.email
            );
            println!(
                "🔢 Transaction count after creation: {}\n",
                context.transaction_count()
            );
            user
        }
        Err(e) => {
            println!("❌ Failed to create user: {:?}", e);
            return;
        }
    };

    // 2. Find the user we just created
    println!("🔍 Finding user by ID...");
    let find_parameters = FindUserProps { user_id: user.id };

    let _found_user = match FindUser::execute(&mut context, &find_parameters) {
        Ok(user) => {
            println!("✅ Found user: {} (Email: {})", user.name, user.email);
            println!(
                "🔢 Transaction count after find: {} (no increment for reads)\n",
                context.transaction_count()
            );
            user
        }
        Err(e) => {
            println!("❌ Failed to find user: {:?}", e);
            return;
        }
    };

    // 3. Update the user's information
    println!("📝 Updating user information...");
    let update_parameters = UpdateUserProps {
        user_id: user.id,
        name: Some("Alice Smith".to_string()), // Changed name after marriage
        email: Some("alice.smith@example.com".to_string()),
    };

    let _updated_user = match UpdateUser::execute(&mut context, &update_parameters) {
        Ok(user) => {
            println!("✅ Updated user: {} (Email: {})", user.name, user.email);
            println!(
                "🔢 Final transaction count: {}\n",
                context.transaction_count()
            );
            user
        }
        Err(e) => {
            println!("❌ Failed to update user: {:?}", e);
            return;
        }
    };

    // 4. Demonstrate error handling
    println!("❌ Demonstrating error handling...");
    let invalid_parameters = CreateUserProps {
        name: "Bob".to_string(),
        email: "invalid-email".to_string(), // Missing @ symbol
    };

    match CreateUser::execute(&mut context, &invalid_parameters) {
        Ok(_) => println!("This shouldn't happen!"),
        Err(e) => println!("✅ Caught expected error: {:?}", e),
    }

    // 5. Try to find a non-existent user
    let missing_parameters = FindUserProps { user_id: 999 };
    match FindUser::execute(&mut context, &missing_parameters) {
        Ok(_) => println!("This shouldn't happen!"),
        Err(e) => println!("✅ Caught expected error: {:?}", e),
    }

    println!("\n🎉 Basic usage example completed successfully!");
    println!("💡 Key takeaways:");
    println!("   • Operations are executed via Type::execute(&mut context, &parameters)");
    println!("   • Context maintains state (transactions, cache) across operations");
    println!("   • Type-safe properties prevent runtime errors");
    println!("   • Comprehensive error handling with domain-specific error types");
}
