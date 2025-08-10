# Installing from Crates.io

## Overview

Installing from crates.io is the recommended method for most users. It provides the latest stable release with automatic dependency management.

## Quick Installation

### **1. Add to Cargo.toml**

```toml
[dependencies]
deribit-fix = "0.1.0"
```

### **2. Install Dependencies**

```bash
cargo build
```

## Detailed Installation

### **Step 1: Create New Project (Optional)**

If you're starting a new project:

```bash
cargo new my_trading_bot
cd my_trading_bot
```

### **Step 2: Add Dependencies**

Edit your `Cargo.toml` file:

```toml
[package]
name = "my_trading_bot"
version = "0.1.0"
edition = "2021"

[dependencies]
deribit-fix = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
log = "0.4"
env_logger = "0.10"
```

### **Step 3: Build and Verify**

```bash
# Check compilation
cargo check

# Run tests
cargo test

# Build release version
cargo build --release
```

## Version Management

### **Latest Stable Version**

```toml
deribit-fix = "0.1.0"
```

### **Specific Version**

```toml
deribit-fix = "=0.1.0"
```

### **Version Range**

```toml
deribit-fix = ">=0.1.0, <0.2.0"
```

### **Latest Development Version**

```toml
deribit-fix = "0.1.0-alpha.1"
```

## Dependency Features

### **Default Features**

The crate includes these features by default:
- **async**: Async/await support
- **serde**: Serialization support
- **chrono**: Date/time handling
- **log**: Logging support

### **Optional Features**

```toml
[dependencies]
deribit-fix = { version = "0.1.0", features = ["tracing", "metrics"] }
```

Available features:
- **tracing**: Advanced tracing support
- **metrics**: Performance metrics
- **clap**: Command-line interface
- **config**: Configuration file support

## Workspace Setup

### **Multi-Crate Project**

```toml
[workspace]
members = [
    "trading_bot",
    "risk_manager",
    "data_processor"
]

[workspace.dependencies]
deribit-fix = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

### **Individual Crate Dependencies**

```toml
# trading_bot/Cargo.toml
[dependencies]
deribit-fix.workspace = true
tokio.workspace = true
```

## Verification Commands

### **Check Dependencies**

```bash
# List all dependencies
cargo tree

# Check for outdated dependencies
cargo outdated

# Audit for security vulnerabilities
cargo audit
```

### **Run Examples**

```bash
# List available examples
cargo run --example

# Run specific example
cargo run --example basic_connection

# Run with debug output
RUST_LOG=debug cargo run --example market_data
```

## Troubleshooting

### **Common Issues**

#### **Compilation Errors**
```bash
# Update Rust toolchain
rustup update

# Clean and rebuild
cargo clean
cargo build
```

#### **Dependency Conflicts**
```bash
# Check dependency tree
cargo tree

# Update lockfile
cargo update
```

#### **Feature Conflicts**
```toml
# Ensure compatible feature combinations
[dependencies]
deribit-fix = { version = "0.1.0", default-features = false, features = ["async"] }
```

### **Getting Help**

- Check the [API Reference](../../02_api_reference/main.md)
- Review [Usage Examples](../usage/main.md)
- Open an issue on GitHub
- Check the troubleshooting guide

## Next Steps

After successful installation:

1. **[Basic Examples](../usage/basic_example.md)** - Learn fundamental operations
2. **[Configuration](../architecture/main.md)** - Understand client setup
3. **[API Reference](../../02_api_reference/main.md)** - Explore available functionality

---

**Installation successful?** Move on to [Basic Usage Examples](../usage/basic_example.md) to start trading!
