# Find Product Operation

Refer to /Users/wballard/github/apithing/ideas/start.md

## Goal
Implement the `FindProduct` operation to complete the product API family demonstration.

## Tasks
- Create `FindProduct` struct (zero-sized type)
- Implement `ApiOperation<DatabaseContext, FindProductProps>` for `FindProduct`
- Business logic:
  - Check cache for product using key format "product_{id}"
  - Parse cached data format "name:price:category"
  - Handle price parsing from string to f64
  - Construct and return `Product` struct
  - Return `ProductError::NotFound` if not in cache or parse fails
- Add comprehensive rustdoc documentation
- Include error handling for parse failures

## Success Criteria
- Implementation integrates with cache format from `CreateProduct`
- Cache lookup and parsing work correctly
- Price parsing handles potential errors gracefully
- Error cases return appropriate `ProductError::NotFound`
- Documentation explains multi-step parsing process

## Implementation Details
```rust
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
```

This completes the product API family and demonstrates parsing of more complex cached data formats.