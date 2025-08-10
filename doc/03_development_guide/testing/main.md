# Testing Guide

This section provides comprehensive guidance on testing the `deribit-fix` crate, including unit tests, integration tests, benchmarks, and testing best practices.

## Overview

Testing is a critical component of the `deribit-fix` crate development process. We employ a multi-layered testing strategy to ensure reliability, performance, and correctness across all components.

## Testing Strategy

Our testing approach follows the testing pyramid:
- **Unit Tests**: Fast, isolated tests for individual functions and methods
- **Integration Tests**: Tests for module interactions and API boundaries
- **Property Tests**: Tests for invariants and edge cases
- **Benchmarks**: Performance testing and regression detection
- **Mock Tests**: Tests with controlled external dependencies

## Quick Start

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run integration tests only
cargo test --test '*'

# Run benchmarks
cargo bench

# Run tests with coverage
cargo tarpaulin
```

## Test Organization

- **Unit Tests**: Located alongside source code in `mod.rs` files
- **Integration Tests**: Located in `tests/integration/`
- **Benchmarks**: Located in `benches/`
- **Test Utilities**: Located in `tests/unit/`

## Key Testing Areas

- **Connection Management**: TCP connection handling, reconnection logic
- **Session Management**: Authentication, heartbeat, sequence management
- **Message Handling**: FIX message parsing, validation, serialization
- **Order Management**: Order creation, modification, cancellation
- **Market Data**: Subscription, data processing, error handling
- **Error Handling**: Error propagation, recovery strategies
- **Configuration**: Config loading, validation, merging

## Testing Tools

- **Rust Test Framework**: Built-in testing support
- **Mockall**: Mocking framework for external dependencies
- **Criterion**: Benchmarking framework
- **Tarpaulin**: Code coverage analysis
- **Proptest**: Property-based testing

## Continuous Integration

All tests are automatically run on:
- Pull requests
- Merges to main branch
- Nightly builds
- Release candidates

## Next Steps

- [Unit Testing](./unit_testing.md) - Detailed unit testing guide
- [Integration Testing](./integration_testing.md) - Integration testing strategies
- [Benchmarking](./benchmarking.md) - Performance testing guide
- [Mock Testing](./mock_testing.md) - Testing with external dependencies
- [Test Utilities](./test_utilities.md) - Common testing helpers

## Best Practices

1. **Test Naming**: Use descriptive test names that explain the scenario
2. **Test Isolation**: Each test should be independent and not affect others
3. **Edge Cases**: Test boundary conditions and error scenarios
4. **Performance**: Include benchmarks for performance-critical code
5. **Coverage**: Aim for high test coverage, especially for critical paths
6. **Documentation**: Document complex test scenarios and setup requirements

## Getting Help

If you encounter testing issues:
1. Check the test output for detailed error messages
2. Review the test documentation in each module
3. Consult the integration test examples
4. Open an issue with test failure details
