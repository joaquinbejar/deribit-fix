<div style="text-align: center;">
<img src="https://raw.githubusercontent.com/joaquinbejar/deribit-fix/refs/heads/main/doc/images/logo.png" alt="deribit-fix" style="width: 80%; height: 80%;">
</div>

[![Dual License](https://img.shields.io/badge/license-MIT-blue)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/deribit-fix.svg)](https://crates.io/crates/deribit-fix)
[![Downloads](https://img.shields.io/crates/d/deribit-fix.svg)](https://crates.io/crates/deribit-fix)
[![Stars](https://img.shields.io/github/stars/joaquinbejar/deribit-fix.svg)](https://github.com/joaquinbejar/deribit-fix/stargazers)
[![Issues](https://img.shields.io/github/issues/joaquinbejar/deribit-fix.svg)](https://github.com/joaquinbejar/deribit-fix/issues)
[![PRs](https://img.shields.io/github/issues-pr/joaquinbejar/deribit-fix.svg)](https://github.com/joaquinbejar/deribit-fix/pulls)
[![Build Status](https://img.shields.io/github/workflow/status/joaquinbejar/deribit-fix/CI)](https://github.com/joaquinbejar/deribit-fix/actions)
[![Coverage](https://img.shields.io/codecov/c/github/joaquinbejar/deribit-fix)](https://codecov.io/gh/joaquinbejar/deribit-fix)
[![Dependencies](https://img.shields.io/librariesio/github/joaquinbejar/deribit-fix)](https://libraries.io/github/joaquinbejar/deribit-fix)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/deribit-fix)
[![Wiki](https://img.shields.io/badge/wiki-latest-blue.svg)](https://deepwiki.com/joaquinbejar/deribit-fix)

## Deribit FIX Framework

A comprehensive FIX protocol client framework for Deribit cryptocurrency exchange.
This library provides a reusable foundation for building trading applications
that connect to Deribit using the FIX protocol.

### Overview

Deribit FIX API is based on FIX version 4.4 with some tags from version 5.0 and custom tags.
This framework implements the complete Deribit FIX specification, providing a robust
and type-safe interface for cryptocurrency derivatives trading.

#### Supported Environments
- **Production**: `www.deribit.com:9880` (raw TCP) / `www.deribit.com:9883` (SSL)
- **Test**: `test.deribit.com:9881` (raw TCP) / `test.deribit.com:9883` (SSL)

### Key Features

#### Core FIX Protocol Support
- **Session Management**: Logon(A), Logout(5), Heartbeat(0), Test Request(1)
- **Message Sequencing**: Resend Request(2), Sequence Reset(4)
- **Error Handling**: Reject(3) messages with proper error codes

#### Trading Operations
- **Order Management**: New Order Single(D), Order Cancel Request(F), Order Cancel/Replace Request(G)
- **Mass Operations**: Order Mass Cancel Request(q), Order Mass Status Request(AF)
- **Execution Reports**: Real-time order status updates and fill notifications
- **Position Management**: Request For Positions(AN), Position Report(AP)

#### Market Data
- **Real-time Data**: Market Data Request(V), Market Data Snapshot/Full Refresh(W)
- **Incremental Updates**: Market Data Incremental Refresh(X)
- **Security Information**: Security List Request(x), Security Definition Request(c)
- **Instrument Status**: Security Status Request(e), Security Status(f)

#### Advanced Features
- **Market Making**: Mass Quote(i), Quote Request(R), Quote Cancel(Z)
- **RFQ System**: RFQ Request(AH), Quote Status Report(AI)
- **Risk Management**: MMProtection Limits(MM), MMProtection Reset(MZ)
- **Trade Reporting**: TradeCaptureReportRequest(AD), TradeCaptureReport(AE)

#### Authentication & Security
- **Secure Authentication**: SHA256-based authentication with nonce
- **Application Registration**: Support for registered applications with DeribitAppSig
- **Cancel on Disconnect**: Automatic order cancellation on connection loss
- **User Management**: User Request(BE), User Response(BF)

### Technical Features

- **Async/Await**: Full async support with Tokio runtime
- **Connection Management**: Automatic reconnection with configurable backoff
- **Message Validation**: Comprehensive FIX message parsing and validation
- **Type Safety**: Strongly typed message structures and enums
- **Error Handling**: Detailed error types with context
- **Logging**: Configurable logging with tracing support
- **Testing**: Comprehensive test suite with mock server support

## Configuration

The framework uses environment variables for configuration with sensible defaults. Copy `.env.example` to `.env` and customize as needed.

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `DERIBIT_USERNAME` | "" | Your Deribit username |
| `DERIBIT_PASSWORD` | "" | Your Deribit password |
| `DERIBIT_HOST` | test.deribit.com / www.deribit.com | FIX server hostname |
| `DERIBIT_PORT` | 9881 / 9880 / 9883 | FIX server port |
| `DERIBIT_USE_SSL` | false | Enable SSL/TLS connection |
| `DERIBIT_TEST_MODE` | true | Use test environment |
| `DERIBIT_HEARTBEAT_INTERVAL` | 30 | Heartbeat interval in seconds |
| `DERIBIT_CONNECTION_TIMEOUT` | 10 | Connection timeout in seconds |
| `DERIBIT_RECONNECT_ATTEMPTS` | 3 | Number of reconnection attempts |
| `DERIBIT_RECONNECT_DELAY` | 5 | Delay between reconnections in seconds |
| `DERIBIT_ENABLE_LOGGING` | true | Enable logging |
| `DERIBIT_LOG_LEVEL` | info | Log level (trace, debug, info, warn, error) |
| `DERIBIT_SENDER_COMP_ID` | CLIENT | FIX sender company ID |
| `DERIBIT_TARGET_COMP_ID` | DERIBITSERVER | FIX target company ID |
| `DERIBIT_CANCEL_ON_DISCONNECT` | false | Cancel orders on disconnect |
| `DERIBIT_APP_ID` | - | Application ID (optional) |
| `DERIBIT_APP_SECRET` | - | Application secret (optional) |

### Quick Start

```rust
use deribit_fix::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Setup logging
    setup_logger();
    
    // Create configuration from environment variables
    let config = DeribitFixConfig::new()
        .with_credentials(
            "your_username".to_string(),
            "your_password".to_string()
        );
    
    // Create and connect client
    let mut client = DeribitFixClient::new(config).await?;
    client.connect().await?;
    
    // Your trading logic here
    
    client.disconnect().await?;
    Ok(())
}
```

### Configuration Examples

```rust
// Test environment (default)
let config = DeribitFixConfig::new();

// Production environment
let config = DeribitFixConfig::production();

// Production with SSL
let config = DeribitFixConfig::production_ssl();

// Custom configuration
let config = DeribitFixConfig::new()
    .with_endpoint("custom.host.com".to_string(), 9999)
    .with_ssl(true)
    .with_heartbeat_interval(60)
    .with_cancel_on_disconnect(true);
```

## Contribution and Contact

We welcome contributions to this project! If you would like to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and ensure that the project still builds and all tests pass.
4. Commit your changes and push your branch to your forked repository.
5. Submit a pull request to the main repository.

If you have any questions, issues, or would like to provide feedback, please feel free to contact the project maintainer:

**Joaquin Bejar Garcia**
- Email: jb@taunais.com
- GitHub: [joaquinbejar](https://github.com/joaquinbejar)

We appreciate your interest and look forward to your contributions!

## ✍️ License

Licensed under MIT license

## Disclaimer

This software is not officially associated with Deribit. Trading financial instruments carries risk, and this library is provided as-is without any guarantees. Always test thoroughly with a demo account before using in a live trading environment.
