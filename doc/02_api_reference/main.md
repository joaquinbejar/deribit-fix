# API Reference

This section provides comprehensive documentation of the public API surface of the `deribit-fix` crate, including all public structs, enums, traits, and functions.

## Overview

The `deribit-fix` crate exposes a clean, async-first API designed for high-performance FIX protocol communication with Deribit's trading platform. The API is organized into logical modules that handle different aspects of trading operations.

## API Design Principles

- **Async-First**: All I/O operations are asynchronous using Rust's async/await
- **Error Handling**: Comprehensive error types with clear recovery strategies
- **Type Safety**: Strong typing throughout the API surface
- **Builder Pattern**: Fluent interfaces for complex object construction
- **Configuration**: Flexible configuration system with sensible defaults
- **Logging**: Structured logging for debugging and monitoring

## Core API Components

### Main Client Interface
The `DeribitFixClient` struct provides the primary interface for all trading operations:

```rust
pub struct DeribitFixClient {
    // Internal implementation details
}

impl DeribitFixClient {
    // Connection management
    pub async fn new(config: Config) -> Result<Self, DeribitFixError>
    pub async fn connect(&mut self) -> Result<(), DeribitFixError>
    pub async fn disconnect(&mut self) -> Result<(), DeribitFixError>
    
    // Order management
    pub async fn place_order(&mut self, order: Order) -> Result<OrderResponse, DeribitFixError>
    pub async fn cancel_order(&mut self, order_id: &str) -> Result<CancelResponse, DeribitFixError>
    pub async fn modify_order(&mut self, order_id: &str, modifications: OrderModification) -> Result<OrderResponse, DeribitFixError>
    
    // Market data
    pub async fn subscribe_market_data(&mut self, symbol: &str, depth: MarketDepth) -> Result<MarketDataSubscription, DeribitFixError>
    pub async fn unsubscribe_market_data(&mut self, subscription_id: &str) -> Result<(), DeribitFixError>
    
    // Position management
    pub async fn get_positions(&mut self) -> Result<Vec<Position>, DeribitFixError>
    pub async fn get_position(&mut self, symbol: &str) -> Result<Position, DeribitFixError>
    
    // Account information
    pub async fn get_account_summary(&mut self) -> Result<AccountSummary, DeribitFixError>
    pub async fn get_trade_history(&mut self, symbol: Option<&str>, limit: Option<u32>) -> Result<Vec<Trade>, DeribitFixError>
}
```

### Configuration System
The configuration system provides flexible setup options:

```rust
pub struct Config {
    pub connection: ConnectionConfig,
    pub authentication: AuthConfig,
    pub trading: TradingConfig,
    pub logging: LoggingConfig,
    pub performance: PerformanceConfig,
}

// Builder pattern for easy configuration
let config = ConfigBuilder::new()
    .from_env()?
    .from_file("config.toml")?
    .build()?;
```

### Error Handling
Comprehensive error types for different failure modes:

```rust
pub enum DeribitFixError {
    Connection(std::io::Error),
    Authentication(String),
    FixProtocol(String),
    BusinessLogic(String),
    Configuration(String),
    Timeout(String),
    // ... and many more
}
```

## API Modules

### [Modules](modules/main.md)
Documentation for all public modules in the crate:
- **client**: Main client interface and implementation
- **config**: Configuration management and validation
- **error**: Error types and handling utilities
- **message**: FIX message construction and parsing
- **session**: FIX session management
- **types**: Common types and enumerations

### [Structs](structs/main.md)
Detailed documentation for all public structs:
- **Order**: Trading order representation
- **OrderResponse**: Response to order operations
- **Position**: Trading position information (with dedicated PositionReport builder support)
- **MarketData**: Market data structures (including snapshot-only fields: mark, funding, index)
- **AccountSummary**: Account information
- **Config**: Configuration structures

### [Enhanced Features](enhanced_features.md)
Details on Deribit-specific enhancements such as snapshot-only fields in Market Data Snapshot and the dedicated Position Report builder.

### [Traits](traits/main.md)
Documentation for public traits and interfaces:
- **Executable**: Trait for executable trading operations
- **Serializable**: Trait for FIX message serialization
- **Validatable**: Trait for order and message validation

## API Usage Patterns

### 1. Basic Client Setup
```rust
use deribit_fix::{DeribitFixClient, Config, ConfigBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = ConfigBuilder::new()
        .from_env()?
        .build()?;
    
    // Create client
    let mut client = DeribitFixClient::new(config).await?;
    
    // Connect to Deribit
    client.connect().await?;
    
    // Use client for trading operations
    // ...
    
    // Disconnect
    client.disconnect().await?;
    
    Ok(())
}
```

### 2. Order Management
```rust
use deribit_fix::{Order, OrderSide, OrderType, TimeInForce};

// Create an order
let order = Order {
    symbol: "BTC-PERPETUAL".to_string(),
    side: OrderSide::Buy,
    order_type: OrderType::Limit,
    quantity: 1.0,
    price: Some(50000.0),
    time_in_force: TimeInForce::GoodTillCancel,
    client_order_id: "order_123".to_string(),
};

// Place the order
let response = client.place_order(order).await?;
println!("Order placed: {}", response.order_id);

// Cancel the order
let cancel_response = client.cancel_order(&response.order_id).await?;
println!("Order cancelled: {}", cancel_response.order_id);
```

### 3. Market Data Subscription
```rust
use deribit_fix::MarketDepth;

// Subscribe to market data
let subscription = client.subscribe_market_data("BTC-PERPETUAL", MarketDepth::Level2).await?;

// Handle market data updates
// (Implementation depends on your specific needs)

// Unsubscribe when done
client.unsubscribe_market_data(&subscription.subscription_id).await?;
```

### 4. Position Management
```rust
// Get all positions
let positions = client.get_positions().await?;
for position in positions {
    println!("{}: {} @ {}", position.symbol, position.size, position.average_price);
}

// Get specific position
let btc_position = client.get_position("BTC-PERPETUAL").await?;
println!("BTC Position: {} @ {}", btc_position.size, btc_position.average_price);
```

## Error Handling Patterns

### 1. Basic Error Handling
```rust
match client.place_order(order).await {
    Ok(response) => {
        println!("Order placed successfully: {}", response.order_id);
    }
    Err(DeribitFixError::InsufficientFunds(msg)) => {
        eprintln!("Insufficient funds: {}", msg);
    }
    Err(DeribitFixError::InvalidOrder(msg)) => {
        eprintln!("Invalid order: {}", msg);
    }
    Err(error) => {
        eprintln!("Unexpected error: {}", error);
    }
}
```

### 2. Retry Logic
```rust
use deribit_fix::BackoffStrategy;

let result = client.execute_with_retry(
    || client.place_order(order.clone()),
    3,
    BackoffStrategy::Exponential {
        initial_delay: Duration::from_millis(100),
        max_delay: Duration::from_secs(5),
        multiplier: 2.0,
        jitter: true,
    },
).await?;
```

## Performance Considerations

### 1. Connection Pooling
The client supports connection pooling for high-frequency trading:

```rust
let config = Config {
    performance: PerformanceConfig {
        connection_pool_size: 5,
        message_buffer_size: 10000,
        // ... other settings
    },
    // ... other config
};
```

### 2. Batch Operations
For multiple orders, use batch processing:

```rust
let orders = vec![order1, order2, order3];
let responses = client.place_orders_batch(orders).await?;
```

### 3. Async Streams
For real-time data, use async streams:

```rust
use futures::StreamExt;

let mut market_data_stream = client.market_data_stream("BTC-PERPETUAL").await?;
while let Some(update) = market_data_stream.next().await {
    match update {
        Ok(data) => println!("Market update: {:?}", data),
        Err(error) => eprintln!("Stream error: {}", error),
    }
}
```

## Thread Safety

The `DeribitFixClient` is **not** `Send` or `Sync` by default, as it contains mutable state. For multi-threaded usage:

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

let client = Arc::new(Mutex::new(DeribitFixClient::new(config).await?));

// Clone the Arc for each task
let client_clone = Arc::clone(&client);
tokio::spawn(async move {
    let mut client = client_clone.lock().await;
    client.place_order(order).await?;
    Ok::<(), DeribitFixError>(())
});
```

## API Versioning

The current API version is **1.0.0**. Breaking changes will be introduced in major version updates with appropriate migration guides.

## Migration from Previous Versions

### From 0.x to 1.0
- Configuration builder pattern is now the recommended approach
- Error types have been reorganized for better categorization
- Some method signatures have been updated for consistency
- See the [changelog](03_development_guide/changelog/main.md) for detailed migration notes

## Next Steps

- Explore the [Modules](modules/main.md) for detailed module documentation
- Review the [Structs](structs/main.md) for data structure details
- Learn about the [Traits](traits/main.md) for interface definitions
- Check the [Development Guide](03_development_guide/main.md) for testing and contributing information
