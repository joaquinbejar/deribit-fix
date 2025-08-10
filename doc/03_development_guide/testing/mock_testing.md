# Mock Testing Guide

Mock testing allows us to test the `deribit-fix` crate without external dependencies like network connections or the actual Deribit API. This guide covers mocking strategies, tools, and best practices.

## Overview

Mock testing is essential for:
- Testing without external dependencies
- Controlling test scenarios and responses
- Testing error conditions and edge cases
- Ensuring test reliability and speed
- Testing components in isolation

## Mocking Tools

### Mockall

Mockall is the primary mocking framework for Rust, providing powerful mocking capabilities.

```toml
# Cargo.toml
[dev-dependencies]
mockall = "0.12"
```

### Basic Mock Structure

```rust
use mockall::*;
use deribit_fix::*;

#[automock]
trait Connection {
    async fn connect(&self) -> Result<(), DeribitFixError>;
    async fn disconnect(&self) -> Result<(), DeribitFixError>;
    async fn send_message(&self, message: &FixMessage) -> Result<(), DeribitFixError>;
    async fn receive_message(&self) -> Result<FixMessage, DeribitFixError>;
}

#[automock]
trait SessionManager {
    fn is_active(&self) -> bool;
    fn next_outbound_seq_num(&mut self) -> u32;
    fn reset_outbound_sequence(&mut self, seq_num: u32);
    async fn authenticate(&mut self, credentials: &AuthCredentials) -> Result<(), DeribitFixError>;
}
```

## Connection Mocking

### Mock Connection Implementation

```rust
// tests/mocks/mock_connection.rs
use mockall::*;
use deribit_fix::*;
use std::sync::Arc;
use tokio::sync::Mutex;

#[automock]
pub trait Connection {
    async fn connect(&self) -> Result<(), DeribitFixError>;
    async fn disconnect(&self) -> Result<(), DeribitFixError>;
    async fn send_message(&self, message: &FixMessage) -> Result<(), DeribitFixError>;
    async fn receive_message(&self) -> Result<FixMessage, DeribitFixError>;
    fn is_connected(&self) -> bool;
}

pub struct MockConnectionManager {
    connection: Arc<MockConnection>,
    message_queue: Arc<Mutex<Vec<FixMessage>>>,
}

impl MockConnectionManager {
    pub fn new() -> Self {
        let mut connection = MockConnection::new();
        
        // Setup default expectations
        connection
            .expect_connect()
            .times(1..)
            .returning(|| Ok(()));
        
        connection
            .expect_is_connected()
            .returning(|| true);
        
        Self {
            connection: Arc::new(connection),
            message_queue: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    pub fn expect_connect_failure(&mut self, error: DeribitFixError) {
        let mut connection = MockConnection::new();
        connection
            .expect_connect()
            .times(1)
            .returning(move || Err(error.clone()));
        
        self.connection = Arc::new(connection);
    }
    
    pub fn expect_send_message_success(&mut self) {
        let mut connection = MockConnection::new();
        connection
            .expect_send_message()
            .times(1..)
            .returning(|_| Ok(()));
        
        self.connection = Arc::new(connection);
    }
    
    pub fn expect_send_message_failure(&mut self, error: DeribitFixError) {
        let mut connection = MockConnection::new();
        connection
            .expect_send_message()
            .times(1)
            .returning(move |_| Err(error.clone()));
        
        self.connection = Arc::new(connection);
    }
    
    pub fn queue_response(&self, message: FixMessage) {
        let queue = self.message_queue.clone();
        tokio::spawn(async move {
            let mut queue = queue.lock().await;
            queue.push(message);
        });
    }
    
    pub fn connection(&self) -> Arc<MockConnection> {
        self.connection.clone()
    }
}
```

### Connection Testing with Mocks

```rust
#[tokio::test]
async fn test_connection_success() {
    let mut mock_manager = MockConnectionManager::new();
    mock_manager.expect_send_message_success();
    
    let connection = mock_manager.connection();
    
    // Test connection
    let result = connection.connect().await;
    assert!(result.is_ok());
    
    // Test message sending
    let message = FixMessage::new()
        .with_msg_type("D")
        .with_sender_comp_id("CLIENT")
        .with_target_comp_id("DERIBIT");
    
    let send_result = connection.send_message(&message).await;
    assert!(send_result.is_ok());
}

#[tokio::test]
async fn test_connection_failure() {
    let mut mock_manager = MockConnectionManager::new();
    let error = DeribitFixError::ConnectionFailed {
        source: std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "test"),
        context: ErrorContext::new("test_connection"),
    };
    
    mock_manager.expect_connect_failure(error);
    let connection = mock_manager.connection();
    
    let result = connection.connect().await;
    assert!(matches!(result, Err(DeribitFixError::ConnectionFailed { .. })));
}

#[tokio::test]
async fn test_message_send_failure() {
    let mut mock_manager = MockConnectionManager::new();
    let error = DeribitFixError::MessageSendFailed {
        source: std::io::Error::new(std::io::ErrorKind::BrokenPipe, "test"),
        context: ErrorContext::new("test_send"),
    };
    
    mock_manager.expect_send_message_failure(error);
    let connection = mock_manager.connection();
    
    let message = FixMessage::new()
        .with_msg_type("D")
        .with_sender_comp_id("CLIENT")
        .with_target_comp_id("DERIBIT");
    
    let result = connection.send_message(&message).await;
    assert!(matches!(result, Err(DeribitFixError::MessageSendFailed { .. })));
}
```

## Session Mocking

### Mock Session Manager

```rust
// tests/mocks/mock_session.rs
use mockall::*;
use deribit_fix::*;

#[automock]
pub trait SessionManager {
    fn is_active(&self) -> bool;
    fn next_outbound_seq_num(&mut self) -> u32;
    fn reset_outbound_sequence(&mut self, seq_num: u32);
    async fn authenticate(&mut self, credentials: &AuthCredentials) -> Result<(), DeribitFixError>;
    async fn logout(&mut self) -> Result<(), DeribitFixError>;
    fn generate_heartbeat(&self) -> FixMessage;
}

pub struct MockSessionManagerBuilder {
    session: MockSessionManager,
}

impl MockSessionManagerBuilder {
    pub fn new() -> Self {
        let mut session = MockSessionManager::new();
        
        // Default expectations
        session
            .expect_is_active()
            .returning(|| true);
        
        session
            .expect_next_outbound_seq_num()
            .returning(|| 1);
        
        Self { session }
    }
    
    pub fn with_sequence_numbers(mut self, start_seq: u32) -> Self {
        let mut seq_num = start_seq;
        self.session
            .expect_next_outbound_seq_num()
            .returning(move || {
                seq_num += 1;
                seq_num
            });
        self
    }
    
    pub fn with_authentication_success(mut self) -> Self {
        self.session
            .expect_authenticate()
            .times(1..)
            .returning(|_| Ok(()));
        self
    }
    
    pub fn with_authentication_failure(mut self, error: DeribitFixError) -> Self {
        self.session
            .expect_authenticate()
            .times(1)
            .returning(move |_| Err(error.clone()));
        self
    }
    
    pub fn with_logout_success(mut self) -> Self {
        self.session
            .expect_logout()
            .times(1..)
            .returning(|| Ok(()));
        self
    }
    
    pub fn build(self) -> MockSessionManager {
        self.session
    }
}
```

### Session Testing with Mocks

```rust
#[tokio::test]
async fn test_session_authentication_success() {
    let session = MockSessionManagerBuilder::new()
        .with_authentication_success()
        .with_logout_success()
        .build();
    
    let credentials = AuthCredentials::new("test_key", "test_secret");
    
    let auth_result = session.authenticate(&credentials).await;
    assert!(auth_result.is_ok());
    
    assert!(session.is_active());
}

#[tokio::test]
async fn test_session_authentication_failure() {
    let error = DeribitFixError::AuthenticationFailed {
        reason: "Invalid credentials".to_string(),
        context: ErrorContext::new("test_auth"),
    };
    
    let session = MockSessionManagerBuilder::new()
        .with_authentication_failure(error)
        .build();
    
    let credentials = AuthCredentials::new("invalid_key", "invalid_secret");
    
    let auth_result = session.authenticate(&credentials).await;
    assert!(matches!(auth_result, Err(DeribitFixError::AuthenticationFailed { .. })));
}

#[tokio::test]
async fn test_session_sequence_management() {
    let session = MockSessionManagerBuilder::new()
        .with_sequence_numbers(100)
        .build();
    
    let seq1 = session.next_outbound_seq_num();
    let seq2 = session.next_outbound_seq_num();
    let seq3 = session.next_outbound_seq_num();
    
    assert_eq!(seq1, 101);
    assert_eq!(seq2, 102);
    assert_eq!(seq3, 103);
}
```

## Message Handler Mocking

### Mock Message Handler

```rust
// tests/mocks/mock_message_handler.rs
use mockall::*;
use deribit_fix::*;

#[automock]
pub trait MessageHandler {
    async fn handle_incoming_message(&mut self, message: FixMessage) -> Result<(), DeribitFixError>;
    async fn handle_outgoing_message(&mut self, message: &FixMessage) -> Result<(), DeribitFixError>;
    fn validate_message(&self, message: &FixMessage) -> Result<(), DeribitFixError>;
}

pub struct MockMessageHandlerBuilder {
    handler: MockMessageHandler,
}

impl MockMessageHandlerBuilder {
    pub fn new() -> Self {
        let mut handler = MockMessageHandler::new();
        
        // Default expectations
        handler
            .expect_validate_message()
            .returning(|_| Ok(()));
        
        Self { handler }
    }
    
    pub fn with_validation_success(mut self) -> Self {
        self.handler
            .expect_validate_message()
            .times(1..)
            .returning(|_| Ok(()));
        self
    }
    
    pub fn with_validation_failure(mut self, error: DeribitFixError) -> Self {
        self.handler
            .expect_validate_message()
            .times(1)
            .returning(move |_| Err(error.clone()));
        self
    }
    
    pub fn with_incoming_message_success(mut self) -> Self {
        self.handler
            .expect_handle_incoming_message()
            .times(1..)
            .returning(|_| Ok(()));
        self
    }
    
    pub fn with_outgoing_message_success(mut self) -> Self {
        self.handler
            .expect_handle_outgoing_message()
            .times(1..)
            .returning(|_| Ok(()));
        self
    }
    
    pub fn build(self) -> MockMessageHandler {
        self.handler
    }
}
```

### Message Handler Testing

```rust
#[tokio::test]
async fn test_message_validation_success() {
    let handler = MockMessageHandlerBuilder::new()
        .with_validation_success()
        .build();
    
    let message = FixMessage::new()
        .with_msg_type("D")
        .with_sender_comp_id("CLIENT")
        .with_target_comp_id("DERIBIT");
    
    let result = handler.validate_message(&message);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_message_validation_failure() {
    let error = DeribitFixError::InvalidMessage {
        reason: "Missing required field".to_string(),
        context: ErrorContext::new("test_validation"),
    };
    
    let handler = MockMessageHandlerBuilder::new()
        .with_validation_failure(error)
        .build();
    
    let message = FixMessage::new()
        .with_msg_type("D")
        .with_sender_comp_id("CLIENT");
    // Missing target_comp_id
    
    let result = handler.validate_message(&message);
    assert!(matches!(result, Err(DeribitFixError::InvalidMessage { .. })));
}

#[tokio::test]
async fn test_incoming_message_handling() {
    let handler = MockMessageHandlerBuilder::new()
        .with_incoming_message_success()
        .build();
    
    let message = FixMessage::new()
        .with_msg_type("8") // Execution Report
        .with_sender_comp_id("DERIBIT")
        .with_target_comp_id("CLIENT");
    
    let result = handler.handle_incoming_message(message).await;
    assert!(result.is_ok());
}
```

## Order Management Mocking

### Mock Order Handler

```rust
// tests/mocks/mock_order_handler.rs
use mockall::*;
use deribit_fix::*;

#[automock]
pub trait OrderHandler {
    async fn place_order(&mut self, order: Order) -> Result<String, DeribitFixError>;
    async fn cancel_order(&mut self, order_id: &str) -> Result<(), DeribitFixError>;
    async fn modify_order(&mut self, modification: OrderModification) -> Result<(), DeribitFixError>;
    async fn get_order_status(&self, order_id: &str) -> Result<OrderStatus, DeribitFixError>;
}

pub struct MockOrderHandlerBuilder {
    handler: MockOrderHandler,
}

impl MockOrderHandlerBuilder {
    pub fn new() -> Self {
        let mut handler = MockOrderHandler::new();
        
        // Default expectations
        handler
            .expect_place_order()
            .returning(|_| Ok("test_order_id".to_string()));
        
        Self { handler }
    }
    
    pub fn with_order_placement_success(mut self, order_id: String) -> Self {
        self.handler
            .expect_place_order()
            .times(1..)
            .returning(move |_| Ok(order_id.clone()));
        self
    }
    
    pub fn with_order_placement_failure(mut self, error: DeribitFixError) -> Self {
        self.handler
            .expect_place_order()
            .times(1)
            .returning(move |_| Err(error.clone()));
        self
    }
    
    pub fn with_order_cancellation_success(mut self) -> Self {
        self.handler
            .expect_cancel_order()
            .times(1..)
            .returning(|_| Ok(()));
        self
    }
    
    pub fn with_order_status(mut self, status: OrderStatus) -> Self {
        self.handler
            .expect_get_order_status()
            .times(1..)
            .returning(move |_| Ok(status.clone()));
        self
    }
    
    pub fn build(self) -> MockOrderHandler {
        self.handler
    }
}
```

### Order Handler Testing

```rust
#[tokio::test]
async fn test_order_placement_success() {
    let handler = MockOrderHandlerBuilder::new()
        .with_order_placement_success("order_123".to_string())
        .with_order_status(OrderStatus::New)
        .build();
    
    let order = Order::new()
        .with_symbol("BTC-PERPETUAL")
        .with_side(OrderSide::Buy)
        .with_order_type(OrderType::Limit)
        .with_quantity(0.1)
        .with_price(50000.0);
    
    let order_id = handler.place_order(order).await.unwrap();
    assert_eq!(order_id, "order_123");
    
    let status = handler.get_order_status(&order_id).await.unwrap();
    assert_eq!(status.status, OrderStatus::New);
}

#[tokio::test]
async fn test_order_placement_failure() {
    let error = DeribitFixError::OrderRejected {
        reason: "Insufficient funds".to_string(),
        context: ErrorContext::new("test_order"),
    };
    
    let handler = MockOrderHandlerBuilder::new()
        .with_order_placement_failure(error)
        .build();
    
    let order = Order::new()
        .with_symbol("BTC-PERPETUAL")
        .with_side(OrderSide::Buy)
        .with_order_type(OrderType::Limit)
        .with_quantity(1000.0) // Large quantity
        .with_price(50000.0);
    
    let result = handler.place_order(order).await;
    assert!(matches!(result, Err(DeribitFixError::OrderRejected { .. })));
}

#[tokio::test]
async fn test_order_cancellation() {
    let handler = MockOrderHandlerBuilder::new()
        .with_order_cancellation_success()
        .build();
    
    let result = handler.cancel_order("order_123").await;
    assert!(result.is_ok());
}
```

## Market Data Mocking

### Mock Market Data Handler

```rust
// tests/mocks/mock_market_data_handler.rs
use mockall::*;
use deribit_fix::*;
use futures::stream::{self, StreamExt};

#[automock]
pub trait MarketDataHandler {
    async fn subscribe_market_data(&mut self, subscription: MarketDataSubscription) -> Result<BoxStream<'static, Result<MarketDataUpdate, DeribitFixError>>, DeribitFixError>;
    async fn unsubscribe_market_data(&mut self, symbol: &str) -> Result<(), DeribitFixError>;
    async fn get_market_data_snapshot(&self, symbol: &str, depth: u32) -> Result<MarketDataSnapshot, DeribitFixError>;
}

pub struct MockMarketDataHandlerBuilder {
    handler: MockMarketDataHandler,
}

impl MockMarketDataHandlerBuilder {
    pub fn new() -> Self {
        let mut handler = MockMarketDataHandler;
        Self { handler }
    }
    
    pub fn with_subscription_success(mut self, updates: Vec<MarketDataUpdate>) -> Self {
        let stream = stream::iter(updates.into_iter().map(Ok));
        
        self.handler
            .expect_subscribe_market_data()
            .times(1..)
            .returning(move |_| Ok(Box::pin(stream.clone())));
        
        self
    }
    
    pub fn with_snapshot_success(mut self, snapshot: MarketDataSnapshot) -> Self {
        self.handler
            .expect_get_market_data_snapshot()
            .times(1..)
            .returning(move |_, _| Ok(snapshot.clone()));
        
        self
    }
    
    pub fn build(self) -> MockMarketDataHandler {
        self.handler
    }
}
```

### Market Data Testing

```rust
#[tokio::test]
async fn test_market_data_subscription() {
    let snapshot = MarketDataSnapshot {
        symbol: "BTC-PERPETUAL".to_string(),
        depth: 10,
        bids: vec![(50000.0, 0.1), (49999.0, 0.2)],
        asks: vec![(50001.0, 0.1), (50002.0, 0.2)],
        timestamp: chrono::Utc::now(),
    };
    
    let updates = vec![
        MarketDataUpdate::Snapshot(snapshot.clone()),
        MarketDataUpdate::Incremental {
            symbol: "BTC-PERPETUAL".to_string(),
            changes: vec![MarketDataChange::BidUpdate { price: 50000.0, quantity: 0.15 }],
            timestamp: chrono::Utc::now(),
        },
    ];
    
    let handler = MockMarketDataHandlerBuilder::new()
        .with_subscription_success(updates)
        .build();
    
    let subscription = MarketDataSubscription::new()
        .with_symbol("BTC-PERPETUAL")
        .with_depth(10);
    
    let mut stream = handler.subscribe_market_data(subscription).await.unwrap();
    
    let first_update = stream.next().await.unwrap().unwrap();
    assert!(matches!(first_update, MarketDataUpdate::Snapshot { .. }));
    
    let second_update = stream.next().await.unwrap().unwrap();
    assert!(matches!(second_update, MarketDataUpdate::Incremental { .. }));
}

#[tokio::test]
async fn test_market_data_snapshot() {
    let snapshot = MarketDataSnapshot {
        symbol: "BTC-PERPETUAL".to_string(),
        depth: 20,
        bids: vec![(50000.0, 0.1), (49999.0, 0.2)],
        asks: vec![(50001.0, 0.1), (50002.0, 0.2)],
        timestamp: chrono::Utc::now(),
    };
    
    let handler = MockMarketDataHandlerBuilder::new()
        .with_snapshot_success(snapshot.clone())
        .build();
    
    let result = handler.get_market_data_snapshot("BTC-PERPETUAL", 20).await;
    assert!(result.is_ok());
    
    let retrieved_snapshot = result.unwrap();
    assert_eq!(retrieved_snapshot.symbol, "BTC-PERPETUAL");
    assert_eq!(retrieved_snapshot.depth, 20);
    assert!(!retrieved_snapshot.bids.is_empty());
    assert!(!retrieved_snapshot.asks.is_empty());
}
```

## Error Scenario Mocking

### Mock Error Scenarios

```rust
// tests/mocks/mock_error_scenarios.rs
use mockall::*;
use deribit_fix::*;

pub struct MockErrorScenarioBuilder {
    connection: MockConnection,
    session: MockSessionManager,
    order_handler: MockOrderHandler,
}

impl MockErrorScenarioBuilder {
    pub fn new() -> Self {
        Self {
            connection: MockConnection::new(),
            session: MockSessionManager::new(),
            order_handler: MockOrderHandler::new(),
        }
    }
    
    pub fn with_connection_timeout(mut self) -> Self {
        self.connection
            .expect_connect()
            .times(1)
            .returning(|| {
                tokio::time::sleep(Duration::from_millis(100)).await;
                Err(DeribitFixError::ConnectionTimeout {
                    context: ErrorContext::new("test_timeout"),
                })
            });
        self
    }
    
    pub fn with_authentication_failure(mut self) -> Self {
        self.session
            .expect_authenticate()
            .times(1)
            .returning(|_| {
                Err(DeribitFixError::AuthenticationFailed {
                    reason: "Invalid API key".to_string(),
                    context: ErrorContext::new("test_auth"),
                })
            });
        self
    }
    
    pub fn with_order_rejection(mut self) -> Self {
        self.order_handler
            .expect_place_order()
            .times(1)
            .returning(|_| {
                Err(DeribitFixError::OrderRejected {
                    reason: "Insufficient margin".to_string(),
                    context: ErrorContext::new("test_order"),
                })
            });
        self
    }
    
    pub fn build(self) -> (MockConnection, MockSessionManager, MockOrderHandler) {
        (self.connection, self.session, self.order_handler)
    }
}
```

### Error Scenario Testing

```rust
#[tokio::test]
async fn test_connection_timeout_scenario() {
    let (connection, _, _) = MockErrorScenarioBuilder::new()
        .with_connection_timeout()
        .build();
    
    let start = Instant::now();
    let result = connection.connect().await;
    let duration = start.elapsed();
    
    assert!(matches!(result, Err(DeribitFixError::ConnectionTimeout { .. })));
    assert!(duration >= Duration::from_millis(100));
}

#[tokio::test]
async fn test_authentication_failure_scenario() {
    let (_, session, _) = MockErrorScenarioBuilder::new()
        .with_authentication_failure()
        .build();
    
    let credentials = AuthCredentials::new("invalid_key", "invalid_secret");
    let result = session.authenticate(&credentials).await;
    
    assert!(matches!(result, Err(DeribitFixError::AuthenticationFailed { .. })));
}

#[tokio::test]
async fn test_order_rejection_scenario() {
    let (_, _, order_handler) = MockErrorScenarioBuilder::new()
        .with_order_rejection()
        .build();
    
    let order = Order::new()
        .with_symbol("BTC-PERPETUAL")
        .with_side(OrderSide::Buy)
        .with_order_type(OrderType::Limit)
        .with_quantity(1000.0)
        .with_price(50000.0);
    
    let result = order_handler.place_order(order).await;
    assert!(matches!(result, Err(DeribitFixError::OrderRejected { .. })));
}
```

## Integration with Real Tests

### Combining Mocks with Real Components

```rust
// tests/integration/mock_integration_tests.rs
use deribit_fix::*;
use tests::mocks::*;

#[tokio::test]
async fn test_client_with_mock_connection() {
    let mut mock_connection = MockConnection::new();
    mock_connection
        .expect_connect()
        .times(1)
        .returning(|| Ok(()));
    
    mock_connection
        .expect_send_message()
        .times(1..)
        .returning(|_| Ok(()));
    
    let client = DeribitFixClient::with_connection(mock_connection);
    
    // Test client operations with mock connection
    let result = client.connect().await;
    assert!(result.is_ok());
    
    let message = FixMessage::new()
        .with_msg_type("D")
        .with_sender_comp_id("CLIENT")
        .with_target_comp_id("DERIBIT");
    
    let send_result = client.send_message(&message).await;
    assert!(send_result.is_ok());
}
```

## Best Practices

### Mock Design

1. **Realistic Behavior**: Mocks should behave like real components
2. **Configurable Responses**: Allow customization of mock responses
3. **State Management**: Maintain mock state when appropriate
4. **Error Simulation**: Test both success and failure scenarios

### Mock Organization

1. **Separate Mock Modules**: Organize mocks by functionality
2. **Builder Pattern**: Use builders for complex mock setup
3. **Reusable Mocks**: Create mocks that can be reused across tests
4. **Mock Factories**: Use factories for common mock configurations

### Testing Strategy

1. **Unit Testing**: Use mocks for isolated component testing
2. **Integration Testing**: Combine mocks with real components
3. **Error Testing**: Use mocks to simulate error conditions
4. **Performance Testing**: Use mocks to test performance characteristics

### Mock Maintenance

1. **Keep Mocks Simple**: Avoid complex mock logic
2. **Update Mocks**: Keep mocks in sync with interface changes
3. **Document Mocks**: Document mock behavior and usage
4. **Test Mocks**: Ensure mocks work correctly

## Next Steps

- [Test Utilities](./test_utilities.md) - Common testing helpers
- [Unit Testing](./unit_testing.md) - Individual component testing
- [Integration Testing](./integration_testing.md) - Module interaction testing
- [Benchmarking](./benchmarking.md) - Performance testing
