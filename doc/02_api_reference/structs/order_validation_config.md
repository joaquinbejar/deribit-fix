# OrderValidationConfig

## Overview

The `OrderValidationConfig` struct defines configuration parameters for order validation, including business rules, risk checks, and validation thresholds.

## Purpose

- **Business Rules**: Defines order validation business logic and constraints
- **Risk Checks**: Configures risk validation parameters and thresholds
- **Validation Levels**: Controls the strictness of order validation
- **Custom Rules**: Allows for instrument-specific validation rules

## Public Interface

### Struct Definition

```rust
pub struct OrderValidationConfig {
    pub validation_level: ValidationLevel,
    pub max_order_value: f64,
    pub min_order_value: f64,
    pub max_order_size: f64,
    pub min_order_size: f64,
    pub price_tolerance: f64,
    pub quantity_tolerance: f64,
    pub max_orders_per_instrument: usize,
    pub max_orders_per_account: usize,
    pub allow_duplicate_orders: bool,
    pub duplicate_order_window: Duration,
    pub business_rules: BusinessRules,
    pub risk_checks: RiskCheckConfig,
    pub instrument_specific_rules: HashMap<String, InstrumentValidationRules>,
    pub validation_timeout: Duration,
    pub enable_async_validation: bool,
}
```

### Key Methods

```rust
impl OrderValidationConfig {
    /// Create default validation configuration
    pub fn default() -> Self

    /// Create strict validation configuration
    pub fn strict() -> Self

    /// Create relaxed validation configuration
    pub fn relaxed() -> Self

    /// Validate order against configuration
    pub fn validate_order(&self, order: &Order, context: &ValidationContext) -> Result<(), Vec<ValidationError>>

    /// Check if order passes basic validation
    pub fn basic_validation(&self, order: &Order) -> Result<(), Vec<ValidationError>>

    /// Check if order passes risk validation
    pub fn risk_validation(&self, order: &Order, context: &ValidationContext) -> Result<(), Vec<ValidationError>>

    /// Check if order passes business rule validation
    pub fn business_rule_validation(&self, order: &Order, context: &ValidationContext) -> Result<(), Vec<ValidationError>>

    /// Get instrument-specific validation rules
    pub fn get_instrument_rules(&self, instrument: &str) -> Option<&InstrumentValidationRules>

    /// Update validation configuration
    pub fn update_config(&mut self, updates: ValidationConfigUpdate) -> Result<(), ValidationConfigError>

    /// Validate configuration itself
    pub fn validate_config(&self) -> Result<(), ValidationConfigError>
}
```

## Usage Examples

### Creating Validation Configuration

```rust
use deribit_fix::types::{OrderValidationConfig, ValidationLevel, BusinessRules, RiskCheckConfig, InstrumentValidationRules};
use std::collections::HashMap;
use std::time::Duration;

// Create default validation configuration
let default_config = OrderValidationConfig::default();

// Create strict validation for institutional clients
let strict_config = OrderValidationConfig {
    validation_level: ValidationLevel::Strict,
    max_order_value: 1000000.0, // $1M max order value
    min_order_value: 100.0,     // $100 min order value
    max_order_size: 1000.0,     // 1000 contracts max
    min_order_size: 0.001,      // 0.001 contracts min
    price_tolerance: 0.01,      // 1% price tolerance
    quantity_tolerance: 0.001,  // 0.1% quantity tolerance
    max_orders_per_instrument: 100,
    max_orders_per_account: 1000,
    allow_duplicate_orders: false,
    duplicate_order_window: Duration::from_secs(60), // 1 minute
    business_rules: BusinessRules::strict(),
    risk_checks: RiskCheckConfig::strict(),
    instrument_specific_rules: HashMap::new(),
    validation_timeout: Duration::from_millis(100), // 100ms timeout
    enable_async_validation: true,
};

// Create relaxed validation for testing
let relaxed_config = OrderValidationConfig::relaxed();
```

### Order Validation

```rust
// Create validation context
let context = ValidationContext {
    account_id: "ACC001".to_string(),
    current_positions: vec![],
    account_balance: 100000.0,
    market_conditions: MarketConditions::Normal,
    timestamp: Utc::now(),
};

// Validate order against configuration
let order = Order::new(
    "BTC-PERPETUAL".to_string(),
    OrderSide::Buy,
    OrderType::Limit,
    1.0,
);

let validation_result = config.validate_order(&order, &context);
match validation_result {
    Ok(()) => println!("Order passes all validation checks"),
    Err(errors) => {
        for error in errors {
            eprintln!("Validation error: {:?}", error);
        }
    }
}

// Perform specific validation checks
let basic_result = config.basic_validation(&order);
let risk_result = config.risk_validation(&order, &context);
let business_result = config.business_rule_validation(&order, &context);

// Check all validation results
if basic_result.is_ok() && risk_result.is_ok() && business_result.is_ok() {
    println!("Order is fully validated");
}
```

### Instrument-Specific Rules

```rust
// Add instrument-specific validation rules
let mut config = OrderValidationConfig::default();

let btc_rules = InstrumentValidationRules {
    max_order_value: 500000.0,  // Lower limit for BTC
    max_order_size: 500.0,      // Lower size limit for BTC
    price_tolerance: 0.005,     // Tighter price tolerance
    custom_checks: vec![
        "check_btc_volatility".to_string(),
        "check_btc_correlation".to_string(),
    ],
};

config.instrument_specific_rules.insert("BTC-PERPETUAL".to_string(), btc_rules);

// Get instrument-specific rules
if let Some(rules) = config.get_instrument_rules("BTC-PERPETUAL") {
    println!("BTC max order value: {}", rules.max_order_value);
    println!("BTC price tolerance: {}", rules.price_tolerance);
}
```

### Dynamic Configuration Updates

```rust
// Update validation configuration based on market conditions
let mut config = OrderValidationConfig::default();

if market_volatility > 0.5 {
    // Tighten validation during high volatility
    let updates = ValidationConfigUpdate {
        price_tolerance: Some(0.005),      // Reduce from 0.01 to 0.005
        max_order_value: Some(500000.0),   // Reduce from 1M to 500K
        validation_level: Some(ValidationLevel::Strict),
        ..ValidationConfigUpdate::default()
    };

    config.update_config(updates)?;
    println!("Validation configuration updated for high volatility");
}

// Validate the updated configuration
config.validate_config()?;
```

## Module Dependencies

### Direct Dependencies

- **`types`**: `ValidationLevel`, `BusinessRules`, `RiskCheckConfig`, `InstrumentValidationRules`
- **`model`**: `Order`, `ValidationContext`, `ValidationError`
- **`std::collections`**: `HashMap`
- **`std::time`**: `Duration`
- **`chrono`**: `Utc`

### Related Types

- **`ValidationLevel`**: Enum defining validation strictness levels
- **`BusinessRules`**: Business logic validation rules
- **`RiskCheckConfig`**: Risk validation configuration
- **`InstrumentValidationRules`**: Instrument-specific validation rules
- **`ValidationContext`**: Context for validation operations
- **`ValidationError`**: Specific error types for validation failures
- **`ValidationConfigUpdate`**: Structure for updating validation configuration
- **`ValidationConfigError`**: Errors related to configuration updates

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_validation_config() {
        let config = OrderValidationConfig::default();
        
        assert_eq!(config.validation_level, ValidationLevel::Normal);
        assert!(config.max_order_value > 0.0);
        assert!(config.min_order_value > 0.0);
        assert!(config.max_order_size > 0.0);
        assert!(config.min_order_size > 0.0);
    }

    #[test]
    fn test_strict_validation_config() {
        let config = OrderValidationConfig::strict();
        
        assert_eq!(config.validation_level, ValidationLevel::Strict);
        assert!(config.price_tolerance < 0.01);
        assert!(!config.allow_duplicate_orders);
        assert!(config.validation_timeout < Duration::from_millis(200));
    }

    #[test]
    fn test_basic_validation() {
        let config = OrderValidationConfig::default();
        let order = Order::new(
            "BTC-PERPETUAL".to_string(),
            OrderSide::Buy,
            OrderType::Limit,
            1.0,
        );

        let result = config.basic_validation(&order);
        assert!(result.is_ok());
    }

    #[test]
    fn test_instrument_specific_rules() {
        let mut config = OrderValidationConfig::default();
        let rules = InstrumentValidationRules::default();
        
        config.instrument_specific_rules.insert("TEST-INST".to_string(), rules);
        
        let retrieved_rules = config.get_instrument_rules("TEST-INST");
        assert!(retrieved_rules.is_some());
        
        let non_existent_rules = config.get_instrument_rules("NON-EXISTENT");
        assert!(non_existent_rules.is_none());
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_full_order_validation() {
    let config = OrderValidationConfig::strict();
    let order = create_test_order("BTC-PERPETUAL");
    let context = create_test_validation_context();

    let result = config.validate_order(&order, &context);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_configuration_updates() {
    let mut config = OrderValidationConfig::default();
    let updates = ValidationConfigUpdate {
        max_order_value: Some(500000.0),
        validation_level: Some(ValidationLevel::Strict),
        ..ValidationConfigUpdate::default()
    };

    let result = config.update_config(updates);
    assert!(result.is_ok());
    
    assert_eq!(config.max_order_value, 500000.0);
    assert_eq!(config.validation_level, ValidationLevel::Strict);
}
```

## Performance Considerations

- **Validation Caching**: Cache validation results for similar orders
- **Async Validation**: Use async validation for complex checks
- **Batch Validation**: Validate multiple orders in batch operations
- **Timeout Management**: Ensure validation operations complete within reasonable time
- **Memory Usage**: Optimize memory usage for large rule sets

## Security Considerations

- **Input Validation**: Validate all configuration parameters
- **Access Control**: Restrict modification of validation configuration
- **Audit Logging**: Log all validation configuration changes
- **Rule Validation**: Ensure business rules are logically consistent
- **Performance Limits**: Prevent validation operations from causing DoS
