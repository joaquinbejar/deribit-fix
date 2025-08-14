//! # Deribit FIX Framework
//!
//! [![Dual License](https://img.shields.io/badge/license-MIT-blue)](./LICENSE)
//! [![Crates.io](https://img.shields.io/crates/v/deribit-fix.svg)](https://crates.io/crates/deribit-fix)
//! [![Downloads](https://img.shields.io/crates/d/deribit-fix.svg)](https://crates.io/crates/deribit-fix)
//! [![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/deribit-fix)
//!
//! **Version:** 0.2.0 | **Status:** Production Ready | **Repository:** <https://github.com/joaquinbejar/deribit-fix>
//!
//! A comprehensive, production-ready FIX protocol client framework for Deribit cryptocurrency exchange.
//! This library provides a complete, type-safe, and async foundation for building sophisticated trading applications
//! that connect to Deribit using the FIX 4.4 protocol with custom extensions.
//!
//! ## 🏗️ Project Status
//!
//! **Current Version: 0.2.0** - Feature Complete & Production Ready
//!
//! This release represents a complete implementation of the Deribit FIX specification with:
//! - ✅ Full FIX 4.4 protocol support with Deribit extensions
//! - ✅ Complete trading operations (orders, positions, mass operations)
//! - ✅ Real-time market data streaming and snapshots
//! - ✅ Comprehensive session management and error handling
//! - ✅ Production-grade connection management with SSL support
//! - ✅ Extensive test suite with 90%+ coverage
//! - ✅ Rich examples and documentation
//!
//! ## 🚀 Quick Start
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! deribit-fix = "0.2.0"
//! ```
//!
//! Basic usage:
//! ```rust,no_run
//! use deribit_fix::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let config = DeribitFixConfig::default()
//!         .with_credentials("your_key".to_string(), "your_secret".to_string())
//!         .with_heartbeat_interval(30);
//!
//!     let mut client = DeribitFixClient::new(config).await?;
//!     client.connect().await?;
//!     
//!     // Start trading!
//!     let positions = client.get_positions().await?;
//!     println!("Current positions: {}", positions.len());
//!     
//!     client.disconnect().await?;
//!     Ok(())
//! }
//! ```
//!
//! ## 🌐 Supported Environments
//!
//! | Environment | Raw TCP | SSL |
//! |-------------|---------|-----|
//! | **Production** | `www.deribit.com:9880` | `www.deribit.com:9883` |
//! | **Test** | `test.deribit.com:9881` | `test.deribit.com:9883` |
//!
//! ## 🎯 Core Features
//!
//! ### 📡 Session Management
//! - **Logon/Logout (A/5)**: Secure authentication with SHA256 + nonce
//! - **Heartbeat (0)**: Configurable keep-alive mechanism
//! - **Test Request (1)**: Connection health monitoring
//! - **Resend Request (2)**: Message recovery and gap filling
//! - **Sequence Reset (4)**: Sequence number management
//! - **Reject (3)**: Comprehensive error handling with detailed codes
//!
//! ### 📈 Trading Operations
//! - **Order Management**:
//!   - New Order Single (D) - Place orders with full parameter support
//!   - Order Cancel Request (F) - Cancel individual orders
//!   - Order Cancel/Replace Request (G) - Modify existing orders
//! - **Mass Operations**:
//!   - Order Mass Cancel Request (q) - Cancel multiple orders
//!   - Order Mass Status Request (AF) - Bulk order status queries
//! - **Execution Reports**: Real-time order status updates and fill notifications
//! - **Position Management**:
//!   - Request For Positions (AN) - Query current positions
//!   - Position Report (AP) - Real-time position updates
//!
//! ### 📊 Market Data
//! - **Real-time Streaming**:
//!   - Market Data Request (V) - Subscribe to live data feeds
//!   - Market Data Snapshot/Full Refresh (W) - Complete market snapshots
//!   - Market Data Incremental Refresh (X) - Efficient incremental updates
//! - **Security Information**:
//!   - Security List Request (x) - Available instruments
//!   - Security Definition Request (c) - Detailed instrument specifications
//!   - Security Status Request (e) / Security Status (f) - Instrument status updates
//!
//! ### 💼 Advanced Trading Features
//! - **Market Making**:
//!   - Mass Quote (i) - Bulk quote submission
//!   - Quote Request (R) - Request for quotes
//!   - Quote Cancel (Z) - Quote cancellation
//! - **RFQ (Request for Quote) System**:
//!   - RFQ Request (AH) - Submit RFQ requests
//!   - Quote Status Report (AI) - Quote status updates
//! - **Risk Management**:
//!   - MMProtection Limits (MM) - Market maker protection
//!   - MMProtection Reset (MZ) - Reset protection limits
//! - **Trade Reporting**:
//!   - TradeCaptureReportRequest (AD) - Request trade reports
//!   - TradeCaptureReport (AE) - Trade execution reports
//!
//! ### 🔐 Security & Authentication
//! - **SHA256 Authentication**: Secure credential-based authentication
//! - **Application Registration**: Support for registered apps with DeribitAppSig
//! - **Cancel on Disconnect**: Automatic order cancellation on connection loss
//! - **User Management**: User Request (BE) / User Response (BF)
//! - **SSL/TLS Support**: Encrypted connections for production environments
//!
//! ## ⚡ Technical Architecture
//!
//! ### 🏗️ Built on Modern Rust
//! - **Rust 2024 Edition**: Latest language features and performance
//! - **Async/Await**: Full async support with Tokio runtime
//! - **Type Safety**: Zero-cost abstractions with compile-time guarantees
//! - **Memory Safety**: No segfaults, buffer overflows, or memory leaks
//!
//! ### 🔌 Connection Management
//! - **Automatic Reconnection**: Configurable backoff strategies
//! - **Connection Pooling**: Efficient resource utilization
//! - **Timeout Handling**: Robust timeout management
//! - **SSL/TLS Support**: Production-grade encrypted connections
//!
//! ### 🎯 Message Processing
//! - **FIX Protocol Compliance**: Full FIX 4.4 with Deribit extensions
//! - **Message Validation**: Comprehensive parsing and validation
//! - **Sequence Management**: Automatic sequence number handling
//! - **Gap Detection**: Automatic message gap detection and recovery
//!
//! ### 🛡️ Error Handling
//! - **Detailed Error Types**: Comprehensive error classification
//! - **Context Preservation**: Rich error context for debugging
//! - **Recovery Strategies**: Automatic recovery from transient errors
//! - **Logging Integration**: Structured logging with tracing support
//!
//! ## 📚 Examples & Documentation
//!
//! The framework includes comprehensive examples covering all major use cases:
//!
//! ### 🔧 Basic Examples
//! - **Basic Client** (`examples/basic/`): Simple client setup and connection
//! - **Login Test** (`examples/session/login_test.rs`): Authentication examples
//! - **Heartbeat** (`examples/session/heartbeat_example.rs`): Keep-alive handling
//!
//! ### 📊 Trading Examples
//! - **Order Management** (`examples/order_management/`): Complete order lifecycle
//! - **Position Management** (`examples/position_management/`): Position tracking
//! - **Market Data** (`examples/market_data/`): Real-time data streaming
//!
//! ### 🚨 Advanced Examples
//! - **Error Handling** (`examples/error_handling/`): Comprehensive error scenarios
//! - **Session Management** (`examples/session/session_management.rs`): Advanced session handling
//! - **Test Requests** (`examples/session/test_request_example.rs`): Connection monitoring
//! - **Resend Requests** (`examples/session/resend_request_example.rs`): Message recovery
//!
//! ## 🧪 Testing & Quality
//!
//! ### 🔬 Comprehensive Test Suite
//! - **Unit Tests**: 100+ unit tests covering all modules
//! - **Integration Tests**: End-to-end scenarios with mock servers
//! - **Coverage**: 90%+ code coverage with detailed reports
//! - **Continuous Integration**: Automated testing on multiple platforms
//!
//! ### 📊 Quality Assurance
//! - **Clippy Linting**: Strict code quality enforcement
//! - **Rustfmt**: Consistent code formatting
//! - **Documentation**: 100% public API documentation
//! - **Benchmarks**: Performance regression testing
//!
//! ## 📦 Installation & Setup
//!
//! ### Prerequisites
//! - Rust 1.75+ (specified in `rust-toolchain.toml`)
//! - Tokio async runtime
//! - Valid Deribit API credentials
//!
//! ### Build Commands
//! ```bash
//! # Standard build
//! cargo build
//!
//! # Release build (recommended for production)
//! cargo build --release
//!
//! # Run tests
//! cargo test
//!
//! # Run examples
//! cargo run --example basic_client
//! ```
//!
//! ## 🚀 Performance
//!
//! - **Low Latency**: Optimized for high-frequency trading
//! - **Memory Efficient**: Zero-copy message parsing where possible
//! - **Async I/O**: Non-blocking network operations
//! - **Connection Pooling**: Efficient resource utilization
//!
//! ## 🛠️ Development Tools
//!
//! The project includes comprehensive development tooling:
//! ```bash
//! # Format code
//! make fmt
//!
//! # Lint code
//! make lint
//!
//! # Run tests
//! make test
//!
//! # Generate documentation
//! make doc
//!
//! # Run benchmarks
//! make bench
//! ```
//!
//! ## ⚠️ Important Notes
//!
//! - **Testing Required**: Always test with demo account before live trading
//! - **Risk Management**: Implement proper risk controls in your application
//! - **Rate Limits**: Respect Deribit's API rate limits
//! - **Error Handling**: Implement robust error handling for production use
//!
//! ## 📄 License & Disclaimer
//!
//! Licensed under MIT License. This software is not officially associated with Deribit.
//! Trading financial instruments carries risk - use at your own discretion.
//!

pub mod client;
pub mod config;
pub mod connection;
/// FIX protocol constants
pub mod constants;
pub mod error;
pub mod message;
/// FIX message models and data structures
pub mod model;
pub mod session;

pub use client::DeribitFixClient;
pub use config::DeribitFixConfig;
pub use error::{DeribitFixError, Result};
pub use message::admin::*;
pub use model::*;

/// Re-export commonly used types for convenience
pub mod prelude {

    pub use crate::{
        client::DeribitFixClient,
        config::DeribitFixConfig,
        error::{DeribitFixError, Result},
        message::admin::*,
        message::market_data::*,
        message::orders::*,
        message::security_list::*,
        model::*,
    };
}
