# Installing from Source

## Overview

Installing from source gives you access to the latest development features, custom compilation options, and debugging capabilities. This method is recommended for developers who want to contribute or need the most recent features.

## Prerequisites

### **System Requirements**
- **Rust**: 1.70.0 or later (nightly recommended for latest features)
- **Git**: For cloning the repository
- **Build Tools**: C compiler and development libraries
- **SSL Libraries**: OpenSSL development packages

### **Install Rust (if not already installed)**

```bash
# Install Rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add nightly toolchain (optional)
rustup toolchain install nightly
rustup default nightly

# Verify installation
rustc --version
cargo --version
```

### **Install System Dependencies**

#### **Ubuntu/Debian**
```bash
sudo apt update
sudo apt install build-essential pkg-config libssl-dev
```

#### **CentOS/RHEL/Fedora**
```bash
sudo yum groupinstall "Development Tools"
sudo yum install openssl-devel
# or for Fedora
sudo dnf groupinstall "Development Tools"
sudo dnf install openssl-devel
```

#### **macOS**
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install OpenSSL via Homebrew
brew install openssl
```

#### **Windows**
```bash
# Install Visual Studio Build Tools
# Download from: https://visualstudio.microsoft.com/downloads/

# Install OpenSSL
# Download from: https://slproweb.com/products/Win32OpenSSL.html
```

## Clone and Build

### **Step 1: Clone Repository**

```bash
# Clone the main repository
git clone https://github.com/joaquinbejar/deribit.git
cd deribit/deribit-fix

# Or clone directly
git clone https://github.com/joaquinbejar/deribit-fix.git
cd deribit-fix
```

### **Step 2: Checkout Branch**

```bash
# Checkout main branch
git checkout main

# Or checkout specific version
git checkout v0.1.0

# Or checkout development branch
git checkout develop
```

### **Step 3: Build from Source**

```bash
# Check compilation
cargo check

# Run tests
cargo test

# Build debug version
cargo build

# Build release version
cargo build --release

# Build with specific features
cargo build --release --features "tracing,metrics"
```

## Development Setup

### **Install Development Dependencies**

```bash
# Install additional tools
cargo install cargo-watch
cargo install cargo-audit
cargo install cargo-outdated
cargo install cargo-tarpaulin

# Install code quality tools
rustup component add rustfmt
rustup component add clippy
```

### **Configure Development Environment**

#### **VS Code Setup**
```json
// .vscode/settings.json
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.cargo.buildScripts.enable": true,
    "rust-analyzer.procMacro.enable": true
}
```

#### **Rust Analyzer Configuration**
```toml
# rust-project.toml
[workspace]
members = ["."]
```

### **Run Development Tools**

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Run tests with coverage
cargo tarpaulin

# Check for security issues
cargo audit

# Check for outdated dependencies
cargo outdated
```

## Custom Build Options

### **Feature Selection**

```bash
# Build with specific features
cargo build --release --features "tracing,metrics,clap"

# Build without default features
cargo build --release --no-default-features --features "async"

# Build with all features
cargo build --release --all-features
```

### **Optimization Flags**

```bash
# Set optimization level
RUSTFLAGS="-C opt-level=3" cargo build --release

# Enable link-time optimization
RUSTFLAGS="-C lto=fat" cargo build --release

# Set target CPU
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

### **Cross-Compilation**

```bash
# Install target
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu

# Build for specific target
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
```

## Testing and Validation

### **Run Test Suite**

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_connection

# Run tests with output
cargo test -- --nocapture

# Run integration tests
cargo test --test integration_tests
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

### **Performance Testing**

```bash
# Run benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench performance

# Profile with perf
cargo build --release
perf record --call-graph=dwarf ./target/release/example
```

## Troubleshooting

### **Common Build Issues**

#### **OpenSSL Errors**
```bash
# Set OpenSSL paths
export OPENSSL_DIR=/usr/local/opt/openssl
export OPENSSL_LIB_DIR=/usr/local/opt/openssl/lib
export OPENSSL_INCLUDE_DIR=/usr/local/opt/openssl/include
```

#### **Linker Errors**
```bash
# Set library paths
export LIBRARY_PATH=/usr/local/lib:$LIBRARY_PATH
export LD_LIBRARY_PATH=/usr/local/lib:$LD_LIBRARY_PATH
```

#### **Rust Version Issues**
```bash
# Update Rust
rustup update

# Use specific toolchain
rustup override set 1.75.0
rustup override set nightly
```

### **Performance Issues**

#### **Slow Builds**
```bash
# Enable parallel compilation
export RUSTFLAGS="-C codegen-units=16"

# Use incremental compilation
export CARGO_INCREMENTAL=1

# Use sccache for faster rebuilds
cargo install sccache
export RUSTC_WRAPPER=sccache
```

#### **Memory Issues**
```bash
# Limit parallel jobs
cargo build --jobs 4

# Use specific target
cargo build --target x86_64-unknown-linux-gnu
```

## Contributing

            ### **Fork and Clone**

            ```bash
            # Fork the repository on GitHub
            # Then clone your fork
            git clone https://github.com/your-username/deribit-fix.git
            cd deribit-fix

            # Add upstream remote
            git remote add upstream https://github.com/joaquinbejar/deribit-fix.git
            ```

### **Development Workflow**

```bash
# Create feature branch
git checkout -b feature/new-message-type

# Make changes and test
cargo test
cargo fmt
cargo clippy

# Commit changes
git add .
git commit -m "feat: implement new message type"

# Push to your fork
git push origin feature/new-message-type
```

## Next Steps

After successful source installation:

1. **[Basic Examples](../usage/basic_example.md)** - Learn fundamental operations
2. **[Development Guide](../../03_development_guide/main.md)** - Contribute to the project
3. **[API Reference](../../02_api_reference/main.md)** - Explore available functionality

---

**Source installation complete?** Move on to [Basic Usage Examples](../usage/basic_example.md) to start trading!
