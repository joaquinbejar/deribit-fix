# Struct Documentation

This section provides comprehensive documentation for all public structs within the `deribit-fix` crate. Each struct is documented with its purpose, fields, methods, and usage examples.

## Overview

The `deribit-fix` crate provides several key structs that represent different aspects of FIX protocol functionality:

- **Client Structs**: Main client interface and connection management
- **Configuration Structs**: Settings and parameters for the client
- **Message Structs**: FIX protocol message representation
- **Data Structs**: Trading data and market information
- **Utility Structs**: Helper structures for common operations

## Core Structs

### [DeribitFixClient](client_struct.md)

The main client struct that provides the primary interface for FIX protocol operations.

**Purpose**: High-level client for trading operations, market data, and connection management.

**Key Features**:
- Connection establishment and management
- Order placement and cancellation
- Market data subscription
- Error handling and recovery
- Session management

**Example Usage**:
```rust
use deribit_fix::DeribitFixClient;
use deribit_fix::Config;

let config = Config::new()
    .with_api_key("your_api_key")
    .with_api_secret("your_api_secret")
    .with_environment(Environment::Test)
    .build();

let client = DeribitFixClient::new(config);

// Connect to Deribit
client.connect().await?;

// Place an order
let order = Order::new()
    .with_symbol("BTC-PERPETUAL")
    .with_side(OrderSide::Buy)
    .with_order_type(OrderType::Limit)
    .with_quantity(0.1)
    .with_price(50000.0)
    .build();

let order_id = client.place_order(order).await?;
println!("Order placed with ID: {}", order_id);
```

### [Config](config_struct.md)

Configuration struct that holds all client settings and parameters.

**Purpose**: Centralized configuration management for connection, authentication, and trading parameters.

**Key Features**:
- Environment-specific settings
- Connection parameters
- Authentication credentials
- Trading preferences
- Logging configuration

**Example Usage**:
```rust
use deribit_fix::Config;
use deribit_fix::Environment;

let config = Config::new()
    .with_environment(Environment::Production)
    .with_api_key("your_api_key")
    .with_api_secret("your_api_secret")
    .with_host("www.deribit.com")
    .with_port(443)
    .with_heartbeat_interval(30)
    .with_connection_timeout(Duration::from_secs(10))
    .with_max_retries(3)
    .build();

// Validate configuration
config.validate()?;
```

### [FixMessage](message_struct.md)

Represents a FIX protocol message with header, body, and trailer.

**Purpose**: Core data structure for FIX protocol communication.

**Key Features**:
- FIX message structure (header, body, trailer)
- Field validation and parsing
- Serialization and deserialization
- Message type handling

**Example Usage**:
```rust
use deribit_fix::FixMessage;
use deribit_fix::Order;

let order = Order::new()
    .with_symbol("BTC-PERPETUAL")
    .with_side(OrderSide::Buy)
    .with_quantity(0.1)
    .build();

let message = FixMessage::new()
    .with_msg_type("D") // New Order Single
    .with_sender_comp_id("CLIENT")
    .with_target_comp_id("DERIBIT")
    .with_msg_seq_num(1)
    .with_sending_time(chrono::Utc::now())
    .with_order(order)
    .build();

// Convert to FIX string
let fix_string = message.to_string();
println!("FIX Message: {}", fix_string);

// Parse from FIX string
let parsed_message = FixMessage::from_str(&fix_string)?;
assert_eq!(message, parsed_message);
```

## Configuration Structs

### [ConnectionConfig](connection_config_struct.md)

Configuration for connection-related parameters.

**Purpose**: Manages connection settings like host, port, timeouts, and retry logic.

**Fields**:
```rust
pub struct ConnectionConfig {
    pub host: String,
    pub port: u16,
    pub timeout: Duration,
    pub max_retries: u32,
    pub retry_delay: Duration,
    pub keep_alive: bool,
    pub tcp_nodelay: bool,
}
```

**Example Usage**:
```rust
use deribit_fix::ConnectionConfig;
use std::time::Duration;

let connection_config = ConnectionConfig::new()
    .with_host("www.deribit.com")
    .with_port(443)
    .with_timeout(Duration::from_secs(10))
    .with_max_retries(3)
    .with_retry_delay(Duration::from_millis(100))
    .with_keep_alive(true)
    .with_tcp_nodelay(true)
    .build();
```

### [AuthConfig](auth_config_struct.md)

Configuration for authentication parameters.

**Purpose**: Manages API credentials and authentication settings.

**Fields**:
```rust
pub struct AuthConfig {
    pub api_key: String,
    pub api_secret: String,
    pub nonce_window: Duration,
    pub signature_method: SignatureMethod,
}
```

**Example Usage**:
```rust
use deribit_fix::AuthConfig;
use deribit_fix::SignatureMethod;
use std::time::Duration;

let auth_config = AuthConfig::new()
    .with_api_key("your_api_key")
    .with_api_secret("your_api_secret")
    .with_nonce_window(Duration::from_secs(30))
    .with_signature_method(SignatureMethod::Sha256)
    .build();
```

### [TradingConfig](trading_config_struct.md)

Configuration for trading-related parameters.

**Purpose**: Manages trading preferences and risk management settings.

**Fields**:
```rust
pub struct TradingConfig {
    pub default_time_in_force: TimeInForce,
    pub max_order_size: f64,
    pub max_position_size: f64,
    pub risk_limits: RiskLimits,
    pub order_timeout: Duration,
}
```

**Example Usage**:
```rust
use deribit_fix::TradingConfig;
use deribit_fix::TimeInForce;
use std::time::Duration;

let trading_config = TradingConfig::new()
    .with_default_time_in_force(TimeInForce::Day)
    .with_max_order_size(1.0)
    .with_max_position_size(10.0)
    .with_order_timeout(Duration::from_secs(60))
    .build();
```

## Message Structs

### [MessageHeader](message_header_struct.md)

Header section of a FIX message.

**Purpose**: Contains standard FIX header fields like message type, sequence number, and timestamps.

**Fields**:
```rust
pub struct MessageHeader {
    pub begin_string: String,
    pub body_length: u32,
    pub msg_type: String,
    pub msg_seq_num: u32,
    pub sending_time: DateTime<Utc>,
    pub sender_comp_id: String,
    pub target_comp_id: String,
}
```

**Example Usage**:
```rust
use deribit_fix::MessageHeader;
use chrono::Utc;

let header = MessageHeader::new()
    .with_begin_string("FIX.4.4")
    .with_msg_type("D")
    .with_msg_seq_num(1)
    .with_sending_time(Utc::now())
    .with_sender_comp_id("CLIENT")
    .with_target_comp_id("DERIBIT")
    .build();
```

### [MessageBody](message_body_struct.md)

Body section of a FIX message containing the main message data.

**Purpose**: Holds the specific message content and fields.

**Fields**:
```rust
pub struct MessageBody {
    pub fields: HashMap<String, String>,
    pub order: Option<Order>,
    pub execution_report: Option<ExecutionReport>,
    pub market_data: Option<MarketData>,
}
```

**Example Usage**:
```rust
use deribit_fix::MessageBody;
use deribit_fix::Order;
use std::collections::HashMap;

let mut fields = HashMap::new();
fields.insert("21".to_string(), "1".to_string()); // HandlInst
fields.insert("55".to_string(), "BTC-PERPETUAL".to_string()); // Symbol
fields.insert("54".to_string(), "1".to_string()); // Side
fields.insert("60".to_string(), "0".to_string()); // TransactTime

let body = MessageBody::new()
    .with_fields(fields)
    .with_order(Some(order))
    .build();
```

### [MessageTrailer](message_trailer_struct.md)

Trailer section of a FIX message containing checksum and other end-of-message fields.

**Purpose**: Contains message validation and end-of-message indicators.

**Fields**:
```rust
pub struct MessageTrailer {
    pub checksum: u8,
    pub signature: Option<String>,
    pub signature_length: Option<u32>,
}
```

**Example Usage**:
```rust
use deribit_fix::MessageTrailer;

let trailer = MessageTrailer::new()
    .with_checksum(42)
    .with_signature(Some("abc123".to_string()))
    .with_signature_length(Some(6))
    .build();
```

## Data Structs

### [Order](order_struct.md)

Represents a trading order with all its properties.

**Purpose**: Core data structure for order management and trading operations.

**Fields**:
```rust
pub struct Order {
    pub cl_ord_id: String,
    pub symbol: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub quantity: f64,
    pub price: Option<f64>,
    pub stop_price: Option<f64>,
    pub time_in_force: TimeInForce,
    pub order_status: OrderStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

**Example Usage**:
```rust
use deribit_fix::Order;
use deribit_fix::{OrderSide, OrderType, TimeInForce};
use chrono::Utc;

let order = Order::new()
    .with_cl_ord_id("order_123")
    .with_symbol("BTC-PERPETUAL")
    .with_side(OrderSide::Buy)
    .with_order_type(OrderType::Limit)
    .with_quantity(0.1)
    .with_price(50000.0)
    .with_time_in_force(TimeInForce::Day)
    .with_created_at(Utc::now())
    .build();

// Validate order
order.validate()?;

// Check order properties
assert_eq!(order.symbol(), "BTC-PERPETUAL");
assert_eq!(order.side(), OrderSide::Buy);
assert_eq!(order.quantity(), 0.1);
```

### [ExecutionReport](execution_report_struct.md)

Represents an execution report for an order.

**Purpose**: Provides status updates and execution details for orders.

**Fields**:
```rust
pub struct ExecutionReport {
    pub order_id: String,
    pub cl_ord_id: String,
    pub exec_id: String,
    pub exec_type: ExecType,
    pub ord_status: OrderStatus,
    pub symbol: String,
    pub side: OrderSide,
    pub leaves_qty: f64,
    pub cum_qty: f64,
    pub avg_px: f64,
    pub last_qty: Option<f64>,
    pub last_px: Option<f64>,
    pub transact_time: DateTime<Utc>,
}
```

**Example Usage**:
```rust
use deribit_fix::ExecutionReport;
use deribit_fix::{ExecType, OrderStatus, OrderSide};
use chrono::Utc;

let report = ExecutionReport::new()
    .with_order_id("12345")
    .with_cl_ord_id("order_123")
    .with_exec_id("exec_678")
    .with_exec_type(ExecType::New)
    .with_ord_status(OrderStatus::New)
    .with_symbol("BTC-PERPETUAL")
    .with_side(OrderSide::Buy)
    .with_leaves_qty(0.1)
    .with_cum_qty(0.0)
    .with_avg_px(0.0)
    .with_transact_time(Utc::now())
    .build();

// Check execution status
match report.exec_type() {
    ExecType::New => println!("Order accepted"),
    ExecType::Filled => println!("Order filled"),
    ExecType::Cancelled => println!("Order cancelled"),
    _ => println!("Other status"),
}
```

### [MarketData](market_data_struct.md)

Represents market data information.

**Purpose**: Contains real-time market information like prices, volumes, and order book data.

**Fields**:
```rust
pub struct MarketData {
    pub symbol: String,
    pub bid_price: Option<f64>,
    pub ask_price: Option<f64>,
    pub last_price: Option<f64>,
    pub volume: Option<f64>,
    pub timestamp: DateTime<Utc>,
    pub order_book: Option<OrderBook>,
}
```

**Example Usage**:
```rust
use deribit_fix::MarketData;
use chrono::Utc;

let market_data = MarketData::new()
    .with_symbol("BTC-PERPETUAL")
    .with_bid_price(49950.0)
    .with_ask_price(50050.0)
    .with_last_price(50000.0)
    .with_volume(100.5)
    .with_timestamp(Utc::now())
    .build();

// Access market data
if let Some(bid) = market_data.bid_price() {
    println!("Best bid: {}", bid);
}

if let Some(ask) = market_data.ask_price() {
    println!("Best ask: {}", ask);
}

// Calculate spread
if let (Some(bid), Some(ask)) = (market_data.bid_price(), market_data.ask_price()) {
    let spread = ask - bid;
    println!("Spread: {}", spread);
}
```

## Utility Structs

### [ErrorContext](error_context_struct.md)

Provides context information for errors.

**Purpose**: Enhances error information with operation context and additional details.

**Fields**:
```rust
pub struct ErrorContext {
    pub operation: String,
    pub timestamp: DateTime<Utc>,
    pub additional_info: HashMap<String, String>,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
}
```

**Example Usage**:
```rust
use deribit_fix::ErrorContext;
use std::collections::HashMap;
use chrono::Utc;

let mut info = HashMap::new();
info.insert("symbol".to_string(), "BTC-PERPETUAL".to_string());
info.insert("order_id".to_string(), "12345".to_string());

let context = ErrorContext::new("place_order")
    .with_timestamp(Utc::now())
    .with_additional_info(info)
    .with_user_id("user123")
    .with_session_id("session456")
    .build();
```

### [SessionInfo](session_info_struct.md)

Contains session-related information.

**Purpose**: Tracks session state and connection information.

**Fields**:
```rust
pub struct SessionInfo {
    pub session_id: String,
    pub sender_comp_id: String,
    pub target_comp_id: String,
    pub connected_at: DateTime<Utc>,
    pub last_heartbeat: Option<DateTime<Utc>>,
    pub outbound_seq_num: u32,
    pub inbound_seq_num: u32,
    pub is_authenticated: bool,
}
```

**Example Usage**:
```rust
use deribit_fix::SessionInfo;
use chrono::Utc;

let session_info = SessionInfo::new()
    .with_session_id("session_123")
    .with_sender_comp_id("CLIENT")
    .with_target_comp_id("DERIBIT")
    .with_connected_at(Utc::now())
    .with_outbound_seq_num(1)
    .with_inbound_seq_num(1)
    .with_is_authenticated(false)
    .build();

// Check session status
if session_info.is_authenticated() {
    println!("Session authenticated");
} else {
    println!("Session not authenticated");
}

// Check sequence numbers
println!("Outbound sequence: {}", session_info.outbound_seq_num());
println!("Inbound sequence: {}", session_info.inbound_seq_num());
```

## Builder Pattern

Most structs in the crate use the builder pattern for easy construction:

```rust
// Instead of setting fields individually
let mut order = Order::default();
order.symbol = "BTC-PERPETUAL".to_string();
order.side = OrderSide::Buy;
order.quantity = 0.1;

// Use the builder pattern
let order = Order::new()
    .with_symbol("BTC-PERPETUAL")
    .with_side(OrderSide::Buy)
    .with_quantity(0.1)
    .build();
```

## Validation

Many structs include validation methods:

```rust
// Validate order before sending
let order = Order::new()
    .with_symbol("BTC-PERPETUAL")
    .with_side(OrderSide::Buy)
    .with_quantity(0.1)
    .build();

match order.validate() {
    Ok(()) => {
        // Order is valid, proceed
        client.place_order(order).await?;
    }
    Err(e) => {
        eprintln!("Invalid order: {}", e);
        return Err(e);
    }
}
```

## Serialization

Structs support serialization for configuration and persistence:

```rust
use serde_json;

// Serialize configuration
let config = Config::new()
    .with_api_key("key")
    .with_api_secret("secret")
    .build();

let json = serde_json::to_string_pretty(&config)?;
std::fs::write("config.json", json)?;

// Deserialize configuration
let json = std::fs::read_to_string("config.json")?;
let config: Config = serde_json::from_str(&json)?;
```

## Summary

The `deribit-fix` crate provides a comprehensive set of structs that:

- **Represent FIX Protocol Elements**: Messages, orders, and market data
- **Manage Configuration**: Connection, authentication, and trading settings
- **Handle Data Flow**: Order placement, execution reports, and market updates
- **Provide Utilities**: Error context, session information, and validation

Each struct is designed with:
- **Builder Pattern**: Easy construction and configuration
- **Validation**: Built-in validation for data integrity
- **Serialization**: Support for configuration persistence
- **Comprehensive Documentation**: Clear examples and usage patterns

For detailed information about specific structs, see the individual struct documentation files.
