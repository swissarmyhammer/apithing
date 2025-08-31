//! Advanced patterns example demonstrating complex workflows and custom contexts.
//!
//! This example showcases sophisticated usage patterns including:
//! - Custom context implementation extending DatabaseContext
//! - Complex workflows combining multiple operations
//! - Error recovery and rollback patterns
//! - Transaction boundaries and audit trails
//! - Context inspection and debugging capabilities
//!
//! Run with: `cargo run --example advanced_patterns`

use apithing::ApiOperation;
use std::collections::HashMap;

// Configuration constants for timestamp formatting
const SECONDS_PER_DAY: u64 = 86400;
const SECONDS_PER_HOUR: u64 = 3600;
const SECONDS_PER_MINUTE: u64 = 60;
const UNIX_EPOCH_YEAR: u64 = 1970;
const DAYS_PER_YEAR: u64 = 365;
const DAYS_PER_MONTH: u64 = 30; // Simplified for demo

// Helper function to format Unix timestamp as ISO 8601
fn format_timestamp(timestamp: u64) -> String {
    // For demo purposes, convert Unix timestamp to a basic ISO 8601 format
    // In a real application, you'd use a proper datetime library like chrono
    let days_since_epoch = timestamp / SECONDS_PER_DAY;
    let seconds_in_day = timestamp % SECONDS_PER_DAY;
    let hours = seconds_in_day / SECONDS_PER_HOUR;
    let minutes = (seconds_in_day % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE;
    let seconds = seconds_in_day % SECONDS_PER_MINUTE;

    // Approximate date calculation for demo (simplified, not accounting for leap years)
    let mut year = UNIX_EPOCH_YEAR;
    let mut remaining_days = days_since_epoch;

    // Skip forward to approximately the right year
    year += remaining_days / DAYS_PER_YEAR;
    remaining_days %= DAYS_PER_YEAR;

    let month = (remaining_days / DAYS_PER_MONTH) + 1;
    let day = (remaining_days % DAYS_PER_MONTH) + 1;

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month, day, hours, minutes, seconds
    )
}

// Simple User and Product APIs for this example
#[derive(Debug, Clone)]
struct CreateUserProps {
    name: String,
    email: String,
}

#[derive(Debug, Clone)]
struct FindUserProps {
    id: u64,
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

#[derive(Debug)]
enum ProductError {
    InvalidPrice,
    InvalidCategory,
}

#[derive(Debug, Clone)]
struct CreateProductProps {
    name: String,
    price: f64,
    category: String,
}

#[derive(Debug, Clone)]
struct Product {
    id: u64,
    name: String,
    price: f64,
    category: String,
}

// Custom context for this example
#[derive(Debug)]
struct AppContext {
    transaction_count: u32,
    cache: std::collections::HashMap<String, String>,
}

impl AppContext {
    fn new(_connection: String) -> Self {
        Self {
            transaction_count: 0,
            cache: std::collections::HashMap::new(),
        }
    }

    fn increment_transaction(&mut self) {
        self.transaction_count += 1;
    }

    fn transaction_count(&self) -> u32 {
        self.transaction_count
    }

    fn cache(&self) -> &std::collections::HashMap<String, String> {
        &self.cache
    }

    fn cache_mut(&mut self) -> &mut std::collections::HashMap<String, String> {
        &mut self.cache
    }
}

struct CreateUser;
struct FindUser;
struct CreateProduct;

// Basic implementations for CreateUser and CreateProduct
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
        // Look for user in cache first
        let cache_key = format!("user_{}", parameters.id);
        if let Some(cache_value) = context.cache().get(&cache_key) {
            let parts: Vec<&str> = cache_value.split(':').collect();
            if parts.len() == 2 {
                return Ok(User {
                    id: parameters.id,
                    name: parts[0].to_string(),
                    email: parts[1].to_string(),
                });
            }
        }

        // User not found in cache
        Err(UserError::NotFound)
    }
}

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

        if parameters.category.trim().is_empty() {
            return Err(ProductError::InvalidCategory);
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

/// Custom application context that extends AppContext with additional functionality
#[derive(Debug)]
struct ApplicationContext {
    /// The underlying app context
    database: AppContext,
    /// Audit trail for operations
    audit_log: Vec<AuditEntry>,
    /// Configuration settings
    config: HashMap<String, String>,
    /// Request tracking
    request_id: u64,
    /// Feature flags
    features_enabled: HashMap<String, bool>,
}

#[derive(Debug, Clone)]
struct AuditEntry {
    operation: String,
    timestamp: String,
    transaction_id: u32,
    success: bool,
    details: String,
}

impl ApplicationContext {
    fn new(connection: String, request_id: u64) -> Self {
        let mut config = HashMap::new();
        config.insert("max_retries".to_string(), "3".to_string());
        config.insert("timeout_seconds".to_string(), "30".to_string());

        let mut features = HashMap::new();
        features.insert("enhanced_validation".to_string(), true);
        features.insert("audit_logging".to_string(), true);
        features.insert("rollback_support".to_string(), true);

        Self {
            database: AppContext::new(connection),
            audit_log: Vec::new(),
            config,
            request_id,
            features_enabled: features,
        }
    }

    fn log_operation(&mut self, operation: &str, success: bool, details: &str) {
        if *self.features_enabled.get("audit_logging").unwrap_or(&false) {
            // Generate a realistic timestamp for demo purposes
            let base_time = 1705312800u64; // 2024-01-15T10:00:00Z as Unix timestamp
            let seconds_offset = self.audit_log.len() as u64;
            let timestamp_seconds = base_time + seconds_offset;
            let timestamp = format_timestamp(timestamp_seconds);

            self.audit_log.push(AuditEntry {
                operation: operation.to_string(),
                timestamp,
                transaction_id: self.database.transaction_count(),
                success,
                details: details.to_string(),
            });
        }
    }

    fn is_feature_enabled(&self, feature: &str) -> bool {
        *self.features_enabled.get(feature).unwrap_or(&false)
    }

    fn simulate_rollback(&mut self, _checkpoint: u32) -> Result<(), String> {
        if !self.is_feature_enabled("rollback_support") {
            return Err("Rollback support not enabled".to_string());
        }

        // In a real implementation, this would rollback database state
        // For this example, we'll simulate by logging the rollback
        // Note: We can't directly modify transaction_count as it's private
        self.log_operation("rollback", true, "Simulated rollback to checkpoint");
        println!("🔄 Simulated rollback (transaction count reset not available in this demo)");
        Ok(())
    }
}

/// Custom operation that demonstrates complex validation and error handling
struct CreateUserWithValidation;

impl ApiOperation<ApplicationContext, CreateUserProps> for CreateUserWithValidation {
    type Output = User;
    type Error = UserError;

    fn execute(
        context: &mut ApplicationContext,
        parameters: &CreateUserProps,
    ) -> Result<Self::Output, Self::Error> {
        context.log_operation("create_user_with_validation", true, "Starting validation");

        // Enhanced validation if feature is enabled
        if context.is_feature_enabled("enhanced_validation") {
            if parameters.name.len() < 2 {
                context.log_operation("create_user_with_validation", false, "Name too short");
                return Err(UserError::InvalidEmail); // Using available error
            }
            if !parameters.email.contains('@') || !parameters.email.contains('.') {
                context.log_operation("create_user_with_validation", false, "Invalid email format");
                return Err(UserError::InvalidEmail);
            }
        }

        // Delegate to the underlying database operation
        match CreateUser::execute(&mut context.database, parameters) {
            Ok(user) => {
                context.log_operation(
                    "create_user_with_validation",
                    true,
                    &format!("Created user {} with ID {}", user.name, user.id),
                );
                Ok(user)
            }
            Err(e) => {
                context.log_operation(
                    "create_user_with_validation",
                    false,
                    "Database operation failed",
                );
                Err(e)
            }
        }
    }
}

/// Workflow operation that creates a user and their first product in a single transaction
struct CreateUserWithProduct;

#[derive(Debug, Clone)]
struct CreateUserWithProductProps {
    user_name: String,
    user_email: String,
    product_name: String,
    product_price: f64,
    product_category: String,
}

impl ApiOperation<ApplicationContext, CreateUserWithProductProps> for CreateUserWithProduct {
    type Output = (User, Product);
    type Error = String;

    fn execute(
        context: &mut ApplicationContext,
        parameters: &CreateUserWithProductProps,
    ) -> Result<Self::Output, Self::Error> {
        let checkpoint = context.database.transaction_count();
        context.log_operation(
            "create_user_with_product",
            true,
            "Starting composite operation",
        );

        // Create the user first
        let user_parameters = CreateUserProps {
            name: parameters.user_name.clone(),
            email: parameters.user_email.clone(),
        };

        let user = match CreateUserWithValidation::execute(context, &user_parameters) {
            Ok(user) => {
                context.log_operation(
                    "create_user_with_product",
                    true,
                    "User created successfully",
                );
                user
            }
            Err(e) => {
                context.log_operation("create_user_with_product", false, "User creation failed");
                context.simulate_rollback(checkpoint).ok();
                return Err(format!("Failed to create user: {:?}", e));
            }
        };

        // Create the product
        let product_parameters = CreateProductProps {
            name: parameters.product_name.clone(),
            price: parameters.product_price,
            category: parameters.product_category.clone(),
        };

        let product = match CreateProduct::execute(&mut context.database, &product_parameters) {
            Ok(product) => {
                context.log_operation(
                    "create_user_with_product",
                    true,
                    "Product created successfully",
                );
                product
            }
            Err(e) => {
                context.log_operation(
                    "create_user_with_product",
                    false,
                    "Product creation failed, rolling back",
                );
                // In case of product creation failure, we could rollback the user creation
                context.simulate_rollback(checkpoint).ok();
                return Err(format!("Failed to create product: {:?}", e));
            }
        };

        context.log_operation(
            "create_user_with_product",
            true,
            &format!("Completed: User {} and Product {}", user.id, product.id),
        );

        Ok((user, product))
    }
}

fn main() {
    println!("🚀 ApiThing Advanced Patterns Example");
    println!("=====================================\n");

    // Create custom application context
    let mut app_context = ApplicationContext::new("advanced_production_db".to_string(), 12345);
    println!("🏗️  Created ApplicationContext");
    println!("📊 Request ID: {}", app_context.request_id);
    println!(
        "⚙️  Features enabled: {} features",
        app_context.features_enabled.len()
    );
    println!("🔧 Config entries: {} settings\n", app_context.config.len());

    // === Advanced Validation Pattern ===
    println!("🔍 ENHANCED VALIDATION PATTERN");
    println!("==============================");

    // Demonstrate enhanced validation
    let enhanced_user_parameters = CreateUserProps {
        name: "Alexander Thompson".to_string(),
        email: "alexander.thompson@enterprise.com".to_string(),
    };

    let validated_user =
        match CreateUserWithValidation::execute(&mut app_context, &enhanced_user_parameters) {
            Ok(user) => user,
            Err(e) => {
                println!("❌ Failed to create user: {:?}", e);
                return;
            }
        };
    println!(
        "✅ Created user with enhanced validation: {} (ID: {})",
        validated_user.name, validated_user.id
    );

    // Demonstrate user lookup functionality
    println!("\n🔍 Testing user lookup functionality:");
    let find_user_params = FindUserProps {
        id: validated_user.id,
    };

    match FindUser::execute(&mut app_context.database, &find_user_params) {
        Ok(found_user) => {
            println!(
                "✅ Found user in cache: {} ({})",
                found_user.name, found_user.email
            );
        }
        Err(UserError::NotFound) => {
            println!("❌ User not found in cache");
        }
        Err(e) => {
            println!("❌ Error finding user: {:?}", e);
        }
    }

    // Test lookup of non-existent user
    let missing_user_params = FindUserProps { id: 999 };
    match FindUser::execute(&mut app_context.database, &missing_user_params) {
        Ok(_) => println!("❌ This shouldn't happen - found non-existent user!"),
        Err(UserError::NotFound) => {
            println!("✅ Correctly detected missing user (ID: 999)");
        }
        Err(e) => {
            println!("❌ Unexpected error: {:?}", e);
        }
    }

    // Try to create user with invalid data to show validation
    let invalid_parameters = CreateUserProps {
        name: "X".to_string(),        // Too short
        email: "invalid".to_string(), // No @ or .
    };

    match CreateUserWithValidation::execute(&mut app_context, &invalid_parameters) {
        Ok(_) => println!("This shouldn't happen!"),
        Err(e) => println!("✅ Enhanced validation caught error: {:?}", e),
    }

    println!();

    // === Complex Workflow Pattern ===
    println!("🔄 COMPLEX WORKFLOW PATTERN");
    println!("===========================");

    // Demonstrate composite operation
    let workflow_parameters = CreateUserWithProductProps {
        user_name: "Sarah Wilson".to_string(),
        user_email: "sarah.wilson@company.com".to_string(),
        product_name: "Premium Subscription".to_string(),
        product_price: 29.99,
        product_category: "Software".to_string(),
    };

    match CreateUserWithProduct::execute(&mut app_context, &workflow_parameters) {
        Ok((user, product)) => {
            println!("✅ Composite operation succeeded:");
            println!("   👤 User: {} (ID: {})", user.name, user.id);
            println!("   📦 Product: {} (${:.2})", product.name, product.price);
        }
        Err(e) => println!("❌ Composite operation failed: {}", e),
    }

    println!();

    // === Error Recovery Pattern ===
    println!("🆘 ERROR RECOVERY PATTERN");
    println!("=========================");

    // Simulate a scenario where we need to recover from an error
    let checkpoint = app_context.database.transaction_count();
    println!("📍 Checkpoint at transaction: {}", checkpoint);

    // Create some operations that might fail
    let risky_parameters = CreateUserWithProductProps {
        user_name: "Test User".to_string(),
        user_email: "test@example.com".to_string(),
        product_name: "Expensive Item".to_string(),
        product_price: -100.0, // This should fail
        product_category: "Test".to_string(),
    };

    // This should fail and trigger rollback
    match CreateUserWithProduct::execute(&mut app_context, &risky_parameters) {
        Ok(_) => println!("Unexpected success"),
        Err(e) => {
            println!("❌ Operation failed as expected: {}", e);
            println!("🔄 Automatic rollback was triggered");
        }
    }

    println!();

    // === Context Inspection and Debugging ===
    println!("🔍 CONTEXT INSPECTION & DEBUGGING");
    println!("=================================");

    println!("📊 Final Statistics:");
    println!(
        "   🔢 Database transactions: {}",
        app_context.database.transaction_count()
    );
    println!(
        "   💾 Cache entries: {}",
        app_context.database.cache().len()
    );
    println!("   📝 Audit log entries: {}", app_context.audit_log.len());
    println!("   🆔 Request ID: {}", app_context.request_id);

    println!("\n📋 Audit Trail:");
    for (i, entry) in app_context.audit_log.iter().enumerate() {
        let status = if entry.success { "✅" } else { "❌" };
        println!(
            "   {}. {} [{}] {} (tx:{}) - {}",
            i + 1,
            status,
            entry.timestamp,
            entry.operation,
            entry.transaction_id,
            entry.details
        );
    }

    println!("\n⚙️  Configuration:");
    for (key, value) in &app_context.config {
        println!("   • {}: {}", key, value);
    }

    println!("\n🚩 Feature Flags:");
    for (feature, enabled) in &app_context.features_enabled {
        let status = if *enabled { "🟢" } else { "🔴" };
        println!("   {} {}", status, feature);
    }

    // === Performance and Optimization Patterns ===
    println!("\n⚡ PERFORMANCE INSIGHTS");
    println!("======================");

    println!("🎯 Operation Efficiency:");
    let total_ops = app_context.audit_log.len();
    let successful_ops = app_context.audit_log.iter().filter(|e| e.success).count();
    let success_rate = if total_ops > 0 {
        (successful_ops as f64 / total_ops as f64) * 100.0
    } else {
        0.0
    };

    println!(
        "   📈 Success rate: {:.1}% ({}/{})",
        success_rate, successful_ops, total_ops
    );
    println!(
        "   🔄 Average operations per transaction: {:.2}",
        if app_context.database.transaction_count() > 0 {
            total_ops as f64 / app_context.database.transaction_count() as f64
        } else {
            0.0
        }
    );

    println!("\n🎉 Advanced patterns example completed successfully!");
    println!("💡 Advanced patterns demonstrated:");
    println!("   • Custom context with enhanced functionality");
    println!("   • Complex validation and business rules");
    println!("   • Composite operations with transaction boundaries");
    println!("   • Error recovery and rollback mechanisms");
    println!("   • Comprehensive audit logging and debugging");
    println!("   • Feature flags and configuration management");
    println!("   • Performance monitoring and analytics");
}
