# Test Utilities and Helpers

This document provides a comprehensive guide to the test utilities and helpers available in the `deribit-fix` crate for writing effective and maintainable tests.

## Overview

Test utilities help reduce code duplication, provide consistent test data, and simplify test setup across different test types. The `deribit-fix` crate provides a comprehensive set of utilities for:

- Test data generation
- Mock object builders
- Test environment setup
- Common testing patterns
- Test assertions and validators

## Test Data Generators

### Order Generators

Generate test orders with different characteristics:

```rust
use deribit_fix::*;
use deribit_fix::test_utils::*;

#[test]
fn test_order_generation() {
    // Generate a basic test order
    let basic_order = create_test_order();
    assert_eq!(basic_order.symbol(), "BTC-PERPETUAL");
    assert_eq!(basic_order.side(), OrderSide::Buy);
    
    // Generate order with specific parameters
    let custom_order = create_test_order()
        .with_symbol("ETH-PERPETUAL")
        .with_side(OrderSide::Sell)
        .with_quantity(1.5)
        .with_price(3000.0);
    
    assert_eq!(custom_order.symbol(), "ETH-PERPETUAL");
    assert_eq!(custom_order.quantity(), 1.5);
}

#[test]
fn test_order_variations() {
    // Generate different order types
    let limit_order = create_limit_order("BTC-PERPETUAL", OrderSide::Buy, 0.1, 50000.0);
    let market_order = create_market_order("ETH-PERPETUAL", OrderSide::Sell, 2.0);
    let stop_order = create_stop_order("BTC-PERPETUAL", OrderSide::Sell, 0.1, 45000.0);
    
    assert_eq!(limit_order.order_type(), OrderType::Limit);
    assert_eq!(market_order.order_type(), OrderType::Market);
    assert_eq!(stop_order.order_type(), OrderType::Stop);
}
```

### FIX Message Generators

Generate test FIX messages for different scenarios:

```rust
use deribit_fix::*;
use deribit_fix::test_utils::*;

#[test]
fn test_fix_message_generation() {
    // Generate basic logon message
    let logon_msg = create_logon_message("CLIENT", "DERIBIT", 1);
    assert_eq!(logon_msg.msg_type(), "A");
    assert_eq!(logon_msg.sender_comp_id(), "CLIENT");
    
    // Generate order message
    let order_msg = create_order_message("CLIENT", "DERIBIT", 2, &create_test_order());
    assert_eq!(order_msg.msg_type(), "D");
    assert_eq!(order_msg.msg_seq_num(), 2);
    
    // Generate heartbeat message
    let heartbeat_msg = create_heartbeat_message("CLIENT", "DERIBIT", 3);
    assert_eq!(heartbeat_msg.msg_type(), "0");
}
```

### Configuration Generators

Generate test configurations for different environments:

```rust
use deribit_fix::*;
use deribit_fix::test_utils::*;

#[test]
fn test_config_generation() {
    // Generate test environment config
    let test_config = create_test_config();
    assert_eq!(test_config.environment, Environment::Test);
    assert_eq!(test_config.api_key, "test_key");
    
    // Generate production-like config
    let prod_config = create_production_config("real_key", "real_secret");
    assert_eq!(prod_config.environment, Environment::Production);
    assert_eq!(prod_config.api_key, "real_key");
    
    // Generate config with custom settings
    let custom_config = create_custom_config()
        .with_heartbeat_interval(15)
        .with_connection_timeout(Duration::from_secs(10))
        .with_max_retries(5);
    
    assert_eq!(custom_config.heartbeat_interval, 15);
    assert_eq!(custom_config.connection_timeout, Duration::from_secs(10));
}
```

## Mock Builders

### Connection Mock Builder

Build mock connections with specific behaviors:

```rust
use deribit_fix::*;
use deribit_fix::test_utils::*;

#[test]
fn test_mock_connection_builder() {
    let mock_connection = MockConnectionBuilder::new()
        .with_connect_success()
        .with_send_message_success()
        .with_receive_message(create_heartbeat_message("DERIBIT", "CLIENT", 1))
        .with_disconnect_success()
        .build();
    
    // Test the mock connection
    let result = mock_connection.connect().await;
    assert!(result.is_ok());
    
    let send_result = mock_connection.send_message(&create_test_message()).await;
    assert!(send_result.is_ok());
}
```

### Session Mock Builder

Build mock sessions with authentication and sequence management:

```rust
use deribit_fix::*;
use deribit_fix::test_utils::*;

#[test]
fn test_mock_session_builder() {
    let mock_session = MockSessionBuilder::new()
        .with_authentication_success("test_key", "test_secret")
        .with_sequence_numbers(1, 1)
        .with_heartbeat_interval(30)
        .with_logout_success()
        .build();
    
    // Test authentication
    let credentials = AuthCredentials::new("test_key", "test_secret");
    let auth_result = mock_session.authenticate(&credentials).await;
    assert!(auth_result.is_ok());
    
    // Test sequence management
    assert_eq!(mock_session.next_outbound_seq_num(), 1);
    assert_eq!(mock_session.next_inbound_seq_num(), 1);
}
```

### Error Mock Builder

Simulate specific error conditions:

```rust
use deribit_fix::*;
use deribit_fix::test_utils::*;

#[test]
fn test_error_simulation() {
    let mock_connection = MockConnectionBuilder::new()
        .with_connect_failure(DeribitFixError::ConnectionFailed {
            source: std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "test"),
            context: ErrorContext::new("test_operation"),
        })
        .build();
    
    // Test connection failure
    let result = mock_connection.connect().await;
    assert!(result.is_err());
    
    match result {
        Err(DeribitFixError::ConnectionFailed { .. }) => {
            // Expected error
        }
        _ => panic!("Unexpected error type"),
    }
}
```

## Test Environment Setup

### Integration Test Environment

Set up a complete test environment for integration tests:

```rust
use deribit_fix::*;
use deribit_fix::test_utils::*;

#[tokio::test]
async fn test_integration_environment() {
    let env = setup_integration_test().await;
    
    // Environment provides test configuration
    let config = env.test_config();
    assert_eq!(config.environment, Environment::Test);
    
    // Environment provides test client
    let client = env.test_client();
    assert!(client.is_connected());
    
    // Clean up
    teardown_integration_test(env).await;
}
```

### Unit Test Environment

Set up isolated test environment for unit tests:

```rust
use deribit_fix::*;
use deribit_fix::test_utils::*;

#[test]
fn test_unit_environment() {
    let env = setup_unit_test();
    
    // Environment provides isolated components
    let config = env.config();
    let session = env.session();
    
    // Test components in isolation
    assert_eq!(config.environment, Environment::Test);
    assert!(!session.is_active());
    
    teardown_unit_test(env);
}
```

## Common Testing Patterns

### Async Test Patterns

Handle async operations in tests:

```rust
use deribit_fix::*;
use deribit_fix::test_utils::*;

#[tokio::test]
async fn test_async_operations() {
    let client = create_test_client().await;
    
    // Test async order placement
    let order = create_test_order();
    let result = client.place_order(order).await;
    assert!(result.is_ok());
    
    // Test async market data subscription
    let subscription = client.subscribe_market_data("BTC-PERPETUAL").await;
    assert!(subscription.is_ok());
}

#[tokio::test]
async fn test_timeout_handling() {
    let client = create_test_client_with_timeout(Duration::from_millis(100)).await;
    
    // Test operation that should timeout
    let result = client.place_order(create_test_order()).await;
    match result {
        Err(DeribitFixError::Timeout { .. }) => {
            // Expected timeout
        }
        _ => panic!("Expected timeout error"),
    }
}
```

### Error Testing Patterns

Test error conditions systematically:

```rust
use deribit_fix::*;
use deribit_fix::test_utils::*;

#[test]
fn test_error_conditions() {
    // Test invalid configuration
    let invalid_config = Config::new()
        .with_api_key("")
        .with_api_secret("");
    
    let validation_result = invalid_config.validate();
    assert!(validation_result.is_err());
    
    // Test invalid order
    let invalid_order = Order::new()
        .with_symbol("")
        .with_quantity(-1.0);
    
    let order_validation = invalid_order.validate();
    assert!(order_validation.is_err());
}

#[test]
fn test_error_recovery() {
    let client = create_test_client_with_retry().await;
    
    // Simulate temporary failure
    let mock_connection = MockConnectionBuilder::new()
        .with_connect_failure_then_success()
        .build();
    
    client.set_connection(mock_connection);
    
    // Should succeed after retry
    let result = client.connect().await;
    assert!(result.is_ok());
}
```

## Test Assertions and Validators

### Order Validation

Validate order properties and state:

```rust
use deribit_fix::*;
use deribit_fix::test_utils::*;

#[test]
fn test_order_validation() {
    let order = create_test_order();
    
    // Validate order properties
    assert_order_valid(&order);
    assert_order_side(&order, OrderSide::Buy);
    assert_order_type(&order, OrderType::Limit);
    assert_order_quantity(&order, 0.1);
    assert_order_price(&order, 50000.0);
}

#[test]
fn test_order_state_transitions() {
    let mut order = create_test_order();
    
    // Test state transitions
    assert_order_status(&order, OrderStatus::New);
    
    order.set_status(OrderStatus::PartiallyFilled);
    assert_order_status(&order, OrderStatus::PartiallyFilled);
    
    order.set_status(OrderStatus::Filled);
    assert_order_status(&order, OrderStatus::Filled);
}
```

### FIX Message Validation

Validate FIX message structure and content:

```rust
use deribit_fix::*;
use deribit_fix::test_utils::*;

#[test]
fn test_fix_message_validation() {
    let message = create_test_message();
    
    // Validate message structure
    assert_fix_message_valid(&message);
    assert_message_type(&message, "D");
    assert_sender_comp_id(&message, "CLIENT");
    assert_target_comp_id(&message, "DERIBIT");
    
    // Validate required fields
    assert_required_field_present(&message, "35"); // MsgType
    assert_required_field_present(&message, "49"); // SenderCompID
    assert_required_field_present(&message, "56"); // TargetCompID
}

#[test]
fn test_fix_message_serialization() {
    let message = create_test_message();
    
    // Test serialization round-trip
    let serialized = message.to_string();
    let deserialized = FixMessage::from_str(&serialized).unwrap();
    
    assert_eq!(message, deserialized);
}
```

## Performance Testing Utilities

### Load Testing

Generate load for performance testing:

```rust
use deribit_fix::*;
use deribit_fix::test_utils::*;

#[tokio::test]
async fn test_load_generation() {
    let client = create_test_client().await;
    
    // Generate load with multiple concurrent orders
    let orders = generate_test_orders(100);
    let results = client.place_orders_concurrent(&orders).await;
    
    // Validate results
    assert_eq!(results.len(), 100);
    assert!(results.iter().all(|r| r.is_ok()));
}

#[tokio::test]
async fn test_throughput_measurement() {
    let client = create_test_client().await;
    
    // Measure throughput
    let start = std::time::Instant::now();
    let orders = generate_test_orders(1000);
    
    let results = client.place_orders_concurrent(&orders).await;
    let duration = start.elapsed();
    
    let throughput = results.len() as f64 / duration.as_secs_f64();
    println!("Throughput: {:.2} orders/second", throughput);
    
    assert!(throughput > 100.0); // Minimum throughput requirement
}
```

## Test Utilities Organization

### Module Structure

```rust
// src/test_utils/mod.rs
pub mod generators;
pub mod mocks;
pub mod environment;
pub mod assertions;
pub mod performance;

pub use generators::*;
pub use mocks::*;
pub use environment::*;
pub use assertions::*;
pub use performance::*;
```

### Feature Flags

```toml
# Cargo.toml
[features]
test-utils = []  # Enable test utilities
mock-testing = ["test-utils", "mockall"]  # Enable mock testing
integration-testing = ["test-utils"]  # Enable integration testing
```

## Best Practices

### Test Data Management

1. **Isolation**: Each test should use unique test data
2. **Cleanup**: Always clean up test data after tests
3. **Randomization**: Use random data when order doesn't matter
4. **Consistency**: Use consistent test data across related tests

### Mock Usage

1. **Specificity**: Make mocks as specific as possible
2. **Verification**: Verify mock expectations were met
3. **Isolation**: Don't share mocks between tests
4. **Realism**: Make mock behavior realistic

### Performance Testing

1. **Baselines**: Establish performance baselines
2. **Regression Detection**: Monitor for performance regressions
3. **Realistic Load**: Use realistic load patterns
4. **Monitoring**: Monitor system resources during tests

## Running Tests with Utilities

```bash
# Run tests with test utilities
cargo test --features test-utils

# Run tests with mock testing
cargo test --features mock-testing

# Run integration tests
cargo test --features integration-testing --test '*'

# Run performance tests
cargo bench --features test-utils
```

## Troubleshooting

### Common Issues

1. **Mock Expectations Not Met**: Check mock setup and test logic
2. **Test Data Conflicts**: Ensure test data isolation
3. **Async Test Failures**: Check timeout settings and async patterns
4. **Performance Test Variability**: Use statistical analysis for results

### Debug Tools

```rust
// Enable debug logging in tests
#[test]
fn test_with_debug() {
    env_logger::init();
    
    // Test code with debug output
    let result = function_under_test();
    println!("Debug: {:?}", result);
    
    assert!(result.is_ok());
}
```

## Summary

Test utilities in `deribit-fix` provide a comprehensive foundation for writing effective tests. They help:

- Reduce code duplication
- Ensure consistent test data
- Simplify complex test scenarios
- Improve test maintainability
- Support different testing strategies

By using these utilities effectively, you can write robust, maintainable tests that thoroughly validate the crate's functionality.
