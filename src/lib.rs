//! # Deribit FIX Framework
//!
//! A comprehensive FIX protocol client framework for Deribit cryptocurrency exchange.
//! This library provides a reusable foundation for building trading applications
//! that connect to Deribit using the FIX protocol.
//!
//! ## Overview
//!
//! Deribit FIX API is based on FIX version 4.4 with some tags from version 5.0 and custom tags.
//! This framework implements the complete Deribit FIX specification, providing a robust
//! and type-safe interface for cryptocurrency derivatives trading.
//!
//! ### Supported Environments
//! - **Production**: `www.deribit.com:9880` (raw TCP) / `www.deribit.com:9883` (SSL)
//! - **Test**: `test.deribit.com:9881` (raw TCP) / `test.deribit.com:9883` (SSL)
//!
//! ## Key Features
//!
//! ### Core FIX Protocol Support
//! - **Session Management**: Logon(A), Logout(5), Heartbeat(0), Test Request(1)
//! - **Message Sequencing**: Resend Request(2), Sequence Reset(4)
//! - **Error Handling**: Reject(3) messages with proper error codes
//!
//! ### Trading Operations
//! - **Order Management**: New Order Single(D), Order Cancel Request(F), Order Cancel/Replace Request(G)
//! - **Mass Operations**: Order Mass Cancel Request(q), Order Mass Status Request(AF)
//! - **Execution Reports**: Real-time order status updates and fill notifications
//! - **Position Management**: Request For Positions(AN), Position Report(AP)
//!
//! ### Market Data
//! - **Real-time Data**: Market Data Request(V), Market Data Snapshot/Full Refresh(W)
//! - **Incremental Updates**: Market Data Incremental Refresh(X)
//! - **Security Information**: Security List Request(x), Security Definition Request(c)
//! - **Instrument Status**: Security Status Request(e), Security Status(f)
//!
//! ### Advanced Features
//! - **Market Making**: Mass Quote(i), Quote Request(R), Quote Cancel(Z)
//! - **RFQ System**: RFQ Request(AH), Quote Status Report(AI)
//! - **Risk Management**: MMProtection Limits(MM), MMProtection Reset(MZ)
//! - **Trade Reporting**: TradeCaptureReportRequest(AD), TradeCaptureReport(AE)
//!
//! ### Authentication & Security
//! - **Secure Authentication**: SHA256-based authentication with nonce
//! - **Application Registration**: Support for registered applications with DeribitAppSig
//! - **Cancel on Disconnect**: Automatic order cancellation on connection loss
//! - **User Management**: User Request(BE), User Response(BF)
//!
//! ## Technical Features
//!
//! - **Async/Await**: Full async support with Tokio runtime
//! - **Connection Management**: Automatic reconnection with configurable backoff
//! - **Message Validation**: Comprehensive FIX message parsing and validation
//! - **Type Safety**: Strongly typed message structures and enums
//! - **Error Handling**: Detailed error types with context
//! - **Logging**: Configurable logging with tracing support
//! - **Testing**: Comprehensive test suite with mock server support
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
        message::security_list::*,
        model::*,
    };
}
