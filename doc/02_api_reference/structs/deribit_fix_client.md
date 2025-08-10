# DeribitFixClient

## Overview

`DeribitFixClient` is the main client for interacting with Deribit's FIX API. It provides a high-level interface for all trading operations, session management, and connection handling.

## Purpose

- **Main client**: Single entry point for all operations
- **Connection management**: Automatic TCP connection handling and reconnections
- **Session management**: Authentication, heartbeat, and session state management
- **Operation orchestration**: Coordination between different business handlers
- **Error handling**: Centralized error management and retries

## Public Interface

### Struct Definition

```rust
pub struct DeribitFixClient {
    config: Config,
    connection: ConnectionManager,
    session: SessionManager,
    order_handler: OrderHandler,
    market_data_handler: MarketDataHandler,
}
```

### Key Methods

#### Connection Management

```rust
impl DeribitFixClient {
    /// Creates a new client instance
    pub fn new(config: Config) -> Result<Self, DeribitFixError>
    
    /// Establishes connection to Deribit
    pub async fn connect(&mut self) -> Result<(), DeribitFixError>
    
    /// Closes the connection
    pub async fn disconnect(&mut self) -> Result<(), DeribitFixError>
    
    /// Checks if connected
    pub fn is_connected(&self) -> bool
}
```

#### Session Management

```rust
impl DeribitFixClient {
    /// Authenticates with Deribit
    pub async fn logon(&mut self) -> Result<(), DeribitFixError>
    
    /// Logs out from Deribit
    pub async fn logout(&mut self) -> Result<(), DeribitFixError>
    
    /// Sends heartbeat
    pub async fn heartbeat(&mut self) -> Result<(), DeribitFixError>
    
    /// Gets session info
    pub fn session_info(&self) -> Option<&SessionInfo>
}
```

#### Trading Operations

```rust
impl DeribitFixClient {
    /// Places a new order
    pub async fn place_order(&mut self, order: Order) -> Result<String, DeribitFixError>
    
    /// Cancels an existing order
    pub async fn cancel_order(&mut self, order_id: &str) -> Result<(), DeribitFixError>
    
    /// Replaces an existing order
    pub async fn replace_order(&mut self, order_id: &str, new_order: Order) -> Result<String, DeribitFixError>
    
    /// Gets order status
    pub async fn get_order_status(&mut self, order_id: &str) -> Result<OrderStatus, DeribitFixError>
}
```

#### Market Data

```rust
impl DeribitFixClient {
    /// Subscribes to market data
    pub async fn subscribe_market_data(&mut self, instrument: &str) -> Result<(), DeribitFixError>
    
    /// Unsubscribes from market data
    pub async fn unsubscribe_market_data(&mut self, instrument: &str) -> Result<(), DeribitFixError>
    
    /// Gets current market data
    pub async fn get_market_data(&mut self, instrument: &str) -> Result<MarketData, DeribitFixError>
}
```

#### Position Management

```rust
impl DeribitFixClient {
    /// Gets current positions
    pub async fn get_positions(&mut self) -> Result<Vec<Position>, DeribitFixError>
    
    /// Gets position for specific instrument
    pub async fn get_position(&mut self, instrument: &str) -> Result<Position, DeribitFixError>
    
    /// Closes a position
    pub async fn close_position(&mut self, instrument: &str) -> Result<(), DeribitFixError>
}
```

#### Error Handling and Retry

```rust
impl DeribitFixClient {
    /// Executes operation with automatic retry
    pub async fn execute_with_retry<F, T>(
        &mut self,
        operation: F,
        max_retries: usize,
        backoff_strategy: BackoffStrategy
    ) -> Result<T, DeribitFixError>
    where
        F: FnMut() -> Future<Output = Result<T, DeribitFixError>> + Send + Sync,
        F::Future: Send,
        T: Send + Sync
}
```

## Usage Examples

### Basic Setup and Connection

```rust
use deribit_fix::{DeribitFixClient, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::default()
        .with_api_key("your_api_key")
        .with_api_secret("your_api_secret")
        .with_testnet(true);
    
    let mut client = DeribitFixClient::new(config)?;
    
    // Connect to Deribit
    client.connect().await?;
    
    // Authenticate
    client.logon().await?;
    
    Ok(())
}
```

### Advanced Trading Operations

```rust
use deribit_fix::{DeribitFixClient, Order, OrderSide, OrderType, TimeInForce};

async fn place_limit_order(
    client: &mut DeribitFixClient,
    instrument: &str,
    side: OrderSide,
    quantity: f64,
    price: f64
) -> Result<String, DeribitFixError> {
    let order = Order {
        instrument: instrument.to_string(),
        side,
        order_type: OrderType::Limit,
        quantity,
        price: Some(price),
        time_in_force: TimeInForce::GoodTillCancel,
        ..Default::default()
    };
    
    client.place_order(order).await
}

async fn manage_order_lifecycle(
    client: &mut DeribitFixClient,
    instrument: &str
) -> Result<(), DeribitFixError> {
    // Place order
    let order_id = place_limit_order(
        client, instrument, OrderSide::Buy, 1.0, 50000.0
    ).await?;
    
    // Check status
    let status = client.get_order_status(&order_id).await?;
    println!("Order status: {:?}", status);
    
    // Cancel if needed
    if status == OrderStatus::New {
        client.cancel_order(&order_id).await?;
    }
    
    Ok(())
}
```

### Market Data Subscription

```rust
use deribit_fix::DeribitFixClient;

async fn subscribe_and_monitor(
    client: &mut DeribitFixClient,
    instrument: &str
) -> Result<(), DeribitFixError> {
    // Subscribe to market data
    client.subscribe_market_data(instrument).await?;
    
    // Get current data
    let market_data = client.get_market_data(instrument).await?;
    println!("Current price: {}", market_data.last_price);
    
    // Unsubscribe when done
    client.unsubscribe_market_data(instrument).await?;
    
    Ok(())
}
```

### Error Handling with Retry

```rust
use deribit_fix::{DeribitFixClient, BackoffStrategy};

async fn robust_order_placement(
    client: &mut DeribitFixClient,
    order: Order
) -> Result<String, DeribitFixError> {
    let backoff = BackoffStrategy::Exponential {
        base: Duration::from_secs(1),
        max: Duration::from_secs(30)
    };
    
    client.execute_with_retry(
        || async { client.place_order(order.clone()) },
        3,
        backoff
    ).await
}
```

## Performance Characteristics

- **Async-first**: Todas las operaciones son asíncronas para máxima eficiencia
- **Connection pooling**: Reutilización de conexiones para operaciones de alta frecuencia
- **Batch operations**: Soporte para operaciones en lote cuando sea posible
- **Memory efficient**: Gestión eficiente de memoria para operaciones de larga duración

## Thread Safety

- **Send + Sync**: El cliente implementa `Send + Sync` para uso en múltiples hilos
- **Arc<Mutex<>>**: Para uso compartido entre hilos, envuelve en `Arc<Mutex<>>`
- **Clone**: El cliente puede ser clonado para uso en diferentes tareas

```rust
use std::sync::{Arc, Mutex};
use tokio::task;

let client = Arc::new(Mutex::new(client));

// Spawn multiple tasks
let client1 = Arc::clone(&client);
let client2 = Arc::clone(&client);

let task1 = task::spawn(async move {
    let mut client = client1.lock().unwrap();
    client.place_order(order1).await
});

let task2 = task::spawn(async move {
    let mut client = client2.lock().unwrap();
    client.place_order(order2).await
});
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    
    #[tokio::test]
    async fn test_client_creation() {
        let config = Config::default();
        let client = DeribitFixClient::new(config);
        assert!(client.is_ok());
    }
    
    #[tokio::test]
    async fn test_connection_flow() {
        let config = Config::default();
        let mut client = DeribitFixClient::new(config).unwrap();
        
        // Test connection
        assert!(!client.is_connected());
        // Note: Would need mock connection for actual test
    }
}
```

### Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_full_trading_flow() {
        let config = Config::default()
            .with_testnet(true);
        let mut client = DeribitFixClient::new(config).unwrap();
        
        // Test full flow with testnet
        // This would require actual testnet credentials
    }
}
```

## Module Dependencies

- **config**: Para configuración del cliente
- **connection**: Para gestión de conexiones TCP
- **session**: Para gestión de sesiones FIX
- **message**: Para construcción y parsing de mensajes FIX
- **types**: Para tipos de datos de trading
- **error**: Para manejo de errores

## Related Types

- **Config**: Configuración del cliente
- **Order**: Órdenes de trading
- **ExecutionReport**: Reportes de ejecución
- **MarketData**: Datos de mercado
- **Position**: Posiciones abiertas
- **SessionInfo**: Información de sesión
- **DeribitFixError**: Errores del cliente
