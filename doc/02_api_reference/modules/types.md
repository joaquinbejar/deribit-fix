# types Module

## Overview

The `types` module provides core data types for the `deribit-fix` crate, including trading entities (orders, executions, positions), market data structures, and domain-specific enums and constants.

## Purpose

- **Core Data Types**: Define the fundamental data structures used throughout the crate
- **Domain Modeling**: Represent trading concepts in a type-safe manner
- **Serialization**: Support conversion to/from FIX protocol format
- **Validation**: Ensure data integrity and business rule compliance

## Public Interface

### Trading Entities

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub instrument: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub quantity: f64,
    pub price: Option<f64>,
    pub stop_price: Option<f64>,
    pub time_in_force: TimeInForce,
    pub post_only: bool,
    pub reduce_only: bool,
    pub client_order_id: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionReport {
    pub order_id: String,
    pub exec_id: String,
    pub exec_type: ExecType,
    pub ord_status: OrderStatus,
    pub symbol: String,
    pub side: OrderSide,
    pub leaves_qty: f64,
    pub cum_qty: f64,
    pub avg_px: Option<f64>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub instrument: String,
    pub size: f64,
    pub average_price: f64,
    pub liquidation_price: f64,
    pub margin: f64,
    pub unrealized_pnl: f64,
    pub realized_pnl: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketData {
    pub instrument: String,
    pub bid_price: Option<f64>,
    pub ask_price: Option<f64>,
    pub last_price: Option<f64>,
    pub volume: f64,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    pub instrument: String,
    pub bids: Vec<OrderBookEntry>,
    pub asks: Vec<OrderBookEntry>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookEntry {
    pub price: f64,
    pub quantity: f64,
    pub order_count: Option<u32>,
}
```

### Core Enums

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    Buy = 1,
    Sell = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    Market = 1,
    Limit = 2,
    Stop = 3,
    StopLimit = 4,
    MarketIfTouched = 5,
    LimitIfTouched = 6,
    MarketWithLeftOverAsLimit = 7,
    PreviousFundValuedOrder = 8,
    NextFundValuedOrder = 9,
    Pegged = 10,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeInForce {
    Day = 0,
    GoodTillCancel = 1,
    AtTheOpening = 2,
    ImmediateOrCancel = 3,
    FillOrKill = 4,
    GoodTillCrossing = 5,
    GoodTillDate = 6,
    AtTheClose = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecType {
    New = 0,
    PartialFill = 1,
    Fill = 2,
    DoneForDay = 3,
    Canceled = 4,
    Replace = 5,
    PendingCancel = 6,
    Stopped = 7,
    Rejected = 8,
    Suspended = 9,
    PendingNew = 10,
    Calculated = 11,
    Expired = 12,
    Restated = 13,
    PendingReplace = 14,
    Trade = 15,
    TradeCorrect = 16,
    TradeCancel = 17,
    OrderStatus = 18,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    New = 0,
    PartiallyFilled = 1,
    Filled = 2,
    DoneForDay = 3,
    Canceled = 4,
    PendingCancel = 6,
    Stopped = 7,
    Rejected = 8,
    Suspended = 9,
    PendingNew = 10,
    Calculated = 11,
    Expired = 12,
    AcceptedForBidding = 13,
    PendingReplace = 14,
}
```

### Instrument Types

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstrumentType {
    Future = 1,
    Option = 2,
    Perpetual = 3,
    Spot = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptionType {
    Call = 1,
    Put = 2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instrument {
    pub symbol: String,
    pub instrument_type: InstrumentType,
    pub underlying: String,
    pub expiry: Option<DateTime<Utc>>,
    pub strike: Option<f64>,
    pub option_type: Option<OptionType>,
    pub contract_size: f64,
    pub tick_size: f64,
    pub min_qty: f64,
    pub max_qty: f64,
    pub is_active: bool,
}
```

### Configuration Types

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskLimits {
    pub max_position_size: f64,
    pub max_order_size: f64,
    pub max_leverage: f64,
    pub margin_requirement: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderValidationConfig {
    pub validate_price_ticks: bool,
    pub validate_quantity_limits: bool,
    pub validate_margin_requirements: bool,
    pub allow_post_only: bool,
    pub allow_reduce_only: bool,
}
```

## Usage Examples

### Creating Orders

```rust
use deribit_fix::types::{Order, OrderSide, OrderType, TimeInForce};
use chrono::Utc;

// Create a limit buy order
let order = Order {
    id: "ORDER001".to_string(),
    instrument: "BTC-PERPETUAL".to_string(),
    side: OrderSide::Buy,
    order_type: OrderType::Limit,
    quantity: 1.0,
    price: Some(50000.0),
    stop_price: None,
    time_in_force: TimeInForce::GoodTillCancel,
    post_only: false,
    reduce_only: false,
    client_order_id: Some("CLIENT_ORDER_001".to_string()),
    timestamp: Utc::now(),
};

// Create a stop-loss order
let stop_order = Order {
    id: "STOP001".to_string(),
    instrument: "BTC-PERPETUAL".to_string(),
    side: OrderSide::Sell,
    order_type: OrderType::Stop,
    quantity: 1.0,
    price: None,
    stop_price: Some(45000.0),
    time_in_force: TimeInForce::GoodTillCancel,
    post_only: false,
    reduce_only: true,
    client_order_id: None,
    timestamp: Utc::now(),
};
```

### Working with Market Data

```rust
use deribit_fix::types::{MarketData, OrderBook, OrderBookEntry};

// Create market data snapshot
let market_data = MarketData {
    instrument: "BTC-PERPETUAL".to_string(),
    bid_price: Some(49950.0),
    ask_price: Some(50050.0),
    last_price: Some(50000.0),
    volume: 1000.0,
    high: Some(51000.0),
    low: Some(49000.0),
    timestamp: Utc::now(),
};

// Create order book
let order_book = OrderBook {
    instrument: "BTC-PERPETUAL".to_string(),
    bids: vec![
        OrderBookEntry { price: 49950.0, quantity: 2.0, order_count: Some(5) },
        OrderBookEntry { price: 49900.0, quantity: 1.5, order_count: Some(3) },
        OrderBookEntry { price: 49850.0, quantity: 3.0, order_count: Some(7) },
    ],
    asks: vec![
        OrderBookEntry { price: 50050.0, quantity: 1.0, order_count: Some(2) },
        OrderBookEntry { price: 50100.0, quantity: 2.5, order_count: Some(4) },
        OrderBookEntry { price: 50150.0, quantity: 1.8, order_count: Some(6) },
    ],
    timestamp: Utc::now(),
};

// Calculate spread
let spread = order_book.asks[0].price - order_book.bids[0].price;
println!("Spread: {}", spread); // 100.0

// Calculate total bid volume
let total_bid_volume: f64 = order_book.bids.iter().map(|entry| entry.quantity).sum();
println!("Total bid volume: {}", total_bid_volume); // 6.5
```

### Position Management

```rust
use deribit_fix::types::{Position, OrderSide};

// Create a long position
let long_position = Position {
    instrument: "BTC-PERPETUAL".to_string(),
    size: 2.0,
    average_price: 50000.0,
    liquidation_price: 45000.0,
    margin: 10000.0,
    unrealized_pnl: 2000.0, // Assuming current price is 51000
    realized_pnl: 500.0,
    timestamp: Utc::now(),
};

// Create a short position
let short_position = Position {
    instrument: "BTC-PERPETUAL".to_string(),
    size: -1.5,
    average_price: 51000.0,
    liquidation_price: 55000.0,
    margin: 7500.0,
    unrealized_pnl: -1500.0, // Assuming current price is 50000
    realized_pnl: -200.0,
    timestamp: Utc::now(),
};

// Calculate total position value
let total_position_value = long_position.size.abs() * long_position.average_price;
println!("Total position value: {}", total_position_value); // 100000.0

// Calculate margin ratio
let margin_ratio = long_position.margin / total_position_value;
println!("Margin ratio: {:.2}%", margin_ratio * 100.0); // 10.00%
```

### Instrument Management

```rust
use deribit_fix::types::{Instrument, InstrumentType, OptionType};

// Create a perpetual contract
let btc_perp = Instrument {
    symbol: "BTC-PERPETUAL".to_string(),
    instrument_type: InstrumentType::Perpetual,
    underlying: "BTC".to_string(),
    expiry: None,
    strike: None,
    option_type: None,
    contract_size: 1.0,
    tick_size: 0.5,
    min_qty: 0.001,
    max_qty: 1000.0,
    is_active: true,
};

// Create a call option
let btc_call = Instrument {
    symbol: "BTC-30JUN23-50000-C".to_string(),
    instrument_type: InstrumentType::Option,
    underlying: "BTC".to_string(),
    expiry: Some(DateTime::parse_from_rfc3339("2023-06-30T08:00:00Z").unwrap()),
    strike: Some(50000.0),
    option_type: Some(OptionType::Call),
    contract_size: 1.0,
    tick_size: 0.1,
    min_qty: 0.01,
    max_qty: 100.0,
    is_active: true,
};

// Validate order against instrument
fn validate_order_against_instrument(order: &Order, instrument: &Instrument) -> Result<(), String> {
    // Check quantity limits
    if order.quantity < instrument.min_qty || order.quantity > instrument.max_qty {
        return Err(format!(
            "Quantity {} is outside limits [{}, {}]",
            order.quantity, instrument.min_qty, instrument.max_qty
        ));
    }
    
    // Check price tick size
    if let Some(price) = order.price {
        let remainder = (price / instrument.tick_size).fract();
        if remainder.abs() > f64::EPSILON {
            return Err(format!(
                "Price {} is not a multiple of tick size {}",
                price, instrument.tick_size
            ));
        }
    }
    
    Ok(())
}
```

### Enum Pattern Matching

```rust
use deribit_fix::types::{OrderSide, OrderType, TimeInForce, ExecType, OrderStatus};

// Pattern matching with order side
fn get_side_description(side: OrderSide) -> &'static str {
    match side {
        OrderSide::Buy => "Buy order - purchasing the instrument",
        OrderSide::Sell => "Sell order - selling the instrument",
    }
}

// Pattern matching with order type
fn get_order_type_description(order_type: OrderType) -> &'static str {
    match order_type {
        OrderType::Market => "Market order - executed at best available price",
        OrderType::Limit => "Limit order - executed at specified price or better",
        OrderType::Stop => "Stop order - becomes market order when stop price is reached",
        OrderType::StopLimit => "Stop-limit order - becomes limit order when stop price is reached",
        OrderType::MarketIfTouched => "Market-if-touched order - executed when price touches specified level",
        OrderType::LimitIfTouched => "Limit-if-touched order - becomes limit order when price touches specified level",
        OrderType::MarketWithLeftOverAsLimit => "Market order with remaining quantity as limit order",
        OrderType::PreviousFundValuedOrder => "Previous fund valued order",
        OrderType::NextFundValuedOrder => "Next fund valued order",
        OrderType::Pegged => "Pegged order - price is pegged to another order or market",
    }
}

// Pattern matching with execution type
fn is_final_execution(exec_type: ExecType) -> bool {
    matches!(
        exec_type,
        ExecType::Fill | ExecType::DoneForDay | ExecType::Canceled | 
        ExecType::Rejected | ExecType::Expired | ExecType::Trade
    )
}

// Pattern matching with order status
fn can_modify_order(status: OrderStatus) -> bool {
    matches!(
        status,
        OrderStatus::New | OrderStatus::PartiallyFilled | OrderStatus::PendingNew
    )
}
```

### Serialization and Deserialization

```rust
use deribit_fix::types::{Order, OrderSide, OrderType, TimeInForce};
use serde_json;

// Serialize order to JSON
let order = Order {
    id: "ORDER001".to_string(),
    instrument: "BTC-PERPETUAL".to_string(),
    side: OrderSide::Buy,
    order_type: OrderType::Limit,
    quantity: 1.0,
    price: Some(50000.0),
    stop_price: None,
    time_in_force: TimeInForce::GoodTillCancel,
    post_only: false,
    reduce_only: false,
    client_order_id: None,
    timestamp: Utc::now(),
};

let json = serde_json::to_string_pretty(&order)?;
println!("{}", json);

// Deserialize order from JSON
let deserialized_order: Order = serde_json::from_str(&json)?;
assert_eq!(order.id, deserialized_order.id);
assert_eq!(order.side, deserialized_order.side);
assert_eq!(order.order_type, deserialized_order.order_type);

// Serialize to FIX format
let fix_message = order.into();
let fix_string = fix_message.to_fix_string()?;
println!("FIX: {}", fix_string);
```

## Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_creation() {
        let order = Order {
            id: "TEST001".to_string(),
            instrument: "BTC-PERPETUAL".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            quantity: 1.0,
            price: Some(50000.0),
            stop_price: None,
            time_in_force: TimeInForce::GoodTillCancel,
            post_only: false,
            reduce_only: false,
            client_order_id: None,
            timestamp: Utc::now(),
        };
        
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.order_type, OrderType::Limit);
        assert_eq!(order.quantity, 1.0);
        assert_eq!(order.price, Some(50000.0));
    }

    #[test]
    fn test_enum_serialization() {
        let side = OrderSide::Sell;
        let json = serde_json::to_string(&side).unwrap();
        assert_eq!(json, "2");
        
        let deserialized: OrderSide = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, OrderSide::Sell);
    }

    #[test]
    fn test_position_calculations() {
        let position = Position {
            instrument: "BTC-PERPETUAL".to_string(),
            size: 2.0,
            average_price: 50000.0,
            liquidation_price: 45000.0,
            margin: 10000.0,
            unrealized_pnl: 2000.0,
            realized_pnl: 500.0,
            timestamp: Utc::now(),
        };
        
        let position_value = position.size.abs() * position.average_price;
        assert_eq!(position_value, 100000.0);
        
        let margin_ratio = position.margin / position_value;
        assert_eq!(margin_ratio, 0.1);
    }
}
```

## Module Dependencies

- `serde`: Serialization/deserialization
- `chrono`: Timestamp handling
- `rust_decimal`: Precise decimal arithmetic for financial calculations
- `thiserror`: Error types for validation
