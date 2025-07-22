# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
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
- **MsgType enum**: Added SecurityListRequest ('x') and SecurityList ('y') message types
- **Module exports**: Added security_list module to message module and lib.rs prelude
- **Code formatting**: Applied clippy suggestions for inline format arguments
- Code formatting improvements across multiple files for better readability
- Updated session module tests to include new authentication test suite
- Enhanced debug logging in authentication methods

### Fixed
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