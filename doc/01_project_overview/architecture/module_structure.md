# Module Structure

This document provides a detailed breakdown of each module within the `deribit-fix` crate, including their responsibilities, public interfaces, and relationships.

## Core Modules

### `lib.rs` - Main Library Entry Point
**Purpose**: Main entry point and module organization
**Responsibilities**:
- Module declarations and re-exports
- Public API surface definition
- Feature flag management
- Documentation and examples

**Public Interface**:
```rust
pub mod client;
pub mod config;
pub mod error;
pub mod message;
pub mod session;
pub mod types;

pub use client::DeribitFixClient;
pub use config::Config;
pub use error::DeribitFixError;
```

### `client.rs` - Main Client Interface
**Purpose**: High-level client interface for FIX operations
**Responsibilities**:
- Connection management
- Session lifecycle
- High-level trading operations
- Error handling and retry logic

**Key Structs**:
```rust
pub struct DeribitFixClient {
    config: Config,
    session: SessionManager,
    connection: ConnectionManager,
}

impl DeribitFixClient {
    pub async fn new(config: Config) -> Result<Self, DeribitFixError>
    pub async fn connect(&mut self) -> Result<(), DeribitFixError>
    pub async fn disconnect(&mut self) -> Result<(), DeribitFixError>
    pub async fn place_order(&mut self, order: Order) -> Result<OrderResponse, DeribitFixError>
    pub async fn cancel_order(&mut self, order_id: &str) -> Result<CancelResponse, DeribitFixError>
    pub async fn get_positions(&mut self) -> Result<Vec<Position>, DeribitFixError>
}
```

### `config.rs` - Configuration Management
**Purpose**: Configuration loading and validation
**Responsibilities**:
- Environment variable parsing
- Configuration file loading
- Validation and defaults
- Feature flag management

**Key Structs**:
```rust
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub connection: ConnectionConfig,
    pub authentication: AuthConfig,
    pub trading: TradingConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConnectionConfig {
    pub host: String,
    pub port: u16,
    pub use_ssl: bool,
    pub timeout: Duration,
    pub heartbeat_interval: Duration,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    pub api_key: String,
    pub api_secret: String,
    pub client_id: String,
}
```

### `error.rs` - Error Handling
**Purpose**: Comprehensive error types and handling
**Responsibilities**:
- Custom error types for different failure modes
- Error conversion and context
- User-friendly error messages
- Debugging information

**Key Types**:
```rust
#[derive(Debug, thiserror::Error)]
pub enum DeribitFixError {
    #[error("Connection error: {0}")]
    Connection(#[from] std::io::Error),
    
    #[error("Authentication failed: {0}")]
    Authentication(String),
    
    #[error("FIX protocol error: {0}")]
    FixProtocol(String),
    
    #[error("Business logic error: {0}")]
    BusinessLogic(String),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Timeout: {0}")]
    Timeout(String),
}
```

### `message.rs` - FIX Message Handling
**Purpose**: FIX message construction, parsing, and validation
**Responsibilities**:
- FIX message serialization/deserialization
- Field validation and business rules
- Message type mapping
- Tag management

**Key Structs**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixMessage {
    pub msg_type: String,
    pub sender_comp_id: String,
    pub target_comp_id: String,
    pub msg_seq_num: u64,
    pub sending_time: DateTime<Utc>,
    pub fields: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub symbol: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub quantity: f64,
    pub price: Option<f64>,
    pub time_in_force: TimeInForce,
    pub client_order_id: String,
}

#[derive(Debug, Clone)]
pub struct ExecutionReport {
    pub order_id: String,
    pub client_order_id: String,
    pub exec_id: String,
    pub exec_type: ExecType,
    pub ord_status: OrdStatus,
    pub symbol: String,
    pub side: OrderSide,
    pub leaves_qty: f64,
    pub cum_qty: f64,
    pub avg_px: f64,
}
```

### `session.rs` - FIX Session Management
**Purpose**: FIX protocol session lifecycle and state
**Responsibilities**:
- Session establishment and teardown
- Sequence number management
- Heartbeat handling
- Authentication flow

**Key Structs**:
```rust
pub struct SessionManager {
    session_id: String,
    sequence_numbers: SequenceNumbers,
    state: SessionState,
    heartbeat_timer: Option<Interval>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SessionState {
    Disconnected,
    Connecting,
    Connected,
    LoggingIn,
    LoggedIn,
    LoggingOut,
    Error,
}

impl SessionManager {
    pub async fn logon(&mut self, credentials: &AuthConfig) -> Result<(), DeribitFixError>
    pub async fn logout(&mut self) -> Result<(), DeribitFixError>
    pub async fn send_heartbeat(&mut self) -> Result<(), DeribitFixError>
    pub async fn reset_sequence(&mut self) -> Result<(), DeribitFixError>
}
```

### `types.rs` - Common Types and Enums
**Purpose**: Shared types and enumerations
**Responsibilities**:
- Trading-related enums
- FIX field value types
- Common data structures
- Type conversions

**Key Types**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TimeInForce {
    Day,
    GoodTillCancel,
    ImmediateOrCancel,
    FillOrKill,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ExecType {
    New,
    PartialFill,
    Fill,
    DoneForDay,
    Canceled,
    Replaced,
    PendingCancel,
    Stopped,
    Rejected,
    Suspended,
    PendingNew,
    Calculated,
    Expired,
    Restated,
    PendingReplace,
    Trade,
    TradeCorrect,
    TradeCancel,
    OrderStatus,
}
```

## Internal Modules

### `connection.rs` - Network Connection Management
**Purpose**: Low-level network communication
**Responsibilities**:
- TCP connection establishment
- SSL/TLS handling
- Connection pooling
- Reconnection logic

### `heartbeat.rs` - Heartbeat Management
**Purpose**: FIX heartbeat mechanism
**Responsibilities**:
- Heartbeat timer management
- Test request handling
- Connection monitoring
- Health checks

### `sequence.rs` - Sequence Number Management
**Purpose**: FIX sequence number tracking
**Responsibilities**:
- Incoming/outgoing sequence numbers
- Gap detection and handling
- Sequence reset operations
- Duplicate detection

## Module Dependencies

```
lib.rs
├── client.rs
│   ├── session.rs
│   │   ├── connection.rs
│   │   ├── heartbeat.rs
│   │   └── sequence.rs
│   ├── message.rs
│   └── types.rs
├── config.rs
└── error.rs
```

## Public API Surface

The main public API is exposed through:
- `DeribitFixClient`: Main client interface
- `Config`: Configuration management
- `DeribitFixError`: Error handling
- `Order`, `ExecutionReport`, etc.: Trading types

## Module Testing

Each module includes:
- Unit tests for individual functions
- Integration tests for module interactions
- Mock implementations for testing
- Performance benchmarks where applicable

## Extension Points

Modules are designed for easy extension:
- New message types in `message.rs`
- Additional configuration options in `config.rs`
- Custom error types in `error.rs`
- New trading operations in `client.rs`
