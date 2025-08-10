# config Module

## Overview

The `config` module provides configuration management for the `deribit-fix` crate, including connection settings, authentication, trading parameters, and performance tuning options.

## Purpose

- **Configuration Management**: Centralized configuration for all crate components
- **Environment Support**: Support for different deployment environments
- **Security**: Secure handling of sensitive configuration data
- **Validation**: Runtime configuration validation and error reporting

## Public Interface

### Main Configuration Struct

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub connection: ConnectionConfig,
    pub authentication: AuthConfig,
    pub trading: TradingConfig,
    pub logging: LoggingConfig,
    pub performance: PerformanceConfig,
}
```

### Configuration Components

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub host: String,
    pub port: u16,
    pub ssl: bool,
    pub timeout: Duration,
    pub heartbeat_interval: Duration,
    pub reconnect_attempts: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub api_key: String,
    pub api_secret: String,
    pub testnet: bool,
    pub signature_expiry: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingConfig {
    pub default_time_in_force: TimeInForce,
    pub max_order_size: f64,
    pub risk_limits: RiskLimits,
    pub order_validation: OrderValidationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: LogLevel,
    pub format: LogFormat,
    pub output: LogOutput,
    pub file_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub max_concurrent_orders: usize,
    pub buffer_size: usize,
    pub batch_size: usize,
    pub retry_strategy: BackoffStrategy,
}
```

### Configuration Builder

```rust
pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    pub fn new() -> Self
    pub fn with_api_key(mut self, api_key: &str) -> Self
    pub fn with_api_secret(mut self, api_secret: &str) -> Self
    pub fn with_host(mut self, host: &str) -> Self
    pub fn with_port(mut self, port: u16) -> Self
    pub fn with_testnet(mut self, testnet: bool) -> Self
    pub fn with_timeout(mut self, timeout: Duration) -> Self
    pub fn with_heartbeat_interval(mut self, interval: Duration) -> Self
    pub fn with_log_level(mut self, level: LogLevel) -> Self
    pub fn build(self) -> Result<Config, ConfigError>
}
```

## Usage Examples

### Basic Configuration

```rust
use deribit_fix::Config;

let config = Config::default()
    .with_api_key("your_api_key")
    .with_api_secret("your_api_secret")
    .with_testnet(true)
    .with_host("test.deribit.com")
    .with_port(443)
    .with_ssl(true);
```

### Configuration from File

```rust
use deribit_fix::Config;
use std::fs;

// Load from TOML file
let config_content = fs::read_to_string("config.toml")?;
let config: Config = toml::from_str(&config_content)?;

// Load from environment variables
let config = Config::from_env()?;
```

### Advanced Configuration

```rust
use deribit_fix::{Config, LogLevel, BackoffStrategy};
use std::time::Duration;

let config = Config::default()
    .with_api_key("your_api_key")
    .with_api_secret("your_api_secret")
    .with_testnet(false)
    .with_host("www.deribit.com")
    .with_port(443)
    .with_ssl(true)
    .with_timeout(Duration::from_secs(30))
    .with_heartbeat_interval(Duration::from_secs(30))
    .with_log_level(LogLevel::Info)
    .with_retry_strategy(BackoffStrategy::Exponential)
    .with_max_concurrent_orders(100)
    .with_buffer_size(1024);
```

## Configuration Sources

### 1. Default Values

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
```

### 2. Environment Variables

```rust
impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut config = Self::default();
        
        if let Ok(api_key) = std::env::var("DERIBIT_API_KEY") {
            config.authentication.api_key = api_key;
        }
        
        if let Ok(api_secret) = std::env::var("DERIBIT_API_SECRET") {
            config.authentication.api_secret = api_secret;
        }
        
        if let Ok(testnet) = std::env::var("DERIBIT_TESTNET") {
            config.authentication.testnet = testnet.parse().unwrap_or(true);
        }
        
        Ok(config)
    }
}
```

### 3. TOML Files

```toml
# config.toml
[connection]
host = "test.deribit.com"
port = 443
ssl = true
timeout = 30
heartbeat_interval = 30

[authentication]
api_key = "your_api_key"
api_secret = "your_api_secret"
testnet = true

[trading]
default_time_in_force = "GTC"
max_order_size = 1000.0

[logging]
level = "INFO"
format = "json"

[performance]
max_concurrent_orders = 100
buffer_size = 1024
retry_strategy = "exponential"
```

## Configuration Validation

```rust
impl Config {
    pub fn validate(&self) -> Result<(), Vec<ConfigError>> {
        let mut errors = Vec::new();
        
        // Validate connection settings
        if self.connection.host.is_empty() {
            errors.push(ConfigError::InvalidHost);
        }
        
        if self.connection.port == 0 {
            errors.push(ConfigError::InvalidPort);
        }
        
        // Validate authentication
        if self.authentication.api_key.is_empty() {
            errors.push(ConfigError::MissingApiKey);
        }
        
        if self.authentication.api_secret.is_empty() {
            errors.push(ConfigError::MissingApiSecret);
        }
        
        // Validate trading settings
        if self.trading.max_order_size <= 0.0 {
            errors.push(ConfigError::InvalidMaxOrderSize);
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
```

## Security Considerations

### Sensitive Data Masking

```rust
impl AuthConfig {
    pub fn mask_sensitive_data(&self) -> Self {
        Self {
            api_key: "***".to_string(),
            api_secret: "***".to_string(),
            testnet: self.testnet,
            signature_expiry: self.signature_expiry,
        }
    }
}
```

### Environment Variable Security

```rust
impl Config {
    pub fn validate_security(&self) -> Result<(), SecurityError> {
        // Check for hardcoded credentials
        if self.authentication.api_key == "your_api_key" {
            return Err(SecurityError::HardcodedCredentials);
        }
        
        // Check for weak secrets
        if self.authentication.api_secret.len() < 32 {
            return Err(SecurityError::WeakSecret);
        }
        
        Ok(())
    }
}
```

## Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.connection.host, "localhost");
        assert_eq!(config.connection.port, 8080);
        assert!(!config.authentication.testnet);
    }

    #[test]
    fn test_config_validation() {
        let mut config = Config::default();
        config.connection.host = "".to_string();
        
        let result = config.validate();
        assert!(result.is_err());
        
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| matches!(e, ConfigError::InvalidHost)));
    }

    #[test]
    fn test_config_from_env() {
        std::env::set_var("DERIBIT_API_KEY", "test_key");
        std::env::set_var("DERIBIT_API_SECRET", "test_secret");
        
        let config = Config::from_env().unwrap();
        assert_eq!(config.authentication.api_key, "test_key");
        assert_eq!(config.authentication.api_secret, "test_secret");
    }
}
```

## Module Dependencies

- `serde`: Serialization/deserialization
- `chrono`: Duration handling
- `log`: Logging configuration
- `thiserror`: Error types
