# Client Functions

This document describes the main client functions available in the `deribit-fix` crate.

## Overview

The client functions provide the main interface for interacting with the Deribit FIX gateway. These functions handle connection management, order placement, market data retrieval, and position management.

## Main Functions

### Connection Management

#### `connect()`
Establishes a connection to the Deribit FIX gateway.

```rust
pub async fn connect(&mut self) -> Result<(), DeribitFixError>
```

**Parameters:** None

**Returns:** `Result<(), DeribitFixError>`

**Example:**
```rust
let mut client = DeribitFixClient::new(config).await?;
client.connect().await?;
```

#### `disconnect()`
Gracefully disconnects from the FIX gateway.

```rust
pub async fn disconnect(&mut self) -> Result<(), DeribitFixError>
```

**Parameters:** None

**Returns:** `Result<(), DeribitFixError>`

**Example:**
```rust
client.disconnect().await?;
```

#### `is_connected()`
Checks if the client is currently connected.

```rust
pub fn is_connected(&self) -> bool
```

**Parameters:** None

**Returns:** `bool`

**Example:**
```rust
if client.is_connected() {
    println!("Client is connected");
}
```

### Order Management

#### `place_order()`
Places a new order on the exchange.

```rust
pub async fn place_order(&mut self, order: Order) -> Result<String, DeribitFixError>
```

**Parameters:**
- `order: Order` - The order to place

**Returns:** `Result<String, DeribitFixError>` - Order ID if successful

**Example:**
```rust
let order = Order::new(
    "BTC-PERPETUAL",
    OrderSide::Buy,
    OrderType::Limit,
    1000.0,
    50000.0
)?;

let order_id = client.place_order(order).await?;
println!("Order placed with ID: {}", order_id);
```

#### `cancel_order()`
Cancels an existing order.

```rust
pub async fn cancel_order(&mut self, order_id: &str) -> Result<(), DeribitFixError>
```

**Parameters:**
- `order_id: &str` - The ID of the order to cancel

**Returns:** `Result<(), DeribitFixError>`

**Example:**
```rust
client.cancel_order("order_123").await?;
println!("Order cancelled successfully");
```

#### `modify_order()`
Modifies an existing order.

```rust
pub async fn modify_order(&mut self, order_id: &str, modifications: OrderModification) -> Result<(), DeribitFixError>
```

**Parameters:**
- `order_id: &str` - The ID of the order to modify
- `modifications: OrderModification` - The modifications to apply

**Returns:** `Result<(), DeribitFixError>`

**Example:**
```rust
let modifications = OrderModification::new()
    .with_price(51000.0)
    .with_quantity(1500.0);

client.modify_order("order_123", modifications).await?;
```

#### `get_order_status()`
Retrieves the current status of an order.

```rust
pub async fn get_order_status(&self, order_id: &str) -> Result<OrderStatus, DeribitFixError>
```

**Parameters:**
- `order_id: &str` - The ID of the order

**Returns:** `Result<OrderStatus, DeribitFixError>`

**Example:**
```rust
let status = client.get_order_status("order_123").await?;
println!("Order status: {:?}", status);
```

### Market Data Functions

#### `subscribe_market_data()`
Subscribes to real-time market data for an instrument.

```rust
pub async fn subscribe_market_data(&mut self, instrument: &str) -> Result<(), DeribitFixError>
```

**Parameters:**
- `instrument: &str` - The instrument symbol

**Returns:** `Result<(), DeribitFixError>`

**Example:**
```rust
client.subscribe_market_data("BTC-PERPETUAL").await?;
```

#### `get_order_book()`
Retrieves the current order book for an instrument.

```rust
pub async fn get_order_book(&self, instrument: &str) -> Result<OrderBook, DeribitFixError>
```

**Parameters:**
- `instrument: &str` - The instrument symbol

**Returns:** `Result<OrderBook, DeribitFixError>`

**Example:**
```rust
let order_book = client.get_order_book("BTC-PERPETUAL").await?;
println!("Best bid: {}", order_book.best_bid().unwrap_or(0.0));
println!("Best ask: {}", order_book.best_ask().unwrap_or(0.0));
```

#### `get_ticker()`
Retrieves the current ticker information for an instrument.

```rust
pub async fn get_ticker(&self, instrument: &str) -> Result<MarketData, DeribitFixError>
```

**Parameters:**
- `instrument: &str` - The instrument symbol

**Returns:** `Result<MarketData, DeribitFixError>`

**Example:**
```rust
let ticker = client.get_ticker("BTC-PERPETUAL").await?;
println!("Last price: {}", ticker.last_price);
println!("24h volume: {}", ticker.volume_24h);
```

### Position Management

#### `get_positions()`
Retrieves all current positions.

```rust
pub async fn get_positions(&self) -> Result<Vec<Position>, DeribitFixError>
```

**Parameters:** None

**Returns:** `Result<Vec<Position>, DeribitFixError>`

**Example:**
```rust
let positions = client.get_positions().await?;
for position in positions {
    println!("Instrument: {}, Size: {}, PnL: {}", 
             position.instrument, position.size, position.unrealized_pnl);
}
```

#### `get_position()`
Retrieves a specific position by instrument.

```rust
pub async fn get_position(&self, instrument: &str) -> Result<Option<Position>, DeribitFixError>
```

**Parameters:**
- `instrument: &str` - The instrument symbol

**Returns:** `Result<Option<Position>, DeribitFixError>`

**Example:**
```rust
if let Some(position) = client.get_position("BTC-PERPETUAL").await? {
    println!("Position size: {}", position.size);
} else {
    println!("No position for BTC-PERPETUAL");
}
```

### Account Functions

#### `get_account_summary()`
Retrieves account summary information.

```rust
pub async fn get_account_summary(&self) -> Result<AccountSummary, DeribitFixError>
```

**Parameters:** None

**Returns:** `Result<AccountSummary, DeribitFixError>`

**Example:**
```rust
let account = client.get_account_summary().await?;
println!("Balance: {}", account.balance);
println!("Equity: {}", account.equity);
```

#### `get_margin_info()`
Retrieves margin information for the account.

```rust
pub async fn get_margin_info(&self) -> Result<MarginInfo, DeribitFixError>
```

**Parameters:** None

**Returns:** `Result<MarginInfo, DeribitFixError>`

**Example:**
```rust
let margin = client.get_margin_info().await?;
println!("Used margin: {}", margin.used_margin);
println!("Free margin: {}", margin.free_margin);
```

## Error Handling

All functions return `Result<T, DeribitFixError>` to handle potential errors. Common error types include:

- `ConnectionError` - Network or connection issues
- `AuthenticationError` - Invalid credentials
- `OrderError` - Order-related issues
- `MarketDataError` - Market data retrieval issues

## Async Operations

All functions that perform network operations are asynchronous and should be awaited. This allows for non-blocking operation and better performance in concurrent scenarios.

## Best Practices

1. **Error Handling**: Always check the result of function calls and handle errors appropriately
2. **Connection Management**: Ensure the client is connected before calling trading functions
3. **Resource Cleanup**: Call `disconnect()` when done to properly close the connection
4. **Async/Await**: Use proper async/await patterns when calling these functions
5. **Rate Limiting**: Be aware of exchange rate limits and implement appropriate delays

## See Also

- [DeribitFixClient Struct](../structs/deribit_fix_client.md)
- [Order Struct](../structs/order.md)
- [Position Struct](../structs/position.md)
- [MarketData Struct](../structs/market_data.md)
- [Error Handling](../../01_project_overview/architecture/error_handling.md)
