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

### Current Version: 0.1.1

**Release Date**: December 2024  
**Status**: Patch Release

This patch release includes documentation updates and feature completeness confirmations for market data snapshots and position reports.

#### What's New

- **Market Data Snapshot (35=W)**: Added snapshot-only fields (MarkPrice, CurrentFunding, Funding8h, UnderlyingPx, ContractMultiplier, PutOrCall) to emitted messages and docs
- **Position Report (35=AP)**: Confirmed and documented dedicated builder for emission and parser for consumption
- **API Docs**: Updated project overview, API reference for MarketData and Position, and enhanced features documentation

#### Bug Fixes

- **Docs**: Removed outdated references to missing fields and builders

#### Breaking Changes

None

#### Migration Guide

No changes required for existing users.

---

### Previous Version: 0.1.0

**Release Date**: December 2024  
**Status**: Initial Development Release

This is the initial development release of the `deribit-fix` crate, providing core FIX protocol functionality for Deribit trading.

#### What's New

- **Core FIX Protocol Support**: Basic FIX 4.4 message handling
- **Connection Management**: TCP connection establishment and management
- **Session Management**: FIX session lifecycle and sequence management
- **Order Management**: Basic order placement and management
- **Market Data**: Market data subscription and handling
- **Error Handling**: Comprehensive error types and recovery strategies
- **Configuration**: Flexible configuration system with environment support
- **Logging**: Structured logging with configurable levels

#### Breaking Changes

None (initial release)

#### Deprecations

None (initial release)

#### Migration Guide

N/A (initial release)

### Planned Versions

#### Version 0.2.0 (Q1 2025)

**Planned Features**:
- Advanced order types (stop orders, trailing stops)
- Position management and risk controls
- Enhanced market data (order book snapshots, tick data)
- Connection pooling and load balancing
- Performance monitoring and metrics
- Extended FIX message support

**Breaking Changes**:
- Potential API refinements based on user feedback
- Configuration structure improvements

#### Version 0.3.0 (Q2 2025)

**Planned Features**:
- Full FIX 4.4 compliance
- Advanced trading strategies support
- Real-time performance optimization
- Comprehensive testing suite
- Production deployment tools

**Breaking Changes**:
- Final API stabilization
- Performance optimization changes

#### Version 1.0.0 (Q3 2025)

**Planned Features**:
- Production-ready stability
- Full feature parity with Deribit API
- Comprehensive documentation and examples
- Performance benchmarks and SLAs
- Enterprise support features

**Breaking Changes**:
- API stability guarantee
- Long-term support commitment

## Change Categories

### 🔥 Breaking Changes

Breaking changes require updates to user code and are marked with major version increments.

**Examples**:
- Function signature changes
- Struct field modifications
- Trait requirement updates
- Configuration format changes

**Migration Support**:
- Detailed migration guides for each breaking change
- Deprecation warnings in previous versions
- Compatibility layers when possible

### ✨ New Features

New features add functionality without breaking existing code.

**Examples**:
- New order types
- Additional market data feeds
- Enhanced configuration options
- New utility functions

**Documentation**:
- Comprehensive examples for new features
- API reference updates
- Migration guides for feature adoption

### 🐛 Bug Fixes

Bug fixes correct issues without changing the public API.

**Examples**:
- Connection stability improvements
- Message parsing corrections
- Error handling enhancements
- Performance optimizations

**Testing**:
- Regression tests for fixed bugs
- Performance benchmarks
- Integration test coverage

### ⚡ Performance Improvements

Performance improvements enhance speed and efficiency.

**Examples**:
- Reduced latency
- Increased throughput
- Lower memory usage
- Better CPU utilization

**Benchmarks**:
- Before/after performance metrics
- Load testing results
- Resource usage comparisons

### 📚 Documentation

Documentation updates improve user experience and developer onboarding.

**Examples**:
- API reference updates
- New examples and tutorials
- Architecture documentation
- Best practices guides

**Quality**:
- Regular documentation reviews
- User feedback integration
- Example code validation

## Version Compatibility

### Rust Version Support

| deribit-fix Version | Minimum Rust Version | Recommended Rust Version |
|---------------------|----------------------|--------------------------|
| 0.1.x               | 1.70.0               | 1.75.0                  |
| 0.2.x               | 1.70.0               | 1.75.0                  |
| 0.3.x               | 1.70.0               | 1.75.0                  |
| 1.0.x               | 1.70.0               | 1.75.0                  |

### Dependency Compatibility

| deribit-fix Version | tokio Version | serde Version | chrono Version |
|---------------------|---------------|---------------|----------------|
| 0.1.x               | 1.28+        | 1.0+         | 0.4+          |
| 0.2.x               | 1.28+        | 1.0+         | 0.4+          |
| 0.3.x               | 1.28+        | 1.0+         | 0.4+          |
| 1.0.x               | 1.28+        | 1.0+         | 0.4+          |

## Release Process

### Release Cycle

- **Patch Releases** (0.1.1, 0.1.2): Bug fixes and minor improvements
- **Minor Releases** (0.2.0, 0.3.0): New features and enhancements
- **Major Releases** (1.0.0): Breaking changes and major milestones

### Release Schedule

- **Monthly**: Patch releases for critical bug fixes
- **Quarterly**: Minor releases for new features
- **As Needed**: Major releases for breaking changes

### Release Checklist

- [ ] All tests pass
- [ ] Benchmarks show no regressions
- [ ] Documentation is updated
- [ ] Changelog is complete
- [ ] Version is bumped
- [ ] Release notes are written
- [ ] GitHub release is created
- [ ] Crates.io is updated

## Contributing to Changelog

### For Contributors

When contributing changes, please:

1. **Update the changelog** with your changes
2. **Use appropriate categories** for change classification
3. **Provide clear descriptions** of what changed
4. **Include migration notes** for breaking changes
5. **Reference related issues** and pull requests

### Change Description Format

```markdown
### Changed
- **API**: Description of API changes
- **Configuration**: Description of config changes
- **Performance**: Description of performance changes

### Added
- **Feature**: Description of new feature
- **Function**: Description of new function

### Fixed
- **Bug**: Description of bug fix
- **Issue**: Reference to related issue
```

## Historical Context

### Development Milestones

- **Q4 2024**: Initial development and core functionality
- **Q1 2025**: Feature expansion and API refinement
- **Q2 2025**: Performance optimization and testing
- **Q3 2025**: Production readiness and stability

### Key Decisions

- **FIX Protocol**: Chose FIX 4.4 for maximum compatibility
- **Async Rust**: Built on tokio for high-performance async operations
- **Error Handling**: Comprehensive error types with recovery strategies
- **Configuration**: Flexible configuration with environment support

## Support and Maintenance

### Long-Term Support

- **Version 1.0+**: 2 years of active support
- **Version 0.x**: 1 year of active support
- **Security Updates**: Extended support for critical security issues

### Deprecation Policy

- **6 months notice** for feature deprecation
- **12 months notice** for breaking changes
- **Migration guides** for all deprecated features
- **Compatibility layers** when possible

## Questions and Feedback

For questions about the changelog or release process:

- **GitHub Issues**: [Create an issue](https://github.com/joaquinbejar/deribit-fix/issues)
- **Discussions**: [Join discussions](https://github.com/joaquinbejar/deribit-fix/discussions)
- **Contact**: [jb@taunais.com](mailto:jb@taunais.com)

## Summary

The changelog provides a comprehensive record of all changes to the `deribit-fix` crate, helping users:

- **Track Changes**: Understand what's new in each version
- **Plan Upgrades**: Prepare for breaking changes and new features
- **Debug Issues**: Identify when issues were introduced or fixed
- **Contribute**: Understand the project's evolution and direction

Regular updates ensure transparency and help build a strong, informed user community.
