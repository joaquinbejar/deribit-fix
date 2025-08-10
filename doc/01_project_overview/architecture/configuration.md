# Configuration System Architecture

This document explains the configuration system architecture of the `deribit-fix` crate, including how configuration is loaded, validated, and used throughout the system.

## Overview

The configuration system is designed to be flexible, secure, and easy to use. It supports multiple configuration sources with a clear precedence order and comprehensive validation.

## Configuration Sources

### 1. Environment Variables (Highest Priority)
Configuration can be set via environment variables for easy deployment and containerization:

```bash
# Connection settings
export DERIBIT_FIX_HOST="www.deribit.com"
export DERIBIT_FIX_PORT="8443"
export DERIBIT_FIX_USE_SSL="true"
export DERIBIT_FIX_TIMEOUT="30"
export DERIBIT_FIX_HEARTBEAT_INTERVAL="30"

# Authentication
export DERIBIT_FIX_API_KEY="your_api_key"
export DERIBIT_FIX_API_SECRET="your_api_secret"
export DERIBIT_FIX_CLIENT_ID="your_client_id"

# Trading settings
export DERIBIT_FIX_DEFAULT_SYMBOL="BTC-PERPETUAL"
export DERIBIT_FIX_MAX_ORDER_SIZE="100.0"
export DERIBIT_FIX_RISK_LIMITS_ENABLED="true"

# Logging
export DERIBIT_FIX_LOG_LEVEL="info"
export DERIBIT_FIX_LOG_FORMAT="json"
export DERIBIT_FIX_LOG_FILE="/var/log/deribit-fix.log"
```

### 2. Configuration Files
Configuration files in TOML format for persistent settings:

```toml
# config.toml
[connection]
host = "www.deribit.com"
port = 8443
use_ssl = true
timeout = 30
heartbeat_interval = 30
reconnect_attempts = 5
reconnect_delay = 1000

[authentication]
api_key = "your_api_key"
api_secret = "your_api_secret"
client_id = "your_client_id"
auth_timeout = 10

[trading]
default_symbol = "BTC-PERPETUAL"
max_order_size = 100.0
risk_limits_enabled = true
position_limits = { "BTC-PERPETUAL" = 1000.0, "ETH-PERPETUAL" = 5000.0 }

[logging]
level = "info"
format = "json"
file = "/var/log/deribit-fix.log"
console = true
rotation = { max_size = "100MB", max_files = 5 }

[performance]
connection_pool_size = 5
message_buffer_size = 10000
heartbeat_timeout = 60
sequence_reset_threshold = 100
```

### 3. Default Values (Lowest Priority)
Sensible defaults for all configuration options:

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
            host: "www.deribit.com".to_string(),
            port: 8443,
            use_ssl: true,
            timeout: Duration::from_secs(30),
            heartbeat_interval: Duration::from_secs(30),
            reconnect_attempts: 5,
            reconnect_delay: Duration::from_millis(1000),
        }
    }
}
```

## Configuration Structure

### Connection Configuration
```rust
#[derive(Debug, Clone, Deserialize)]
pub struct ConnectionConfig {
    /// FIX gateway hostname
    pub host: String,
    
    /// FIX gateway port
    pub port: u16,
    
    /// Whether to use SSL/TLS
    pub use_ssl: bool,
    
    /// Connection timeout
    pub timeout: Duration,
    
    /// Heartbeat interval in seconds
    pub heartbeat_interval: Duration,
    
    /// Maximum reconnection attempts
    pub reconnect_attempts: u32,
    
    /// Delay between reconnection attempts
    pub reconnect_delay: Duration,
    
    /// Connection keepalive settings
    pub keepalive: Option<KeepaliveConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KeepaliveConfig {
    /// TCP keepalive time
    pub time: Duration,
    
    /// TCP keepalive interval
    pub interval: Duration,
    
    /// TCP keepalive probes
    pub probes: u32,
}
```

### Authentication Configuration
```rust
#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    /// Deribit API key
    pub api_key: String,
    
    /// Deribit API secret
    pub api_secret: String,
    
    /// Client identifier
    pub client_id: String,
    
    /// Authentication timeout
    pub auth_timeout: Duration,
    
    /// Whether to use testnet
    pub testnet: bool,
    
    /// Custom authentication headers
    pub custom_headers: Option<HashMap<String, String>>,
}
```

### Trading Configuration
```rust
#[derive(Debug, Clone, Deserialize)]
pub struct TradingConfig {
    /// Default trading symbol
    pub default_symbol: String,
    
    /// Maximum order size
    pub max_order_size: f64,
    
    /// Whether risk limits are enabled
    pub risk_limits_enabled: bool,
    
    /// Position limits per symbol
    pub position_limits: HashMap<String, f64>,
    
    /// Order validation rules
    pub validation_rules: ValidationRules,
    
    /// Risk management settings
    pub risk_management: RiskManagementConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ValidationRules {
    /// Minimum order size
    pub min_order_size: f64,
    
    /// Maximum order size
    pub max_order_size: f64,
    
    /// Price tick size
    pub price_tick_size: f64,
    
    /// Quantity tick size
    pub quantity_tick_size: f64,
    
    /// Maximum price deviation from market
    pub max_price_deviation: f64,
}
```

### Logging Configuration
```rust
#[derive(Debug, Clone, Deserialize)]
pub struct LoggingConfig {
    /// Log level (trace, debug, info, warn, error)
    pub level: String,
    
    /// Log format (json, text, compact)
    pub format: String,
    
    /// Log file path
    pub file: Option<String>,
    
    /// Whether to log to console
    pub console: bool,
    
    /// Log rotation settings
    pub rotation: Option<RotationConfig>,
    
    /// Custom log filters
    pub filters: Option<Vec<LogFilter>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RotationConfig {
    /// Maximum file size before rotation
    pub max_size: String,
    
    /// Maximum number of rotated files
    pub max_files: u32,
    
    /// Whether to compress rotated files
    pub compress: bool,
}
```

### Performance Configuration
```rust
#[derive(Debug, Clone, Deserialize)]
pub struct PerformanceConfig {
    /// Connection pool size
    pub connection_pool_size: usize,
    
    /// Message buffer size
    pub message_buffer_size: usize,
    
    /// Heartbeat timeout
    pub heartbeat_timeout: Duration,
    
    /// Sequence reset threshold
    pub sequence_reset_threshold: u64,
    
    /// Message processing timeout
    pub message_timeout: Duration,
    
    /// Batch processing settings
    pub batch_processing: BatchConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BatchConfig {
    /// Maximum batch size
    pub max_batch_size: usize,
    
    /// Batch timeout
    pub batch_timeout: Duration,
    
    /// Whether batching is enabled
    pub enabled: bool,
}
```

## Configuration Loading Process

### 1. Configuration Builder Pattern
```rust
pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }
    
    pub fn from_env(mut self) -> Result<Self, ConfigError> {
        // Load from environment variables
        if let Ok(host) = std::env::var("DERIBIT_FIX_HOST") {
            self.config.connection.host = host;
        }
        // ... load other environment variables
        Ok(self)
    }
    
    pub fn from_file(mut self, path: &str) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)?;
        let file_config: Config = toml::from_str(&content)?;
        self.config.merge(file_config);
        Ok(self)
    }
    
    pub fn from_toml(mut self, toml_str: &str) -> Result<Self, ConfigError> {
        let file_config: Config = toml::from_str(toml_str)?;
        self.config.merge(file_config);
        Ok(self)
    }
    
    pub fn build(self) -> Result<Config, ConfigError> {
        self.config.validate()?;
        Ok(self.config)
    }
}
```

### 2. Configuration Merging
```rust
impl Config {
    pub fn merge(&mut self, other: Config) {
        // Merge connection config
        if other.connection.host != "www.deribit.com" {
            self.connection.host = other.connection.host;
        }
        if other.connection.port != 8443 {
            self.connection.port = other.connection.port;
        }
        // ... merge other fields
        
        // Merge nested configurations
        self.trading.merge(other.trading);
        self.logging.merge(other.logging);
    }
    
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validate connection settings
        if self.connection.host.is_empty() {
            return Err(ConfigError::InvalidValue("host cannot be empty".to_string()));
        }
        if self.connection.port == 0 {
            return Err(ConfigError::InvalidValue("port must be greater than 0".to_string()));
        }
        
        // Validate authentication
        if self.authentication.api_key.is_empty() {
            return Err(ConfigError::MissingRequired("api_key".to_string()));
        }
        if self.authentication.api_secret.is_empty() {
            return Err(ConfigError::MissingRequired("api_secret".to_string()));
        }
        
        // Validate trading settings
        if self.trading.max_order_size <= 0.0 {
            return Err(ConfigError::InvalidValue("max_order_size must be positive".to_string()));
        }
        
        Ok(())
    }
}
```

## Configuration Usage

### 1. Client Initialization
```rust
// Load configuration from multiple sources
let config = ConfigBuilder::new()
    .from_env()?
    .from_file("config.toml")?
    .from_toml(r#"
        [connection]
        host = "custom.deribit.com"
        port = 9443
    "#)?
    .build()?;

// Create client with configuration
let mut client = DeribitFixClient::new(config).await?;
```

### 2. Runtime Configuration Updates
```rust
// Update configuration at runtime
client.update_config(|config| {
    config.connection.heartbeat_interval = Duration::from_secs(60);
    config.logging.level = "debug".to_string();
}).await?;
```

### 3. Configuration Hot Reloading
```rust
// Watch for configuration file changes
let config_watcher = ConfigWatcher::new("config.toml")?;
config_watcher.watch(move |new_config| {
    client.update_config(|config| {
        *config = new_config.clone();
    }).await?;
});
```

## Security Considerations

### 1. Sensitive Data Handling
```rust
impl AuthConfig {
    pub fn mask_sensitive_data(&self) -> Self {
        Self {
            api_key: "***".to_string(),
            api_secret: "***".to_string(),
            client_id: self.client_id.clone(),
            auth_timeout: self.auth_timeout,
            testnet: self.testnet,
            custom_headers: self.custom_headers.clone(),
        }
    }
}
```

### 2. Environment Variable Security
- Never log sensitive configuration values
- Use secure environment variable management in production
- Consider using secret management services (HashiCorp Vault, AWS Secrets Manager)

### 3. Configuration Validation
```rust
impl Config {
    pub fn validate_security(&self) -> Result<(), ConfigError> {
        // Check for weak passwords
        if self.authentication.api_secret.len() < 32 {
            return Err(ConfigError::SecurityViolation("API secret too short".to_string()));
        }
        
        // Check for insecure connections
        if !self.connection.use_ssl && self.connection.host != "localhost" {
            return Err(ConfigError::SecurityViolation("SSL required for remote connections".to_string()));
        }
        
        Ok(())
    }
}
```

## Configuration Testing

### 1. Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_validation() {
        let mut config = Config::default();
        config.authentication.api_key = "".to_string();
        
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_config_merging() {
        let mut base_config = Config::default();
        let override_config = Config {
            connection: ConnectionConfig {
                host: "custom.deribit.com".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };
        
        base_config.merge(override_config);
        assert_eq!(base_config.connection.host, "custom.deribit.com");
    }
}
```

### 2. Integration Tests
```rust
#[tokio::test]
async fn test_config_loading() {
    let config = ConfigBuilder::new()
        .from_env()?
        .from_file("test_config.toml")?
        .build()?;
    
    assert_eq!(config.connection.host, "test.deribit.com");
    assert_eq!(config.authentication.testnet, true);
}
```

## Best Practices

### 1. Configuration Organization
- Use descriptive environment variable names
- Group related configuration options
- Provide sensible defaults for all options
- Document all configuration parameters

### 2. Validation and Error Handling
- Validate configuration at startup
- Provide clear error messages for invalid configurations
- Support configuration hot-reloading where appropriate
- Log configuration changes for audit purposes

### 3. Production Deployment
- Use environment variables for sensitive data
- Implement configuration validation in CI/CD pipelines
- Monitor configuration changes in production
- Use configuration management tools for complex deployments
