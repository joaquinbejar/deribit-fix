# OrderHandler Trait

## Overview

The `OrderHandler` trait defines the interface for managing trading orders within the Deribit FIX client. It provides methods for creating, modifying, canceling, and monitoring orders, as well as handling order-related FIX messages and execution reports.

## Purpose

- **Order Management**: Create, modify, and cancel trading orders
- **Order Monitoring**: Track order status and execution
- **Execution Handling**: Process execution reports and fills
- **Order Validation**: Validate order parameters before submission
- **Risk Management**: Apply risk checks and position limits
- **Order Lifecycle**: Manage the complete order lifecycle from creation to execution

## Public Interface

### Trait Definition

```rust
#[async_trait]
pub trait OrderHandler: Send + Sync {
    async fn place_order(&mut self, order: Order) -> Result<String, OrderHandlerError>;
    async fn modify_order(&mut self, order_id: &str, modifications: OrderModification) -> Result<(), OrderHandlerError>;
    async fn cancel_order(&mut self, order_id: &str, reason: Option<&str>) -> Result<(), OrderHandlerError>;
    async fn cancel_all_orders(&mut self, instrument: Option<&str>) -> Result<u32, OrderHandlerError>;
    async fn get_order_status(&mut self, order_id: &str) -> Result<OrderStatus, OrderHandlerError>;
    async fn get_open_orders(&mut self, instrument: Option<&str>) -> Result<Vec<Order>, OrderHandlerError>;
    async fn get_order_history(&mut self, filter: OrderHistoryFilter) -> Result<Vec<Order>, OrderHandlerError>;
    async fn handle_execution_report(&mut self, report: ExecutionReport) -> Result<(), OrderHandlerError>;
    async fn handle_order_cancel_reject(&mut self, reject: OrderCancelReject) -> Result<(), OrderHandlerError>;
    async fn handle_order_reject(&mut self, reject: OrderReject) -> Result<(), OrderHandlerError>;
    async fn validate_order(&mut self, order: &Order) -> Result<(), Vec<OrderValidationError>>;
    async fn calculate_margin_requirement(&mut self, order: &Order) -> Result<f64, OrderHandlerError>;
    fn get_order_stats(&self) -> OrderHandlerStats;
    async fn resend_rejected_order(&mut self, order: &Order, reason: &str) -> Result<String, OrderHandlerError>;
    async fn get_order_by_cl_ord_id(&mut self, cl_ord_id: &str) -> Result<Option<Order>, OrderHandlerError>;
    async fn get_orders_by_instrument(&mut self, instrument: &str) -> Result<Vec<Order>, OrderHandlerError>;
}
```

### Associated Types

```rust
pub enum OrderHandlerError {
    OrderNotFound(String),
    InvalidOrderParameters(Vec<OrderValidationError>),
    InsufficientMargin(f64),
    RiskLimitExceeded(String),
    InstrumentNotTradeable(String),
    OrderAlreadyExists(String),
    OrderInInvalidState(String, OrderStatus),
    ExchangeError(String),
    NetworkError(String),
    ValidationError(Vec<OrderValidationError>),
}

pub enum OrderValidationError {
    InvalidSymbol(String),
    InvalidQuantity(f64),
    InvalidPrice(f64),
    InvalidOrderType(OrderType),
    InvalidTimeInForce(TimeInForce),
    InvalidSide(OrderSide),
    QuantityBelowMinimum(f64, f64),
    PriceBelowMinimum(f64, f64),
    PriceAboveMaximum(f64, f64),
    InvalidExpiration(DateTime<Utc>),
    InsufficientBalance(f64, f64),
    RiskLimitViolation(String),
}

pub struct OrderModification {
    pub new_price: Option<f64>,
    pub new_quantity: Option<f64>,
    pub new_time_in_force: Option<TimeInForce>,
    pub new_expiration: Option<DateTime<Utc>>,
    pub new_stop_price: Option<f64>,
    pub new_trailing_stop: Option<TrailingStop>,
}

pub struct OrderHistoryFilter {
    pub instrument: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub order_status: Option<OrderStatus>,
    pub order_side: Option<OrderSide>,
    pub limit: Option<u32>,
}

pub struct OrderHandlerStats {
    pub total_orders_placed: u64,
    pub total_orders_filled: u64,
    pub total_orders_cancelled: u64,
    pub total_orders_rejected: u64,
    pub total_volume_traded: f64,
    pub total_notional_value: f64,
    pub average_fill_time: Duration,
    pub success_rate: f64,
    pub last_order_time: Option<DateTime<Utc>>,
}

pub struct TrailingStop {
    pub activation_price: f64,
    pub trailing_amount: f64,
    pub trailing_type: TrailingStopType,
}

pub enum TrailingStopType {
    Fixed,
    Percentage,
}
```

## Usage Examples

### Basic Order Placement

```rust
use deribit_fix::traits::OrderHandler;
use deribit_fix::types::{Order, OrderSide, OrderType, TimeInForce};

struct MyOrderHandler;

#[async_trait]
impl OrderHandler for MyOrderHandler {
    async fn place_order(&mut self, order: Order) -> Result<String, OrderHandlerError> {
        // Validate the order first
        self.validate_order(&order).await?;
        
        // Check margin requirements
        let margin_required = self.calculate_margin_requirement(&order).await?;
        if !self.has_sufficient_margin(margin_required).await? {
            return Err(OrderHandlerError::InsufficientMargin(margin_required));
        }
        
        // Submit order to exchange
        let order_id = self.submit_order_to_exchange(&order).await?;
        
        // Store order locally
        self.store_order(order_id.clone(), order).await?;
        
        Ok(order_id)
    }
    
    // ... implement other required methods
}
```

### Order Modification

```rust
impl MyOrderHandler {
    async fn modify_existing_order(&mut self, order_id: &str, mods: OrderModification) -> Result<(), OrderHandlerError> {
        // Get current order
        let current_order = self.get_order_status(order_id).await?;
        
        // Check if order can be modified
        if !current_order.status.is_modifiable() {
            return Err(OrderHandlerError::OrderInInvalidState(
                order_id.to_string(),
                current_order.status
            ));
        }
        
        // Create modified order
        let modified_order = self.apply_modifications(&current_order, &mods)?;
        
        // Validate modified order
        self.validate_order(&modified_order).await?;
        
        // Submit modification to exchange
        self.submit_order_modification(order_id, &mods).await?;
        
        // Update local order
        self.update_stored_order(order_id, modified_order).await?;
        
        Ok(())
    }
}
```

### Order Cancellation

```rust
impl MyOrderHandler {
    async fn cancel_order_with_retry(&mut self, order_id: &str) -> Result<(), OrderHandlerError> {
        let mut attempts = 0;
        let max_attempts = 3;
        
        while attempts < max_attempts {
            match self.cancel_order(order_id, Some("User requested cancellation")).await {
                Ok(()) => return Ok(()),
                Err(OrderHandlerError::OrderNotFound(_)) => {
                    // Order already doesn't exist
                    return Ok(());
                }
                Err(OrderHandlerError::OrderInInvalidState(_, status)) if status.is_cancellable() => {
                    // Order state changed, try again
                    attempts += 1;
                    tokio::time::sleep(Duration::from_millis(100 * attempts as u64)).await;
                    continue;
                }
                Err(e) => return Err(e),
            }
        }
        
        Err(OrderHandlerError::ExchangeError("Failed to cancel order after retries".to_string()))
    }
}
```

### Execution Report Handling

```rust
impl MyOrderHandler {
    async fn process_execution_report(&mut self, report: ExecutionReport) -> Result<(), OrderHandlerError> {
        let order_id = &report.order_id;
        
        // Update order status
        self.update_order_status(order_id, &report.exec_type, &report.ord_status).await?;
        
        // Handle fills
        if report.exec_type == ExecType::Fill || report.exec_type == ExecType::PartialFill {
            self.process_fill(order_id, &report).await?;
            
            // Update position if needed
            if let Some(position_update) = self.calculate_position_update(&report).await? {
                self.update_position(position_update).await?;
            }
        }
        
        // Handle rejections
        if report.exec_type == ExecType::Rejected {
            self.handle_order_rejection(order_id, &report).await?;
        }
        
        // Notify subscribers
        self.notify_order_update(order_id, &report).await?;
        
        Ok(())
    }
}
```

### Order Validation

```rust
impl MyOrderHandler {
    async fn comprehensive_order_validation(&self, order: &Order) -> Result<(), Vec<OrderValidationError>> {
        let mut errors = Vec::new();
        
        // Basic field validation
        if order.symbol.is_empty() {
            errors.push(OrderValidationError::InvalidSymbol("Symbol cannot be empty".to_string()));
        }
        
        if order.quantity <= 0.0 {
            errors.push(OrderValidationError::InvalidQuantity(order.quantity));
        }
        
        if order.price <= 0.0 {
            errors.push(OrderValidationError::InvalidPrice(order.price));
        }
        
        // Instrument-specific validation
        if let Some(instrument) = self.get_instrument(&order.symbol).await? {
            if !instrument.is_tradeable() {
                errors.push(OrderValidationError::InstrumentNotTradeable(order.symbol.clone()));
            }
            
            // Check quantity limits
            if !instrument.is_valid_quantity(order.quantity) {
                errors.push(OrderValidationError::QuantityBelowMinimum(
                    order.quantity,
                    instrument.min_quantity
                ));
            }
            
            // Check price limits
            if let Some(next_price) = instrument.next_valid_price(order.price, order.side) {
                if (next_price - order.price).abs() > f64::EPSILON {
                    errors.push(OrderValidationError::InvalidPrice(next_price));
                }
            }
        }
        
        // Risk validation
        if let Err(risk_error) = self.validate_risk_limits(order).await {
            errors.push(OrderValidationError::RiskLimitViolation(risk_error));
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
```

### Order Monitoring

```rust
impl MyOrderHandler {
    async fn monitor_order_lifecycle(&mut self, order_id: &str) -> Result<OrderStatus, OrderHandlerError> {
        let mut last_status = None;
        let timeout = Duration::from_secs(300); // 5 minutes
        let start_time = Instant::now();
        
        while start_time.elapsed() < timeout {
            let current_status = self.get_order_status(order_id).await?;
            
            if current_status.status != last_status {
                log::info!("Order {} status changed: {:?}", order_id, current_status.status);
                last_status = Some(current_status.status.clone());
                
                // Check if order is in final state
                if current_status.status.is_final() {
                    return Ok(current_status);
                }
            }
            
            // Wait before next check
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        
        Err(OrderHandlerError::ExchangeError("Order monitoring timeout".to_string()))
    }
}
```

## Module Dependencies

- **`types`**: Uses `Order`, `ExecutionReport`, `OrderStatus`, and related types
- **`error`**: Uses `OrderHandlerError` and validation error types
- **`message`**: May interact with FIX message handling
- **`session`**: May use session management for order submission

## Related Types

- **`Order`**: The core order structure being managed
- **`ExecutionReport`**: Reports of order execution from the exchange
- **`OrderStatus`**: Current status of an order
- **`OrderHandlerError`**: Error types for order operations
- **`OrderModification`**: Parameters for modifying existing orders

## Testing

### Unit Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_order_validation() {
        let handler = MyOrderHandler::new();
        let order = create_test_order();
        
        let result = handler.validate_order(&order).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_order_placement() {
        let mut handler = MyOrderHandler::new();
        let order = create_test_order();
        
        let order_id = handler.place_order(order).await.unwrap();
        assert!(!order_id.is_empty());
    }
}
```

### Integration Testing

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_order_lifecycle() {
        let mut handler = MyOrderHandler::new();
        let order = create_test_order();
        
        // Place order
        let order_id = handler.place_order(order).await.unwrap();
        
        // Check status
        let status = handler.get_order_status(&order_id).await.unwrap();
        assert_eq!(status.status, OrderStatus::New);
        
        // Cancel order
        handler.cancel_order(&order_id, None).await.unwrap();
        
        // Verify cancellation
        let final_status = handler.get_order_status(&order_id).await.unwrap();
        assert!(final_status.status.is_cancelled());
    }
}
```

## Performance Considerations

- **Async Operations**: All operations are async to avoid blocking
- **Batch Operations**: Consider batching multiple order operations
- **Caching**: Cache frequently accessed order data
- **Connection Pooling**: Use connection pools for exchange communication
- **Rate Limiting**: Respect exchange rate limits

## Security Considerations

- **Order Validation**: Validate all order parameters before submission
- **Risk Limits**: Enforce risk management rules
- **Access Control**: Restrict order operations to authorized users
- **Audit Logging**: Log all order operations and changes
- **Input Sanitization**: Sanitize all order input data
