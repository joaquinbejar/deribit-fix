# ExecutionReport

## Overview

`ExecutionReport` represents the result of order execution in the Deribit FIX system. It provides detailed information about order status changes, fills, and execution details.

## Purpose

- **Order status tracking**: Reports changes in order status
- **Fill information**: Details about executed quantities and prices
- **Execution confirmation**: Confirms order actions and results
- **Trade reporting**: Links orders to actual trades

## Public Interface

### Struct Definition

```rust
pub struct ExecutionReport {
    pub order_id: String,
    pub client_order_id: Option<String>,
    pub symbol: String,
    pub side: OrderSide,
    pub ord_type: OrderType,
    pub order_qty: f64,
    pub price: Option<f64>,
    pub exec_type: ExecType,
    pub ord_status: OrderStatus,
    pub cum_qty: f64,
    pub avg_price: Option<f64>,
    pub last_qty: Option<f64>,
    pub last_price: Option<f64>,
    pub leaves_qty: f64,
    pub transact_time: DateTime<Utc>,
    pub exec_id: String,
    pub exec_ref_id: Option<String>,
    pub text: Option<String>,
    pub reject_reason: Option<String>,
}
```

### Key Methods

```rust
impl ExecutionReport {
    /// Creates a new execution report
    pub fn new(order_id: String, symbol: String, side: OrderSide, ord_type: OrderType) -> Self
    
    /// Checks if the order is fully filled
    pub fn is_filled(&self) -> bool
    
    /// Gets remaining quantity
    pub fn remaining_quantity(&self) -> f64
    
    /// Updates with new fill information
    pub fn update_fill(&mut self, qty: f64, price: f64, exec_id: String)
    
    /// Creates a copy with updated status
    pub fn with_status(mut self, status: OrderStatus) -> Self
    pub fn with_exec_type(mut self, exec_type: ExecType) -> Self
}
```

## Usage Examples

### Creating Execution Reports

```rust
use deribit_fix::{ExecutionReport, OrderSide, OrderType, ExecType, OrderStatus};

// Create new order confirmation
let report = ExecutionReport::new(
    "ORDER_123".to_string(),
    "BTC-PERPETUAL".to_string(),
    OrderSide::Buy,
    OrderType::Limit
)
.with_status(OrderStatus::New)
.with_exec_type(ExecType::New);

// Update with fill information
let mut report = ExecutionReport::new(
    "ORDER_123".to_string(),
    "BTC-PERPETUAL".to_string(),
    OrderSide::Buy,
    OrderType::Limit
);
report.update_fill(0.5, 50000.0, "EXEC_001".to_string());
```

### Order Status Monitoring

```rust
fn monitor_execution(report: &ExecutionReport) {
    match report.exec_type {
        ExecType::New => println!("Order {} is new", report.order_id),
        ExecType::PartialFill => {
            println!("Order {} partially filled: {}/{}", 
                report.order_id, report.cum_qty, report.order_qty);
        },
        ExecType::Fill => println!("Order {} fully filled", report.order_id),
        ExecType::Cancelled => println!("Order {} cancelled", report.order_id),
        ExecType::Rejected => {
            if let Some(reason) = &report.reject_reason {
                println!("Order {} rejected: {}", report.order_id, reason);
            }
        },
        _ => println!("Order {} status: {:?}", report.order_id, report.exec_type),
    }
}
```

## Module Dependencies

- **types**: For `OrderSide`, `OrderType`, `ExecType`, `OrderStatus`
- **chrono**: For `DateTime<Utc>` timestamp handling

## Related Types

- **Order**: The original order being reported
- **OrderSide**: Buy/Sell side
- **OrderType**: Type of order
- **ExecType**: Execution type
- **OrderStatus**: Current order status
