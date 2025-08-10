# Basic Usage Examples

## Overview

This guide provides step-by-step examples for basic operations with the Deribit FIX Client. These examples cover the fundamental operations needed to get started with FIX trading.

## Prerequisites

Before running these examples, ensure you have:

1. **API Credentials**: Deribit API key and secret
2. **Dependencies**: Required Rust crates in your `Cargo.toml`
3. **Environment**: Tokio runtime configured

## Basic Connection Example

### **1. Simple Connection**

```rust
use deribit_fix::client::FixClient;
use deribit_fix::config::FixConfig;
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    // Create configuration
    let config = FixConfig::new()
        .with_sender_comp_id("YOUR_SENDER_ID".to_string())
        .with_target_comp_id("DERIBIT".to_string())
        .with_api_key("YOUR_API_KEY".to_string())
        .with_secret_key("YOUR_SECRET_KEY".to_string())
        .with_host("test.deribit.com".to_string())
        .with_port(9883)
        .with_use_ssl(true);

    // Create client
    let mut client = FixClient::new(config);
    
    // Connect to Deribit
    info!("Connecting to Deribit...");
    client.connect().await?;
    
    info!("Successfully connected to Deribit!");
    
    // Keep connection alive
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    
    Ok(())
}
```

### **2. Connection with Error Handling**

```rust
use deribit_fix::client::FixClient;
use deribit_fix::config::FixConfig;
use deribit_fix::error::DeribitFixError;
use log::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let config = FixConfig::new()
        .with_sender_comp_id("YOUR_SENDER_ID".to_string())
        .with_target_comp_id("DERIBIT".to_string())
        .with_api_key("YOUR_API_KEY".to_string())
        .with_secret_key("YOUR_SECRET_KEY".to_string());

    let mut client = FixClient::new(config);
    
    match client.connect().await {
        Ok(_) => {
            info!("Successfully connected to Deribit");
            
            // Check connection status
            if client.is_connected().await? {
                info!("Connection is active");
            } else {
                error!("Connection is not active");
            }
        }
        Err(DeribitFixError::ConnectionFailed) => {
            error!("Failed to connect to Deribit");
            return Err("Connection failed".into());
        }
        Err(DeribitFixError::AuthenticationFailed) => {
            error!("Authentication failed - check your API credentials");
            return Err("Authentication failed".into());
        }
        Err(e) => {
            error!("Unexpected error: {}", e);
            return Err(e.into());
        }
    }
    
    Ok(())
}
```

## Authentication Examples

### **1. Basic Logon**

```rust
use deribit_fix::client::FixClient;
use deribit_fix::config::FixConfig;
use deribit_fix::message::admin::Logon;
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let config = FixConfig::new()
        .with_sender_comp_id("YOUR_SENDER_ID".to_string())
        .with_target_comp_id("DERIBIT".to_string())
        .with_api_key("YOUR_API_KEY".to_string())
        .with_secret_key("YOUR_SECRET_KEY".to_string());

    let mut client = FixClient::new(config);
    
    // Connect first
    client.connect().await?;
    
    // Send logon message
    let logon = Logon::new(
        "YOUR_SENDER_ID".to_string(),
        "DERIBIT".to_string(),
        "YOUR_API_KEY".to_string(),
        "YOUR_SECRET_KEY".to_string(),
    );
    
    client.send_logon(logon).await?;
    info!("Logon message sent successfully");
    
    // Wait for logon response
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    
    if client.is_authenticated().await? {
        info!("Successfully authenticated with Deribit");
    } else {
        error!("Authentication failed");
    }
    
    Ok(())
}
```

### **2. Logon with Heartbeat**

```rust
use deribit_fix::client::FixClient;
use deribit_fix::config::FixConfig;
use deribit_fix::message::admin::Logon;
use tokio::time::{sleep, Duration};
use log::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let config = FixConfig::new()
        .with_sender_comp_id("YOUR_SENDER_ID".to_string())
        .with_target_comp_id("DERIBIT".to_string())
        .with_api_key("YOUR_API_KEY".to_string())
        .with_secret_key("YOUR_SECRET_KEY".to_string())
        .with_heartbeat_interval(30);

    let mut client = FixClient::new(config);
    
    // Connect and authenticate
    client.connect().await?;
    
    let logon = Logon::new(
        "YOUR_SENDER_ID".to_string(),
        "DERIBIT".to_string(),
        "YOUR_API_KEY".to_string(),
        "YOUR_SECRET_KEY".to_string(),
    );
    
    client.send_logon(logon).await?;
    
    // Keep connection alive with heartbeat
    let mut interval = tokio::time::interval(Duration::from_secs(30));
    
    loop {
        interval.tick().await;
        
        if client.is_connected().await? {
            match client.send_heartbeat().await {
                Ok(_) => info!("Heartbeat sent successfully"),
                Err(e) => {
                    warn!("Failed to send heartbeat: {}", e);
                    // Try to reconnect
                    if let Err(e) = client.reconnect().await {
                        error!("Reconnection failed: {}", e);
                        break;
                    }
                }
            }
        } else {
            warn!("Connection lost, attempting to reconnect...");
            if let Err(e) = client.reconnect().await {
                error!("Reconnection failed: {}", e);
                break;
            }
        }
    }
    
    Ok(())
}
```

## Heartbeat and Keep-Alive

### **1. Basic Heartbeat Loop**

```rust
use deribit_fix::client::FixClient;
use deribit_fix::config::FixConfig;
use tokio::time::{sleep, Duration};
use log::info;

async fn heartbeat_loop(client: &mut FixClient) -> Result<(), Box<dyn std::error::Error>> {
    let mut interval = tokio::time::interval(Duration::from_secs(30));
    
    loop {
        interval.tick().await;
        
        if client.is_connected().await? {
            client.send_heartbeat().await?;
            info!("Heartbeat sent at {}", chrono::Utc::now());
        } else {
            info!("Connection lost, stopping heartbeat loop");
            break;
        }
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let config = FixConfig::new()
        .with_sender_comp_id("YOUR_SENDER_ID".to_string())
        .with_target_comp_id("DERIBIT".to_string())
        .with_api_key("YOUR_API_KEY".to_string())
        .with_secret_key("YOUR_SECRET_KEY".to_string());

    let mut client = FixClient::new(config);
    client.connect().await?;
    
    // Start heartbeat loop
    heartbeat_loop(&mut client).await?;
    
    Ok(())
}
```

### **2. Heartbeat with Connection Monitoring**

```rust
use deribit_fix::client::FixClient;
use deribit_fix::config::FixConfig;
use tokio::time::{sleep, Duration};
use log::{info, warn, error};

struct ConnectionManager {
    client: FixClient,
    heartbeat_interval: u64,
    max_reconnect_attempts: u32,
}

impl ConnectionManager {
    fn new(config: FixConfig) -> Self {
        Self {
            client: FixClient::new(config),
            heartbeat_interval: 30,
            max_reconnect_attempts: 5,
        }
    }
    
    async fn maintain_connection(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut reconnect_attempts = 0;
        
        loop {
            if !self.client.is_connected().await? {
                warn!("Connection lost, attempting to reconnect...");
                
                if reconnect_attempts >= self.max_reconnect_attempts {
                    error!("Max reconnection attempts reached");
                    break;
                }
                
                match self.client.reconnect().await {
                    Ok(_) => {
                        info!("Successfully reconnected");
                        reconnect_attempts = 0;
                    }
                    Err(e) => {
                        error!("Reconnection failed: {}", e);
                        reconnect_attempts += 1;
                        sleep(Duration::from_secs(5)).await;
                        continue;
                    }
                }
            }
            
            // Send heartbeat
            if let Err(e) = self.client.send_heartbeat().await {
                warn!("Failed to send heartbeat: {}", e);
            }
            
            sleep(Duration::from_secs(self.heartbeat_interval)).await;
        }
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let config = FixConfig::new()
        .with_sender_comp_id("YOUR_SENDER_ID".to_string())
        .with_target_comp_id("DERIBIT".to_string())
        .with_api_key("YOUR_API_KEY".to_string())
        .with_secret_key("YOUR_SECRET_KEY".to_string());

    let mut manager = ConnectionManager::new(config);
    manager.client.connect().await?;
    
    manager.maintain_connection().await?;
    
    Ok(())
}
```

## Error Handling Examples

### **1. Basic Error Handling**

```rust
use deribit_fix::client::FixClient;
use deribit_fix::config::FixConfig;
use deribit_fix::error::DeribitFixError;
use log::{info, error, warn};

async fn handle_connection_errors(client: &mut FixClient) -> Result<(), Box<dyn std::error::Error>> {
    match client.connect().await {
        Ok(_) => {
            info!("Successfully connected");
            Ok(())
        }
        Err(DeribitFixError::ConnectionFailed) => {
            error!("Connection failed - check network and firewall settings");
            Err("Connection failed".into())
        }
        Err(DeribitFixError::AuthenticationFailed) => {
            error!("Authentication failed - verify API credentials");
            Err("Authentication failed".into())
        }
        Err(DeribitFixError::InvalidConfiguration) => {
            error!("Invalid configuration - check your config parameters");
            Err("Invalid configuration".into())
        }
        Err(e) => {
            error!("Unexpected error: {}", e);
            Err(e.into())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let config = FixConfig::new()
        .with_sender_comp_id("YOUR_SENDER_ID".to_string())
        .with_target_comp_id("DERIBIT".to_string())
        .with_api_key("YOUR_API_KEY".to_string())
        .with_secret_key("YOUR_SECRET_KEY".to_string());

    let mut client = FixClient::new(config);
    
    handle_connection_errors(&mut client).await?;
    
    Ok(())
}
```

### **2. Retry Logic with Exponential Backoff**

```rust
use deribit_fix::client::FixClient;
use deribit_fix::config::FixConfig;
use tokio::time::{sleep, Duration};
use log::{info, warn, error};

async fn retry_connection(
    client: &mut FixClient,
    max_attempts: u32,
    base_delay: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut attempts = 0;
    
    loop {
        attempts += 1;
        
        match client.connect().await {
            Ok(_) => {
                info!("Successfully connected on attempt {}", attempts);
                return Ok(());
            }
            Err(e) => {
                if attempts >= max_attempts {
                    error!("Failed to connect after {} attempts: {}", max_attempts, e);
                    return Err(e.into());
                }
                
                let delay = base_delay * 2_u64.pow(attempts - 1);
                warn!("Connection attempt {} failed, retrying in {}ms...", attempts, delay);
                
                sleep(Duration::from_millis(delay)).await;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let config = FixConfig::new()
        .with_sender_comp_id("YOUR_SENDER_ID".to_string())
        .with_target_comp_id("DERIBIT".to_string())
        .with_api_key("YOUR_API_KEY".to_string())
        .with_secret_key("YOUR_SECRET_KEY".to_string());

    let mut client = FixClient::new(config);
    
    retry_connection(&mut client, 5, 1000).await?;
    
    Ok(())
}
```

## Configuration Examples

### **1. Environment-Based Configuration**

```rust
use deribit_fix::client::FixClient;
use deribit_fix::config::FixConfig;
use std::env;
use log::info;

fn load_config_from_env() -> FixConfig {
    let sender_id = env::var("DERIBIT_SENDER_ID")
        .unwrap_or_else(|_| "DEFAULT_SENDER".to_string());
    
    let api_key = env::var("DERIBIT_API_KEY")
        .expect("DERIBIT_API_KEY environment variable must be set");
    
    let secret_key = env::var("DERIBIT_SECRET_KEY")
        .expect("DERIBIT_SECRET_KEY environment variable must be set");
    
    let host = env::var("DERIBIT_HOST")
        .unwrap_or_else(|_| "test.deribit.com".to_string());
    
    let port: u16 = env::var("DERIBIT_PORT")
        .unwrap_or_else(|_| "9883".to_string())
        .parse()
        .expect("DERIBIT_PORT must be a valid port number");
    
    let use_ssl = env::var("DERIBIT_USE_SSL")
        .unwrap_or_else(|_| "true".to_string())
        .parse()
        .unwrap_or(true);
    
    FixConfig::new()
        .with_sender_comp_id(sender_id)
        .with_target_comp_id("DERIBIT".to_string())
        .with_api_key(api_key)
        .with_secret_key(secret_key)
        .with_host(host)
        .with_port(port)
        .with_use_ssl(use_ssl)
        .with_heartbeat_interval(30)
        .with_cancel_on_disconnect(true)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let config = load_config_from_env();
    info!("Configuration loaded from environment variables");
    
    let mut client = FixClient::new(config);
    client.connect().await?;
    
    info!("Successfully connected using environment configuration");
    
    Ok(())
}
```

### **2. Configuration File Loading**

```rust
use deribit_fix::client::FixClient;
use deribit_fix::config::FixConfig;
use serde::Deserialize;
use std::fs;
use log::info;

#[derive(Debug, Deserialize)]
struct ConfigFile {
    sender: SenderConfig,
    credentials: CredentialsConfig,
    connection: ConnectionConfig,
}

#[derive(Debug, Deserialize)]
struct SenderConfig {
    comp_id: String,
    target_comp_id: String,
}

#[derive(Debug, Deserialize)]
struct CredentialsConfig {
    api_key: String,
    secret_key: String,
}

#[derive(Debug, Deserialize)]
struct ConnectionConfig {
    host: String,
    port: u16,
    use_ssl: bool,
    heartbeat_interval: u64,
}

fn load_config_from_file(path: &str) -> Result<FixConfig, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string(path)?;
    let config: ConfigFile = toml::from_str(&config_content)?;
    
    Ok(FixConfig::new()
        .with_sender_comp_id(config.sender.comp_id)
        .with_target_comp_id(config.sender.target_comp_id)
        .with_api_key(config.credentials.api_key)
        .with_secret_key(config.credentials.secret_key)
        .with_host(config.connection.host)
        .with_port(config.connection.port)
        .with_use_ssl(config.connection.use_ssl)
        .with_heartbeat_interval(config.connection.heartbeat_interval))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let config = load_config_from_file("config.toml")?;
    info!("Configuration loaded from config.toml");
    
    let mut client = FixClient::new(config);
    client.connect().await?;
    
    info!("Successfully connected using configuration file");
    
    Ok(())
}
```

## Complete Working Example

### **Full Basic Client**

```rust
use deribit_fix::client::FixClient;
use deribit_fix::config::FixConfig;
use deribit_fix::error::DeribitFixError;
use tokio::time::{sleep, Duration};
use log::{info, warn, error};

struct BasicDeribitClient {
    client: FixClient,
    is_running: bool,
}

impl BasicDeribitClient {
    fn new(config: FixConfig) -> Self {
        Self {
            client: FixClient::new(config),
            is_running: false,
        }
    }
    
    async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting Deribit FIX client...");
        
        // Connect to Deribit
        self.connect().await?;
        
        // Authenticate
        self.authenticate().await?;
        
        // Start main loop
        self.is_running = true;
        self.main_loop().await?;
        
        Ok(())
    }
    
    async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Connecting to Deribit...");
        
        match self.client.connect().await {
            Ok(_) => {
                info!("Successfully connected to Deribit");
                Ok(())
            }
            Err(DeribitFixError::ConnectionFailed) => {
                error!("Connection failed - check network settings");
                Err("Connection failed".into())
            }
            Err(e) => {
                error!("Unexpected connection error: {}", e);
                Err(e.into())
            }
        }
    }
    
    async fn authenticate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Authenticating with Deribit...");
        
        // For now, we'll assume the client handles authentication internally
        // In a real implementation, you would send a Logon message
        
        sleep(Duration::from_secs(1)).await;
        
        if self.client.is_connected().await? {
            info!("Successfully authenticated with Deribit");
            Ok(())
        } else {
            error!("Authentication failed");
            Err("Authentication failed".into())
        }
    }
    
    async fn main_loop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting main client loop...");
        
        let mut heartbeat_interval = tokio::time::interval(Duration::from_secs(30));
        
        while self.is_running {
            tokio::select! {
                _ = heartbeat_interval.tick() => {
                    if let Err(e) = self.client.send_heartbeat().await {
                        warn!("Failed to send heartbeat: {}", e);
                        self.handle_connection_issue().await?;
                    } else {
                        info!("Heartbeat sent successfully");
                    }
                }
                _ = sleep(Duration::from_secs(1)) => {
                    // Check connection status
                    if !self.client.is_connected().await? {
                        warn!("Connection lost, attempting to reconnect...");
                        self.handle_connection_issue().await?;
                    }
                }
            }
        }
        
        Ok(())
    }
    
    async fn handle_connection_issue(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        warn!("Handling connection issue...");
        
        if let Err(e) = self.client.reconnect().await {
            error!("Reconnection failed: {}", e);
            self.is_running = false;
            return Err(e.into());
        }
        
        info!("Successfully reconnected");
        Ok(())
    }
    
    async fn stop(&mut self) {
        info!("Stopping Deribit FIX client...");
        self.is_running = false;
        
        if let Err(e) = self.client.disconnect().await {
            error!("Error during disconnect: {}", e);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    // Create configuration
    let config = FixConfig::new()
        .with_sender_comp_id("YOUR_SENDER_ID".to_string())
        .with_target_comp_id("DERIBIT".to_string())
        .with_api_key("YOUR_API_KEY".to_string())
        .with_secret_key("YOUR_SECRET_KEY".to_string())
        .with_host("test.deribit.com".to_string())
        .with_port(9883)
        .with_use_ssl(true)
        .with_heartbeat_interval(30);
    
    // Create and start client
    let mut client = BasicDeribitClient::new(config);
    
    // Handle shutdown gracefully
    let shutdown_signal = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for ctrl+c signal");
    };
    
    tokio::select! {
        _ = client.start() => {
            info!("Client stopped normally");
        }
        _ = shutdown_signal => {
            info!("Shutdown signal received");
            client.stop().await;
        }
    }
    
    Ok(())
}
```

## Next Steps

After mastering these basic examples:

1. **[Advanced Examples](advanced_example.md)** - Learn complex trading operations
2. **[API Reference](../../02_api_reference/main.md)** - Explore all available functionality
3. **[Architecture](../architecture/main.md)** - Understand internal design

---

**Ready for more advanced operations?** Check out the [Advanced Examples](advanced_example.md) to learn about order management, market data, and trading strategies!
