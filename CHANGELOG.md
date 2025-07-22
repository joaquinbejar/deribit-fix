# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Market Data FIX Messages**: Complete implementation of Market Data Request (MsgType='V'), Market Data Request Reject (MsgType='Y'), Market Data Snapshot/Full Refresh (MsgType='W'), and Market Data Incremental Refresh (MsgType='X') messages
- New `message::market_data` module with comprehensive FIX protocol support for market data operations
- Market Data Request message with all Deribit-specific fields and subscription options
- Market Data Request Reject message with detailed rejection reasons
- Market Data Snapshot/Full Refresh message for order book and trade data
- Market Data Incremental Refresh message for real-time updates
- Six new enums with FIX protocol conversions:
  - `MdSubscriptionRequestType` (Snapshot, SnapshotPlusUpdates, Unsubscribe)
  - `MdUpdateType` (FullRefresh, IncrementalRefresh)
  - `MdEntryType` (Bid, Offer, Trade, IndexValue, SettlementPrice)
  - `MdUpdateAction` (New, Change, Delete)
  - `MdReqRejReason` (UnknownSymbol, InsufficientPermissions, etc.)
- Supporting structures: `MdEntry` for market data entries
- 9 comprehensive unit tests covering all Market Data functionality
- **Security List FIX Messages**: Complete implementation of Security List Request (MsgType='x') and Security List (MsgType='y') messages
- New `message::security_list` module with comprehensive FIX protocol support
- Security List Request message with all Deribit-specific fields and filters
- Security List Response message with repeating groups for multiple securities
- Five new enums with FIX protocol conversions:
  - `SecurityListRequestType` (Snapshot, SnapshotAndUpdates)
  - `SubscriptionRequestType` (Snapshot, SnapshotPlusUpdates, Unsubscribe)
  - `SecurityType` (FxSpot, Future, Option, Perpetual, etc.)
  - `PutOrCall` (Put, Call)
  - `SecurityStatus` (Active, Inactive, Expired, Closed, etc.)
- Supporting structures: `SecurityInfo`, `SecurityAltId`, `TickRule`
- 17 comprehensive unit tests covering all Security List functionality
- Bidirectional enum conversions with proper error handling
- Integration with existing MessageBuilder infrastructure
- Comprehensive unit tests for Deribit FIX authentication (`tests/unit/session/auth_tests.rs`)
- 8 new authentication test cases covering nonce generation, password hashing, and specification compliance
- Helper function `generate_test_auth_data()` for testing authentication logic
- Public access to `Session::generate_auth_data()` method for testing purposes
- Authentication tests validate compliance with official Deribit FIX API specification

### Changed
- **MsgType enum**: Added Market Data message types (V, W, X, Y) and Security List message types (x, y)
- **Module exports**: Added market_data module to message module and lib.rs prelude
- **Module exports**: Added security_list module to message module and lib.rs prelude
- **Code formatting**: Applied clippy suggestions for inline format arguments
- Code formatting improvements across multiple files for better readability
- Updated session module tests to include new authentication test suite
- Enhanced debug logging in authentication methods

### Fixed
- **Market Data compilation errors**: Resolved MessageBuilder usage and enum naming conflicts
- Fixed MessageBuilder constructor to use `new().msg_type()` pattern instead of `new(MsgType)`
- Renamed `SubscriptionRequestType` to `MdSubscriptionRequestType` to avoid naming conflicts
- Fixed return type conversions from `FixMessage` to `String` using `build()?.to_string()`
- Resolved ambiguous glob re-exports between market_data and security_list modules
- **Security List compilation errors**: Resolved all type conversion and import issues
- Fixed enum to string conversions using explicit `i32::from(enum).to_string()`
- Corrected boolean to string conversions with explicit `.to_string()` calls
- Added missing `Copy` trait to `SecurityType` enum for better performance
- Removed unused `HashMap` import to eliminate warnings
- Fixed 6 clippy warnings using inline format arguments
- **CRITICAL**: Fixed Deribit FIX authentication implementation to comply with official specification
- Corrected RawData format from nonce-only to `timestamp.nonce` (separated by ASCII period)
- Fixed password hash calculation to use `base64(sha256(RawData ++ access_secret))` instead of `base64(sha256(nonce ++ access_secret))`
- Implemented cryptographically secure random nonce generation (32+ bytes)
- Fixed timestamp generation to use strictly increasing milliseconds
- Resolved authentication failures (`invalid_nonce_format` and `invalid_credentials` errors)
- Successful authentication and session establishment with Deribit test server confirmed

### Security
- Enhanced nonce generation with cryptographically secure random number generator
- Implemented minimum 32-byte nonce length as recommended by Deribit security guidelines
- Proper handling of sensitive authentication data in debug logs
- Validated authentication flow against official Deribit FIX API security requirements

## [0.1.0] - 2025-07-22

### Added
- Initial Alpha release
- Basic FIX protocol implementation for Deribit exchange
- TCP/TLS connection management
- Message parsing and construction
- Session management with state tracking
- Configuration management
- Error handling framework
- Basic examples and documentation