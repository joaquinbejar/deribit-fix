# Functions

This section provides comprehensive documentation for all public functions within the `deribit-fix` crate. Functions are the primary interface for interacting with the library and performing various operations.

## Overview

Functions in `deribit-fix` are organized into several categories:
- **Client Functions**: Core client operations and lifecycle management
- **Connection Functions**: Network connection management and utilities
- **Session Functions**: FIX session management and authentication
- **Message Functions**: FIX message creation, parsing, and manipulation
- **Utility Functions**: Common utilities and helper functions
- **Configuration Functions**: Configuration management and validation

## Function Categories

### Client Functions
Core functions for client operations and lifecycle management.

### Connection Functions
Functions for managing network connections and communication.

### Session Functions
Functions for FIX session management and authentication.

### Message Functions
Functions for working with FIX messages and protocols.

### Utility Functions
Common utility functions and helpers.

### Configuration Functions
Functions for configuration management and validation.

## Client Functions

### `DeribitFixClient::new`
Creates a new FIX client instance.

```rust
use deribit_fix::DeribitFixClient;
use deribit_fix::config::Config;

// Create client with default configuration
let client = DeribitFixClient::new();

// Create client with custom configuration
let config = Config::builder()
    .endpoint("wss://www.deribit.com/ws/api/v2")
    .api_key("your_api_key")
    .api_secret("your_api_secret")
    .build()?;

let client = DeribitFixClient::with_config(config);
```

### `DeribitFixClient::connect`
Establishes a connection to the Deribit FIX gateway.

```rust
use deribit_fix::DeribitFixClient;

let mut client = DeribitFixClient::new();

// Connect to the server
match client.connect().await {
    Ok(()) => println!("Connected successfully"),
    Err(e) => eprintln!("Connection failed: {}", e),
}
```

### `DeribitFixClient::disconnect`
Closes the connection to the server.

```rust
use deribit_fix::DeribitFixClient;

let mut client = DeribitFixClient::new();

// Disconnect from the server
match client.disconnect().await {
    Ok(()) => println!("Disconnected successfully"),
    Err(e) => eprintln!("Disconnect failed: {}", e),
}
```

### `DeribitFixClient::is_connected`
Checks if the client is currently connected.

```rust
use deribit_fix::DeribitFixClient;

let client = DeribitFixClient::new();

if client.is_connected() {
    println!("Client is connected");
} else {
    println!("Client is not connected");
}
```

### `DeribitFixClient::place_order`
Places a new order on the exchange.

```rust
use deribit_fix::{DeribitFixClient, Order, OrderSide, OrderType, TimeInForce};
use rust_decimal::Decimal;

let mut client = DeribitFixClient::new();

let order = Order {
    symbol: "BTC-PERP".to_string(),
    side: OrderSide::Buy,
    order_type: OrderType::Limit,
    quantity: Decimal::new(100, 0), // 1.00 BTC
    price: Some(Decimal::new(50000, 0)), // $50,000
    time_in_force: TimeInForce::Day,
    ..Default::default()
};

match client.place_order(order).await {
    Ok(execution_report) => {
        println!("Order placed successfully: {}", execution_report.order_id);
    }
    Err(e) => eprintln!("Failed to place order: {}", e),
}
```

### `DeribitFixClient::cancel_order`
Cancels an existing order.

```rust
use deribit_fix::DeribitFixClient;

let mut client = DeribitFixClient::new();

match client.cancel_order("order_123").await {
    Ok(execution_report) => {
        println!("Order cancelled successfully: {}", execution_report.order_id);
    }
    Err(e) => eprintln!("Failed to cancel order: {}", e),
}
```

### `DeribitFixClient::get_order_status`
Retrieves the status of an order.

```rust
use deribit_fix::DeribitFixClient;

let client = DeribitFixClient::new();

match client.get_order_status("order_123").await {
    Ok(execution_report) => {
        println!("Order status: {:?}", execution_report.exec_type);
        println!("Filled quantity: {}", execution_report.cum_qty);
    }
    Err(e) => eprintln!("Failed to get order status: {}", e),
}
```

### `DeribitFixClient::get_active_orders`
Retrieves all active orders for the current session.

```rust
use deribit_fix::DeribitFixClient;

let client = DeribitFixClient::new();

match client.get_active_orders().await {
    Ok(orders) => {
        println!("Found {} active orders:", orders.len());
        for order in orders {
            println!("- {}: {} {} @ {}", 
                order.order_id, 
                order.side, 
                order.cum_qty, 
                order.symbol
            );
        }
    }
    Err(e) => eprintln!("Failed to get active orders: {}", e),
}
```

## Connection Functions

### `ConnectionManager::new`
Creates a new connection manager instance.

```rust
use deribit_fix::connection::ConnectionManager;
use deribit_fix::config::ConnectionConfig;

let config = ConnectionConfig {
    endpoint: "wss://www.deribit.com/ws/api/v2".to_string(),
    timeout: Duration::from_secs(30),
    max_reconnect_attempts: 5,
    reconnect_delay: Duration::from_secs(1),
};

let connection_manager = ConnectionManager::new(config);
```

### `ConnectionManager::connect`
Establishes a connection using the connection manager.

```rust
use deribit_fix::connection::ConnectionManager;

let mut connection_manager = ConnectionManager::new(config);

match connection_manager.connect().await {
    Ok(()) => println!("Connection established"),
    Err(e) => eprintln!("Connection failed: {}", e),
}
```

### `ConnectionManager::send`
Sends data over the connection.

```rust
use deribit_fix::connection::ConnectionManager;

let mut connection_manager = ConnectionManager::new(config);

let message = b"8=FIX.4.4|9=123|35=A|49=CLIENT|56=SERVER|34=1|52=20231201-10:00:00|98=0|108=30|10=123|";

match connection_manager.send(message).await {
    Ok(()) => println!("Message sent successfully"),
    Err(e) => eprintln!("Failed to send message: {}", e),
}
```

### `ConnectionManager::receive`
Receives data from the connection.

```rust
use deribit_fix::connection::ConnectionManager;

let mut connection_manager = ConnectionManager::new(config);

match connection_manager.receive().await {
    Ok(data) => {
        println!("Received {} bytes", data.len());
        if let Ok(message) = String::from_utf8(data) {
            println!("Message: {}", message);
        }
    }
    Err(e) => eprintln!("Failed to receive data: {}", e),
}
```

## Session Functions

### `SessionManager::new`
Creates a new session manager instance.

```rust
use deribit_fix::session::SessionManager;
use deribit_fix::config::AuthConfig;

let auth_config = AuthConfig {
    api_key: "your_api_key".to_string(),
    api_secret: "your_api_secret".to_string(),
    testnet: false,
};

let session_manager = SessionManager::new(auth_config);
```

### `SessionManager::logon`
Initiates a FIX logon session.

```rust
use deribit_fix::session::SessionManager;

let mut session_manager = SessionManager::new(auth_config);

match session_manager.logon().await {
    Ok(()) => println!("Logged in successfully"),
    Err(e) => eprintln!("Logon failed: {}", e),
}
```

### `SessionManager::logout`
Terminates the FIX session.

```rust
use deribit_fix::session::SessionManager;

let mut session_manager = SessionManager::new(auth_config);

match session_manager.logout().await {
    Ok(()) => println!("Logged out successfully"),
    Err(e) => eprintln!("Logout failed: {}", e),
}
```

### `SessionManager::heartbeat`
Sends a heartbeat message to maintain the session.

```rust
use deribit_fix::session::SessionManager;

let mut session_manager = SessionManager::new(auth_config);

match session_manager.heartbeat().await {
    Ok(()) => println!("Heartbeat sent"),
    Err(e) => eprintln!("Heartbeat failed: {}", e),
}
```

### `SessionManager::is_logged_in`
Checks if the session is currently active.

```rust
use deribit_fix::session::SessionManager;

let session_manager = SessionManager::new(auth_config);

if session_manager.is_logged_in() {
    println!("Session is active");
} else {
    println!("Session is not active");
}
```

## Message Functions

### `FixMessage::new`
Creates a new FIX message.

```rust
use deribit_fix::message::FixMessage;
use deribit_fix::message::MessageHeader;

let header = MessageHeader {
    msg_type: "A".to_string(), // Logon
    sender_comp_id: "CLIENT".to_string(),
    target_comp_id: "SERVER".to_string(),
    msg_seq_num: 1,
    sending_time: chrono::Utc::now(),
};

let message = FixMessage::new(header);
```

### `FixMessage::logon`
Creates a logon message with authentication.

```rust
use deribit_fix::message::FixMessage;

let message = FixMessage::logon(
    "your_api_key",
    "your_api_secret",
    1, // sequence number
)?;

println!("Logon message: {}", message);
```

### `FixMessage::logout`
Creates a logout message.

```rust
use deribit_fix::message::FixMessage;

let message = FixMessage::logout(1)?; // sequence number

println!("Logout message: {}", message);
```

### `FixMessage::heartbeat`
Creates a heartbeat message.

```rust
use deribit_fix::message::FixMessage;

let message = FixMessage::heartbeat(1)?; // sequence number

println!("Heartbeat message: {}", message);
```

### `FixMessage::new_order_single`
Creates a new order single message.

```rust
use deribit_fix::message::FixMessage;
use deribit_fix::model::{Order, OrderSide, OrderType, TimeInForce};
use rust_decimal::Decimal;

let order = Order {
    symbol: "BTC-PERP".to_string(),
    side: OrderSide::Buy,
    order_type: OrderType::Limit,
    quantity: Decimal::new(100, 0), // 1.00 BTC
    price: Some(Decimal::new(50000, 0)), // $50,000
    time_in_force: TimeInForce::Day,
    ..Default::default()
};

let message = FixMessage::new_order_single(&order, 1)?; // sequence number

println!("New order message: {}", message);
```

### `FixMessage::parse`
Parses a FIX message from a string.

```rust
use deribit_fix::message::FixMessage;

let fix_string = "8=FIX.4.4|9=123|35=A|49=CLIENT|56=SERVER|34=1|52=20231201-10:00:00|98=0|108=30|10=123|";

match FixMessage::parse(fix_string) {
    Ok(message) => {
        println!("Parsed message type: {}", message.msg_type());
        println!("Sender: {}", message.sender_comp_id());
        println!("Target: {}", message.target_comp_id());
    }
    Err(e) => eprintln!("Failed to parse message: {}", e),
}
```

### `FixMessage::to_string`
Converts a FIX message to its string representation.

```rust
use deribit_fix::message::FixMessage;

let message = FixMessage::heartbeat(1)?;
let fix_string = message.to_string();

println!("FIX string: {}", fix_string);
```

## Utility Functions

### `generate_nonce`
Generates a unique nonce for authentication.

```rust
use deribit_fix::utils::generate_nonce;

let nonce = generate_nonce();
println!("Generated nonce: {}", nonce);
```

### `calculate_signature`
Calculates the authentication signature for API requests.

```rust
use deribit_fix::utils::calculate_signature;

let api_key = "your_api_key";
let api_secret = "your_api_secret";
let nonce = generate_nonce();
let timestamp = chrono::Utc::now().timestamp_millis();

let signature = calculate_signature(api_secret, &nonce, timestamp)?;
println!("Signature: {}", signature);
```

### `validate_fix_message`
Validates a FIX message for protocol compliance.

```rust
use deribit_fix::utils::validate_fix_message;

let fix_string = "8=FIX.4.4|9=123|35=A|49=CLIENT|56=SERVER|34=1|52=20231201-10:00:00|98=0|108=30|10=123|";

match validate_fix_message(fix_string) {
    Ok(()) => println!("Message is valid"),
    Err(e) => eprintln!("Message validation failed: {}", e),
}
```

### `calculate_checksum`
Calculates the FIX message checksum.

```rust
use deribit_fix::utils::calculate_checksum;

let message_body = "8=FIX.4.4|9=123|35=A|49=CLIENT|56=SERVER|34=1|52=20231201-10:00:00|98=0|108=30|";
let checksum = calculate_checksum(message_body);

println!("Checksum: {:03}", checksum);
```

## Configuration Functions

### `Config::from_file`
Loads configuration from a TOML file.

```rust
use deribit_fix::config::Config;

let config = Config::from_file("config.toml").await?;

println!("Loaded config for endpoint: {}", config.connection.endpoint);
```

### `Config::from_env`
Loads configuration from environment variables.

```rust
use deribit_fix::config::Config;

let config = Config::from_env()?;

println!("Loaded config from environment");
```

### `Config::validate`
Validates the configuration for correctness.

```rust
use deribit_fix::config::Config;

let config = Config::builder()
    .endpoint("wss://www.deribit.com/ws/api/v2")
    .api_key("your_api_key")
    .api_secret("your_api_secret")
    .build()?;

match config.validate() {
    Ok(()) => println!("Configuration is valid"),
    Err(e) => eprintln!("Configuration validation failed: {}", e),
}
```

### `Config::merge`
Merges two configuration objects.

```rust
use deribit_fix::config::Config;

let base_config = Config::default();
let override_config = Config::builder()
    .endpoint("wss://test.deribit.com/ws/api/v2")
    .testnet(true)
    .build()?;

let merged_config = base_config.merge(override_config)?;

println!("Merged endpoint: {}", merged_config.connection.endpoint);
println!("Testnet: {}", merged_config.auth.testnet);
```

## Error Handling Functions

### `DeribitFixError::is_retryable`
Checks if an error can be retried.

```rust
use deribit_fix::error::DeribitFixError;

let error = DeribitFixError::ConnectionError("Connection lost".to_string());

if error.is_retryable() {
    println!("Error can be retried");
} else {
    println!("Error cannot be retried");
}
```

### `DeribitFixError::is_fatal`
Checks if an error is fatal and requires client restart.

```rust
use deribit_fix::error::DeribitFixError;

let error = DeribitFixError::AuthenticationError("Invalid API key".to_string());

if error.is_fatal() {
    println!("Fatal error, client restart required");
} else {
    println!("Non-fatal error");
}
```

## Performance Functions

### `benchmark_message_parsing`
Benchmarks FIX message parsing performance.

```rust
use deribit_fix::utils::benchmark_message_parsing;

let fix_string = "8=FIX.4.4|9=123|35=A|49=CLIENT|56=SERVER|34=1|52=20231201-10:00:00|98=0|108=30|10=123|";
let iterations = 10000;

let result = benchmark_message_parsing(fix_string, iterations)?;

println!("Average parsing time: {:?}", result.average_time);
println!("Throughput: {:.2} messages/second", result.throughput);
```

### `benchmark_connection_throughput`
Benchmarks connection throughput.

```rust
use deribit_fix::utils::benchmark_connection_throughput;

let config = ConnectionConfig::default();
let message_size = 1024; // bytes
let duration = Duration::from_secs(10);

let result = benchmark_connection_throughput(config, message_size, duration).await?;

println!("Average throughput: {:.2} messages/second", result.throughput);
println!("Average latency: {:?}", result.average_latency);
```

## Testing Functions

### `create_test_client`
Creates a test client for testing purposes.

```rust
use deribit_fix::testing::create_test_client;

let test_client = create_test_client().await?;

// Use test client for testing
let result = test_client.place_order(test_order).await;
```

### `create_mock_connection`
Creates a mock connection for testing.

```rust
use deribit_fix::testing::create_mock_connection;

let mock_connection = create_mock_connection().await?;

// Use mock connection for testing
let result = mock_connection.send(test_message).await;
```

## Function Signatures

### Complete Function List
Here's a complete list of all public functions in the crate:

```rust
// Client Functions
DeribitFixClient::new() -> Self
DeribitFixClient::with_config(config: Config) -> Self
DeribitFixClient::connect() -> Result<(), DeribitFixError>
DeribitFixClient::disconnect() -> Result<(), DeribitFixError>
DeribitFixClient::is_connected() -> bool
DeribitFixClient::place_order(order: Order) -> Result<ExecutionReport, DeribitFixError>
DeribitFixClient::cancel_order(order_id: &str) -> Result<ExecutionReport, DeribitFixError>
DeribitFixClient::get_order_status(order_id: &str) -> Result<ExecutionReport, DeribitFixError>
DeribitFixClient::get_active_orders() -> Result<Vec<ExecutionReport>, DeribitFixError>

// Connection Functions
ConnectionManager::new(config: ConnectionConfig) -> Self
ConnectionManager::connect() -> Result<(), DeribitFixError>
ConnectionManager::send(data: &[u8]) -> Result<(), DeribitFixError>
ConnectionManager::receive() -> Result<Vec<u8>, DeribitFixError>

// Session Functions
SessionManager::new(config: AuthConfig) -> Self
SessionManager::logon() -> Result<(), DeribitFixError>
SessionManager::logout() -> Result<(), DeribitFixError>
SessionManager::heartbeat() -> Result<(), DeribitFixError>
SessionManager::is_logged_in() -> bool

// Message Functions
FixMessage::new(header: MessageHeader) -> Self
FixMessage::logon(api_key: &str, api_secret: &str, seq_num: u32) -> Result<Self, DeribitFixError>
FixMessage::logout(seq_num: u32) -> Result<Self, DeribitFixError>
FixMessage::heartbeat(seq_num: u32) -> Result<Self, DeribitFixError>
FixMessage::new_order_single(order: &Order, seq_num: u32) -> Result<Self, DeribitFixError>
FixMessage::parse(input: &str) -> Result<Self, DeribitFixError>
FixMessage::to_string() -> String

// Utility Functions
generate_nonce() -> String
calculate_signature(secret: &str, nonce: &str, timestamp: i64) -> Result<String, DeribitFixError>
validate_fix_message(input: &str) -> Result<(), DeribitFixError>
calculate_checksum(message: &str) -> u8

// Configuration Functions
Config::from_file(path: &str) -> Result<Self, DeribitFixError>
Config::from_env() -> Result<Self, DeribitFixError>
Config::validate() -> Result<(), DeribitFixError>
Config::merge(other: Config) -> Result<Self, DeribitFixError>
```

## Best Practices

### Function Usage
- **Error Handling**: Always handle errors returned by functions
- **Async Operations**: Use `.await` for async functions
- **Resource Management**: Ensure proper cleanup with disconnect functions
- **Configuration**: Validate configuration before using client functions

### Performance Considerations
- **Connection Reuse**: Reuse connections when possible
- **Batch Operations**: Use batch functions for multiple operations
- **Async Patterns**: Leverage async/await for non-blocking operations
- **Memory Management**: Be mindful of message buffer sizes

### Testing
- **Mock Functions**: Use test utility functions for isolated testing
- **Error Scenarios**: Test both success and error cases
- **Performance Testing**: Use benchmark functions to measure performance
- **Integration Testing**: Test with real connections when possible

## Related Documentation

- [Modules](modules.md) - Overview of all modules
- [Structs](structs.md) - Documentation of public structs
- [Traits](traits.md) - Documentation of public traits
- [API Reference](main.md) - Main API documentation
- [Testing Guide](../03_development_guide/testing/main.md) - Testing strategies and examples
