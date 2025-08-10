# Installation Guide

## Overview

This guide covers how to install and set up the Deribit FIX Client in your Rust project. The crate is available on crates.io and can be installed via Cargo.

## Installation Methods

### **From Crates.io (Recommended)**
Add the crate to your `Cargo.toml` for the latest stable release.

### **From Source**
Build from the latest development version for cutting-edge features.

### **Dependencies**
Install required system dependencies for optimal performance.

## Quick Start

### **1. Add to Cargo.toml**

```toml
[dependencies]
deribit-fix = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
```

### **2. Basic Usage**

```rust
use deribit_fix::client::FixClient;
use deribit_fix::config::FixConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = FixConfig::new()
        .with_sender_comp_id("YOUR_SENDER_ID".to_string())
        .with_target_comp_id("DERIBIT".to_string())
        .with_api_key("YOUR_API_KEY".to_string())
        .with_secret_key("YOUR_SECRET_KEY".to_string());

    let mut client = FixClient::new(config);
    client.connect().await?;
    
    Ok(())
}
```

## Installation Options

### **[From Crates.io](installing_from_crates.md)**
- Latest stable release
- Automatic dependency resolution
- Easy version management

### **[From Source](installing_from_source.md)**
- Latest development features
- Custom compilation options
- Debugging capabilities

## System Requirements

### **Minimum Requirements**
- **Rust**: 1.70.0 or later
- **Cargo**: Latest stable version
- **OS**: Linux, macOS, or Windows
- **Memory**: 512MB RAM
- **Storage**: 100MB free space

### **Recommended Requirements**
- **Rust**: 1.75.0 or later
- **Memory**: 2GB+ RAM
- **Storage**: 500MB+ free space
- **Network**: Low-latency internet connection

## Dependencies

### **Required Dependencies**
- **tokio**: Async runtime for high-performance networking
- **serde**: Serialization/deserialization framework
- **chrono**: Date and time handling
- **log**: Logging framework

### **Optional Dependencies**
- **tracing**: Advanced tracing and observability
- **metrics**: Performance metrics collection
- **clap**: Command-line argument parsing

## Configuration

### **Environment Variables**
```bash
export DERIBIT_API_KEY="your_api_key"
export DERIBIT_SECRET_KEY="your_secret_key"
export DERIBIT_SENDER_ID="your_sender_id"
export DERIBIT_TARGET_ID="DERIBIT"
```

### **Configuration File**
```toml
# config.toml
[sender]
comp_id = "YOUR_SENDER_ID"
target_comp_id = "DERIBIT"

[credentials]
api_key = "your_api_key"
secret_key = "your_secret_key"

[connection]
host = "fix.deribit.com"
port = 443
use_ssl = true
```

## Verification

### **Check Installation**
```bash
cargo check
cargo test
cargo doc --open
```

### **Run Examples**
```bash
cargo run --example basic_connection
cargo run --example market_data
cargo run --example place_order
```

## Troubleshooting

### **Common Issues**
- **Compilation Errors**: Ensure Rust version compatibility
- **Connection Issues**: Check network and firewall settings
- **Authentication Errors**: Verify API credentials
- **Performance Issues**: Check system resources

### **Getting Help**
- Check the [API Reference](../02_api_reference/main.md)
- Review [Usage Examples](usage/main.md)
- Open an issue on GitHub
- Check the troubleshooting guide

## Next Steps

After successful installation:

1. **[Basic Examples](usage/basic_example.md)** - Learn fundamental operations
2. **[Configuration](architecture/main.md)** - Understand client setup
3. **[API Reference](../02_api_reference/main.md)** - Explore available functionality
4. **[Testing](../03_development_guide/testing/main.md)** - Verify your setup

---

**Installation complete?** Move on to [Basic Usage Examples](usage/basic_example.md) to start trading!
