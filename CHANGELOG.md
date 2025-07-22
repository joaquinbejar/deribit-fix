# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive unit tests for Deribit FIX authentication (`tests/unit/session/auth_tests.rs`)
- 8 new authentication test cases covering nonce generation, password hashing, and specification compliance
- Helper function `generate_test_auth_data()` for testing authentication logic
- Public access to `Session::generate_auth_data()` method for testing purposes
- Authentication tests validate compliance with official Deribit FIX API specification

### Changed
- Code formatting improvements across multiple files for better readability
- Updated session module tests to include new authentication test suite
- Enhanced debug logging in authentication methods

### Fixed
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