# Create Product Operation

Refer to /Users/wballard/github/apithing/ideas/start.md

## Goal
Implement the `CreateProduct` operation to demonstrate multi-family API usage with shared context.

## Tasks
- Create `CreateProduct` struct (zero-sized type)
- Implement `ApiOperation<DatabaseContext, CreateProductProps>` for `CreateProduct`
- Business logic:
  - Validate price is non-negative
  - Increment transaction counter
  - Generate product ID from transaction count
  - Store product in cache using format "product_{id}" -> "name:price:category"
  - Return created `Product` struct
- Handle error cases (invalid price)
- Add comprehensive rustdoc documentation with examples

## Success Criteria
- Implementation reuses same `DatabaseContext` as user operations
- Price validation works correctly
- Product creation updates shared transaction counter
- Cache storage uses distinct namespace from users
- Error handling covers edge cases
- Documentation shows multi-family context sharing

## Implementation Details
```rust
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
        
        // Cache with format "name:price:category"
        context.cache.insert(
            format!("product_{}", product.id),
            format!("{}:{}:{}", product.name, product.price, product.category)
        );
        
        Ok(product)
    }
}
```

This demonstrates how different API families can share context while maintaining their own data formats and validation rules.