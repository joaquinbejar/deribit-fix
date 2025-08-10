# Order

## Overview

`Order` represents a trading order in the Deribit FIX system. It encapsulates all the necessary information to place, modify, or cancel orders on the exchange, including order type, side, quantity, price, and various order parameters.

## Purpose

- **Order representation**: Core data structure for all trading operations
- **Order placement**: Used when submitting new orders to the exchange
- **Order modification**: Used when updating existing orders
- **Order validation**: Ensures order parameters meet exchange requirements
- **Serialization**: Can be converted to/from FIX messages and other formats

## Public Interface

### Struct Definition

```rust
pub struct Order {
    pub order_id: Option<String>,
    pub client_order_id: Option<String>,
    pub instrument: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub quantity: f64,
    pub price: Option<f64>,
    pub stop_price: Option<f64>,
    pub time_in_force: TimeInForce,
    pub expire_time: Option<DateTime<Utc>>,
    pub reduce_only: bool,
    pub post_only: bool,
    pub iceberg_qty: Option<f64>,
    pub working_indicator: bool,
    pub order_status: OrderStatus,
    pub cum_qty: f64,
    pub avg_price: Option<f64>,
    pub last_qty: Option<f64>,
    pub last_price: Option<f64>,
    pub created_time: DateTime<Utc>,
    pub updated_time: DateTime<Utc>,
}
```

### Key Fields

#### Required Fields

```rust
impl Order {
    /// Instrument identifier (e.g., "BTC-PERPETUAL")
    pub instrument: String,
    
    /// Order side (Buy or Sell)
    pub side: OrderSide,
    
    /// Type of order (Market, Limit, Stop, etc.)
    pub order_type: OrderType,
    
    /// Order quantity
    pub quantity: f64,
    
    /// Time in force policy
    pub time_in_force: TimeInForce,
}
```

#### Optional Fields

```rust
impl Order {
    /// Order price (required for Limit orders)
    pub price: Option<f64>,
    
    /// Stop price for Stop orders
    pub stop_price: Option<f64>,
    
    /// Expiration time for GTD orders
    pub expire_time: Option<DateTime<Utc>>,
    
    /// Reduce-only flag
    pub reduce_only: bool,
    
    /// Post-only flag
    pub post_only: bool,
    
    /// Iceberg quantity for hidden orders
    pub iceberg_qty: Option<f64>,
}
```

#### Status Fields

```rust
impl Order {
    /// Current order status
    pub order_status: OrderStatus,
    
    /// Cumulative filled quantity
    pub cum_qty: f64,
    
    /// Average fill price
    pub avg_price: Option<f64>,
    
    /// Last fill quantity
    pub last_qty: Option<f64>,
    
    /// Last fill price
    pub last_price: Option<f64>,
}
```

### Methods

```rust
impl Order {
    /// Creates a new order with default values
    pub fn new(instrument: String, side: OrderSide, order_type: OrderType, quantity: f64) -> Self
    
    /// Validates the order parameters
    pub fn validate(&self) -> Result<(), Vec<OrderValidationError>>
    
    /// Checks if the order is valid for submission
    pub fn is_valid_for_submission(&self) -> bool
    
    /// Calculates the remaining quantity
    pub fn remaining_quantity(&self) -> f64
    
    /// Checks if the order is fully filled
    pub fn is_filled(&self) -> bool
    
    /// Checks if the order can be cancelled
    pub fn can_cancel(&self) -> bool
    
    /// Creates a copy with updated fields
    pub fn with_price(mut self, price: f64) -> Self
    pub fn with_stop_price(mut self, stop_price: f64) -> Self
    pub fn with_time_in_force(mut self, time_in_force: TimeInForce) -> Self
    pub fn with_expire_time(mut self, expire_time: DateTime<Utc>) -> Self
    pub fn with_reduce_only(mut self, reduce_only: bool) -> Self
    pub fn with_post_only(mut self, post_only: bool) -> Self
}
```

## Usage Examples

### Creating Basic Orders

```rust
use deribit_fix::{Order, OrderSide, OrderType, TimeInForce};
use chrono::Utc;

// Create a simple limit order
let limit_order = Order::new(
    "BTC-PERPETUAL".to_string(),
    OrderSide::Buy,
    OrderType::Limit,
    1.0
)
.with_price(50000.0)
.with_time_in_force(TimeInForce::GoodTillCancel);

// Create a market order
let market_order = Order::new(
    "ETH-PERPETUAL".to_string(),
    OrderSide::Sell,
    OrderType::Market,
    10.0
)
.with_time_in_force(TimeInForce::ImmediateOrCancel);

// Create a stop order
let stop_order = Order::new(
    "BTC-PERPETUAL".to_string(),
    OrderSide::Sell,
    OrderType::Stop,
    0.5
)
.with_stop_price(45000.0)
.with_time_in_force(TimeInForce::GoodTillCancel);
```

### Advanced Order Types

```rust
use deribit_fix::{Order, OrderSide, OrderType, TimeInForce};
use chrono::{Utc, Duration};

// Create a GTD (Good Till Date) order
let gtd_order = Order::new(
    "BTC-PERPETUAL".to_string(),
    OrderSide::Buy,
    OrderType::Limit,
    2.0
)
.with_price(48000.0)
.with_time_in_force(TimeInForce::GoodTillDate)
.with_expire_time(Utc::now() + Duration::hours(24));

// Create a reduce-only order
let reduce_order = Order::new(
    "ETH-PERPETUAL".to_string(),
    OrderSide::Sell,
    OrderType::Limit,
    5.0
)
.with_price(3000.0)
.with_reduce_only(true)
.with_time_in_force(TimeInForce::GoodTillCancel);

// Create a post-only order
let post_order = Order::new(
    "BTC-PERPETUAL".to_string(),
    OrderSide::Buy,
    OrderType::Limit,
    0.1
)
.with_price(50000.0)
.with_post_only(true)
.with_time_in_force(TimeInForce::GoodTillCancel);
```

### Order Validation

```rust
use deribit_fix::{Order, OrderSide, OrderType, TimeInForce};

fn create_validated_order(
    instrument: &str,
    side: OrderSide,
    order_type: OrderType,
    quantity: f64,
    price: Option<f64>
) -> Result<Order, Vec<OrderValidationError>> {
    let mut order = Order::new(
        instrument.to_string(),
        side,
        order_type,
        quantity
    );
    
    // Add price for limit orders
    if order_type == OrderType::Limit {
        if let Some(p) = price {
            order = order.with_price(p);
        } else {
            return Err(vec![OrderValidationError::PriceRequiredForLimitOrder]);
        }
    }
    
    // Validate the order
    order.validate()?;
    
    Ok(order)
}

// Usage
let order = create_validated_order(
    "BTC-PERPETUAL",
    OrderSide::Buy,
    OrderType::Limit,
    1.0,
    Some(50000.0)
)?;
```

### Order Lifecycle Management

```rust
use deribit_fix::{Order, OrderStatus};

fn monitor_order_status(order: &Order) {
    match order.order_status {
        OrderStatus::New => println!("Order {} is new", order.client_order_id.as_ref().unwrap()),
        OrderStatus::PartiallyFilled => {
            println!("Order {} is {}% filled", 
                order.client_order_id.as_ref().unwrap(),
                (order.cum_qty / order.quantity) * 100.0
            );
        },
        OrderStatus::Filled => println!("Order {} is fully filled", order.client_order_id.as_ref().unwrap()),
        OrderStatus::Cancelled => println!("Order {} was cancelled", order.client_order_id.as_ref().unwrap()),
        OrderStatus::Rejected => println!("Order {} was rejected", order.client_order_id.as_ref().unwrap()),
        _ => println!("Order {} has status: {:?}", order.client_order_id.as_ref().unwrap(), order.order_status),
    }
}

fn can_modify_order(order: &Order) -> bool {
    matches!(order.order_status, OrderStatus::New | OrderStatus::PartiallyFilled)
}

fn should_cancel_order(order: &Order) -> bool {
    order.order_status == OrderStatus::New && 
    order.created_time < Utc::now() - Duration::minutes(5)
}
```

### Order Serialization

```rust
use deribit_fix::Order;
use serde_json;

// Serialize to JSON
let order = Order::new("BTC-PERPETUAL".to_string(), OrderSide::Buy, OrderType::Limit, 1.0)
    .with_price(50000.0);
let json = serde_json::to_string_pretty(&order)?;
println!("Order JSON: {}", json);

// Deserialize from JSON
let order: Order = serde_json::from_str(&json)?;

// Convert to FIX message
let fix_message = FixMessage::from(order);
let fix_string = fix_message.to_fix_string()?;
println!("FIX message: {}", fix_string);
```

## Order Types and Constraints

### Market Orders

```rust
impl Order {
    pub fn new_market_order(
        instrument: String,
        side: OrderSide,
        quantity: f64
    ) -> Self {
        Self::new(instrument, side, OrderType::Market, quantity)
            .with_time_in_force(TimeInForce::ImmediateOrCancel)
    }
}

// Market orders cannot have price
let market_order = Order::new_market_order(
    "BTC-PERPETUAL".to_string(),
    OrderSide::Buy,
    1.0
);
```

### Limit Orders

```rust
impl Order {
    pub fn new_limit_order(
        instrument: String,
        side: OrderSide,
        quantity: f64,
        price: f64
    ) -> Self {
        Self::new(instrument, side, OrderType::Limit, quantity)
            .with_price(price)
            .with_time_in_force(TimeInForce::GoodTillCancel)
    }
}

// Limit orders require price
let limit_order = Order::new_limit_order(
    "BTC-PERPETUAL".to_string(),
    OrderSide::Sell,
    0.5,
    55000.0
);
```

### Stop Orders

```rust
impl Order {
    pub fn new_stop_order(
        instrument: String,
        side: OrderSide,
        quantity: f64,
        stop_price: f64
    ) -> Self {
        Self::new(instrument, side, OrderType::Stop, quantity)
            .with_stop_price(stop_price)
            .with_time_in_force(TimeInForce::GoodTillCancel)
    }
}

// Stop orders require stop price
let stop_order = Order::new_stop_order(
    "BTC-PERPETUAL".to_string(),
    OrderSide::Sell,
    1.0,
    45000.0
);
```

## Validation Rules

### Basic Validation

```rust
impl Order {
    pub fn validate(&self) -> Result<(), Vec<OrderValidationError>> {
        let mut errors = Vec::new();
        
        // Check required fields
        if self.instrument.is_empty() {
            errors.push(OrderValidationError::InstrumentRequired);
        }
        
        if self.quantity <= 0.0 {
            errors.push(OrderValidationError::InvalidQuantity);
        }
        
        // Check order type specific rules
        match self.order_type {
            OrderType::Limit => {
                if self.price.is_none() {
                    errors.push(OrderValidationError::PriceRequiredForLimitOrder);
                } else if let Some(price) = self.price {
                    if price <= 0.0 {
                        errors.push(OrderValidationError::InvalidPrice);
                    }
                }
            },
            OrderType::Stop => {
                if self.stop_price.is_none() {
                    errors.push(OrderValidationError::StopPriceRequired);
                }
            },
            OrderType::Market => {
                // Market orders cannot have price
                if self.price.is_some() {
                    errors.push(OrderValidationError::PriceNotAllowedForMarketOrder);
                }
            },
            _ => {}
        }
        
        // Check time in force rules
        if self.time_in_force == TimeInForce::GoodTillDate && self.expire_time.is_none() {
            errors.push(OrderValidationError::ExpireTimeRequiredForGTD);
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
```

### Business Rule Validation

```rust
impl Order {
    pub fn validate_business_rules(&self, config: &TradingConfig) -> Result<(), Vec<OrderValidationError>> {
        let mut errors = Vec::new();
        
        // Check quantity limits
        if self.quantity < config.min_order_size {
            errors.push(OrderValidationError::QuantityBelowMinimum(config.min_order_size));
        }
        
        if self.quantity > config.max_order_size {
            errors.push(OrderValidationError::QuantityAboveMaximum(config.max_order_size));
        }
        
        // Check price tick size
        if let Some(price) = self.price {
            let tick_size = config.price_tick_size;
            if (price / tick_size).fract() != 0.0 {
                errors.push(OrderValidationError::InvalidPriceTickSize(tick_size));
            }
        }
        
        // Check quantity tick size
        let tick_size = config.quantity_tick_size;
        if (self.quantity / tick_size).fract() != 0.0 {
            errors.push(OrderValidationError::InvalidQuantityTickSize(tick_size));
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_order_creation() {
        let order = Order::new(
            "BTC-PERPETUAL".to_string(),
            OrderSide::Buy,
            OrderType::Limit,
            1.0
        );
        
        assert_eq!(order.instrument, "BTC-PERPETUAL");
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.order_type, OrderType::Limit);
        assert_eq!(order.quantity, 1.0);
    }
    
    #[test]
    fn test_limit_order_validation() {
        let order = Order::new(
            "BTC-PERPETUAL".to_string(),
            OrderSide::Buy,
            OrderType::Limit,
            1.0
        );
        
        // Should fail without price
        assert!(order.validate().is_err());
        
        // Should pass with price
        let order = order.with_price(50000.0);
        assert!(order.validate().is_ok());
    }
    
    #[test]
    fn test_market_order_validation() {
        let order = Order::new(
            "BTC-PERPETUAL".to_string(),
            OrderSide::Buy,
            OrderType::Market,
            1.0
        );
        
        // Should pass without price
        assert!(order.validate().is_ok());
        
        // Should fail with price
        let order = order.with_price(50000.0);
        assert!(order.validate().is_err());
    }
    
    #[test]
    fn test_order_status_checks() {
        let mut order = Order::new(
            "BTC-PERPETUAL".to_string(),
            OrderSide::Buy,
            OrderType::Limit,
            1.0
        ).with_price(50000.0);
        
        // New order
        assert!(!order.is_filled());
        assert_eq!(order.remaining_quantity(), 1.0);
        assert!(order.can_cancel());
        
        // Partially filled
        order.cum_qty = 0.5;
        assert!(!order.is_filled());
        assert_eq!(order.remaining_quantity(), 0.5);
        assert!(order.can_cancel());
        
        // Fully filled
        order.cum_qty = 1.0;
        assert!(order.is_filled());
        assert_eq!(order.remaining_quantity(), 0.0);
        assert!(!order.can_cancel());
    }
}
```

### Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_order_serialization() {
        let order = Order::new(
            "BTC-PERPETUAL".to_string(),
            OrderSide::Buy,
            OrderType::Limit,
            1.0
        ).with_price(50000.0);
        
        // Test JSON serialization
        let json = serde_json::to_string(&order).unwrap();
        let deserialized: Order = serde_json::from_str(&json).unwrap();
        
        assert_eq!(order.instrument, deserialized.instrument);
        assert_eq!(order.side, deserialized.side);
        assert_eq!(order.order_type, deserialized.order_type);
        assert_eq!(order.quantity, deserialized.quantity);
        assert_eq!(order.price, deserialized.price);
    }
    
    #[test]
    fn test_order_fix_conversion() {
        let order = Order::new(
            "BTC-PERPETUAL".to_string(),
            OrderSide::Buy,
            OrderType::Limit,
            1.0
        ).with_price(50000.0);
        
        // Convert to FIX message
        let fix_message = FixMessage::from(order.clone());
        
        // Convert back to Order
        let converted_order = Order::try_from(fix_message).unwrap();
        
        assert_eq!(order.instrument, converted_order.instrument);
        assert_eq!(order.side, converted_order.side);
        assert_eq!(order.order_type, converted_order.order_type);
        assert_eq!(order.quantity, converted_order.quantity);
        assert_eq!(order.price, converted_order.price);
    }
}
```

## Module Dependencies

- **types**: For `OrderSide`, `OrderType`, `TimeInForce`, `OrderStatus`
- **chrono**: For `DateTime<Utc>` timestamp handling
- **serde**: For serialization/deserialization
- **error**: For `OrderValidationError`

## Related Types

- **OrderSide**: Buy/Sell side of the order
- **OrderType**: Type of order (Market, Limit, Stop, etc.)
- **TimeInForce**: Time in force policy
- **OrderStatus**: Current status of the order
- **ExecutionReport**: Result of order execution
- **OrderValidationError**: Validation error types
