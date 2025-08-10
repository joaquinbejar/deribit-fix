# Config

## Overview

`Config` is the main configuration structure for the `DeribitFixClient`. It provides centralized and validated configuration for all client operations, including connection, authentication, trading, and performance.

## Purpose

- **Centralized configuration**: Single place for all client configuration
- **Validation**: Automatic verification of configuration values
- **Flexibility**: Multiple configuration sources (defaults, environment variables, files)
- **Security**: Secure handling of credentials and sensitive configurations
- **Extensibility**: Easy addition of new configuration options

## Public Interface

### Struct Definition

```rust
pub struct Config {
    pub connection: ConnectionConfig,
    pub authentication: AuthConfig,
    pub trading: TradingConfig,
    pub logging: LoggingConfig,
    pub performance: PerformanceConfig,
}
```

### Nested Configuration Structs

#### ConnectionConfig

```rust
pub struct ConnectionConfig {
    pub host: String,
    pub port: u16,
    pub use_ssl: bool,
    pub connection_timeout: Duration,
    pub read_timeout: Duration,
    pub write_timeout: Duration,
    pub max_reconnect_attempts: usize,
    pub reconnect_delay: Duration,
}
```

#### AuthConfig

```rust
pub struct AuthConfig {
    pub api_key: String,
    pub api_secret: String,
    pub testnet: bool,
    pub account_id: Option<String>,
    pub session_timeout: Duration,
}
```

#### TradingConfig

```rust
pub struct TradingConfig {
    pub default_time_in_force: TimeInForce,
    pub max_order_size: f64,
    pub min_order_size: f64,
    pub price_tick_size: f64,
    pub quantity_tick_size: f64,
    pub risk_limits: RiskLimits,
}
```

#### LoggingConfig

```rust
pub struct LoggingConfig {
    pub level: LogLevel,
    pub enable_console: bool,
    pub enable_file: bool,
    pub log_file_path: Option<String>,
    pub max_file_size: u64,
    pub max_files: usize,
}
```

#### PerformanceConfig

```rust
pub struct PerformanceConfig {
    pub max_concurrent_orders: usize,
    pub order_batch_size: usize,
    pub heartbeat_interval: Duration,
    pub sequence_check_enabled: bool,
    pub connection_pool_size: usize,
}
```

### ConfigBuilder

```rust
pub struct ConfigBuilder {
    connection: Option<ConnectionConfig>,
    authentication: Option<AuthConfig>,
    trading: Option<TradingConfig>,
    logging: Option<LoggingConfig>,
    performance: Option<PerformanceConfig>,
}

impl ConfigBuilder {
    pub fn new() -> Self
    pub fn with_host(mut self, host: &str) -> Self
    pub fn with_port(mut self, port: u16) -> Self
    pub fn with_api_key(mut self, api_key: &str) -> Self
    pub fn with_api_secret(mut self, api_secret: &str) -> Self
    pub fn with_testnet(mut self, testnet: bool) -> Self
    pub fn with_ssl(mut self, use_ssl: bool) -> Self
    pub fn with_timeout(mut self, timeout: Duration) -> Self
    pub fn with_log_level(mut self, level: LogLevel) -> Self
    pub fn build(self) -> Result<Config, ConfigError>
}
```

## Usage Examples

### Basic Configuration

```rust
use deribit_fix::{Config, ConfigBuilder};
use std::time::Duration;

// Create basic configuration
let config = Config::default();

// Create configuration with builder
let config = ConfigBuilder::new()
    .with_host("test.deribit.com")
    .with_port(443)
    .with_api_key("your_api_key")
    .with_api_secret("your_api_secret")
    .with_testnet(true)
    .with_ssl(true)
    .with_timeout(Duration::from_secs(30))
    .build()?;
```

### Environment-based Configuration

```rust
use deribit_fix::Config;

// Load from environment variables
let config = Config::from_env()?;

// Environment variables used:
// DERIBIT_HOST=test.deribit.com
// DERIBIT_PORT=443
// DERIBIT_API_KEY=your_api_key
// DERIBIT_API_SECRET=your_api_secret
// DERIBIT_TESTNET=true
// DERIBIT_SSL=true
// DERIBIT_TIMEOUT=30
```

### File-based Configuration

```rust
use deribit_fix::Config;

// Load from TOML file
let config = Config::from_file("config.toml")?;

// Load from JSON file
let config = Config::from_json_file("config.json")?;

// Load from YAML file
let config = Config::from_yaml_file("config.yaml")?;
```

### Advanced Configuration

```rust
use deribit_fix::{Config, ConfigBuilder, LogLevel, TimeInForce, RiskLimits};
use std::time::Duration;

let config = ConfigBuilder::new()
    // Connection settings
    .with_host("test.deribit.com")
    .with_port(443)
    .with_ssl(true)
    .with_timeout(Duration::from_secs(30))
    
    // Authentication
    .with_api_key("your_api_key")
    .with_api_secret("your_api_secret")
    .with_testnet(true)
    
    // Trading settings
    .with_default_time_in_force(TimeInForce::GoodTillCancel)
    .with_max_order_size(1000.0)
    .with_min_order_size(0.001)
    .with_price_tick_size(0.5)
    .with_quantity_tick_size(0.001)
    
    // Logging
    .with_log_level(LogLevel::Info)
    .with_enable_console(true)
    .with_enable_file(true)
    .with_log_file_path("deribit.log")
    
    // Performance
    .with_max_concurrent_orders(100)
    .with_order_batch_size(10)
    .with_heartbeat_interval(Duration::from_secs(30))
    .with_connection_pool_size(5)
    
    .build()?;
```

### Configuration Merging

```rust
use deribit_fix::Config;

// Start with default configuration
let mut config = Config::default();

// Merge with file configuration
let file_config = Config::from_file("config.toml")?;
config.merge(file_config)?;

// Merge with environment variables
let env_config = Config::from_env()?;
config.merge(env_config)?;

// Override specific values
config.override_connection_host("custom.deribit.com");
config.override_api_key("override_key");
```

### Configuration Validation

```rust
use deribit_fix::Config;

let config = Config::from_file("config.toml")?;

// Validate entire configuration
config.validate()?;

// Validate specific sections
config.connection.validate()?;
config.authentication.validate()?;
config.trading.validate()?;

// Check for security issues
config.validate_security()?;
```

## Configuration Sources

### Default Values

```rust
impl Default for Config {
    fn default() -> Self {
        Self {
            connection: ConnectionConfig::default(),
            authentication: AuthConfig::default(),
            trading: TradingConfig::default(),
            logging: LoggingConfig::default(),
            performance: PerformanceConfig::default(),
        }
    }
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            host: "test.deribit.com".to_string(),
            port: 443,
            use_ssl: true,
            connection_timeout: Duration::from_secs(30),
            read_timeout: Duration::from_secs(30),
            write_timeout: Duration::from_secs(30),
            max_reconnect_attempts: 5,
            reconnect_delay: Duration::from_secs(5),
        }
    }
}
```

### Environment Variables

```rust
impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut builder = ConfigBuilder::new();
        
        if let Ok(host) = std::env::var("DERIBIT_HOST") {
            builder = builder.with_host(&host);
        }
        
        if let Ok(port) = std::env::var("DERIBIT_PORT") {
            if let Ok(port_num) = port.parse::<u16>() {
                builder = builder.with_port(port_num);
            }
        }
        
        if let Ok(api_key) = std::env::var("DERIBIT_API_KEY") {
            builder = builder.with_api_key(&api_key);
        }
        
        if let Ok(api_secret) = std::env::var("DERIBIT_API_SECRET") {
            builder = builder.with_api_secret(&api_secret);
        }
        
        if let Ok(testnet) = std::env::var("DERIBIT_TESTNET") {
            let is_testnet = testnet.to_lowercase() == "true";
            builder = builder.with_testnet(is_testnet);
        }
        
        builder.build()
    }
}
```

### Configuration Files

```toml
# config.toml
[connection]
host = "test.deribit.com"
port = 443
use_ssl = true
connection_timeout = 30
read_timeout = 30
write_timeout = 30
max_reconnect_attempts = 5
reconnect_delay = 5

[authentication]
api_key = "your_api_key"
api_secret = "your_api_secret"
testnet = true
account_id = "optional_account_id"
session_timeout = 3600

[trading]
default_time_in_force = "GoodTillCancel"
max_order_size = 1000.0
min_order_size = 0.001
price_tick_size = 0.5
quantity_tick_size = 0.001

[logging]
level = "Info"
enable_console = true
enable_file = true
log_file_path = "deribit.log"
max_file_size = 10485760
max_files = 5

[performance]
max_concurrent_orders = 100
order_batch_size = 10
heartbeat_interval = 30
sequence_check_enabled = true
connection_pool_size = 5
```

## Validation

### Configuration Validation

```rust
impl Config {
    pub fn validate(&self) -> Result<(), Vec<ConfigValidationError>> {
        let mut errors = Vec::new();
        
        // Validate connection
        if let Err(e) = self.connection.validate() {
            errors.extend(e);
        }
        
        // Validate authentication
        if let Err(e) = self.authentication.validate() {
            errors.extend(e);
        }
        
        // Validate trading
        if let Err(e) = self.trading.validate() {
            errors.extend(e);
        }
        
        // Validate logging
        if let Err(e) = self.logging.validate() {
            errors.extend(e);
        }
        
        // Validate performance
        if let Err(e) = self.performance.validate() {
            errors.extend(e);
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
```

### Security Validation

```rust
impl Config {
    pub fn validate_security(&self) -> Result<(), SecurityError> {
        // Check for weak API keys
        if self.authentication.api_key.len() < 16 {
            return Err(SecurityError::WeakApiKey);
        }
        
        // Check for weak API secrets
        if self.authentication.api_secret.len() < 32 {
            return Err(SecurityError::WeakApiSecret);
        }
        
        // Check for production credentials in testnet
        if self.authentication.testnet && 
           self.authentication.api_key.contains("prod") {
            return Err(SecurityError::ProductionCredentialsInTestnet);
        }
        
        Ok(())
    }
}
```

## Security Considerations

### Credential Masking

```rust
impl AuthConfig {
    pub fn mask_sensitive_data(&self) -> Self {
        Self {
            api_key: format!("{}...", &self.api_key[..8]),
            api_secret: "***MASKED***".to_string(),
            testnet: self.testnet,
            account_id: self.account_id.clone(),
            session_timeout: self.session_timeout,
        }
    }
}
```

### Weak Secret Detection

```rust
impl AuthConfig {
    pub fn is_weak_secret(&self) -> bool {
        // Check for common weak patterns
        let weak_patterns = [
            "password",
            "123456",
            "qwerty",
            "admin",
            "test",
        ];
        
        weak_patterns.iter().any(|pattern| {
            self.api_secret.to_lowercase().contains(pattern)
        })
    }
}
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.connection.host, "test.deribit.com");
        assert_eq!(config.connection.port, 443);
        assert!(config.connection.use_ssl);
    }
    
    #[test]
    fn test_config_builder() {
        let config = ConfigBuilder::new()
            .with_host("custom.deribit.com")
            .with_port(8080)
            .with_testnet(false)
            .build()
            .unwrap();
        
        assert_eq!(config.connection.host, "custom.deribit.com");
        assert_eq!(config.connection.port, 8080);
        assert!(!config.authentication.testnet);
    }
    
    #[test]
    fn test_config_validation() {
        let config = Config::default();
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_config_merging() {
        let mut base = Config::default();
        let override_config = ConfigBuilder::new()
            .with_host("override.deribit.com")
            .build()
            .unwrap();
        
        base.merge(override_config).unwrap();
        assert_eq!(base.connection.host, "override.deribit.com");
    }
}
```

### Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::fs;
    
    #[test]
    fn test_file_config_loading() {
        let temp_file = NamedTempFile::new().unwrap();
        let config_content = r#"
            [connection]
            host = "file.deribit.com"
            port = 8080
        "#;
        fs::write(&temp_file, config_content).unwrap();
        
        let config = Config::from_file(temp_file.path()).unwrap();
        assert_eq!(config.connection.host, "file.deribit.com");
        assert_eq!(config.connection.port, 8080);
    }
    
    #[test]
    fn test_environment_config_loading() {
        std::env::set_var("DERIBIT_HOST", "env.deribit.com");
        std::env::set_var("DERIBIT_PORT", "9090");
        
        let config = Config::from_env().unwrap();
        assert_eq!(config.connection.host, "env.deribit.com");
        assert_eq!(config.connection.port, 9090);
        
        // Clean up
        std::env::remove_var("DERIBIT_HOST");
        std::env::remove_var("DERIBIT_PORT");
    }
}
```

## Module Dependencies

- **error**: Para tipos de error de configuración
- **types**: Para tipos de datos como `LogLevel`, `TimeInForce`
- **serde**: Para serialización/deserialización de archivos de configuración

## Related Types

- **ConfigBuilder**: Constructor fluido para Config
- **ConnectionConfig**: Configuración de conexión
- **AuthConfig**: Configuración de autenticación
- **TradingConfig**: Configuración de trading
- **LoggingConfig**: Configuración de logging
- **PerformanceConfig**: Configuración de rendimiento
- **ConfigError**: Errores de configuración
- **ConfigValidationError**: Errores de validación
- **SecurityError**: Errores de seguridad
