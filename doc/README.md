# Deribit FIX Client Documentation

Welcome to the comprehensive documentation for the **Deribit FIX Client** - a high-performance, type-safe FIX 4.4 client implementation in Rust for trading on Deribit cryptocurrency exchange.

## 📚 Documentation Structure

This documentation is organized into three main levels:

### **Level 1: Main Sections**
- **[Project Overview](01_project_overview/main.md)** - Get started with the crate, installation, and basic usage
- **[API Reference](02_api_reference/main.md)** - Complete API documentation and examples
- **[Development Guide](03_development_guide/main.md)** - Contributing, testing, and development workflows

### **Quick Start**

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
    
    // Your trading logic here
    
    Ok(())
}
```

## 🚀 Key Features

- **FIX 4.4 Protocol**: Full compliance with FIX 4.4 specification
- **Type Safety**: Rust's type system ensures compile-time safety
- **Async/Await**: Built on Tokio for high-performance async operations
- **Comprehensive Coverage**: 23/37 FIX message types implemented
- **Deribit Integration**: Optimized for Deribit's specific FIX API requirements
- **Error Handling**: Robust error handling with detailed error types

## 📊 Implementation Status

| Category | Status | Coverage |
|----------|--------|----------|
| **Administrative Messages** | ✅ Complete | 100% |
| **Market Data** | ✅ Complete | 100% |
| **Security Information** | ✅ Complete | 100% |
| **Order Management** | ⚠️ Partial | 86% |
| **Execution Reports** | ❌ Missing | 0% |
| **Quote Management** | ❌ Missing | 0% |

**Overall Coverage: 62% (23/37 messages)**

## 🔗 Quick Links

- **[Installation Guide](01_project_overview/installation/main.md)** - Add to your project
- **[Basic Examples](01_project_overview/usage/basic_example.md)** - Get started quickly
- **[API Reference](02_api_reference/main.md)** - Full API documentation
- **[Contributing](03_development_guide/contributing/main.md)** - Help improve the project

## 📖 What's Next?

1. **New to FIX?** Start with [Project Overview](01_project_overview/main.md)
2. **Ready to code?** Jump to [Basic Examples](01_project_overview/usage/basic_example.md)
3. **Need API details?** Check [API Reference](02_api_reference/main.md)
4. **Want to contribute?** Read [Contributing Guide](03_development_guide/contributing/main.md)

---

*This documentation is generated from the source code and updated with each release. For the latest information, always refer to the source repository.*
