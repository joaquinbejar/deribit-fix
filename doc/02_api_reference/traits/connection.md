# Connection

## Overview

The `Connection` trait defines the interface for managing network connections to Deribit's FIX gateway, including connection lifecycle, message handling, and error management.

## Purpose

- **Connection Management**: Establishes and maintains network connections
- **Message Transport**: Handles FIX message transmission and reception
- **Connection State**: Manages connection status and health monitoring
- **Error Handling**: Provides connection-specific error handling and recovery

## Public Interface

### Trait Definition

```rust
#[async_trait]
pub trait Connection: Send + Sync {
    /// Connect to the FIX gateway
    async fn connect(&mut self) -> Result<(), ConnectionError>

    /// Disconnect from the FIX gateway
    async fn disconnect(&mut self) -> Result<(), ConnectionError>

    /// Check if connection is active
    fn is_connected(&self) -> bool

    /// Get connection status
    fn connection_status(&self) -> ConnectionStatus

    /// Send a FIX message
    async fn send_message(&mut self, message: FixMessage) -> Result<(), ConnectionError>

    /// Receive a FIX message
    async fn receive_message(&mut self) -> Result<Option<FixMessage>, ConnectionError>

    /// Get connection statistics
    fn get_stats(&self) -> ConnectionStats

    /// Ping the connection to check health
    async fn ping(&mut self) -> Result<Duration, ConnectionError>

    /// Reconnect with exponential backoff
    async fn reconnect(&mut self) -> Result<(), ConnectionError>

    /// Get connection configuration
    fn get_config(&self) -> &ConnectionConfig

    /// Set connection event handler
    fn set_event_handler(&mut self, handler: Box<dyn ConnectionEventHandler>)
}
```

### Associated Types

```rust
/// Connection status enumeration
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Authenticating,
    Authenticated,
    Reconnecting,
    Error(ConnectionError),
}

/// Connection statistics
pub struct ConnectionStats {
    pub messages_sent: u64,
    pub messages_received: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub connection_time: Duration,
    pub last_heartbeat: Option<DateTime<Utc>>,
    pub reconnect_count: u32,
    pub errors_count: u32,
}

/// Connection event handler trait
pub trait ConnectionEventHandler: Send + Sync {
    fn on_connect(&self, connection_id: &str);
    fn on_disconnect(&self, connection_id: &str, reason: &str);
    fn on_error(&self, connection_id: &str, error: &ConnectionError);
    fn on_reconnect(&self, connection_id: &str, attempt: u32);
}
```

## Usage Examples

### Basic Connection Management

```rust
use deribit_fix::connection::{Connection, ConnectionStatus, ConnectionConfig};
use deribit_fix::message::FixMessage;

// Create connection with configuration
let mut connection = TcpConnection::new(connection_config);

// Connect to FIX gateway
match connection.connect().await {
    Ok(()) => println!("Successfully connected to Deribit FIX gateway"),
    Err(error) => eprintln!("Failed to connect: {:?}", error),
}

// Check connection status
if connection.is_connected() {
    println!("Connection is active");
    println!("Status: {:?}", connection.connection_status());
}

// Send a FIX message
let message = FixMessage::heartbeat();
match connection.send_message(message).await {
    Ok(()) => println!("Message sent successfully"),
    Err(error) => eprintln!("Failed to send message: {:?}", error),
}

// Receive messages
loop {
    match connection.receive_message().await {
        Ok(Some(message)) => {
            println!("Received message: {:?}", message);
            // Process the message
        }
        Ok(None) => {
            // No message available, continue
            break;
        }
        Err(error) => {
            eprintln!("Error receiving message: {:?}", error);
            break;
        }
    }
}

// Disconnect
connection.disconnect().await?;
```

### Connection Monitoring and Health Checks

```rust
// Monitor connection health
let stats = connection.get_stats();
println!("Messages sent: {}", stats.messages_sent);
println!("Messages received: {}", stats.messages_received);
println!("Connection time: {:?}", stats.connection_time);
println!("Reconnect count: {}", stats.reconnect_count);

// Ping connection to check health
match connection.ping().await {
    Ok(latency) => println!("Connection healthy, latency: {:?}", latency),
    Err(error) => {
        eprintln!("Connection health check failed: {:?}", error);
        
        // Attempt reconnection
        if let Err(reconnect_error) = connection.reconnect().await {
            eprintln!("Reconnection failed: {:?}", reconnect_error);
        }
    }
}
```

### Event Handling

```rust
// Create custom event handler
struct MyConnectionHandler {
    connection_id: String,
}

impl ConnectionEventHandler for MyConnectionHandler {
    fn on_connect(&self, connection_id: &str) {
        println!("Connection {} established", connection_id);
    }

    fn on_disconnect(&self, connection_id: &str, reason: &str) {
        println!("Connection {} disconnected: {}", connection_id, reason);
    }

    fn on_error(&self, connection_id: &str, error: &ConnectionError) {
        eprintln!("Connection {} error: {:?}", connection_id, error);
    }

    fn on_reconnect(&self, connection_id: &str, attempt: u32) {
        println!("Connection {} reconnecting, attempt {}", connection_id, attempt);
    }
}

// Set event handler
let handler = Box::new(MyConnectionHandler {
    connection_id: "main".to_string(),
});
connection.set_event_handler(handler);
```

### Connection Configuration

```rust
// Get connection configuration
let config = connection.get_config();
println!("Host: {}", config.host);
println!("Port: {}", config.port);
println!("Timeout: {:?}", config.timeout);
println!("Keep alive: {}", config.keep_alive);

// Create connection with specific configuration
let connection_config = ConnectionConfig {
    host: "fix.deribit.com".to_string(),
    port: 443,
    timeout: Duration::from_secs(30),
    keep_alive: true,
    max_reconnect_attempts: 5,
    reconnect_delay: Duration::from_secs(5),
    heartbeat_interval: Duration::from_secs(30),
    ..ConnectionConfig::default()
};

let mut connection = TcpConnection::new(connection_config);
```

## Module Dependencies

### Direct Dependencies

- **`async_trait`**: For async trait methods
- **`message`**: `FixMessage`
- **`config`**: `ConnectionConfig`
- **`error`**: `ConnectionError`
- **`chrono`**: `DateTime<Utc>`
- **`std::time`**: `Duration`

### Related Types

- **`ConnectionStatus`**: Enum representing connection states
- **`ConnectionStats`**: Statistics about connection performance
- **`ConnectionConfig`**: Configuration parameters for connections
- **`ConnectionError`**: Specific error types for connection operations
- **`ConnectionEventHandler`**: Trait for handling connection events
- **`FixMessage`**: FIX protocol messages

## Testing

### Mock Testing

```rust
use mockall::predicate::*;
use mockall::*;

mock! {
    ConnectionMock {}
    
    #[async_trait]
    impl Connection for ConnectionMock {
        async fn connect(&mut self) -> Result<(), ConnectionError>;
        async fn disconnect(&mut self) -> Result<(), ConnectionError>;
        fn is_connected(&self) -> bool;
        fn connection_status(&self) -> ConnectionStatus;
        async fn send_message(&mut self, message: FixMessage) -> Result<(), ConnectionError>;
        async fn receive_message(&mut self) -> Result<Option<FixMessage>, ConnectionError>;
        fn get_stats(&self) -> ConnectionStats;
        async fn ping(&mut self) -> Result<Duration, ConnectionError>;
        async fn reconnect(&mut self) -> Result<(), ConnectionError>;
        fn get_config(&self) -> &ConnectionConfig;
        fn set_event_handler(&mut self, handler: Box<dyn ConnectionEventHandler>);
    }
}

#[tokio::test]
async fn test_connection_mock() {
    let mut mock = MockConnectionMock::new();
    
    // Set expectations
    mock.expect_connect()
        .times(1)
        .returning(|| Ok(()));
    
    mock.expect_is_connected()
        .times(1)
        .returning(|| true);
    
    // Test connection
    let result = mock.connect().await;
    assert!(result.is_ok());
    assert!(mock.is_connected());
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_connection_lifecycle() {
    let config = ConnectionConfig::test_config();
    let mut connection = TcpConnection::new(config);

    // Test connection
    assert!(connection.connect().await.is_ok());
    assert!(connection.is_connected());
    assert_eq!(connection.connection_status(), ConnectionStatus::Connected);

    // Test message sending
    let message = FixMessage::heartbeat();
    assert!(connection.send_message(message).await.is_ok());

    // Test disconnection
    assert!(connection.disconnect().await.is_ok());
    assert!(!connection.is_connected());
    assert_eq!(connection.connection_status(), ConnectionStatus::Disconnected);
}

#[tokio::test]
async fn test_connection_reconnection() {
    let config = ConnectionConfig::test_config();
    let mut connection = TcpConnection::new(config);

    // Initial connection
    connection.connect().await?;
    
    // Simulate connection failure and reconnection
    connection.disconnect().await?;
    assert!(connection.reconnect().await.is_ok());
    assert!(connection.is_connected());
}
```

## Performance Considerations

- **Connection Pooling**: Reuse connections when possible
- **Async Operations**: Use async I/O for non-blocking operations
- **Message Batching**: Batch multiple messages when possible
- **Connection Monitoring**: Monitor connection health proactively
- **Efficient Reconnection**: Implement smart reconnection strategies

## Security Considerations

- **TLS Encryption**: Use TLS for secure connections
- **Authentication**: Implement proper authentication mechanisms
- **Connection Validation**: Validate connection parameters
- **Access Control**: Restrict connection access to authorized clients
- **Audit Logging**: Log all connection events and errors
