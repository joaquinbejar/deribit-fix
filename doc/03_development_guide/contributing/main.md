# Contributing to deribit-fix

Welcome to the `deribit-fix` project! This guide provides everything you need to know to contribute to the development of this Rust crate for Deribit FIX protocol integration.

## Overview

The `deribit-fix` crate is an open-source project that provides a high-performance, reliable Rust implementation for trading on Deribit using the FIX protocol. We welcome contributions from the community and appreciate your help in making this project better.

## How to Contribute

There are many ways to contribute to the project:

- **Code Contributions**: Bug fixes, new features, performance improvements
- **Documentation**: Improving guides, examples, and API documentation
- **Testing**: Writing tests, improving test coverage, finding bugs
- **Code Review**: Reviewing pull requests and providing feedback
- **Issue Reporting**: Reporting bugs, suggesting features, discussing improvements

## Quick Start

1. **Fork the repository** on GitHub
2. **Clone your fork** locally
3. **Set up the development environment**
4. **Make your changes**
5. **Run tests** to ensure everything works
6. **Submit a pull request**

## Development Setup

### Prerequisites

- Rust 1.70+ (latest stable recommended)
- Git
- A GitHub account

### Local Setup

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/deribit-fix.git
cd deribit-fix

# Add upstream remote
git remote add upstream https://github.com/joaquinbejar/deribit-fix.git

# Install development dependencies
cargo install cargo-husky
cargo install cargo-tarpaulin
cargo install cargo-criterion

# Set up pre-commit hooks
cargo husky install
```

### Development Workflow

```bash
# Create a new branch for your changes
git checkout -b feature/your-feature-name

# Make your changes
# ... edit files ...

# Run tests
cargo test

# Run linting
cargo clippy

# Format code
cargo fmt

# Commit your changes
git commit -m "feat: add new feature description"

# Push to your fork
git push origin feature/your-feature-name
```

## Code Quality Standards

### Rust Conventions

We follow Rust community standards and best practices:

- **Rust API Guidelines**: Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- **Rust Book**: Follow patterns from [The Rust Programming Language](https://doc.rust-lang.org/book/)
- **Async Rust**: Use async/await patterns consistently
- **Error Handling**: Use `thiserror` for custom error types

### Code Style

- **Formatting**: Use `rustfmt` with default settings
- **Linting**: Pass `clippy` with no warnings
- **Documentation**: Document all public APIs with doc comments
- **Tests**: Maintain high test coverage (>90%)

### Performance

- **Benchmarks**: Add benchmarks for performance-critical code
- **Profiling**: Profile code before and after changes
- **Memory**: Minimize allocations and use efficient data structures
- **Async**: Use efficient async patterns and avoid blocking operations

## Testing Requirements

### Test Coverage

- **Unit Tests**: All public functions must have unit tests
- **Integration Tests**: Test component interactions
- **Property Tests**: Use `proptest` for data validation
- **Benchmarks**: Performance-critical code must have benchmarks

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run integration tests
cargo test --test '*'

# Run benchmarks
cargo bench

# Check test coverage
cargo tarpaulin
```

## Pull Request Process

### Before Submitting

1. **Ensure tests pass**: All tests must pass locally
2. **Check formatting**: Run `cargo fmt` and `cargo clippy`
3. **Update documentation**: Document any new public APIs
4. **Add tests**: Include tests for new functionality
5. **Update changelog**: Document user-facing changes

### Pull Request Template

```markdown
## Description

Brief description of the changes made.

## Type of Change

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## Testing

- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Benchmarks pass
- [ ] Manual testing completed

## Checklist

- [ ] My code follows the style guidelines of this project
- [ ] I have performed a self-review of my own code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have made corresponding changes to the documentation
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] New and existing unit tests pass locally with my changes
- [ ] Any dependent changes have been merged and published in downstream modules
```

## Issue Reporting

### Bug Reports

When reporting bugs, please include:

- **Description**: Clear description of the problem
- **Reproduction**: Steps to reproduce the issue
- **Expected vs Actual**: What you expected vs what happened
- **Environment**: Rust version, OS, dependencies
- **Code Example**: Minimal code that reproduces the issue

### Feature Requests

For feature requests, please include:

- **Description**: Clear description of the feature
- **Use Case**: Why this feature is needed
- **Proposed API**: How you envision the API working
- **Alternatives**: Any alternatives you've considered

## Communication

### Getting Help

- **GitHub Issues**: Use GitHub issues for bugs and feature requests
- **Discussions**: Use GitHub Discussions for questions and general discussion
- **Code Review**: Ask questions in pull request comments

### Community Guidelines

- Be respectful and inclusive
- Help others learn and grow
- Provide constructive feedback
- Follow the project's code of conduct

## Development Areas

### High Priority

- **FIX Protocol Compliance**: Ensure full FIX 4.4 compliance
- **Performance**: Optimize for low latency and high throughput
- **Reliability**: Improve error handling and recovery
- **Testing**: Increase test coverage and add property tests

### Medium Priority

- **Documentation**: Improve examples and API documentation
- **Monitoring**: Add metrics and observability
- **Configuration**: Enhance configuration management
- **Logging**: Improve structured logging and debugging

### Low Priority

- **Examples**: Add more usage examples
- **Benchmarks**: Expand benchmark suite
- **Tools**: Add development and debugging tools

## Getting Started with Development

### First Contribution

If you're new to the project, here are some good first issues:

1. **Documentation**: Fix typos, improve examples
2. **Tests**: Add missing test cases
3. **Examples**: Create usage examples
4. **Linting**: Fix clippy warnings

### Understanding the Codebase

- **Architecture**: Read the [Architecture Guide](../architecture/main.md)
- **API Reference**: Review the [API Reference](../../02_api_reference/main.md)
- **Examples**: Study the [Usage Examples](../usage/basic_example.md)

## Release Process

### Versioning

We follow [Semantic Versioning](https://semver.org/):

- **Major**: Breaking changes
- **Minor**: New features, backward compatible
- **Patch**: Bug fixes, backward compatible

### Release Checklist

- [ ] All tests pass
- [ ] Documentation is updated
- [ ] Changelog is updated
- [ ] Version is bumped
- [ ] Release notes are written
- [ ] GitHub release is created

## Recognition

Contributors are recognized in:

- **Contributors list** on GitHub
- **Changelog** for each release
- **README** for significant contributions
- **Documentation** for major features

## Questions?

If you have questions about contributing:

- Check the [Development Guide](main.md) for detailed information
- Review existing [Issues](https://github.com/joaquinbejar/deribit-fix/issues)
- Join [Discussions](https://github.com/joaquinbejar/deribit-fix/discussions)
- Contact the maintainer: [jb@taunais.com](mailto:jb@taunais.com)

Thank you for contributing to `deribit-fix`!
