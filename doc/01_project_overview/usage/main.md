# Usage Guide

## Overview

This guide provides comprehensive examples of how to use the Deribit FIX Client for various trading operations. From basic connection setup to advanced trading strategies, you'll find working code examples for common use cases.

## Quick Start

### **Basic Connection**

```rust
use deribit_fix::client::FixClient;
use deribit_fix::config::FixConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = FixConfig::new()
        .with_sender_comp_id("YOUR_SENDER_ID".to_string())
        .with_target_comp_id("DERIBIT".to_string())
        .with_api_key("YOUR_API_KEY".to_string())
        .with_secret_key("YOUR_SECRET_KEY".to_string());

    let mut client = FixClient::new(config);
    client.connect().await?;
    
    Ok(())
}
```

## Usage Examples

### **[Basic Examples](basic_example.md)**
- **Connection Management**: Establish and maintain FIX connections
- **Authentication**: Logon and session management
- **Heartbeat**: Keep connections alive
- **Error Handling**: Basic error handling patterns

### **[Advanced Examples](advanced_example.md)**
- **Order Management**: Place, cancel, and modify orders
- **Market Data**: Subscribe to real-time market feeds
- **Position Management**: Track and manage trading positions
- **Risk Management**: Implement position limits and controls

## Common Patterns

### **Async/Await Pattern**

```rust
use tokio::time::{sleep, Duration};

async fn trading_loop(client: &mut FixClient) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        // Your trading logic here
        client.send_heartbeat().await?;
        
        // Wait before next iteration
        sleep(Duration::from_millis(100)).await;
    }
}
```

### **Error Handling Pattern**

```rust
use deribit_fix::error::DeribitFixError;

async fn handle_operation(client: &mut FixClient) -> Result<(), DeribitFixError> {
    match client.place_order(order).await {
        Ok(order_id) => {
            println!("Order placed successfully: {}", order_id);
            Ok(())
        }
        Err(DeribitFixError::AuthenticationFailed) => {
            eprintln!("Authentication failed, reconnecting...");
            client.reconnect().await?;
            Ok(())
        }
        Err(e) => {
            eprintln!("Unexpected error: {}", e);
            Err(e)
        }
    }
}
```

### **Configuration Pattern**

```rust
use deribit_fix::config::FixConfig;

fn create_config() -> FixConfig {
    FixConfig::new()
        .with_sender_comp_id(std::env::var("DERIBIT_SENDER_ID").unwrap_or_default())
        .with_target_comp_id("DERIBIT".to_string())
        .with_api_key(std::env::var("DERIBIT_API_KEY").unwrap_or_default())
        .with_secret_key(std::env::var("DERIBIT_SECRET_KEY").unwrap_or_default())
        .with_host("fix.deribit.com".to_string())
        .with_port(443)
        .with_use_ssl(true)
        .with_heartbeat_interval(30)
        .with_cancel_on_disconnect(true)
}
```

## Trading Operations

### **Order Types**

```rust
use deribit_fix::message::orders::{OrderSide, OrderType, TimeInForce};

// Market Order
let market_order = NewOrderSingle::new(
    "BTC-PERPETUAL".to_string(),
    OrderSide::Buy,
    OrderType::Market,
    TimeInForce::ImmediateOrCancel,
    100.0, // quantity
);

// Limit Order
let limit_order = NewOrderSingle::new(
    "BTC-PERPETUAL".to_string(),
    OrderSide::Sell,
    OrderType::Limit,
    TimeInForce::GoodTillCancelled,
    50.0, // quantity
).with_price(50000.0); // limit price
```

### **Market Data Subscriptions**

```rust
use deribit_fix::message::market_data::{MarketDataRequest, MdEntryType, MdSubscriptionRequestType};

let md_request = MarketDataRequest::subscription(
    "md_req_001".to_string(),
    vec!["BTC-PERPETUAL".to_string()],
    vec![MdEntryType::Bid, MdEntryType::Offer, MdEntryType::Trade],
    MdUpdateType::IncrementalRefresh,
);

client.send_market_data_request(md_request).await?;
```

### **Position Management**

```rust
use deribit_fix::message::orders::RequestForPositions;

let pos_request = RequestForPositions::new(
    "pos_req_001".to_string(),
    "BTC-PERPETUAL".to_string(),
);

client.send_position_request(pos_request).await?;
```

## Best Practices

### **Connection Management**

```rust
// Always handle reconnection
async fn maintain_connection(client: &mut FixClient) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        if !client.is_connected().await? {
            println!("Connection lost, reconnecting...");
            client.reconnect().await?;
        }
        
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
```

### **Rate Limiting**

```rust
use tokio::time::{sleep, Duration};

// Implement rate limiting for high-frequency operations
async fn rate_limited_operation<F, Fut, T>(operation: F, delay_ms: u64) -> Result<T, Box<dyn std::error::Error>>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<T, Box<dyn std::error::Error>>>,
{
    let result = operation().await;
    sleep(Duration::from_millis(delay_ms)).await;
    result
}
```

### **Logging and Monitoring**

```rust
use log::{info, warn, error};

// Structured logging for trading operations
async fn log_trading_operation(operation: &str, details: &str) {
    info!("Trading operation: {} - {}", operation, details);
}

// Error logging with context
async fn log_error_with_context(error: &dyn std::error::Error, context: &str) {
    error!("Error in {}: {}", context, error);
}
```

## Performance Optimization

### **Message Batching**

```rust
// Batch multiple orders for efficiency
async fn batch_orders(client: &mut FixClient, orders: Vec<NewOrderSingle>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut order_ids = Vec::new();
    
    for order in orders {
        let order_id = client.place_order(order).await?;
        order_ids.push(order_id);
        
        // Small delay to avoid overwhelming the exchange
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    
    Ok(order_ids)
}
```

### **Connection Pooling**

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

// Connection pool for high-frequency trading
struct ConnectionPool {
    connections: Vec<Arc<Mutex<FixClient>>>,
    current: usize,
}

impl ConnectionPool {
    async fn get_connection(&mut self) -> Arc<Mutex<FixClient>> {
        let connection = Arc::clone(&self.connections[self.current]);
        self.current = (self.current + 1) % self.connections.len();
        connection
    }
}
```

## Error Handling

### **Common Error Types**

```rust
use deribit_fix::error::DeribitFixError;

match error {
    DeribitFixError::AuthenticationFailed => {
        // Handle authentication issues
        client.reconnect().await?;
    }
    DeribitFixError::ConnectionFailed => {
        // Handle connection issues
        tokio::time::sleep(Duration::from_secs(5)).await;
        client.reconnect().await?;
    }
    DeribitFixError::OrderRejected(reason) => {
        // Handle order rejection
        println!("Order rejected: {}", reason);
    }
    _ => {
        // Handle other errors
        eprintln!("Unexpected error: {}", error);
    }
}
```

### **Retry Logic**

```rust
use tokio::time::{sleep, Duration};

async fn retry_operation<F, Fut, T>(
    operation: F,
    max_retries: u32,
    delay_ms: u64,
) -> Result<T, Box<dyn std::error::Error>>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, Box<dyn std::error::Error>>>,
{
    let mut attempts = 0;
    
    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                attempts += 1;
                if attempts >= max_retries {
                    return Err(e);
                }
                
                println!("Attempt {} failed, retrying in {}ms...", attempts, delay_ms);
                sleep(Duration::from_millis(delay_ms)).await;
            }
        }
    }
}
```

## Next Steps

1. **[Basic Examples](basic_example.md)** - Start with fundamental operations
2. **[Advanced Examples](advanced_example.md)** - Explore complex trading scenarios
3. **[API Reference](../../02_api_reference/main.md)** - Detailed API documentation
4. **[Architecture](../architecture/main.md)** - Understand internal design

---

**Ready to dive deeper?** Check out the [Basic Examples](basic_example.md) to get started with practical code!
