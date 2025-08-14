# Changelog

This document provides a comprehensive history of changes made to the `deribit-fix` crate across all versions. It follows the [Keep a Changelog](https://keepachangelog.com/) format and [Semantic Versioning](https://semver.org/) principles.

## Overview

The changelog tracks all user-facing changes, including:

- **New Features**: New functionality and capabilities
- **Bug Fixes**: Corrections to existing functionality
- **Breaking Changes**: Changes that require user code updates
- **Performance Improvements**: Optimizations and speed enhancements
- **Documentation**: Updates to guides, examples, and API docs
- **Dependencies**: Updates to external dependencies

## Version History

---

## [0.2.0] - 2025-08-14

**Release Date**: August 14, 2025  
**Status**: Production Ready - Feature Complete Release

This is a major feature release that completes the implementation of the Deribit FIX specification, making the framework production-ready with comprehensive functionality.

### 🚀 New Features

#### Core Protocol Implementation
- **Complete FIX 4.4 Support**: Full implementation of FIX 4.4 protocol with Deribit extensions
- **Advanced Session Management**: Enhanced logon/logout, heartbeat, and sequence management
- **Message Recovery**: Comprehensive resend request and gap detection mechanisms
- **SSL/TLS Support**: Production-grade encrypted connections for both test and production environments

#### Trading Operations
- **Order Management System**: Complete order lifecycle management (New Order Single, Cancel, Replace)
- **Mass Operations**: Order Mass Cancel Request and Order Mass Status Request
- **Position Management**: Real-time position tracking and reporting (Request For Positions, Position Report)
- **Execution Reporting**: Real-time order status updates and fill notifications

#### Market Data
- **Real-time Streaming**: Market Data Request with live feed subscriptions
- **Market Snapshots**: Market Data Snapshot/Full Refresh for complete market state
- **Incremental Updates**: Efficient Market Data Incremental Refresh
- **Security Information**: Security List Request and Security Definition Request
- **Instrument Status**: Security Status Request and Security Status updates

#### Advanced Trading Features
- **Market Making**: Mass Quote submission, Quote Request, and Quote Cancel
- **RFQ System**: Request for Quote (RFQ Request, Quote Status Report)
- **Risk Management**: MMProtection Limits and MMProtection Reset
- **Trade Reporting**: TradeCaptureReportRequest and TradeCaptureReport

#### Authentication & Security
- **SHA256 Authentication**: Secure credential-based authentication with nonce support
- **Application Registration**: Support for registered applications with DeribitAppSig
- **Cancel on Disconnect**: Automatic order cancellation on connection loss
- **User Management**: User Request and User Response messages

### 🏗️ Technical Improvements

#### Architecture
- **Async/Await**: Full async support with Tokio runtime
- **Connection Management**: Automatic reconnection with configurable backoff strategies
- **Message Validation**: Comprehensive FIX message parsing and validation
- **Type Safety**: Strongly typed message structures and enums
- **Error Handling**: Detailed error types with rich context

#### Performance
- **Zero-Copy Parsing**: Optimized message parsing for low latency
- **Connection Pooling**: Efficient resource utilization
- **Memory Management**: Optimized memory usage patterns
- **Async I/O**: Non-blocking network operations

#### Development Experience
- **Rich Examples**: 11 comprehensive examples covering all major use cases
- **Extensive Documentation**: 100% public API documentation coverage
- **Development Tools**: Complete Makefile with all necessary commands
- **Testing Suite**: 90%+ code coverage with unit and integration tests

### 📚 Examples Added
- **Basic Client**: Simple client setup and connection
- **Session Management**: Advanced session handling and monitoring
- **Order Management**: Complete order lifecycle examples
- **Position Management**: Position tracking and reporting
- **Market Data Streaming**: Real-time market data consumption
- **Error Handling**: Comprehensive error scenarios and recovery
- **Login Test**: Authentication flow examples
- **Heartbeat Example**: Connection keep-alive handling
- **Test Request Example**: Connection health monitoring
- **Resend Request Example**: Message recovery patterns
- **Reject Example**: Error handling and rejection scenarios

### 🧪 Testing & Quality
- **Unit Tests**: 100+ unit tests covering all modules
- **Integration Tests**: End-to-end scenarios with mock servers
- **Coverage Reporting**: Automated coverage reporting with tarpaulin
- **Continuous Integration**: Automated testing and quality checks
- **Benchmarking**: Performance regression testing
- **Linting**: Strict code quality enforcement with Clippy

### 🛠️ Developer Tools
- **Makefile**: Comprehensive build and development commands
- **Pre-push Hooks**: Automated quality checks before commits
- **Documentation Generation**: Automated README generation from lib.rs
- **Coverage Reports**: HTML and XML coverage report generation
- **Benchmarking**: Performance benchmarking with criterion

### 📖 Documentation
- **Complete API Documentation**: All public APIs fully documented
- **Usage Examples**: Practical examples for all major features
- **Architecture Guide**: Detailed technical architecture documentation
- **Contributing Guide**: Clear guidelines for contributors
- **Installation Guide**: Step-by-step setup instructions

### 🔧 Configuration
- **Flexible Configuration**: Builder pattern for easy configuration
- **Environment Support**: Both production and test environment support
- **Logging Integration**: Configurable logging with tracing support
- **Timeout Management**: Configurable timeouts for all operations

### 🚨 Breaking Changes
None - This is a new major release building on the foundation of 0.1.x

### 📋 Migration Guide
For users upgrading from 0.1.x versions:
- Update your `Cargo.toml` to use version `0.2.0`
- Review the new examples for updated usage patterns
- Consider using the new advanced features for enhanced functionality
- Update import statements to use the `prelude` module for convenience

---

## [0.1.1] - 2024-12-XX

**Release Date**: December 2024  
**Status**: Patch Release

### 🐛 Bug Fixes
- **Documentation**: Fixed outdated references to missing fields and builders
- **Market Data Snapshot (35=W)**: Added missing snapshot-only fields (MarkPrice, CurrentFunding, Funding8h, UnderlyingPx, ContractMultiplier, PutOrCall)
- **Position Report (35=AP)**: Confirmed and documented dedicated builder for emission and parser for consumption

### 📖 Documentation
- **API Docs**: Updated project overview and API reference for MarketData and Position
- **Features Documentation**: Enhanced features documentation with current capabilities

### 🔧 Migration Guide
No changes required for existing users.

---

## [0.1.0] - 2024-12-XX

**Release Date**: December 2024  
**Status**: Initial Development Release

This is the initial development release of the `deribit-fix` crate, providing foundational FIX protocol functionality for Deribit trading.

### 🚀 New Features
- **Core FIX Protocol Support**: Basic FIX 4.4 message handling
- **Connection Management**: TCP connection establishment and management
- **Session Management**: FIX session lifecycle and sequence management
- **Order Management**: Basic order placement and management
- **Market Data**: Market data subscription and handling
- **Error Handling**: Comprehensive error types and recovery strategies
- **Configuration**: Flexible configuration system with environment support
- **Logging**: Structured logging with configurable levels

### 📖 Documentation
- **Initial Documentation**: Basic API documentation and usage examples
- **README**: Project overview and getting started guide

---

## Future Roadmap

### Version 0.3.0 (Planned - Q1 2026)
- **Enhanced Performance**: Further latency optimizations
- **Advanced Risk Management**: Additional risk control features
- **Extended Market Data**: Enhanced market data features
- **Production Monitoring**: Advanced monitoring and alerting

### Version 1.0.0 (Planned - Q2 2026)
- **API Stabilization**: Final API with stability guarantees
- **Production SLAs**: Performance guarantees and SLA definitions
- **Enterprise Features**: Advanced enterprise-grade features
- **Long-term Support**: Commitment to long-term maintenance

---

## Repository Information

- **Repository**: https://github.com/joaquinbejar/deribit-fix
- **Documentation**: https://docs.rs/deribit-fix
- **Crate**: https://crates.io/crates/deribit-fix
- **License**: MIT
- **Maintainer**: Joaquin Bejar Garcia (jb@taunais.com)

## Contributing

We welcome contributions! Please see our [Contributing Guide](../contributing/main.md) for details on how to get started.

## Support

For questions, issues, or support:
- Open an issue on [GitHub Issues](https://github.com/joaquinbejar/deribit-fix/issues)
- Contact the maintainer at jb@taunais.com
- Check the [Documentation](https://docs.rs/deribit-fix)

---

*This changelog is automatically updated with each release. For the most current information, please refer to the repository.*