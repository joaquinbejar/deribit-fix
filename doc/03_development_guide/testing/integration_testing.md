# Integration Testing Guide

Integration testing verifies that multiple components work together correctly. This guide covers testing module interactions, API boundaries, and end-to-end functionality in the `deribit-fix` crate.

## Overview

Integration tests verify that different modules and components work together as expected. They test the public API surface and ensure that the crate functions correctly as a whole.

## Test Organization

### Directory Structure

```
tests/
├── integration/
│   ├── connection_tests.rs
│   ├── session_tests.rs
│   ├── order_management_tests.rs
│   ├── market_data_tests.rs
│   ├── error_handling_tests.rs
│   └── common/
│       ├── mod.rs
│       ├── test_client.rs
│       ├── test_data.rs
│       └── assertions.rs
├── unit/
│   └── ...
└── benches/
    └── ...
```

### Common Test Module

```rust
// tests/integration/common/mod.rs
pub mod test_client;
pub mod test_data;
pub mod assertions;

pub use test_client::*;
pub use test_data::*;
pub use assertions::*;

// Common test setup and teardown
pub async fn setup_integration_test() -> TestEnvironment {
    // Initialize test environment
    TestEnvironment::new()
        .with_test_config()
        .with_mock_deribit_server()
        .await
}

pub async fn teardown_integration_test(env: TestEnvironment) {
    // Clean up test environment
    env.cleanup().await;
}
```

## Connection Integration Tests

### Basic Connection Flow

```rust
// tests/integration/connection_tests.rs
use deribit_fix::*;
use integration_test_common::*;

#[tokio::test]
async fn test_connection_establishment() {
    let env = setup_integration_test().await;
    
    let client = DeribitFixClient::new(env.test_config());
    
    // Test connection establishment
    let result = client.connect().await;
    assert!(result.is_ok());
    
    // Verify connection state
    assert!(client.is_connected());
    
    teardown_integration_test(env).await;
}

#[tokio::test]
async fn test_connection_reconnection() {
    let env = setup_integration_test().await;
    let client = DeribitFixClient::new(env.test_config());
    
    // Establish initial connection
    client.connect().await.unwrap();
    
    // Simulate connection loss
    env.simulate_connection_loss().await;
    
    // Wait for reconnection
    tokio::time::sleep(Duration::from_secs(5)).await;
    
    // Verify reconnection
    assert!(client.is_connected());
    
    teardown_integration_test(env).await;
}

#[tokio::test]
async fn test_connection_authentication() {
    let env = setup_integration_test().await;
    let client = DeribitFixClient::new(env.test_config());
    
    // Connect and authenticate
    let result = client.connect_and_authenticate().await;
    assert!(result.is_ok());
    
    // Verify authentication state
    assert!(client.is_authenticated());
    
    teardown_integration_test(env).await;
}
```

### Connection Error Scenarios

```rust
#[tokio::test]
async fn test_connection_timeout() {
    let env = setup_integration_test().await;
    let mut config = env.test_config();
    config.connection.timeout = Duration::from_millis(100);
    
    let client = DeribitFixClient::new(config);
    
    // Test connection timeout
    let result = client.connect().await;
    assert!(matches!(result, Err(DeribitFixError::ConnectionTimeout { .. })));
    
    teardown_integration_test(env).await;
}

#[tokio::test]
async fn test_connection_refused() {
    let env = setup_integration_test().await;
    let mut config = env.test_config();
    config.connection.host = "127.0.0.1".to_string();
    config.connection.port = 12345; // Invalid port
    
    let client = DeribitFixClient::new(config);
    
    // Test connection refused
    let result = client.connect().await;
    assert!(matches!(result, Err(DeribitFixError::ConnectionFailed { .. })));
    
    teardown_integration_test(env).await;
}
```

## Session Management Integration Tests

### Session Lifecycle

```rust
// tests/integration/session_tests.rs
use deribit_fix::*;
use integration_test_common::*;

#[tokio::test]
async fn test_session_lifecycle() {
    let env = setup_integration_test().await;
    let client = DeribitFixClient::new(env.test_config());
    
    // Connect and authenticate
    client.connect_and_authenticate().await.unwrap();
    
    // Verify session state
    assert!(client.session().is_active());
    assert!(client.session().sequence_numbers().outbound > 0);
    
    // Test heartbeat
    let heartbeat_result = client.send_heartbeat().await;
    assert!(heartbeat_result.is_ok());
    
    // Test logout
    let logout_result = client.logout().await;
    assert!(logout_result.is_ok());
    
    // Verify session termination
    assert!(!client.session().is_active());
    
    teardown_integration_test(env).await;
}

#[tokio::test]
async fn test_session_sequence_management() {
    let env = setup_integration_test().await;
    let client = DeribitFixClient::new(env.test_config());
    
    client.connect_and_authenticate().await.unwrap();
    
    // Send multiple messages and verify sequence numbers
    let initial_seq = client.session().sequence_numbers().outbound;
    
    client.send_heartbeat().await.unwrap();
    client.send_heartbeat().await.unwrap();
    client.send_heartbeat().await.unwrap();
    
    let final_seq = client.session().sequence_numbers().outbound;
    assert_eq!(final_seq, initial_seq + 3);
    
    teardown_integration_test(env).await;
}

#[tokio::test]
async fn test_session_recovery() {
    let env = setup_integration_test().await;
    let client = DeribitFixClient::new(env.test_config());
    
    client.connect_and_authenticate().await.unwrap();
    
    // Simulate sequence gap
    env.simulate_sequence_gap(100).await;
    
    // Test sequence reset
    let reset_result = client.reset_sequence(200).await;
    assert!(reset_result.is_ok());
    
    // Verify sequence reset
    assert_eq!(client.session().sequence_numbers().outbound, 201);
    
    teardown_integration_test(env).await;
}
```

## Order Management Integration Tests

### Order Placement Flow

```rust
// tests/integration/order_management_tests.rs
use deribit_fix::*;
use integration_test_common::*;

#[tokio::test]
async fn test_order_placement_flow() {
    let env = setup_integration_test().await;
    let client = DeribitFixClient::new(env.test_config());
    
    client.connect_and_authenticate().await.unwrap();
    
    // Create test order
    let order = Order::new()
        .with_symbol("BTC-PERPETUAL")
        .with_side(OrderSide::Buy)
        .with_order_type(OrderType::Limit)
        .with_quantity(0.1)
        .with_price(50000.0)
        .with_time_in_force(TimeInForce::Day);
    
    // Place order
    let place_result = client.place_order(order.clone()).await;
    assert!(place_result.is_ok());
    
    let order_id = place_result.unwrap();
    
    // Verify order status
    let status = client.get_order_status(&order_id).await.unwrap();
    assert_eq!(status.status, OrderStatus::New);
    
    // Cancel order
    let cancel_result = client.cancel_order(&order_id).await;
    assert!(cancel_result.is_ok());
    
    teardown_integration_test(env).await;
}

#[tokio::test]
async fn test_order_modification() {
    let env = setup_integration_test().await;
    let client = DeribitFixClient::new(env.test_config());
    
    client.connect_and_authenticate().await.unwrap();
    
    // Place initial order
    let order = Order::new()
        .with_symbol("BTC-PERPETUAL")
        .with_side(OrderSide::Buy)
        .with_order_type(OrderType::Limit)
        .with_quantity(0.1)
        .with_price(50000.0);
    
    let order_id = client.place_order(order).await.unwrap();
    
    // Modify order
    let modification = OrderModification::new()
        .with_order_id(&order_id)
        .with_new_price(51000.0)
        .with_new_quantity(0.2);
    
    let modify_result = client.modify_order(modification).await;
    assert!(modify_result.is_ok());
    
    // Verify modification
    let status = client.get_order_status(&order_id).await.unwrap();
    assert_eq!(status.price, 51000.0);
    assert_eq!(status.quantity, 0.2);
    
    // Cleanup
    client.cancel_order(&order_id).await.unwrap();
    
    teardown_integration_test(env).await;
}

#[tokio::test]
async fn test_bulk_order_operations() {
    let env = setup_integration_test().await;
    let client = DeribitFixClient::new(env.test_config());
    
    client.connect_and_authenticate().await.unwrap();
    
    // Create multiple orders
    let orders = vec![
        Order::new()
            .with_symbol("BTC-PERPETUAL")
            .with_side(OrderSide::Buy)
            .with_order_type(OrderType::Limit)
            .with_quantity(0.1)
            .with_price(50000.0),
        Order::new()
            .with_symbol("BTC-PERPETUAL")
            .with_side(OrderSide::Sell)
            .with_order_type(OrderType::Limit)
            .with_quantity(0.1)
            .with_price(51000.0),
    ];
    
    // Place orders
    let place_results = client.place_orders(orders).await;
    assert!(place_results.is_ok());
    
    let order_ids: Vec<String> = place_results.unwrap();
    assert_eq!(order_ids.len(), 2);
    
    // Cancel all orders
    let cancel_results = client.cancel_orders(&order_ids).await;
    assert!(cancel_results.is_ok());
    
    teardown_integration_test(env).await;
}
```

## Market Data Integration Tests

### Market Data Subscription

```rust
// tests/integration/market_data_tests.rs
use deribit_fix::*;
use integration_test_common::*;
use futures::StreamExt;

#[tokio::test]
async fn test_market_data_subscription() {
    let env = setup_integration_test().await;
    let client = DeribitFixClient::new(env.test_config());
    
    client.connect_and_authenticate().await.unwrap();
    
    // Subscribe to market data
    let subscription = MarketDataSubscription::new()
        .with_symbol("BTC-PERPETUAL")
        .with_depth(10);
    
    let mut stream = client.subscribe_market_data(subscription).await.unwrap();
    
    // Wait for initial snapshot
    let snapshot = stream.next().await.unwrap().unwrap();
    assert_eq!(snapshot.symbol, "BTC-PERPETUAL");
    assert!(!snapshot.bids.is_empty());
    assert!(!snapshot.asks.is_empty());
    
    // Wait for incremental updates
    let update = stream.next().await.unwrap().unwrap();
    assert!(update.is_incremental());
    
    teardown_integration_test(env).await;
}

#[tokio::test]
async fn test_market_data_snapshot() {
    let env = setup_integration_test().await;
    let client = DeribitFixClient::new(env.test_config());
    
    client.connect_and_authenticate().await.unwrap();
    
    // Request market data snapshot
    let snapshot = client.get_market_data_snapshot("BTC-PERPETUAL", 20).await.unwrap();
    
    assert_eq!(snapshot.symbol, "BTC-PERPETUAL");
    assert_eq!(snapshot.depth, 20);
    assert!(!snapshot.bids.is_empty());
    assert!(!snapshot.asks.is_empty());
    
    teardown_integration_test(env).await;
}

#[tokio::test]
async fn test_market_data_unsubscription() {
    let env = setup_integration_test().await;
    let client = DeribitFixClient::new(env.test_config());
    
    client.connect_and_authenticate().await.unwrap();
    
    // Subscribe to market data
    let subscription = MarketDataSubscription::new()
        .with_symbol("BTC-PERPETUAL");
    
    let stream = client.subscribe_market_data(subscription).await.unwrap();
    
    // Unsubscribe
    let unsubscribe_result = client.unsubscribe_market_data("BTC-PERPETUAL").await;
    assert!(unsubscribe_result.is_ok());
    
    teardown_integration_test(env).await;
}
```

## Error Handling Integration Tests

### Error Recovery Scenarios

```rust
// tests/integration/error_handling_tests.rs
use deribit_fix::*;
use integration_test_common::*;

#[tokio::test]
async fn test_connection_error_recovery() {
    let env = setup_integration_test().await;
    let client = DeribitFixClient::new(env.test_config());
    
    client.connect_and_authenticate().await.unwrap();
    
    // Simulate connection error
    env.simulate_connection_error().await;
    
    // Wait for automatic recovery
    tokio::time::sleep(Duration::from_secs(10)).await;
    
    // Verify recovery
    assert!(client.is_connected());
    assert!(client.is_authenticated());
    
    teardown_integration_test(env).await;
}

#[tokio::test]
async fn test_sequence_error_recovery() {
    let env = setup_integration_test().await;
    let client = DeribitFixClient::new(env.test_config());
    
    client.connect_and_authenticate().await.unwrap();
    
    // Simulate sequence error
    env.simulate_sequence_error().await;
    
    // Test automatic sequence reset
    let reset_result = client.handle_sequence_error().await;
    assert!(reset_result.is_ok());
    
    // Verify sequence recovery
    assert!(client.session().sequence_numbers().outbound > 0);
    
    teardown_integration_test(env).await;
}

#[tokio::test]
async fn test_authentication_error_recovery() {
    let env = setup_integration_test().await;
    let client = DeribitFixClient::new(env.test_config());
    
    client.connect_and_authenticate().await.unwrap();
    
    // Simulate authentication error
    env.simulate_authentication_error().await;
    
    // Test automatic re-authentication
    let reauth_result = client.reauthenticate().await;
    assert!(reauth_result.is_ok());
    
    // Verify re-authentication
    assert!(client.is_authenticated());
    
    teardown_integration_test(env).await;
}
```

## Performance Integration Tests

### Load Testing

```rust
// tests/integration/performance_tests.rs
use deribit_fix::*;
use integration_test_common::*;
use tokio::time::Instant;

#[tokio::test]
async fn test_high_frequency_order_placement() {
    let env = setup_integration_test().await;
    let client = DeribitFixClient::new(env.test_config());
    
    client.connect_and_authenticate().await.unwrap();
    
    let start = Instant::now();
    let order_count = 1000;
    
    // Place orders rapidly
    for i in 0..order_count {
        let order = Order::new()
            .with_symbol("BTC-PERPETUAL")
            .with_side(OrderSide::Buy)
            .with_order_type(OrderType::Limit)
            .with_quantity(0.01)
            .with_price(50000.0 + i as f64);
        
        let result = client.place_order(order).await;
        assert!(result.is_ok());
    }
    
    let duration = start.elapsed();
    let orders_per_second = order_count as f64 / duration.as_secs_f64();
    
    // Verify performance requirements
    assert!(orders_per_second >= 100.0); // At least 100 orders per second
    
    teardown_integration_test(env).await;
}

#[tokio::test]
async fn test_concurrent_operations() {
    let env = setup_integration_test().await;
    let client = DeribitFixClient::new(env.test_config());
    
    client.connect_and_authenticate().await.unwrap();
    
    let operation_count = 100;
    
    // Execute operations concurrently
    let handles: Vec<_> = (0..operation_count)
        .map(|i| {
            let client = client.clone();
            tokio::spawn(async move {
                let order = Order::new()
                    .with_symbol("BTC-PERPETUAL")
                    .with_side(OrderSide::Buy)
                    .with_order_type(OrderType::Limit)
                    .with_quantity(0.01)
                    .with_price(50000.0 + i as f64);
                
                client.place_order(order).await
            })
        })
        .collect();
    
    // Wait for all operations to complete
    let results = futures::future::join_all(handles).await;
    
    // Verify all operations succeeded
    for result in results {
        assert!(result.unwrap().is_ok());
    }
    
    teardown_integration_test(env).await;
}
```

## Test Utilities and Helpers

### Test Client Wrapper

```rust
// tests/integration/common/test_client.rs
use deribit_fix::*;
use std::time::Duration;

pub struct TestClient {
    client: DeribitFixClient,
    env: TestEnvironment,
}

impl TestClient {
    pub async fn new(env: TestEnvironment) -> Self {
        let client = DeribitFixClient::new(env.test_config());
        Self { client, env }
    }
    
    pub async fn connect_and_authenticate(&self) -> Result<(), DeribitFixError> {
        self.client.connect_and_authenticate().await
    }
    
    pub async fn wait_for_connection(&self, timeout: Duration) -> bool {
        let start = Instant::now();
        while start.elapsed() < timeout {
            if self.client.is_connected() {
                return true;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        false
    }
    
    pub async fn wait_for_authentication(&self, timeout: Duration) -> bool {
        let start = Instant::now();
        while start.elapsed() < timeout {
            if self.client.is_authenticated() {
                return true;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        false
    }
    
    pub async fn cleanup(&self) {
        // Cancel any open orders
        if let Ok(orders) = self.client.get_open_orders().await {
            for order in orders {
                let _ = self.client.cancel_order(&order.order_id).await;
            }
        }
        
        // Logout
        let _ = self.client.logout().await;
    }
}
```

### Test Data Generators

```rust
// tests/integration/common/test_data.rs
use deribit_fix::*;

pub fn create_test_order(symbol: &str, side: OrderSide, price: f64) -> Order {
    Order::new()
        .with_symbol(symbol)
        .with_side(side)
        .with_order_type(OrderType::Limit)
        .with_quantity(0.1)
        .with_price(price)
        .with_time_in_force(TimeInForce::Day)
}

pub fn create_test_config() -> Config {
    Config::new()
        .with_api_key("test_key")
        .with_api_secret("test_secret")
        .with_environment(Environment::Test)
        .with_connection_timeout(Duration::from_secs(30))
        .with_heartbeat_interval(30)
}

pub fn create_test_market_data_subscription(symbol: &str) -> MarketDataSubscription {
    MarketDataSubscription::new()
        .with_symbol(symbol)
        .with_depth(10)
        .with_update_frequency(UpdateFrequency::RealTime)
}
```

## Running Integration Tests

### Test Execution

```bash
# Run all integration tests
cargo test --test '*'

# Run specific integration test file
cargo test --test connection_tests

# Run integration tests with output
cargo test --test '*' -- --nocapture

# Run integration tests sequentially
cargo test --test '*' -- --test-threads=1

# Run integration tests with specific pattern
cargo test --test '*' -- connection

# Run integration tests with timeout
timeout 300 cargo test --test '*'
```

### Test Environment Setup

```bash
# Set test environment variables
export DERIBIT_TEST_API_KEY="your_test_key"
export DERIBIT_TEST_API_SECRET="your_test_secret"
export DERIBIT_TEST_ENVIRONMENT="test"

# Run tests with specific features
cargo test --test '*' --features integration_testing

# Run tests with logging
RUST_LOG=debug cargo test --test '*'
```

## Best Practices

### Test Organization

1. **Group Related Tests**: Organize tests by functionality
2. **Common Setup**: Use shared test utilities and helpers
3. **Test Isolation**: Ensure tests don't interfere with each other
4. **Cleanup**: Always clean up test resources

### Test Data Management

1. **Test Data Generation**: Use factories and builders for test data
2. **Data Isolation**: Use unique identifiers for each test
3. **State Management**: Track and verify test state changes
4. **Cleanup Verification**: Verify cleanup operations succeed

### Error Testing

1. **Error Scenarios**: Test both success and failure paths
2. **Recovery Testing**: Verify error recovery mechanisms
3. **Edge Cases**: Test boundary conditions and limits
4. **Performance Under Error**: Test behavior during error conditions

### Async Testing

1. **Proper Async Patterns**: Use `tokio::test` and async/await
2. **Timeout Handling**: Set appropriate timeouts for async operations
3. **Resource Management**: Ensure proper cleanup of async resources
4. **Concurrent Testing**: Test concurrent operation scenarios

## Next Steps

- [Benchmarking](./benchmarking.md) - Performance testing guide
- [Mock Testing](./mock_testing.md) - Testing with external dependencies
- [Test Utilities](./test_utilities.md) - Common testing helpers
- [Unit Testing](./unit_testing.md) - Individual component testing
