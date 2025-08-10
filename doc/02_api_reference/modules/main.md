# Module Documentation

This section provides comprehensive documentation for all modules within the `deribit-fix` crate. Each module is documented with its purpose, public interface, and usage examples.

## Overview

The `deribit-fix` crate is organized into logical modules that provide different aspects of FIX protocol functionality:

- **Core Modules**: Essential functionality for FIX protocol handling
- **Business Logic Modules**: Trading operations and market data
- **Infrastructure Modules**: Connection, session, and configuration management
- **Utility Modules**: Helper functions and common operations

## Module Structure

```
src/
├── lib.rs              # Main library entry point
├── client.rs           # Main client interface
├── config.rs           # Configuration management
├── error.rs            # Error types and handling
├── message.rs          # FIX message handling
├── session.rs          # Session management
├── types.rs            # Common types and enums
├── connection.rs       # Connection management
├── heartbeat.rs        # Heartbeat handling
├── sequence.rs         # Sequence number management
└── utils.rs            # Utility functions
```

## Core Modules

### [lib.rs](lib.md)

The main library entry point that re-exports all public APIs and provides the crate's public interface.

**Key Responsibilities**:
- Public API re-exports
- Crate metadata and documentation
- Feature flag management
- Module organization

**Public Interface**:
```rust
pub use client::DeribitFixClient;
pub use config::Config;
pub use error::DeribitFixError;
pub use message::FixMessage;
pub use session::SessionManager;
pub use types::*;
```

### [client.rs](client.md)

The main client interface that provides high-level operations for trading and market data.

**Key Responsibilities**:
- Connection management
- Order placement and management
- Market data subscription
- Error handling and recovery

**Public Interface**:
```rust
pub struct DeribitFixClient {
    // Client implementation
}

impl DeribitFixClient {
    pub async fn connect(&self) -> Result<(), DeribitFixError>;
    pub async fn place_order(&self, order: Order) -> Result<String, DeribitFixError>;
    pub async fn cancel_order(&self, order_id: &str) -> Result<(), DeribitFixError>;
    pub async fn subscribe_market_data(&self, symbol: &str) -> Result<(), DeribitFixError>;
}
```

### [config.rs](config.md)

Configuration management for connection, authentication, and trading parameters.

**Key Responsibilities**:
- Environment-specific configuration
- Connection parameters
- Authentication credentials
- Trading preferences

**Public Interface**:
```rust
pub struct Config {
    pub environment: Environment,
    pub connection: ConnectionConfig,
    pub authentication: AuthConfig,
    pub trading: TradingConfig,
    pub logging: LoggingConfig,
}

impl Config {
    pub fn new() -> ConfigBuilder;
    pub fn validate(&self) -> Result<(), ConfigError>;
    pub fn merge(&mut self, other: Config) -> Result<(), ConfigError>;
}
```

### [error.rs](error.md)

Comprehensive error types and error handling utilities.

**Key Responsibilities**:
- Error classification and categorization
- Error context and recovery information
- Error severity levels
- Error conversion and formatting

**Public Interface**:
```rust
#[derive(Debug, thiserror::Error)]
pub enum DeribitFixError {
    ConnectionFailed { source: std::io::Error, context: ErrorContext },
    AuthenticationFailed { source: Box<dyn std::error::Error>, context: ErrorContext },
    FixProtocolError { code: String, message: String, context: ErrorContext },
    BusinessLogicError { operation: String, reason: String, context: ErrorContext },
    // ... other error variants
}

pub struct ErrorContext {
    pub operation: String,
    pub timestamp: DateTime<Utc>,
    pub additional_info: HashMap<String, String>,
}
```

## Business Logic Modules

### [message.rs](message.md)

FIX message creation, parsing, and validation.

**Key Responsibilities**:
- FIX message structure
- Message serialization/deserialization
- Field validation
- Message type handling

**Public Interface**:
```rust
pub struct FixMessage {
    pub header: MessageHeader,
    pub body: MessageBody,
    pub trailer: MessageTrailer,
}

impl FixMessage {
    pub fn new() -> FixMessageBuilder;
    pub fn from_str(s: &str) -> Result<Self, ParseError>;
    pub fn to_string(&self) -> String;
    pub fn validate(&self) -> Result<(), ValidationError>;
}
```

### [types.rs](types.md)

Common types, enums, and data structures used throughout the crate.

**Key Responsibilities**:
- Order types and status
- Market data structures
- Configuration enums
- Common constants

**Public Interface**:
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
}
```

## Infrastructure Modules

### [session.rs](session.md)

FIX session lifecycle and sequence number management.

**Key Responsibilities**:
- Session establishment and termination
- Sequence number management
- Heartbeat coordination
- Authentication state

**Public Interface**:
```rust
pub struct SessionManager {
    pub sender_comp_id: String,
    pub target_comp_id: String,
    pub heartbeat_interval: u32,
}

impl SessionManager {
    pub fn new() -> SessionManagerBuilder;
    pub async fn authenticate(&mut self, credentials: &AuthCredentials) -> Result<(), DeribitFixError>;
    pub async fn logout(&mut self) -> Result<(), DeribitFixError>;
    pub fn next_outbound_seq_num(&mut self) -> u32;
    pub fn reset_outbound_sequence(&mut self, seq_num: u32);
}
```

### [connection.rs](connection.md)

Low-level connection management and I/O operations.

**Key Responsibilities**:
- TCP connection establishment
- Message sending and receiving
- Connection state management
- Reconnection logic

**Public Interface**:
```rust
pub trait Connection: Send + Sync {
    async fn connect(&self) -> Result<(), DeribitFixError>;
    async fn disconnect(&self) -> Result<(), DeribitFixError>;
    async fn send_message(&self, message: &FixMessage) -> Result<(), DeribitFixError>;
    async fn receive_message(&self) -> Result<FixMessage, DeribitFixError>;
    fn is_connected(&self) -> bool;
}

pub struct TcpConnection {
    pub host: String,
    pub port: u16,
    pub timeout: Duration,
}
```

### [heartbeat.rs](heartbeat.md)

Heartbeat message generation and management.

**Key Responsibilities**:
- Heartbeat message creation
- Heartbeat timing and intervals
- Connection health monitoring
- Automatic heartbeat generation

**Public Interface**:
```rust
pub struct HeartbeatManager {
    pub interval: Duration,
    pub last_heartbeat: Option<DateTime<Utc>>,
}

impl HeartbeatManager {
    pub fn new(interval: Duration) -> Self;
    pub fn should_send_heartbeat(&self) -> bool;
    pub fn generate_heartbeat(&self, session: &SessionManager) -> FixMessage;
    pub fn update_last_heartbeat(&mut self);
}
```

### [sequence.rs](sequence.md)

Sequence number management and validation.

**Key Responsibilities**:
- Inbound/outbound sequence tracking
- Sequence number validation
- Gap detection and handling
- Sequence reset operations

**Public Interface**:
```rust
pub struct SequenceManager {
    pub outbound_seq_num: u32,
    pub inbound_seq_num: u32,
    pub last_inbound_seq_num: Option<u32>,
}

impl SequenceManager {
    pub fn new() -> Self;
    pub fn next_outbound(&mut self) -> u32;
    pub fn validate_inbound(&mut self, seq_num: u32) -> Result<(), SequenceError>;
    pub fn reset_outbound(&mut self, seq_num: u32);
    pub fn reset_inbound(&mut self, seq_num: u32);
}
```

## Utility Modules

### [utils.rs](utils.md)

Common utility functions and helper operations.

**Key Responsibilities**:
- FIX field formatting
- Time and date utilities
- Validation helpers
- Common operations

**Public Interface**:
```rust
pub fn format_fix_time(dt: DateTime<Utc>) -> String;
pub fn parse_fix_time(s: &str) -> Result<DateTime<Utc>, ParseError>;
pub fn validate_symbol(symbol: &str) -> Result<(), ValidationError>;
pub fn calculate_checksum(message: &str) -> u8;
```

## Module Dependencies

### Dependency Graph

```
lib.rs
├── client.rs
│   ├── config.rs
│   ├── connection.rs
│   ├── session.rs
│   ├── message.rs
│   └── error.rs
├── config.rs
│   └── error.rs
├── error.rs
├── message.rs
│   └── types.rs
├── session.rs
│   ├── heartbeat.rs
│   ├── sequence.rs
│   └── error.rs
├── types.rs
├── connection.rs
│   └── error.rs
├── heartbeat.rs
│   ├── message.rs
│   └── session.rs
├── sequence.rs
└── utils.rs
```

### Public vs Internal

**Public Modules** (exposed in lib.rs):
- `client` - Main client interface
- `config` - Configuration management
- `error` - Error types
- `message` - FIX message handling
- `session` - Session management
- `types` - Common types

**Internal Modules** (not directly exposed):
- `connection` - Connection implementation
- `heartbeat` - Heartbeat implementation
- `sequence` - Sequence management
- `utils` - Utility functions

## Module Documentation Standards

### Documentation Requirements

1. **Module Header**: Purpose and overview
2. **Public Interface**: All public items documented
3. **Examples**: Usage examples for key functionality
4. **Error Handling**: Error conditions and recovery
5. **Performance Notes**: Performance characteristics and considerations

### Example Documentation

```rust
/// # FIX Message Handling
///
/// This module provides functionality for creating, parsing, and validating
/// FIX protocol messages according to the FIX 4.4 specification.
///
/// ## Examples
///
/// ```rust
/// use deribit_fix::message::FixMessage;
///
/// let message = FixMessage::new()
///     .with_msg_type("D")
///     .with_sender_comp_id("CLIENT")
///     .with_target_comp_id("DERIBIT")
///     .build();
///
/// assert_eq!(message.msg_type(), "D");
/// ```
///
/// ## Performance
///
/// - Message creation: O(1) amortized
/// - Message parsing: O(n) where n is message length
/// - Message validation: O(n) where n is field count
pub mod message {
    // Module implementation
}
```

## Testing Modules

### Unit Testing

Each module includes comprehensive unit tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_functionality() {
        // Test implementation
    }
}
```

### Integration Testing

Modules are tested together in integration tests:

```rust
// tests/integration/module_integration.rs
use deribit_fix::*;

#[tokio::test]
async fn test_module_integration() {
    // Test module interactions
}
```

## Summary

The `deribit-fix` crate's modular architecture provides:

- **Clear Separation of Concerns**: Each module has a specific responsibility
- **Maintainable Code**: Well-defined interfaces and dependencies
- **Testable Components**: Each module can be tested independently
- **Extensible Design**: New functionality can be added without affecting existing code
- **Comprehensive Documentation**: Each module is thoroughly documented

For detailed information about specific modules, see the individual module documentation files.
