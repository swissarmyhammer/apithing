//! Executor pattern example demonstrating ApiExecutor for stateful operations.
//!
//! This example shows the benefits of using ApiExecutor for managing state
//! across multiple operations:
//! - Centralized context management
//! - Ergonomic API with consistent patterns
//! - Multi-family API usage (users and products)
//! - Context sharing and state inspection
//!
//! Run with: `cargo run --example executor_pattern`

use apithing::{ApiExecutor, ApiOperation};
use std::collections::HashMap;

// Simple User API for this example
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

impl ApiOperation<AppContext, CreateUserProps> for CreateUser {
    type Output = User;
    type Error = UserError;

    fn execute(
        context: &mut AppContext,
        parameters: &CreateUserProps,
    ) -> Result<User, UserError> {
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

    fn execute(
        context: &mut AppContext,
        parameters: &FindUserProps,
    ) -> Result<User, UserError> {
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

// Simple Product API for this example
#[derive(Debug, Clone)]
struct CreateProductProps {
    name: String,
    price: f64,
    category: String,
}

#[derive(Debug, Clone)]
struct FindProductProps {
    product_id: u64,
}

#[derive(Debug, Clone)]
struct Product {
    id: u64,
    name: String,
    price: f64,
    category: String,
}

#[derive(Debug)]
enum ProductError {
    InvalidPrice,
    NotFound,
}

struct CreateProduct;
struct FindProduct;

impl ApiOperation<AppContext, CreateProductProps> for CreateProduct {
    type Output = Product;
    type Error = ProductError;

    fn execute(
        context: &mut AppContext,
        parameters: &CreateProductProps,
    ) -> Result<Product, ProductError> {
        if parameters.price <= 0.0 || parameters.price.is_nan() {
            return Err(ProductError::InvalidPrice);
        }

        context.increment_transaction();
        let product = Product {
            id: context.transaction_count() as u64,
            name: parameters.name.clone(),
            price: parameters.price,
            category: parameters.category.clone(),
        };

        let cache_key = format!("product_{}", product.id);
        let cache_value = format!("{}:{}:{}", product.name, product.price, product.category);
        context.cache_mut().insert(cache_key, cache_value);

        Ok(product)
    }
}

impl ApiOperation<AppContext, FindProductProps> for FindProduct {
    type Output = Product;
    type Error = ProductError;

    fn execute(
        context: &mut AppContext,
        parameters: &FindProductProps,
    ) -> Result<Product, ProductError> {
        let cache_key = format!("product_{}", parameters.product_id);

        if let Some(cached_data) = context.cache().get(&cache_key) {
            let parts: Vec<&str> = cached_data.split(':').collect();
            if parts.len() == 3 {
                if let Ok(price) = parts[1].parse::<f64>() {
                    return Ok(Product {
                        id: parameters.product_id,
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

fn main() {
    println!("🔧 ApiThing Executor Pattern Example");
    println!("====================================\n");

    // Create an executor that manages the context for us
    let mut executor = ApiExecutor::new(AppContext::new("production_db".to_string()));

    println!("🏗️  Created ApiExecutor with context");
    println!("📊 Connection: {}", executor.context().connection_pool());
    println!(
        "🔢 Initial transaction count: {}\n",
        executor.context().transaction_count()
    );

    // === User Operations ===
    println!("👥 USER OPERATIONS");
    println!("==================");

    // Create multiple users using the executor
    let users = vec![
        ("Alice Johnson", "alice@company.com"),
        ("Bob Smith", "bob@company.com"),
        ("Carol Davis", "carol@company.com"),
    ];

    let mut created_users = Vec::new();
    for (name, email) in users {
        match executor.execute(
            CreateUser,
            &CreateUserProps {
                name: name.to_string(),
                email: email.to_string(),
            },
        ) {
            Ok(user) => {
                println!("✅ Created user: {} (ID: {})", user.name, user.id);
                created_users.push(user);
            }
            Err(e) => {
                println!("❌ Failed to create user {}: {:?}", name, e);
                return;
            }
        }
    }

    println!(
        "🔢 Transaction count after user creation: {}\n",
        executor.context().transaction_count()
    );

    // === Product Operations (Same Context) ===
    println!("📦 PRODUCT OPERATIONS");
    println!("====================");

    // Create products using the same executor/context
    let products = vec![
        ("Laptop Pro", 1299.99, "Electronics"),
        ("Office Chair", 249.50, "Furniture"),
        ("Coffee Mug", 12.99, "Kitchen"),
    ];

    let mut created_products = Vec::new();
    for (name, price, category) in products {
        match executor.execute(
            CreateProduct,
            &CreateProductProps {
                name: name.to_string(),
                price,
                category: category.to_string(),
            },
        ) {
            Ok(product) => {
                println!(
                    "✅ Created product: {} (ID: {}, Price: ${:.2})",
                    product.name, product.id, product.price
                );
                created_products.push(product);
            }
            Err(e) => {
                println!("❌ Failed to create product {}: {:?}", name, e);
                return;
            }
        }
    }

    println!(
        "🔢 Transaction count after product creation: {}\n",
        executor.context().transaction_count()
    );

    // === Cross-Family Operations ===
    println!("🔄 CROSS-FAMILY LOOKUPS");
    println!("=======================");

    // Find users and products using the same context
    // This demonstrates how the cache works across different API families

    println!("🔍 Looking up created entities...");

    // Find first user
    if let Some(user) = created_users.first() {
        match executor.execute(FindUser, &FindUserProps { user_id: user.id }) {
            Ok(found_user) => println!("👤 Found user: {} ({})", found_user.name, found_user.email),
            Err(e) => println!("❌ Failed to find user: {:?}", e),
        }
    }

    // Find first product
    if let Some(product) = created_products.first() {
        match executor.execute(
            FindProduct,
            &FindProductProps {
                product_id: product.id,
            },
        ) {
            Ok(found_product) => println!(
                "📦 Found product: {} (${:.2})",
                found_product.name, found_product.price
            ),
            Err(e) => println!("❌ Failed to find product: {:?}", e),
        }
    }

    // === Context Inspection ===
    println!("\n📊 CONTEXT INSPECTION");
    println!("=====================");

    let context = executor.context();
    println!(
        "🔢 Final transaction count: {}",
        context.transaction_count()
    );
    println!("💾 Cache entries: {} items", context.cache().len());

    // Show cache contents
    println!("📋 Cached items:");
    for (key, value) in context.cache().iter() {
        // Truncate long values for display
        let display_value = if value.len() > 50 {
            format!("{}...", &value[..47])
        } else {
            value.clone()
        };
        println!("   • {}: {}", key, display_value);
    }

    // === Executor Benefits Demo ===
    println!("\n💡 EXECUTOR BENEFITS");
    println!("====================");

    // Show how we can access the context for debugging or inspection
    let final_context = executor.context();
    println!("✅ Centralized state management:");
    println!(
        "   • {} total operations executed",
        final_context.transaction_count()
    );
    println!("   • {} entities cached", final_context.cache().len());
    println!("   • Single context shared across {} API families", 2);

    println!("\n🎉 Executor pattern example completed successfully!");
    println!("💡 Key advantages of ApiExecutor:");
    println!("   • Ergonomic API: executor.execute(Operation, &parameters)");
    println!("   • Centralized context management with owned context");
    println!("   • Consistent patterns across different API families");
    println!("   • Easy context inspection and debugging");
    println!("   • Efficient state sharing (cache, connections, etc.)");
}
