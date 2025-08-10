# Traits

This section provides comprehensive documentation for all public traits within the `deribit-fix` crate. Traits define the interfaces and behaviors that types can implement, enabling polymorphism and code reuse.

## Overview

Traits in `deribit-fix` serve several key purposes:
- **Interface Definition**: Define common behavior across different types
- **Abstraction**: Provide generic implementations that work with any type implementing a trait
- **Extensibility**: Allow users to implement custom behavior for existing types
- **Testing**: Enable mock implementations for testing and development

## Trait Categories

### Core Traits
Core traits that define fundamental behaviors and interfaces.

### Business Logic Traits
Traits that define business logic operations and workflows.

### Utility Traits
Traits that provide common utility functionality.

## Core Traits

### `Connection`
Defines the interface for connection management.

```rust
use deribit_fix::connection::Connection;
use deribit_fix::error::DeribitFixError;
use tokio::net::TcpStream;

#[async_trait::async_trait]
pub trait Connection: Send + Sync {
    /// Establishes a connection to the server
    async fn connect(&mut self) -> Result<(), DeribitFixError>;
    
    /// Closes the connection
    async fn disconnect(&mut self) -> Result<(), DeribitFixError>;
    
    /// Checks if the connection is active
    fn is_connected(&self) -> bool;
    
    /// Sends a message over the connection
    async fn send(&mut self, message: &[u8]) -> Result<(), DeribitFixError>;
    
    /// Receives a message from the connection
    async fn receive(&mut self) -> Result<Vec<u8>, DeribitFixError>;
}

// Example implementation for TcpStream
pub struct TcpConnection {
    stream: Option<TcpStream>,
    config: ConnectionConfig,
}

#[async_trait::async_trait]
impl Connection for TcpConnection {
    async fn connect(&mut self) -> Result<(), DeribitFixError> {
        let stream = TcpStream::connect(&self.config.endpoint).await
            .map_err(|e| DeribitFixError::ConnectionError(e.to_string()))?;
        
        self.stream = Some(stream);
        Ok(())
    }
    
    async fn disconnect(&mut self) -> Result<(), DeribitFixError> {
        if let Some(mut stream) = self.stream.take() {
            stream.shutdown().await
                .map_err(|e| DeribitFixError::ConnectionError(e.to_string()))?;
        }
        Ok(())
    }
    
    fn is_connected(&self) -> bool {
        self.stream.is_some()
    }
    
    async fn send(&mut self, message: &[u8]) -> Result<(), DeribitFixError> {
        if let Some(ref mut stream) = self.stream {
            use tokio::io::AsyncWriteExt;
            stream.write_all(message).await
                .map_err(|e| DeribitFixError::ConnectionError(e.to_string()))?;
            Ok(())
        } else {
            Err(DeribitFixError::ConnectionError("Not connected".to_string()))
        }
    }
    
    async fn receive(&mut self) -> Result<Vec<u8>, DeribitFixError> {
        if let Some(ref mut stream) = self.stream {
            use tokio::io::AsyncReadExt;
            let mut buffer = vec![0u8; 1024];
            let n = stream.read(&mut buffer).await
                .map_err(|e| DeribitFixError::ConnectionError(e.to_string()))?;
            buffer.truncate(n);
            Ok(buffer)
        } else {
            Err(DeribitFixError::ConnectionError("Not connected".to_string()))
        }
    }
}
```

### `SessionManager`
Defines the interface for session management.

```rust
use deribit_fix::session::SessionManager;
use deribit_fix::error::DeribitFixError;
use deribit_fix::model::FixMessage;

#[async_trait::async_trait]
pub trait SessionManager: Send + Sync {
    /// Initiates a logon session
    async fn logon(&mut self) -> Result<(), DeribitFixError>;
    
    /// Terminates the session
    async fn logout(&mut self) -> Result<(), DeribitFixError>;
    
    /// Sends a heartbeat message
    async fn heartbeat(&mut self) -> Result<(), DeribitFixError>;
    
    /// Checks if the session is active
    fn is_logged_in(&self) -> bool;
    
    /// Gets the current sequence number
    fn sequence_number(&self) -> u32;
    
    /// Increments the sequence number
    fn increment_sequence(&mut self);
    
    /// Resets the sequence number
    fn reset_sequence(&mut self, new_sequence: u32);
    
    /// Processes incoming messages
    async fn process_message(&mut self, message: FixMessage) -> Result<(), DeribitFixError>;
}

// Example implementation
pub struct FixSession {
    logged_in: bool,
    sequence_number: u32,
    config: AuthConfig,
}

#[async_trait::async_trait]
impl SessionManager for FixSession {
    async fn logon(&mut self) -> Result<(), DeribitFixError> {
        // Create logon message
        let logon_msg = FixMessage::logon(
            &self.config.api_key,
            &self.config.api_secret,
            self.sequence_number,
        )?;
        
        // Send logon message
        // ... implementation details ...
        
        self.logged_in = true;
        self.increment_sequence();
        Ok(())
    }
    
    async fn logout(&mut self) -> Result<(), DeribitFixError> {
        let logout_msg = FixMessage::logout(self.sequence_number)?;
        
        // Send logout message
        // ... implementation details ...
        
        self.logged_in = false;
        self.increment_sequence();
        Ok(())
    }
    
    async fn heartbeat(&mut self) -> Result<(), DeribitFixError> {
        let heartbeat_msg = FixMessage::heartbeat(self.sequence_number)?;
        
        // Send heartbeat message
        // ... implementation details ...
        
        self.increment_sequence();
        Ok(())
    }
    
    fn is_logged_in(&self) -> bool {
        self.logged_in
    }
    
    fn sequence_number(&self) -> u32 {
        self.sequence_number
    }
    
    fn increment_sequence(&mut self) {
        self.sequence_number += 1;
    }
    
    fn reset_sequence(&mut self, new_sequence: u32) {
        self.sequence_number = new_sequence;
    }
    
    async fn process_message(&mut self, message: FixMessage) -> Result<(), DeribitFixError> {
        match message.msg_type() {
            "0" => { // Heartbeat
                // Process heartbeat
                Ok(())
            }
            "5" => { // Logout
                self.logged_in = false;
                Ok(())
            }
            "A" => { // Logon
                self.logged_in = true;
                Ok(())
            }
            _ => {
                // Process other message types
                Ok(())
            }
        }
    }
}
```

## Business Logic Traits

### `OrderHandler`
Defines the interface for order management operations.

```rust
use deribit_fix::model::{Order, ExecutionReport};
use deribit_fix::error::DeribitFixError;

#[async_trait::async_trait]
pub trait OrderHandler: Send + Sync {
    /// Places a new order
    async fn place_order(&mut self, order: Order) -> Result<ExecutionReport, DeribitFixError>;
    
    /// Cancels an existing order
    async fn cancel_order(&mut self, order_id: &str) -> Result<ExecutionReport, DeribitFixError>;
    
    /// Modifies an existing order
    async fn modify_order(&mut self, order: Order) -> Result<ExecutionReport, DeribitFixError>;
    
    /// Gets the status of an order
    async fn get_order_status(&self, order_id: &str) -> Result<ExecutionReport, DeribitFixError>;
    
    /// Gets all active orders
    async fn get_active_orders(&self) -> Result<Vec<ExecutionReport>, DeribitFixError>;
}

// Example implementation
pub struct DefaultOrderHandler {
    client: DeribitFixClient,
}

#[async_trait::async_trait]
impl OrderHandler for DefaultOrderHandler {
    async fn place_order(&mut self, order: Order) -> Result<ExecutionReport, DeribitFixError> {
        self.client.place_order(order).await
    }
    
    async fn cancel_order(&mut self, order_id: &str) -> Result<ExecutionReport, DeribitFixError> {
        self.client.cancel_order(order_id).await
    }
    
    async fn modify_order(&mut self, order: Order) -> Result<ExecutionReport, DeribitFixError> {
        self.client.modify_order(order).await
    }
    
    async fn get_order_status(&self, order_id: &str) -> Result<ExecutionReport, DeribitFixError> {
        self.client.get_order_status(order_id).await
    }
    
    async fn get_active_orders(&self) -> Result<Vec<ExecutionReport>, DeribitFixError> {
        self.client.get_active_orders().await
    }
}
```

### `MarketDataHandler`
Defines the interface for market data operations.

```rust
use deribit_fix::model::MarketData;
use deribit_fix::error::DeribitFixError;

#[async_trait::async_trait]
pub trait MarketDataHandler: Send + Sync {
    /// Subscribes to market data for a specific instrument
    async fn subscribe_market_data(&mut self, instrument: &str) -> Result<(), DeribitFixError>;
    
    /// Unsubscribes from market data
    async fn unsubscribe_market_data(&mut self, instrument: &str) -> Result<(), DeribitFixError>;
    
    /// Gets a snapshot of market data
    async fn get_market_data_snapshot(&self, instrument: &str) -> Result<MarketData, DeribitFixError>;
    
    /// Gets real-time market data updates
    async fn get_market_data_stream(&self, instrument: &str) -> Result<tokio::sync::mpsc::Receiver<MarketData>, DeribitFixError>;
}

// Example implementation
pub struct DefaultMarketDataHandler {
    client: DeribitFixClient,
}

#[async_trait::async_trait]
impl MarketDataHandler for DefaultMarketDataHandler {
    async fn subscribe_market_data(&mut self, instrument: &str) -> Result<(), DeribitFixError> {
        self.client.subscribe_market_data(instrument).await
    }
    
    async fn unsubscribe_market_data(&mut self, instrument: &str) -> Result<(), DeribitFixError> {
        self.client.unsubscribe_market_data(instrument).await
    }
    
    async fn get_market_data_snapshot(&self, instrument: &str) -> Result<MarketData, DeribitFixError> {
        self.client.get_market_data_snapshot(instrument).await
    }
    
    async fn get_market_data_stream(&self, instrument: &str) -> Result<tokio::sync::mpsc::Receiver<MarketData>, DeribitFixError> {
        self.client.get_market_data_stream(instrument).await
    }
}
```

## Utility Traits

### `Configurable`
Defines the interface for configuration management.

```rust
use deribit_fix::config::Config;
use deribit_fix::error::DeribitFixError;

pub trait Configurable {
    /// Gets the current configuration
    fn config(&self) -> &Config;
    
    /// Updates the configuration
    fn update_config(&mut self, config: Config) -> Result<(), DeribitFixError>;
    
    /// Validates the current configuration
    fn validate_config(&self) -> Result<(), DeribitFixError>;
    
    /// Reloads configuration from source
    async fn reload_config(&mut self) -> Result<(), DeribitFixError>;
}

// Example implementation
pub struct ConfigurableClient {
    config: Config,
}

impl Configurable for ConfigurableClient {
    fn config(&self) -> &Config {
        &self.config
    }
    
    fn update_config(&mut self, config: Config) -> Result<(), DeribitFixError> {
        config.validate()?;
        self.config = config;
        Ok(())
    }
    
    fn validate_config(&self) -> Result<(), DeribitFixError> {
        self.config.validate()
    }
    
    async fn reload_config(&mut self) -> Result<(), DeribitFixError> {
        let new_config = Config::from_file(&self.config.config_path).await?;
        self.update_config(new_config)
    }
}
```

### `Retryable`
Defines the interface for operations that can be retried.

```rust
use deribit_fix::error::DeribitFixError;
use std::time::Duration;

pub trait Retryable {
    /// The maximum number of retry attempts
    fn max_retries(&self) -> u32;
    
    /// The delay between retry attempts
    fn retry_delay(&self) -> Duration;
    
    /// Whether the operation should be retried for a given error
    fn should_retry(&self, error: &DeribitFixError) -> bool;
    
    /// Executes an operation with retry logic
    async fn execute_with_retry<F, Fut, T>(&self, operation: F) -> Result<T, DeribitFixError>
    where
        F: Fn() -> Fut + Send + Sync,
        Fut: std::future::Future<Output = Result<T, DeribitFixError>> + Send,
        T: Send,
    {
        let mut attempts = 0;
        let max_attempts = self.max_retries();
        
        loop {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(error) => {
                    attempts += 1;
                    
                    if attempts > max_attempts || !self.should_retry(&error) {
                        return Err(error);
                    }
                    
                    tokio::time::sleep(self.retry_delay()).await;
                }
            }
        }
    }
}

// Example implementation
pub struct RetryableClient {
    max_retries: u32,
    retry_delay: Duration,
}

impl Retryable for RetryableClient {
    fn max_retries(&self) -> u32 {
        self.max_retries
    }
    
    fn retry_delay(&self) -> Duration {
        self.retry_delay
    }
    
    fn should_retry(&self, error: &DeribitFixError) -> bool {
        match error {
            DeribitFixError::ConnectionError(_) => true,
            DeribitFixError::TimeoutError => true,
            DeribitFixError::FIXProtocolError(_) => false,
            DeribitFixError::BusinessLogicError(_) => false,
            _ => false,
        }
    }
}
```

## Trait Implementation Patterns

### Default Implementations
Many traits provide default implementations for common operations:

```rust
use deribit_fix::error::DeribitFixError;

pub trait MessageValidator {
    /// Validates a FIX message
    fn validate(&self) -> Result<(), DeribitFixError>;
    
    /// Checks if the message is valid
    fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }
    
    /// Gets validation errors
    fn validation_errors(&self) -> Vec<String> {
        match self.validate() {
            Ok(()) => vec![],
            Err(error) => vec![error.to_string()],
        }
    }
}
```

### Generic Implementations
Traits can be implemented for generic types:

```rust
use deribit_fix::model::FixMessage;
use serde::{Deserialize, Serialize};

pub trait MessageBuilder<T> {
    fn build(&self) -> Result<T, DeribitFixError>;
}

impl<T> MessageBuilder<T> for FixMessage
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    fn build(&self) -> Result<T, DeribitFixError> {
        // Implementation for building T from FixMessage
        todo!()
    }
}
```

### Conditional Implementations
Traits can be implemented conditionally based on type constraints:

```rust
use std::fmt::Debug;

pub trait Debuggable {
    fn debug_info(&self) -> String;
}

impl<T> Debuggable for T
where
    T: Debug,
{
    fn debug_info(&self) -> String {
        format!("{:?}", self)
    }
}
```

## Testing with Traits

### Mock Implementations
Traits enable easy mocking for testing:

```rust
use mockall::automock;
use deribit_fix::error::DeribitFixError;

#[automock]
#[async_trait::async_trait]
pub trait TestConnection {
    async fn send(&mut self, data: &[u8]) -> Result<(), DeribitFixError>;
    async fn receive(&mut self) -> Result<Vec<u8>, DeribitFixError>;
}

#[tokio::test]
async fn test_connection_mock() {
    let mut mock = MockTestConnection::new();
    
    mock.expect_send()
        .times(1)
        .returning(|_| Ok(()));
    
    mock.expect_receive()
        .times(1)
        .returning(|| Ok(b"test data".to_vec()));
    
    // Test with mock
    let result = mock.send(b"hello").await;
    assert!(result.is_ok());
}
```

### Trait Objects
Traits can be used as trait objects for runtime polymorphism:

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Client {
    connection: Arc<Mutex<Box<dyn Connection>>>,
}

impl Client {
    pub fn new(connection: Box<dyn Connection>) -> Self {
        Self {
            connection: Arc::new(Mutex::new(connection)),
        }
    }
    
    pub async fn send_message(&self, message: &[u8]) -> Result<(), DeribitFixError> {
        let mut conn = self.connection.lock().await;
        conn.send(message).await
    }
}
```

## Best Practices

### Trait Design
- **Single Responsibility**: Each trait should have a single, well-defined purpose
- **Minimal Interface**: Include only essential methods in the trait
- **Default Implementations**: Provide sensible defaults where possible
- **Documentation**: Document all trait methods with examples

### Implementation Guidelines
- **Consistent Naming**: Use consistent naming conventions across traits
- **Error Handling**: Return appropriate error types from trait methods
- **Async Support**: Use `async_trait` for async trait methods
- **Send + Sync**: Ensure traits are thread-safe when appropriate

### Testing Considerations
- **Mocking**: Design traits to be easily mockable for testing
- **Trait Objects**: Consider whether traits need to be object-safe
- **Generic Constraints**: Use generic constraints to enable compile-time optimizations

## Related Documentation

- [Modules](modules.md) - Overview of all modules
- [Structs](structs.md) - Documentation of public structs
- [API Reference](main.md) - Main API documentation
- [Testing Guide](../03_development_guide/testing/main.md) - Testing strategies and examples
