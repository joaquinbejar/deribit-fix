# Status and Configuration Enums

This document describes the status and configuration enums available in the `deribit-fix` crate.

## Overview

Status and configuration enums define the various states, levels, and options used for system configuration, validation, and status tracking. These enums provide type safety and clear semantics for system-wide functionality.

## Connection Status

Defines the current status of a network connection.

```rust
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
    Failed,
}
```

**Values:**
- `Disconnected` - No active connection
- `Connecting` - Connection attempt in progress
- `Connected` - Successfully connected
- `Reconnecting` - Attempting to reconnect after disconnection
- `Failed` - Connection attempt failed

**Example:**
```rust
let status = client.connection_status();
match status {
    ConnectionStatus::Disconnected => println!("Not connected"),
    ConnectionStatus::Connecting => println!("Connecting..."),
    ConnectionStatus::Connected => println!("Connected"),
    ConnectionStatus::Reconnecting => println!("Reconnecting..."),
    ConnectionStatus::Failed => println!("Connection failed"),
}
```

## Session Status

Defines the current status of a FIX session.

```rust
pub enum SessionStatus {
    Disconnected,
    Connecting,
    LoggingOn,
    LoggedOn,
    LoggingOut,
    LoggedOut,
    Rejected,
    Error,
}
```

**Values:**
- `Disconnected` - Session is not established
- `Connecting` - Attempting to establish session
- `LoggingOn` - Sending logon message
- `LoggedOn` - Successfully logged on
- `LoggingOut` - Sending logout message
- `LoggedOut` - Successfully logged out
- `Rejected` - Logon was rejected
- `Error` - Session error occurred

**Example:**
```rust
let session_status = client.session_status();
match session_status {
    SessionStatus::Disconnected => println!("Session disconnected"),
    SessionStatus::Connecting => println!("Establishing session..."),
    SessionStatus::LoggingOn => println!("Logging on..."),
    SessionStatus::LoggedOn => println!("Session active"),
    SessionStatus::LoggingOut => println!("Logging out..."),
    SessionStatus::LoggedOut => println!("Session ended"),
    SessionStatus::Rejected => println!("Logon rejected"),
    SessionStatus::Error => println!("Session error"),
}
```

## Validation Level

Defines the strictness level for validation operations.

```rust
pub enum ValidationLevel {
    None,
    Basic,
    Strict,
    Extra,
}
```

**Values:**
- `None` - No validation performed
- `Basic` - Basic validation (required fields only)
- `Strict` - Strict validation (all fields validated)
- `Extra` - Extra validation (including business logic)

**Example:**
```rust
let validation_level = ValidationLevel::Strict;
match validation_level {
    ValidationLevel::None => println!("No validation"),
    ValidationLevel::Basic => println!("Basic validation"),
    ValidationLevel::Strict => println!("Strict validation"),
    ValidationLevel::Extra => println!("Extra validation"),
}
```

## Risk Level

Defines the risk level for risk management operations.

```rust
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}
```

**Values:**
- `Low` - Low risk level
- `Medium` - Medium risk level
- `High` - High risk level
- `Critical` - Critical risk level

**Example:**
```rust
let risk_level = RiskLevel::Medium;
match risk_level {
    RiskLevel::Low => println!("Low risk"),
    RiskLevel::Medium => println!("Medium risk"),
    RiskLevel::High => println!("High risk"),
    RiskLevel::Critical => println!("Critical risk"),
}
```

## Market Data Type

Defines the type of market data.

```rust
pub enum MarketDataType {
    OrderBook,
    Ticker,
    Trade,
    OHLCV,
    FundingRate,
    IndexPrice,
}
```

**Values:**
- `OrderBook` - Order book data
- `Ticker` - Ticker information
- `Trade` - Trade data
- `OHLCV` - Open, High, Low, Close, Volume data
- `FundingRate` - Funding rate information
- `IndexPrice` - Index price data

**Example:**
```rust
let data_type = MarketDataType::OrderBook;
match data_type {
    MarketDataType::OrderBook => println!("Order book data"),
    MarketDataType::Ticker => println!("Ticker data"),
    MarketDataType::Trade => println!("Trade data"),
    MarketDataType::OHLCV => println!("OHLCV data"),
    MarketDataType::FundingRate => println!("Funding rate"),
    MarketDataType::IndexPrice => println!("Index price"),
}
```

## Market Data Update Type

Defines the type of market data update.

```rust
pub enum MarketDataUpdateType {
    Snapshot,
    Incremental,
    FullRefresh,
}
```

**Values:**
- `Snapshot` - Complete snapshot of data
- `Incremental` - Incremental update
- `FullRefresh` - Full refresh of data

**Example:**
```rust
let update_type = MarketDataUpdateType::Incremental;
match update_type {
    MarketDataUpdateType::Snapshot => println!("Data snapshot"),
    MarketDataUpdateType::Incremental => println!("Incremental update"),
    MarketDataUpdateType::FullRefresh => println!("Full refresh"),
}
```

## Order Book Update Type

Defines the type of order book update.

```rust
pub enum OrderBookUpdateType {
    Insert,
    Update,
    Delete,
    Clear,
}
```

**Values:**
- `Insert` - New order book entry
- `Update` - Existing entry updated
- `Delete` - Entry removed
- `Clear` - Order book cleared

**Example:**
```rust
let update_type = OrderBookUpdateType::Insert;
match update_type {
    OrderBookUpdateType::Insert => println!("New entry"),
    OrderBookUpdateType::Update => println!("Entry updated"),
    OrderBookUpdateType::Delete => println!("Entry deleted"),
    OrderBookUpdateType::Clear => println!("Order book cleared"),
}
```

## Trailing Stop Type

Defines the type of trailing stop order.

```rust
pub enum TrailingStopType {
    Percentage,
    Fixed,
    Adaptive,
}
```

**Values:**
- `Percentage` - Percentage-based trailing stop
- `Fixed` - Fixed amount trailing stop
- `Adaptive` - Adaptive trailing stop

**Example:**
```rust
let trailing_type = TrailingStopType::Percentage;
match trailing_type {
    TrailingStopType::Percentage => println!("Percentage trailing stop"),
    TrailingStopType::Fixed => println!("Fixed amount trailing stop"),
    TrailingStopType::Adaptive => println!("Adaptive trailing stop"),
}
```

## Discrepancy Severity

Defines the severity level of validation discrepancies.

```rust
pub enum DiscrepancySeverity {
    Info,
    Warning,
    Error,
    Critical,
}
```

**Values:**
- `Info` - Informational discrepancy
- `Warning` - Warning-level discrepancy
- `Error` - Error-level discrepancy
- `Critical` - Critical-level discrepancy

**Example:**
```rust
let severity = DiscrepancySeverity::Warning;
match severity {
    DiscrepancySeverity::Info => println!("Info discrepancy"),
    DiscrepancySeverity::Warning => println!("Warning discrepancy"),
    DiscrepancySeverity::Error => println!("Error discrepancy"),
    DiscrepancySeverity::Critical => println!("Critical discrepancy"),
}
```

## Margin Type

Defines the type of margin calculation.

```rust
pub enum MarginType {
    Isolated,
    Cross,
    Portfolio,
}
```

**Values:**
- `Isolated` - Isolated margin (per position)
- `Cross` - Cross margin (across positions)
- `Portfolio` - Portfolio margin (portfolio-wide)

**Example:**
```rust
let margin_type = MarginType::Isolated;
match margin_type {
    MarginType::Isolated => println!("Isolated margin"),
    MarginType::Cross => println!("Cross margin"),
    MarginType::Portfolio => println!("Portfolio margin"),
}
```

## Usage Examples

### Connection Status Monitoring

```rust
loop {
    let status = client.connection_status();
    match status {
        ConnectionStatus::Connected => {
            println!("Connection stable");
            break;
        }
        ConnectionStatus::Connecting | ConnectionStatus::Reconnecting => {
            println!("Waiting for connection...");
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
        ConnectionStatus::Failed => {
            eprintln!("Connection failed, retrying...");
            client.reconnect().await?;
        }
        _ => {
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}
```

### Session Management

```rust
let session_status = client.session_status();
match session_status {
    SessionStatus::LoggedOn => {
        println!("Session active, ready to trade");
    }
    SessionStatus::Disconnected => {
        println!("Starting new session");
        client.connect().await?;
    }
    SessionStatus::Rejected => {
        eprintln!("Authentication failed");
        return Err(Error::AuthenticationFailed);
    }
    _ => {
        println!("Session status: {:?}", session_status);
    }
}
```

### Validation Configuration

```rust
let validation_config = OrderValidationConfig::new()
    .with_validation_level(ValidationLevel::Strict)
    .with_max_order_size(10000.0)
    .with_min_order_size(1.0);

let order = Order::new("BTC-PERPETUAL", OrderSide::Buy, OrderType::Limit, 1000.0, 50000.0)?;
validate_order(&order, &validation_config)?;
```

## Best Practices

1. **Status Monitoring**: Regularly check connection and session status
2. **Error Handling**: Handle all possible enum values in match statements
3. **Configuration**: Use appropriate validation levels for different environments
4. **Logging**: Log status changes for debugging and monitoring
5. **Default Values**: Provide sensible defaults for configuration enums

## See Also

- [Connection Trait](../traits/connection.md)
- [SessionManager Trait](../traits/session_manager.md)
- [OrderValidationConfig Struct](../structs/order_validation_config.md)
- [Connection Management](../../01_project_overview/usage/basic_example.md)
