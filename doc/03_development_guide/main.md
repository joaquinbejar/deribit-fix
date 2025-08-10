# Development Guide

This section provides comprehensive guidance for developers working with the `deribit-fix` crate, including testing, contributing, and development workflows.

## Overview

The development guide covers all aspects of developing, testing, and contributing to the `deribit-fix` crate. Whether you're a contributor, maintainer, or just want to understand how the code works, this guide will help you get started.

## Development Environment Setup

### Prerequisites
- **Rust**: Latest stable version (1.70+ recommended)
- **Cargo**: Rust's package manager (included with Rust)
- **Git**: Version control system
- **IDE**: VS Code with rust-analyzer, IntelliJ Rust, or similar
- **Docker**: For running tests and development containers

### Quick Setup
```bash
# Clone the repository
git clone https://github.com/joaquinbejar/deribit-fix.git
cd deribit-fix

# Install dependencies
cargo build

# Run tests
cargo test

# Run linter
cargo clippy

# Format code
cargo fmt
```

## Development Workflow

### 1. Code Organization
The crate follows standard Rust project structure:
```
deribit-fix/
├── src/                    # Source code
│   ├── lib.rs             # Main library entry point
│   ├── client.rs          # Main client implementation
│   ├── config.rs          # Configuration management
│   ├── error.rs           # Error types and handling
│   ├── message.rs         # FIX message handling
│   ├── session.rs         # FIX session management
│   └── types.rs           # Common types and enums
├── tests/                 # Integration tests
├── examples/              # Usage examples
├── benches/               # Performance benchmarks
├── doc/                   # Documentation
└── Cargo.toml            # Project configuration
```

### 2. Development Commands
```bash
# Build the project
cargo build

# Build with optimizations
cargo build --release

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run integration tests
cargo test --test integration_test_name

# Run benchmarks
cargo bench

# Check code without building
cargo check

# Run linter
cargo clippy

# Run linter with all warnings
cargo clippy -- -W clippy::all

# Format code
cargo fmt

# Format code and check
cargo fmt -- --check

# Generate documentation
cargo doc

# Open documentation in browser
cargo doc --open

# Check for security vulnerabilities
cargo audit

# Update dependencies
cargo update
```

### 3. Git Workflow
```bash
# Create feature branch
git checkout -b feature/new-feature

# Make changes and commit
git add .
git commit -m "feat: add new feature"

# Push branch
git push origin feature/new-feature

# Create pull request on GitHub
# After review and approval, merge to main
```

## Code Quality Standards

### 1. Rust Conventions
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use idiomatic Rust patterns
- Prefer `Result<T, E>` over panics
- Use meaningful variable and function names
- Document all public APIs

### 2. Code Style
- Use `rustfmt` for consistent formatting
- Follow Clippy recommendations
- Keep functions focused and small
- Use appropriate error types
- Add comprehensive documentation

### 3. Performance Considerations
- Minimize allocations in hot paths
- Use appropriate data structures
- Profile code for bottlenecks
- Write benchmarks for critical paths
- Consider async/await patterns

## Testing Strategy

### 1. Test Types
- **Unit Tests**: Test individual functions and methods
- **Integration Tests**: Test module interactions
- **Property Tests**: Test invariants with random data
- **Benchmarks**: Measure performance characteristics
- **Mock Tests**: Test with simulated dependencies

### 2. Test Organization
```rust
// Unit tests in source files
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function_name() {
        // Test implementation
    }
}

// Integration tests in tests/ directory
#[tokio::test]
async fn test_client_connection() {
    // Test client connection
}
```

### 3. Test Data
- Use realistic test data
- Test edge cases and error conditions
- Mock external dependencies
- Use test fixtures for complex data

## Documentation Standards

### 1. Code Documentation
```rust
/// Brief description of the function
///
/// Detailed explanation of what the function does,
/// including examples and edge cases.
///
/// # Arguments
/// * `param1` - Description of first parameter
/// * `param2` - Description of second parameter
///
/// # Returns
/// Description of return value
///
/// # Errors
/// Description of possible errors
///
/// # Examples
/// ```
/// use deribit_fix::some_function;
///
/// let result = some_function("example").await?;
/// ```
pub async fn some_function(param1: &str, param2: u32) -> Result<String, DeribitFixError> {
    // Implementation
}
```

### 2. README and Documentation
- Keep README up to date
- Document all configuration options
- Provide usage examples
- Include troubleshooting guides
- Maintain changelog

## Performance Optimization

### 1. Profiling
```bash
# Install cargo-instruments (macOS)
cargo install cargo-instruments

# Profile with instruments
cargo instruments --open

# Install flamegraph
cargo install flamegraph

# Generate flamegraph
cargo flamegraph
```

### 2. Benchmarking
```rust
#[cfg(test)]
mod benches {
    use super::*;
    use criterion::{criterion_group, criterion_main, Criterion};
    
    fn benchmark_function(c: &mut Criterion) {
        c.bench_function("function_name", |b| {
            b.iter(|| {
                // Benchmark code
            });
        });
    }
    
    criterion_group!(benches, benchmark_function);
    criterion_main!(benches);
}
```

## Security Considerations

### 1. Input Validation
- Validate all external inputs
- Sanitize user data
- Use secure defaults
- Implement rate limiting

### 2. Authentication
- Secure API key handling
- Use HTTPS for all connections
- Implement proper session management
- Log security events

### 3. Dependencies
- Regular security audits
- Keep dependencies updated
- Monitor for vulnerabilities
- Use minimal dependencies

## Continuous Integration

### 1. GitHub Actions
The project uses GitHub Actions for CI/CD:
- Automated testing on multiple platforms
- Code quality checks (clippy, fmt)
- Security scanning
- Documentation generation
- Release automation

### 2. Pre-commit Hooks
```bash
# Install pre-commit hooks
cargo install cargo-husky

# Setup hooks
cargo husky install

# Pre-commit will run:
# - cargo fmt -- --check
# - cargo clippy -- -W clippy::all
# - cargo test
```

## Release Process

### 1. Version Management
- Use semantic versioning (MAJOR.MINOR.PATCH)
- Update version in `Cargo.toml`
- Tag releases in Git
- Generate changelog

### 2. Release Checklist
- [ ] All tests pass
- [ ] Documentation is up to date
- [ ] Changelog is updated
- [ ] Version is bumped
- [ ] Release is tagged
- [ ] Crate is published to crates.io

## Troubleshooting

### 1. Common Issues
- **Build failures**: Check Rust version and dependencies
- **Test failures**: Verify test environment and data
- **Performance issues**: Use profiling tools
- **Memory leaks**: Check for circular references

### 2. Debugging
```rust
// Enable debug logging
env_logger::init();
log::set_max_level(log::LevelFilter::Debug);

// Use dbg! macro for quick debugging
dbg!(variable_name);

// Add breakpoints in IDE
// Use println! for simple debugging
```

## Getting Help

### 1. Resources
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust Reference](https://doc.rust-lang.org/reference/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Tokio Documentation](https://tokio.rs/docs/)

### 2. Community
- [Rust Users Forum](https://users.rust-lang.org/)
- [Rust Discord](https://discord.gg/rust-lang)
- [GitHub Issues](https://github.com/joaquinbejar/deribit-fix/issues)
- [GitHub Discussions](https://github.com/joaquinbejar/deribit-fix/discussions)

## Next Steps

- Learn about [Testing](testing/main.md) strategies and tools
- Understand [Contributing](contributing/main.md) guidelines
- Review the [Changelog](changelog/main.md) for version history
- Explore the [API Reference](02_api_reference/main.md) for implementation details
