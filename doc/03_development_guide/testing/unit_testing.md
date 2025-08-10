# Unit Testing Guide

Unit testing is the foundation of our testing strategy. This guide covers how to write effective unit tests for the `deribit-fix` crate.

## Overview

Unit tests verify individual functions, methods, and small components in isolation. They should be fast, reliable, and provide immediate feedback during development.

## Test Structure

### Basic Test Function

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // Arrange
        let input = "test_data";
        
        // Act
        let result = function_to_test(input);
        
        // Assert
        assert_eq!(result, expected_value);
    }
}
```

### Test Module Organization

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Test data setup
    mod setup {
        use super::*;
        
        fn create_test_config() -> Config {
            Config::default()
                .with_api_key("test_key")
                .with_api_secret("test_secret")
        }
    }
    
    // Individual test functions
    mod connection_tests {
        use super::*;
        
        #[test]
        fn test_connection_establishment() {
            // Test implementation
        }
    }
    
    mod message_tests {
        use super::*;
        
        #[test]
        fn test_message_parsing() {
            // Test implementation
        }
    }
}
```

## Testing Core Components

### Configuration Testing

```rust
#[test]
fn test_config_creation() {
    let config = Config::new()
        .with_api_key("test_key")
        .with_api_secret("test_secret")
        .with_environment(Environment::Test);
    
    assert_eq!(config.api_key, "test_key");
    assert_eq!(config.api_secret, "test_secret");
    assert_eq!(config.environment, Environment::Test);
}

#[test]
fn test_config_validation() {
    let mut config = Config::default();
    
    // Test valid configuration
    assert!(config.validate().is_ok());
    
    // Test invalid configuration
    config.api_key = String::new();
    assert!(config.validate().is_err());
}

#[test]
fn test_config_merging() {
    let base_config = Config::default()
        .with_api_key("base_key")
        .with_environment(Environment::Test);
    
    let override_config = Config::default()
        .with_api_secret("override_secret");
    
    let merged = base_config.merge(override_config);
    
    assert_eq!(merged.api_key, "base_key");
    assert_eq!(merged.api_secret, "override_secret");
    assert_eq!(merged.environment, Environment::Test);
}
```

### Error Handling Testing

```rust
#[test]
fn test_error_creation() {
    let error = DeribitFixError::ConnectionFailed {
        source: std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "test"),
        context: ErrorContext::new("test_operation"),
    };
    
    assert_eq!(error.severity(), ErrorSeverity::High);
    assert!(error.to_string().contains("Connection failed"));
}

#[test]
fn test_error_context() {
    let context = ErrorContext::new("order_placement")
        .with_order_id("12345")
        .with_symbol("BTC-PERPETUAL");
    
    assert_eq!(context.operation, "order_placement");
    assert_eq!(context.order_id, Some("12345".to_string()));
    assert_eq!(context.symbol, Some("BTC-PERPETUAL".to_string()));
}

#[test]
fn test_error_recovery() {
    let error = DeribitFixError::ConnectionTimeout {
        context: ErrorContext::new("heartbeat"),
    };
    
    let strategy = error.recovery_strategy();
    assert!(matches!(strategy, RecoveryStrategy::Retry { .. }));
}
```

### Message Handling Testing

```rust
#[test]
fn test_fix_message_creation() {
    let message = FixMessage::new()
        .with_msg_type("D")
        .with_sender_comp_id("CLIENT")
        .with_target_comp_id("DERIBIT")
        .with_msg_seq_num(1)
        .with_sending_time(chrono::Utc::now());
    
    assert_eq!(message.msg_type(), "D");
    assert_eq!(message.sender_comp_id(), "CLIENT");
    assert_eq!(message.target_comp_id(), "DERIBIT");
    assert_eq!(message.msg_seq_num(), 1);
}

#[test]
fn test_fix_message_serialization() {
    let message = FixMessage::new()
        .with_msg_type("D")
        .with_sender_comp_id("CLIENT")
        .with_target_comp_id("DERIBIT");
    
    let serialized = message.to_string();
    assert!(serialized.contains("35=D"));
    assert!(serialized.contains("49=CLIENT"));
    assert!(serialized.contains("56=DERIBIT"));
}

#[test]
fn test_fix_message_parsing() {
    let fix_string = "8=FIX.4.4|35=D|49=CLIENT|56=DERIBIT|34=1|52=20231201-10:00:00.000|";
    let message = FixMessage::from_str(fix_string).unwrap();
    
    assert_eq!(message.msg_type(), "D");
    assert_eq!(message.sender_comp_id(), "CLIENT");
    assert_eq!(message.target_comp_id(), "DERIBIT");
    assert_eq!(message.msg_seq_num(), 1);
}
```

### Session Management Testing

```rust
#[test]
fn test_session_creation() {
    let session = SessionManager::new()
        .with_sender_comp_id("CLIENT")
        .with_target_comp_id("DERIBIT")
        .with_heartbeat_interval(30);
    
    assert_eq!(session.sender_comp_id(), "CLIENT");
    assert_eq!(session.target_comp_id(), "DERIBIT");
    assert_eq!(session.heartbeat_interval(), 30);
}

#[test]
fn test_session_sequence_management() {
    let mut session = SessionManager::new();
    
    // Test sequence increment
    assert_eq!(session.next_outbound_seq_num(), 1);
    assert_eq!(session.next_outbound_seq_num(), 2);
    
    // Test sequence reset
    session.reset_outbound_sequence(100);
    assert_eq!(session.next_outbound_seq_num(), 101);
}

#[test]
fn test_session_heartbeat() {
    let mut session = SessionManager::new()
        .with_heartbeat_interval(30);
    
    let last_heartbeat = session.last_heartbeat_time();
    assert!(last_heartbeat.is_none());
    
    session.send_heartbeat();
    let new_heartbeat = session.last_heartbeat_time();
    assert!(new_heartbeat.is_some());
}
```

## Testing Async Functions

### Async Test Functions

```rust
#[tokio::test]
async fn test_async_connection() {
    let client = DeribitFixClient::new(Config::default());
    
    // Test async connection
    let result = client.connect().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_async_order_placement() {
    let client = DeribitFixClient::new(Config::default());
    
    let order = Order::new()
        .with_symbol("BTC-PERPETUAL")
        .with_side(OrderSide::Buy)
        .with_order_type(OrderType::Limit)
        .with_quantity(0.1)
        .with_price(50000.0);
    
    let result = client.place_order(order).await;
    assert!(result.is_ok());
}
```

### Mock Testing for Async

```rust
#[tokio::test]
async fn test_mock_connection() {
    let mut mock_connection = MockConnection::new();
    mock_connection
        .expect_connect()
        .times(1)
        .returning(|| Ok(()));
    
    let client = DeribitFixClient::with_connection(mock_connection);
    let result = client.connect().await;
    
    assert!(result.is_ok());
}
```

## Property-Based Testing

### Using Proptest

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_config_roundtrip(config in config_strategy()) {
        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&serialized).unwrap();
        assert_eq!(config, deserialized);
    }
    
    #[test]
    fn test_message_validation(msg in fix_message_strategy()) {
        // Test that all generated messages are valid
        assert!(msg.validate().is_ok());
    }
}

fn config_strategy() -> impl Strategy<Value = Config> {
    (any::<String>(), any::<String>(), any::<Environment>())
        .prop_map(|(api_key, api_secret, env)| {
            Config::new()
                .with_api_key(api_key)
                .with_api_secret(api_secret)
                .with_environment(env)
        })
}

fn fix_message_strategy() -> impl Strategy<Value = FixMessage> {
    (any::<String>(), any::<String>(), any::<String>())
        .prop_map(|(msg_type, sender, target)| {
            FixMessage::new()
                .with_msg_type(msg_type)
                .with_sender_comp_id(sender)
                .with_target_comp_id(target)
        })
}
```

## Test Utilities

### Common Test Helpers

```rust
#[cfg(test)]
pub mod test_utils {
    use super::*;
    
    pub fn create_test_order() -> Order {
        Order::new()
            .with_symbol("BTC-PERPETUAL")
            .with_side(OrderSide::Buy)
            .with_order_type(OrderType::Limit)
            .with_quantity(0.1)
            .with_price(50000.0)
    }
    
    pub fn create_test_config() -> Config {
        Config::new()
            .with_api_key("test_key")
            .with_api_secret("test_secret")
            .with_environment(Environment::Test)
    }
    
    pub fn create_test_fix_message() -> FixMessage {
        FixMessage::new()
            .with_msg_type("D")
            .with_sender_comp_id("CLIENT")
            .with_target_comp_id("DERIBIT")
            .with_msg_seq_num(1)
    }
    
    pub async fn setup_test_client() -> DeribitFixClient {
        let config = create_test_config();
        let client = DeribitFixClient::new(config);
        client.connect().await.unwrap();
        client
    }
}
```

## Best Practices

### Test Organization

1. **Group Related Tests**: Use test modules to organize related functionality
2. **Descriptive Names**: Test names should clearly describe what is being tested
3. **Setup and Teardown**: Use helper functions for common test setup
4. **Test Data**: Create reusable test data structures

### Test Isolation

1. **Independent Tests**: Each test should run independently
2. **No Shared State**: Avoid mutable static state between tests
3. **Clean Environment**: Reset any external state after tests
4. **Mock Dependencies**: Use mocks for external dependencies

### Assertions

1. **Specific Assertions**: Use the most specific assertion for the test case
2. **Error Testing**: Test both success and failure scenarios
3. **Edge Cases**: Include boundary conditions and edge cases
4. **Performance**: Test performance characteristics where relevant

### Documentation

1. **Test Purpose**: Document complex test scenarios
2. **Setup Requirements**: Document any special setup needed
3. **Expected Behavior**: Clearly state what the test verifies
4. **Failure Debugging**: Include helpful error messages

## Running Unit Tests

```bash
# Run all unit tests
cargo test

# Run tests in a specific module
cargo test tests::connection_tests

# Run tests with output
cargo test -- --nocapture

# Run tests in parallel (default)
cargo test -- --test-threads=4

# Run tests sequentially
cargo test -- --test-threads=1

# Run tests with specific pattern
cargo test connection

# Run tests with verbose output
cargo test -- --nocapture --test-threads=1
```

## Debugging Failed Tests

### Common Issues

1. **Test Order Dependencies**: Ensure tests are independent
2. **Async Race Conditions**: Use proper async test patterns
3. **Resource Cleanup**: Ensure proper cleanup in async tests
4. **Mock Expectations**: Verify mock expectations are met

### Debugging Commands

```bash
# Run single test with output
cargo test test_name -- --nocapture

# Run test with debug logging
RUST_LOG=debug cargo test test_name

# Run test with backtrace
RUST_BACKTRACE=1 cargo test test_name

# Run test with specific features
cargo test test_name --features debug
```

## Next Steps

- [Integration Testing](./integration_testing.md) - Testing module interactions
- [Benchmarking](./benchmarking.md) - Performance testing
- [Mock Testing](./mock_testing.md) - Testing with external dependencies
- [Test Utilities](./test_utilities.md) - Common testing helpers
