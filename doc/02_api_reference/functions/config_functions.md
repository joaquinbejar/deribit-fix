# Configuration Functions

This document describes the configuration-related functions available in the `deribit-fix` crate.

## Overview

Configuration functions handle the creation, validation, and management of various configuration objects used throughout the system. These functions ensure that all configuration parameters are properly set and validated before use.

## Configuration Builder Functions

### `ConfigBuilder::new()`
Creates a new configuration builder instance.

```rust
pub fn new() -> ConfigBuilder
```

**Parameters:** None

**Returns:** `ConfigBuilder`

**Example:**
```rust
let config_builder = ConfigBuilder::new();
```

### `ConfigBuilder::with_api_key()`
Sets the API key for authentication.

```rust
pub fn with_api_key(mut self, api_key: String) -> ConfigBuilder
```

**Parameters:**
- `api_key: String` - The API key for Deribit

**Returns:** `ConfigBuilder` - Self for method chaining

**Example:**
```rust
let config_builder = ConfigBuilder::new()
    .with_api_key("your_api_key_here".to_string());
```

### `ConfigBuilder::with_secret_key()`
Sets the secret key for authentication.

```rust
pub fn with_secret_key(mut self, secret_key: String) -> ConfigBuilder
```

**Parameters:**
- `secret_key: String` - The secret key for Deribit

**Returns:** `ConfigBuilder` - Self for method chaining

**Example:**
```rust
let config_builder = ConfigBuilder::new()
    .with_api_key("your_api_key_here".to_string())
    .with_secret_key("your_secret_key_here".to_string());
```

### `ConfigBuilder::with_testnet()`
Sets whether to use testnet or mainnet.

```rust
pub fn with_testnet(mut self, testnet: bool) -> ConfigBuilder
```

**Parameters:**
- `testnet: bool` - True for testnet, false for mainnet

**Returns:** `ConfigBuilder` - Self for method chaining

**Example:**
```rust
let config_builder = ConfigBuilder::new()
    .with_api_key("your_api_key_here".to_string())
    .with_secret_key("your_secret_key_here".to_string())
    .with_testnet(true); // Use testnet
```

### `ConfigBuilder::with_connection_config()`
Sets the connection configuration.

```rust
pub fn with_connection_config(mut self, connection_config: ConnectionConfig) -> ConfigBuilder
```

**Parameters:**
- `connection_config: ConnectionConfig` - Connection-specific configuration

**Returns:** `ConfigBuilder` - Self for method chaining

**Example:**
```rust
let connection_config = ConnectionConfig::new()
    .with_host("fix.deribit.com".to_string())
    .with_port(443)
    .with_timeout(Duration::from_secs(30));

let config_builder = ConfigBuilder::new()
    .with_api_key("your_api_key_here".to_string())
    .with_secret_key("your_secret_key_here".to_string())
    .with_testnet(false)
    .with_connection_config(connection_config);
```

### `ConfigBuilder::with_session_config()`
Sets the session configuration.

```rust
pub fn with_session_config(mut self, session_config: SessionConfig) -> ConfigBuilder
```

**Parameters:**
- `session_config: SessionConfig` - Session-specific configuration

**Returns:** `ConfigBuilder` - Self for method chaining

**Example:**
```rust
let session_config = SessionConfig::new()
    .with_sender_comp_id("YOUR_COMP_ID".to_string())
    .with_target_comp_id("DERIBIT".to_string())
    .with_heartbeat_interval(Duration::from_secs(30));

let config_builder = ConfigBuilder::new()
    .with_api_key("your_api_key_here".to_string())
    .with_secret_key("your_secret_key_here".to_string())
    .with_testnet(false)
    .with_session_config(session_config);
```

### `ConfigBuilder::build()`
Builds the final configuration object.

```rust
pub fn build(self) -> Result<Config, ConfigError>
```

**Parameters:** None

**Returns:** `Result<Config, ConfigError>`

**Example:**
```rust
let config = ConfigBuilder::new()
    .with_api_key("your_api_key_here".to_string())
    .with_secret_key("your_secret_key_here".to_string())
    .with_testnet(false)
    .build()?;
```

## Connection Configuration Functions

### `ConnectionConfig::new()`
Creates a new connection configuration.

```rust
pub fn new() -> ConnectionConfig
```

**Parameters:** None

**Returns:** `ConnectionConfig`

**Example:**
```rust
let connection_config = ConnectionConfig::new();
```

### `ConnectionConfig::with_host()`
Sets the FIX gateway host.

```rust
pub fn with_host(mut self, host: String) -> ConnectionConfig
```

**Parameters:**
- `host: String` - The FIX gateway hostname

**Returns:** `ConnectionConfig` - Self for method chaining

**Example:**
```rust
let connection_config = ConnectionConfig::new()
    .with_host("fix.deribit.com".to_string());
```

### `ConnectionConfig::with_port()`
Sets the FIX gateway port.

```rust
pub fn with_port(mut self, port: u16) -> ConnectionConfig
```

**Parameters:**
- `port: u16` - The FIX gateway port

**Returns:** `ConnectionConfig` - Self for method chaining

**Example:**
```rust
let connection_config = ConnectionConfig::new()
    .with_host("fix.deribit.com".to_string())
    .with_port(443);
```

### `ConnectionConfig::with_timeout()`
Sets the connection timeout.

```rust
pub fn with_timeout(mut self, timeout: Duration) -> ConnectionConfig
```

**Parameters:**
- `timeout: Duration` - Connection timeout duration

**Returns:** `ConnectionConfig` - Self for method chaining

**Example:**
```rust
let connection_config = ConnectionConfig::new()
    .with_host("fix.deribit.com".to_string())
    .with_port(443)
    .with_timeout(Duration::from_secs(30));
```

### `ConnectionConfig::with_ssl()`
Sets whether to use SSL/TLS.

```rust
pub fn with_ssl(mut self, ssl: bool) -> ConnectionConfig
```

**Parameters:**
- `ssl: bool` - True to enable SSL/TLS

**Returns:** `ConnectionConfig` - Self for method chaining

**Example:**
```rust
let connection_config = ConnectionConfig::new()
    .with_host("fix.deribit.com".to_string())
    .with_port(443)
    .with_timeout(Duration::from_secs(30))
    .with_ssl(true);
```

## Session Configuration Functions

### `SessionConfig::new()`
Creates a new session configuration.

```rust
pub fn new() -> SessionConfig
```

**Parameters:** None

**Returns:** `SessionConfig`

**Example:**
```rust
let session_config = SessionConfig::new();
```

### `SessionConfig::with_sender_comp_id()`
Sets the sender company ID.

```rust
pub fn with_sender_comp_id(mut self, sender_comp_id: String) -> SessionConfig
```

**Parameters:**
- `sender_comp_id: String` - Your company identifier

**Returns:** `SessionConfig` - Self for method chaining

**Example:**
```rust
let session_config = SessionConfig::new()
    .with_sender_comp_id("YOUR_COMP_ID".to_string());
```

### `SessionConfig::with_target_comp_id()`
Sets the target company ID.

```rust
pub fn with_target_comp_id(mut self, target_comp_id: String) -> SessionConfig
```

**Parameters:**
- `target_comp_id: String` - Target company identifier (usually "DERIBIT")

**Returns:** `SessionConfig` - Self for method chaining

**Example:**
```rust
let session_config = SessionConfig::new()
    .with_sender_comp_id("YOUR_COMP_ID".to_string())
    .with_target_comp_id("DERIBIT".to_string());
```

### `SessionConfig::with_heartbeat_interval()`
Sets the heartbeat interval.

```rust
pub fn with_heartbeat_interval(mut self, interval: Duration) -> SessionConfig
```

**Parameters:**
- `interval: Duration` - Heartbeat interval duration

**Returns:** `SessionConfig` - Self for method chaining

**Example:**
```rust
let session_config = SessionConfig::new()
    .with_sender_comp_id("YOUR_COMP_ID".to_string())
    .with_target_comp_id("DERIBIT".to_string())
    .with_heartbeat_interval(Duration::from_secs(30));
```

### `SessionConfig::with_reset_on_logon()`
Sets whether to reset sequence numbers on logon.

```rust
pub fn with_reset_on_logon(mut self, reset: bool) -> SessionConfig
```

**Parameters:**
- `reset: bool` - True to reset sequence numbers on logon

**Returns:** `SessionConfig` - Self for method chaining

**Example:**
```rust
let session_config = SessionConfig::new()
    .with_sender_comp_id("YOUR_COMP_ID".to_string())
    .with_target_comp_id("DERIBIT".to_string())
    .with_heartbeat_interval(Duration::from_secs(30))
    .with_reset_on_logon(true);
```

## Order Validation Configuration Functions

### `OrderValidationConfig::new()`
Creates a new order validation configuration.

```rust
pub fn new() -> OrderValidationConfig
```

**Parameters:** None

**Returns:** `OrderValidationConfig`

**Example:**
```rust
let validation_config = OrderValidationConfig::new();
```

### `OrderValidationConfig::with_max_order_size()`
Sets the maximum allowed order size.

```rust
pub fn with_max_order_size(mut self, max_size: f64) -> OrderValidationConfig
```

**Parameters:**
- `max_size: f64` - Maximum order size

**Returns:** `OrderValidationConfig` - Self for method chaining

**Example:**
```rust
let validation_config = OrderValidationConfig::new()
    .with_max_order_size(10000.0);
```

### `OrderValidationConfig::with_min_order_size()`
Sets the minimum allowed order size.

```rust
pub fn with_min_order_size(mut self, min_size: f64) -> OrderValidationConfig
```

**Parameters:**
- `min_size: f64` - Minimum order size

**Returns:** `OrderValidationConfig` - Self for method chaining

**Example:**
```rust
let validation_config = OrderValidationConfig::new()
    .with_max_order_size(10000.0)
    .with_min_order_size(1.0);
```

### `OrderValidationConfig::with_price_tick_size()`
Sets the price tick size for validation.

```rust
pub fn with_price_tick_size(mut self, tick_size: f64) -> OrderValidationConfig
```

**Parameters:**
- `tick_size: f64` - Price tick size

**Returns:** `OrderValidationConfig` - Self for method chaining

**Example:**
```rust
let validation_config = OrderValidationConfig::new()
    .with_max_order_size(10000.0)
    .with_min_order_size(1.0)
    .with_price_tick_size(0.5);
```

### `OrderValidationConfig::with_validation_level()`
Sets the validation level.

```rust
pub fn with_validation_level(mut self, level: ValidationLevel) -> OrderValidationConfig
```

**Parameters:**
- `level: ValidationLevel` - Validation strictness level

**Returns:** `OrderValidationConfig` - Self for method chaining

**Example:**
```rust
let validation_config = OrderValidationConfig::new()
    .with_max_order_size(10000.0)
    .with_min_order_size(1.0)
    .with_price_tick_size(0.5)
    .with_validation_level(ValidationLevel::Strict);
```

## Configuration Validation Functions

### `validate_config()`
Validates a complete configuration object.

```rust
pub fn validate_config(config: &Config) -> Result<(), ConfigError>
```

**Parameters:**
- `config: &Config` - The configuration to validate

**Returns:** `Result<(), ConfigError>`

**Example:**
```rust
let config = ConfigBuilder::new()
    .with_api_key("your_api_key_here".to_string())
    .with_secret_key("your_secret_key_here".to_string())
    .with_testnet(false)
    .build()?;

validate_config(&config)?;
```

### `validate_connection_config()`
Validates connection configuration parameters.

```rust
pub fn validate_connection_config(config: &ConnectionConfig) -> Result<(), ConfigError>
```

**Parameters:**
- `config: &ConnectionConfig` - The connection configuration to validate

**Returns:** `Result<(), ConfigError>`

**Example:**
```rust
let connection_config = ConnectionConfig::new()
    .with_host("fix.deribit.com".to_string())
    .with_port(443)
    .with_timeout(Duration::from_secs(30));

validate_connection_config(&connection_config)?;
```

### `validate_session_config()`
Validates session configuration parameters.

```rust
pub fn validate_session_config(config: &SessionConfig) -> Result<(), ConfigError>
```

**Parameters:**
- `config: &SessionConfig` - The session configuration to validate

**Returns:** `Result<(), ConfigError>`

**Example:**
```rust
let session_config = SessionConfig::new()
    .with_sender_comp_id("YOUR_COMP_ID".to_string())
    .with_target_comp_id("DERIBIT".to_string())
    .with_heartbeat_interval(Duration::from_secs(30));

validate_session_config(&session_config)?;
```

## Configuration Loading Functions

### `load_config_from_file()`
Loads configuration from a file.

```rust
pub fn load_config_from_file(path: &str) -> Result<Config, ConfigError>
```

**Parameters:**
- `path: &str` - Path to the configuration file

**Returns:** `Result<Config, ConfigError>`

**Example:**
```rust
let config = load_config_from_file("config.toml")?;
```

### `load_config_from_env()`
Loads configuration from environment variables.

```rust
pub fn load_config_from_env() -> Result<Config, ConfigError>
```

**Parameters:** None

**Returns:** `Result<Config, ConfigError>`

**Example:**
```rust
let config = load_config_from_env()?;
```

### `save_config_to_file()`
Saves configuration to a file.

```rust
pub fn save_config_to_file(config: &Config, path: &str) -> Result<(), ConfigError>
```

**Parameters:**
- `config: &Config` - The configuration to save
- `path: &str` - Path where to save the configuration

**Returns:** `Result<(), ConfigError>`

**Example:**
```rust
save_config_to_file(&config, "config.toml")?;
```

## Error Handling

All configuration functions return `Result<T, ConfigError>` where `ConfigError` can be:

- `MissingRequiredField` - Required configuration field is missing
- `InvalidValue` - Configuration value is invalid
- `FileNotFound` - Configuration file not found
- `ParseError` - Error parsing configuration file
- `ValidationError` - Configuration validation failed

## Best Practices

1. **Use Builder Pattern**: Always use the builder pattern for creating configurations
2. **Validate Early**: Validate configurations before using them
3. **Environment Variables**: Use environment variables for sensitive information
4. **Default Values**: Provide sensible defaults for optional parameters
5. **Error Handling**: Always handle configuration errors appropriately

## See Also

- [Config Struct](../structs/config.md)
- [ConnectionConfig Struct](../structs/config.md)
- [SessionConfig Struct](../structs/config.md)
- [OrderValidationConfig Struct](../structs/order_validation_config.md)
- [Configuration Architecture](../../01_project_overview/architecture/configuration.md)
