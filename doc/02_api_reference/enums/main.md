# Enums

This section provides comprehensive documentation for all public enums within the `deribit-fix` crate. Enums define a set of named constants and provide type-safe alternatives to using raw values.

## Overview

Enums in `deribit-fix` serve several key purposes:
- **Type Safety**: Provide compile-time guarantees for valid values
- **Domain Modeling**: Represent business concepts and states
- **Error Classification**: Categorize different types of errors
- **Configuration Options**: Define valid configuration values
- **Protocol Constants**: Represent FIX protocol constants and message types

## Enum Categories

### Core Enums
Core enums that define fundamental types and states.

### Business Logic Enums
Enums that represent business concepts and trading operations.

### Error Enums
Enums that categorize different types of errors.

### Configuration Enums
Enums that define configuration options and values.

### Protocol Enums
Enums that represent FIX protocol constants and message types.

## Core Enums

### `OrderSide`
Represents the side of an order (buy or sell).

```rust
use deribit_fix::model::OrderSide;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderSide {
    /// Buy order
    Buy,
    /// Sell order
    Sell,
}

// Example usage
let buy_side = OrderSide::Buy;
let sell_side = OrderSide::Sell;

match buy_side {
    OrderSide::Buy => println!("This is a buy order"),
    OrderSide::Sell => println!("This is a sell order"),
}

// Serialization
let json = serde_json::to_string(&buy_side)?;
assert_eq!(json, "\"Buy\"");

// Deserialization
let deserialized: OrderSide = serde_json::from_str("\"Sell\"")?;
assert_eq!(deserialized, OrderSide::Sell);
```

### `OrderType`
Represents the type of an order.

```rust
use deribit_fix::model::OrderType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderType {
    /// Market order
    Market,
    /// Limit order
    Limit,
    /// Stop order
    Stop,
    /// Stop limit order
    StopLimit,
    /// Market if touched order
    MarketIfTouched,
    /// Limit if touched order
    LimitIfTouched,
}

// Example usage
let limit_order = OrderType::Limit;
let market_order = OrderType::Market;

match limit_order {
    OrderType::Limit => println!("This is a limit order"),
    OrderType::Market => println!("This is a market order"),
    OrderType::Stop => println!("This is a stop order"),
    OrderType::StopLimit => println!("This is a stop limit order"),
    OrderType::MarketIfTouched => println!("This is a market if touched order"),
    OrderType::LimitIfTouched => println!("This is a limit if touched order"),
}

// Check if order type requires a price
let requires_price = match limit_order {
    OrderType::Market | OrderType::Stop => false,
    OrderType::Limit | OrderType::StopLimit | OrderType::MarketIfTouched | OrderType::LimitIfTouched => true,
};
```

### `TimeInForce`
Represents the time in force for an order.

```rust
use deribit_fix::model::TimeInForce;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TimeInForce {
    /// Day order (valid until end of trading day)
    Day,
    /// Good till cancelled order
    GTC,
    /// Immediate or cancel order
    IOC,
    /// Fill or kill order
    FOK,
    /// Good till date order
    GTD,
    /// At the opening order
    ATO,
    /// At the close order
    ATC,
}

// Example usage
let day_order = TimeInForce::Day;
let gtc_order = TimeInForce::GTC;

match day_order {
    TimeInForce::Day => println!("Order valid until end of trading day"),
    TimeInForce::GTC => println!("Order good till cancelled"),
    TimeInForce::IOC => println!("Order immediate or cancel"),
    TimeInForce::FOK => println!("Order fill or kill"),
    TimeInForce::GTD => println!("Order good till date"),
    TimeInForce::ATO => println!("Order at the opening"),
    TimeInForce::ATC => println!("Order at the close"),
}

// Check if order can be partially filled
let can_partial_fill = match day_order {
    TimeInForce::IOC | TimeInForce::FOK => false,
    _ => true,
};
```

### `ExecType`
Represents the execution type of an order.

```rust
use deribit_fix::model::ExecType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExecType {
    /// New order
    New,
    /// Partial fill
    PartialFill,
    /// Fill
    Fill,
    /// Done for day
    DoneForDay,
    /// Cancelled
    Cancelled,
    /// Replace
    Replace,
    /// Pending cancel
    PendingCancel,
    /// Stopped
    Stopped,
    /// Rejected
    Rejected,
    /// Suspended
    Suspended,
    /// Pending new
    PendingNew,
    /// Calculated
    Calculated,
    /// Expired
    Expired,
    /// Accepted for bidding
    AcceptedForBidding,
    /// Pending replace
    PendingReplace,
}

// Example usage
let new_exec = ExecType::New;
let filled_exec = ExecType::Fill;

match new_exec {
    ExecType::New => println!("Order is new"),
    ExecType::PartialFill => println!("Order partially filled"),
    ExecType::Fill => println!("Order fully filled"),
    ExecType::DoneForDay => println!("Order done for day"),
    ExecType::Cancelled => println!("Order cancelled"),
    ExecType::Replace => println!("Order replaced"),
    ExecType::PendingCancel => println!("Order pending cancel"),
    ExecType::Stopped => println!("Order stopped"),
    ExecType::Rejected => println!("Order rejected"),
    ExecType::Suspended => println!("Order suspended"),
    ExecType::PendingNew => println!("Order pending new"),
    ExecType::Calculated => println!("Order calculated"),
    ExecType::Expired => println!("Order expired"),
    ExecType::AcceptedForBidding => println!("Order accepted for bidding"),
    ExecType::PendingReplace => println!("Order pending replace"),
}

// Check if execution is terminal
let is_terminal = match filled_exec {
    ExecType::Fill | ExecType::Cancelled | ExecType::Rejected | ExecType::Expired => true,
    _ => false,
};
```

## Business Logic Enums

### `OrderStatus`
Represents the current status of an order.

```rust
use deribit_fix::model::OrderStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderStatus {
    /// Order is pending
    Pending,
    /// Order is active
    Active,
    /// Order is partially filled
    PartiallyFilled,
    /// Order is fully filled
    Filled,
    /// Order is cancelled
    Cancelled,
    /// Order is rejected
    Rejected,
    /// Order is expired
    Expired,
    /// Order is suspended
    Suspended,
}

// Example usage
let active_status = OrderStatus::Active;
let filled_status = OrderStatus::Filled;

match active_status {
    OrderStatus::Pending => println!("Order is pending"),
    OrderStatus::Active => println!("Order is active"),
    OrderStatus::PartiallyFilled => println!("Order is partially filled"),
    OrderStatus::Filled => println!("Order is filled"),
    OrderStatus::Cancelled => println!("Order is cancelled"),
    OrderStatus::Rejected => println!("Order is rejected"),
    OrderStatus::Expired => println!("Order is expired"),
    OrderStatus::Suspended => println!("Order is suspended"),
}

// Check if order can be modified
let can_modify = match active_status {
    OrderStatus::Active | OrderStatus::PartiallyFilled => true,
    _ => false,
};
```

### `InstrumentType`
Represents the type of financial instrument.

```rust
use deribit_fix::model::InstrumentType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InstrumentType {
    /// Perpetual futures
    Perpetual,
    /// Futures with expiration
    Future,
    /// European options
    Option,
    /// Spot instruments
    Spot,
}

// Example usage
let btc_perp = InstrumentType::Perpetual;
let btc_option = InstrumentType::Option;

match btc_perp {
    InstrumentType::Perpetual => println!("This is a perpetual futures contract"),
    InstrumentType::Future => println!("This is a futures contract"),
    InstrumentType::Option => println!("This is an option contract"),
    InstrumentType::Spot => println!("This is a spot instrument"),
}

// Check if instrument has expiration
let has_expiration = match btc_perp {
    InstrumentType::Perpetual | InstrumentType::Spot => false,
    InstrumentType::Future | InstrumentType::Option => true,
};
```

### `OptionType`
Represents the type of option (call or put).

```rust
use deribit_fix::model::OptionType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptionType {
    /// Call option
    Call,
    /// Put option
    Put,
}

// Example usage
let call_option = OptionType::Call;
let put_option = OptionType::Put;

match call_option {
    OptionType::Call => println!("This is a call option"),
    OptionType::Put => println!("This is a put option"),
}

// Check option payoff direction
let is_bullish = match call_option {
    OptionType::Call => true,  // Call options benefit from price increases
    OptionType::Put => false,  // Put options benefit from price decreases
};
```

## Error Enums

### `DeribitFixError`
Represents all possible errors that can occur in the crate.

```rust
use deribit_fix::error::DeribitFixError;

#[derive(Debug, thiserror::Error)]
pub enum DeribitFixError {
    /// Connection-related errors
    #[error("Connection error: {0}")]
    ConnectionError(String),
    
    /// Authentication errors
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
    
    /// FIX protocol errors
    #[error("FIX protocol error: {0}")]
    FIXProtocolError(String),
    
    /// Business logic errors
    #[error("Business logic error: {0}")]
    BusinessLogicError(String),
    
    /// Configuration errors
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    /// Timeout errors
    #[error("Timeout error: {0}")]
    TimeoutError,
    
    /// Serialization errors
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    /// Deserialization errors
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
    
    /// Internal errors
    #[error("Internal error: {0}")]
    InternalError(String),
}

// Example usage
let connection_error = DeribitFixError::ConnectionError("Connection lost".to_string());
let auth_error = DeribitFixError::AuthenticationError("Invalid API key".to_string());

match connection_error {
    DeribitFixError::ConnectionError(msg) => println!("Connection error: {}", msg),
    DeribitFixError::AuthenticationError(msg) => println!("Auth error: {}", msg),
    DeribitFixError::FIXProtocolError(msg) => println!("FIX error: {}", msg),
    DeribitFixError::BusinessLogicError(msg) => println!("Business error: {}", msg),
    DeribitFixError::ConfigurationError(msg) => println!("Config error: {}", msg),
    DeribitFixError::TimeoutError => println!("Timeout error"),
    DeribitFixError::SerializationError(msg) => println!("Serialization error: {}", msg),
    DeribitFixError::DeserializationError(msg) => println!("Deserialization error: {}", msg),
    DeribitFixError::InternalError(msg) => println!("Internal error: {}", msg),
}

// Check if error is retryable
let is_retryable = match connection_error {
    DeribitFixError::ConnectionError(_) | DeribitFixError::TimeoutError => true,
    _ => false,
};
```

### `ErrorSeverity`
Represents the severity level of an error.

```rust
use deribit_fix::error::ErrorSeverity;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorSeverity {
    /// Low severity - informational
    Low,
    /// Medium severity - warning
    Medium,
    /// High severity - error
    High,
    /// Critical severity - fatal
    Critical,
}

// Example usage
let warning_severity = ErrorSeverity::Medium;
let fatal_severity = ErrorSeverity::Critical;

match warning_severity {
    ErrorSeverity::Low => println!("Low severity - informational"),
    ErrorSeverity::Medium => println!("Medium severity - warning"),
    ErrorSeverity::High => println!("High severity - error"),
    ErrorSeverity::Critical => println!("Critical severity - fatal"),
}

// Check if error requires immediate attention
let requires_immediate_attention = match fatal_severity {
    ErrorSeverity::Critical | ErrorSeverity::High => true,
    _ => false,
};
```

## Configuration Enums

### `LogLevel`
Represents the logging level for the application.

```rust
use deribit_fix::config::LogLevel;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LogLevel {
    /// Error level - only errors
    Error,
    /// Warn level - warnings and errors
    Warn,
    /// Info level - info, warnings, and errors
    Info,
    /// Debug level - debug, info, warnings, and errors
    Debug,
    /// Trace level - all messages
    Trace,
}

// Example usage
let info_level = LogLevel::Info;
let debug_level = LogLevel::Debug;

match info_level {
    LogLevel::Error => println!("Logging errors only"),
    LogLevel::Warn => println!("Logging warnings and errors"),
    LogLevel::Info => println!("Logging info, warnings, and errors"),
    LogLevel::Debug => println!("Logging debug, info, warnings, and errors"),
    LogLevel::Trace => println!("Logging all messages"),
}

// Check if level includes debug messages
let includes_debug = match info_level {
    LogLevel::Debug | LogLevel::Trace => true,
    _ => false,
};
```

### `BackoffStrategy`
Represents the retry backoff strategy.

```rust
use deribit_fix::config::BackoffStrategy;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackoffStrategy {
    /// Fixed delay between retries
    Fixed,
    /// Exponential backoff
    Exponential,
    /// Fibonacci backoff
    Fibonacci,
    /// No backoff (immediate retry)
    None,
}

// Example usage
let exp_backoff = BackoffStrategy::Exponential;
let fixed_backoff = BackoffStrategy::Fixed;

match exp_backoff {
    BackoffStrategy::Fixed => println!("Fixed delay between retries"),
    BackoffStrategy::Exponential => println!("Exponential backoff"),
    BackoffStrategy::Fibonacci => println!("Fibonacci backoff"),
    BackoffStrategy::None => println!("No backoff"),
}

// Check if strategy provides increasing delays
let has_increasing_delays = match exp_backoff {
    BackoffStrategy::Exponential | BackoffStrategy::Fibonacci => true,
    BackoffStrategy::Fixed | BackoffStrategy::None => false,
};
```

## Protocol Enums

### `FixMsgType`
Represents FIX message types.

```rust
use deribit_fix::message::FixMsgType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FixMsgType {
    /// Heartbeat
    Heartbeat,
    /// Test Request
    TestRequest,
    /// Resend Request
    ResendRequest,
    /// Reject
    Reject,
    /// Sequence Reset
    SequenceReset,
    /// Logout
    Logout,
    /// Logon
    Logon,
    /// New Order Single
    NewOrderSingle,
    /// Order Cancel Request
    OrderCancelRequest,
    /// Order Cancel/Replace Request
    OrderCancelReplaceRequest,
    /// Execution Report
    ExecutionReport,
    /// Order Cancel Reject
    OrderCancelReject,
    /// Market Data Request
    MarketDataRequest,
    /// Market Data Snapshot/Full Refresh
    MarketDataSnapshotFullRefresh,
    /// Market Data Incremental Refresh
    MarketDataIncrementalRefresh,
    /// Market Data Request Reject
    MarketDataRequestReject,
}

// Example usage
let heartbeat_msg = FixMsgType::Heartbeat;
let new_order_msg = FixMsgType::NewOrderSingle;

match heartbeat_msg {
    FixMsgType::Heartbeat => println!("Heartbeat message"),
    FixMsgType::TestRequest => println!("Test request message"),
    FixMsgType::ResendRequest => println!("Resend request message"),
    FixMsgType::Reject => println!("Reject message"),
    FixMsgType::SequenceReset => println!("Sequence reset message"),
    FixMsgType::Logout => println!("Logout message"),
    FixMsgType::Logon => println!("Logon message"),
    FixMsgType::NewOrderSingle => println!("New order single message"),
    FixMsgType::OrderCancelRequest => println!("Order cancel request message"),
    FixMsgType::OrderCancelReplaceRequest => println!("Order cancel/replace request message"),
    FixMsgType::ExecutionReport => println!("Execution report message"),
    FixMsgType::OrderCancelReject => println!("Order cancel reject message"),
    FixMsgType::MarketDataRequest => println!("Market data request message"),
    FixMsgType::MarketDataSnapshotFullRefresh => println!("Market data snapshot message"),
    FixMsgType::MarketDataIncrementalRefresh => println!("Market data incremental message"),
    FixMsgType::MarketDataRequestReject => println!("Market data request reject message"),
}

// Check if message type is session-related
let is_session_message = match heartbeat_msg {
    FixMsgType::Heartbeat | FixMsgType::TestRequest | FixMsgType::Logon | FixMsgType::Logout => true,
    _ => false,
};
```

### `FixField`
Represents FIX field identifiers.

```rust
use deribit_fix::message::FixField;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FixField {
    /// BeginString (8)
    BeginString = 8,
    /// BodyLength (9)
    BodyLength = 9,
    /// MsgType (35)
    MsgType = 35,
    /// SenderCompID (49)
    SenderCompID = 49,
    /// TargetCompID (56)
    TargetCompID = 56,
    /// MsgSeqNum (34)
    MsgSeqNum = 34,
    /// SendingTime (52)
    SendingTime = 52,
    /// CheckSum (10)
    CheckSum = 10,
    /// ClOrdID (11)
    ClOrdID = 11,
    /// Symbol (55)
    Symbol = 55,
    /// Side (54)
    Side = 54,
    /// OrdType (40)
    OrdType = 40,
    /// OrderQty (38)
    OrderQty = 38,
    /// Price (44)
    Price = 44,
    /// TimeInForce (59)
    TimeInForce = 59,
    /// TransactTime (60)
    TransactTime = 60,
    /// ExecType (150)
    ExecType = 150,
    /// OrdStatus (39)
    OrdStatus = 39,
    /// CumQty (14)
    CumQty = 14,
    /// AvgPx (6)
    AvgPx = 6,
}

// Example usage
let msg_type_field = FixField::MsgType;
let symbol_field = FixField::Symbol;

match msg_type_field {
    FixField::BeginString => println!("BeginString field (8)"),
    FixField::BodyLength => println!("BodyLength field (9)"),
    FixField::MsgType => println!("MsgType field (35)"),
    FixField::SenderCompID => println!("SenderCompID field (49)"),
    FixField::TargetCompID => println!("TargetCompID field (56)"),
    FixField::MsgSeqNum => println!("MsgSeqNum field (34)"),
    FixField::SendingTime => println!("SendingTime field (52)"),
    FixField::CheckSum => println!("CheckSum field (10)"),
    FixField::ClOrdID => println!("ClOrdID field (11)"),
    FixField::Symbol => println!("Symbol field (55)"),
    FixField::Side => println!("Side field (54)"),
    FixField::OrdType => println!("OrdType field (40)"),
    FixField::OrderQty => println!("OrderQty field (38)"),
    FixField::Price => println!("Price field (44)"),
    FixField::TimeInForce => println!("TimeInForce field (59)"),
    FixField::TransactTime => println!("TransactTime field (60)"),
    FixField::ExecType => println!("ExecType field (150)"),
    FixField::OrdStatus => println!("OrdStatus field (39)"),
    FixField::CumQty => println!("CumQty field (14)"),
    FixField::AvgPx => println!("AvgPx field (6)"),
}

// Get field number
let field_number = msg_type_field as u32;
assert_eq!(field_number, 35);

// Check if field is required in header
let is_header_field = match msg_type_field {
    FixField::BeginString | FixField::BodyLength | FixField::MsgType | 
    FixField::SenderCompID | FixField::TargetCompID | FixField::MsgSeqNum | 
    FixField::SendingTime => true,
    _ => false,
};
```

## Enum Implementation Patterns

### Associated Data
Enums can contain associated data for more complex state representation:

```rust
use deribit_fix::model::Order;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderEvent {
    /// Order was created
    Created(Order),
    /// Order was modified
    Modified { old_order: Order, new_order: Order },
    /// Order was cancelled
    Cancelled { order: Order, reason: String },
    /// Order was filled
    Filled { order: Order, fill_price: rust_decimal::Decimal, fill_qty: rust_decimal::Decimal },
    /// Order was rejected
    Rejected { order: Order, reason: String },
}

// Example usage
let order = Order::default();
let event = OrderEvent::Created(order.clone());

match event {
    OrderEvent::Created(order) => println!("Order created: {:?}", order),
    OrderEvent::Modified { old_order, new_order } => {
        println!("Order modified from {:?} to {:?}", old_order, new_order);
    }
    OrderEvent::Cancelled { order, reason } => {
        println!("Order cancelled: {:?}, reason: {}", order, reason);
    }
    OrderEvent::Filled { order, fill_price, fill_qty } => {
        println!("Order filled: {:?} at {} for {}", order, fill_price, fill_qty);
    }
    OrderEvent::Rejected { order, reason } => {
        println!("Order rejected: {:?}, reason: {}", order, reason);
    }
}
```

### Methods on Enums
Enums can implement methods for common operations:

```rust
use deribit_fix::model::OrderSide;

impl OrderSide {
    /// Returns the opposite side
    pub fn opposite(&self) -> Self {
        match self {
            OrderSide::Buy => OrderSide::Sell,
            OrderSide::Sell => OrderSide::Buy,
        }
    }
    
    /// Returns true if this is a buy side
    pub fn is_buy(&self) -> bool {
        matches!(self, OrderSide::Buy)
    }
    
    /// Returns true if this is a sell side
    pub fn is_sell(&self) -> bool {
        matches!(self, OrderSide::Sell)
    }
    
    /// Returns the FIX protocol value
    pub fn fix_value(&self) -> &'static str {
        match self {
            OrderSide::Buy => "1",
            OrderSide::Sell => "2",
        }
    }
}

// Example usage
let buy_side = OrderSide::Buy;
let sell_side = buy_side.opposite();

assert_eq!(sell_side, OrderSide::Sell);
assert!(buy_side.is_buy());
assert!(!buy_side.is_sell());
assert_eq!(buy_side.fix_value(), "1");
```

### Default Implementations
Enums can provide default implementations:

```rust
use deribit_fix::model::OrderType;

impl Default for OrderType {
    fn default() -> Self {
        OrderType::Limit
    }
}

impl OrderType {
    /// Returns the default time in force for this order type
    pub fn default_time_in_force(&self) -> TimeInForce {
        match self {
            OrderType::Market => TimeInForce::IOC,
            OrderType::Limit => TimeInForce::Day,
            OrderType::Stop => TimeInForce::GTC,
            OrderType::StopLimit => TimeInForce::GTC,
            OrderType::MarketIfTouched => TimeInForce::GTC,
            OrderType::LimitIfTouched => TimeInForce::GTC,
        }
    }
    
    /// Returns true if this order type requires a price
    pub fn requires_price(&self) -> bool {
        !matches!(self, OrderType::Market)
    }
}
```

## Best Practices

### Enum Design
- **Single Purpose**: Each enum should represent a single concept
- **Exhaustive Matching**: Use `#[non_exhaustive]` if the enum might grow
- **Clear Naming**: Use descriptive names that clearly indicate the purpose
- **Documentation**: Document each variant with examples

### Implementation Guidelines
- **Derive Traits**: Derive common traits like `Debug`, `Clone`, `PartialEq`
- **Methods**: Implement methods for common operations on enum values
- **Associated Data**: Use associated data for complex state representation
- **Pattern Matching**: Leverage pattern matching for clean, readable code

### Serialization Considerations
- **Serde Support**: Use `serde` attributes for custom serialization
- **Backward Compatibility**: Consider versioning for evolving enums
- **Error Handling**: Provide meaningful error messages for invalid values

## Related Documentation

- [Modules](modules.md) - Overview of all modules
- [Structs](structs.md) - Documentation of public structs
- [Traits](traits.md) - Documentation of public traits
- [Functions](functions.md) - Documentation of public functions
- [API Reference](main.md) - Main API documentation
