# lib Module

## Overview

The `lib` module serves as the main entry point for the `deribit-fix` crate, providing the public API surface and re-exporting key components for easy access.

## Purpose

- **Public API Surface**: Exposes the main types and functions that users need
- **Re-exports**: Provides convenient access to commonly used components
- **Crate Configuration**: Defines crate-level attributes and metadata

## Public Interface

### Re-exported Types

```rust
pub use client::DeribitFixClient;
pub use config::Config;
pub use error::DeribitFixError;
pub use message::FixMessage;
pub use session::SessionManager;
pub use types::{Order, ExecutionReport, MarketData};
```

### Re-exported Functions

```rust
pub fn create_client(config: Config) -> Result<DeribitFixClient, DeribitFixError>
pub fn create_config() -> ConfigBuilder
```

## Usage Examples

### Basic Import

```rust
use deribit_fix::{DeribitFixClient, Config, DeribitFixError};

#[tokio::main]
async fn main() -> Result<(), DeribitFixError> {
    let config = Config::default();
    let client = DeribitFixClient::new(config)?;
    
    // Use the client...
    Ok(())
}
```

### Feature-based Imports

```rust
#[cfg(feature = "advanced")]
use deribit_fix::advanced::AdvancedTrading;

#[cfg(feature = "websocket")]
use deribit_fix::websocket::WebSocketClient;
```

## Module Dependencies

- `client`: Core client functionality
- `config`: Configuration management
- `error`: Error types and handling
- `message`: FIX message handling
- `session`: Session management
- `types`: Core data types

## Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_api_surface() {
        // Verify all public types are accessible
        let _: DeribitFixClient = todo!();
        let _: Config = todo!();
        let _: DeribitFixError = todo!();
    }
}
```

## Documentation Standards

- All public items must have comprehensive documentation
- Include usage examples for complex types
- Document any feature flags that affect the public API
- Maintain backward compatibility for public interfaces
