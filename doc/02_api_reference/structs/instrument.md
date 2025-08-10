# Instrument

## Overview

The `Instrument` struct represents a financial instrument that can be traded on Deribit, including options, futures, and perpetual contracts.

## Purpose

- **Instrument Identification**: Provides unique identifiers and metadata for trading instruments
- **Contract Specifications**: Defines contract terms, expiration, and settlement details
- **Risk Management**: Contains instrument-specific risk parameters and limits
- **Trading Rules**: Defines trading constraints and order validation rules

## Public Interface

### Struct Definition

```rust
pub struct Instrument {
    pub symbol: String,
    pub instrument_id: String,
    pub instrument_type: InstrumentType,
    pub underlying: String,
    pub quote_currency: String,
    pub base_currency: String,
    pub contract_size: f64,
    pub tick_size: f64,
    pub min_qty: f64,
    pub max_qty: f64,
    pub max_price: Option<f64>,
    pub min_price: Option<f64>,
    pub expiration_timestamp: Option<i64>,
    pub settlement_timestamp: Option<i64>,
    pub strike: Option<f64>,
    pub option_type: Option<OptionType>,
    pub is_active: bool,
    pub risk_limits: RiskLimits,
    pub trading_hours: TradingHours,
    pub metadata: HashMap<String, String>,
}
```

### Key Methods

```rust
impl Instrument {
    /// Create a new instrument with basic parameters
    pub fn new(
        symbol: String,
        instrument_type: InstrumentType,
        underlying: String,
        quote_currency: String,
        base_currency: String,
    ) -> Self

    /// Check if the instrument is currently tradeable
    pub fn is_tradeable(&self) -> bool

    /// Validate order parameters against instrument constraints
    pub fn validate_order(&self, order: &Order) -> Result<(), Vec<OrderValidationError>>

    /// Get the next valid price increment
    pub fn next_valid_price(&self, price: f64, side: OrderSide) -> f64

    /// Check if quantity is within valid range
    pub fn is_valid_quantity(&self, qty: f64) -> bool

    /// Get instrument expiration as DateTime
    pub fn expiration_datetime(&self) -> Option<DateTime<Utc>>

    /// Calculate margin requirements for position
    pub fn calculate_margin(&self, qty: f64, price: f64) -> f64

    /// Get instrument-specific risk parameters
    pub fn risk_parameters(&self) -> &RiskLimits
}
```

## Usage Examples

### Creating Basic Instruments

```rust
use deribit_fix::types::{Instrument, InstrumentType, RiskLimits, TradingHours};

// Create a BTC-PERPETUAL instrument
let btc_perp = Instrument::new(
    "BTC-PERPETUAL".to_string(),
    InstrumentType::Perpetual,
    "BTC".to_string(),
    "USD".to_string(),
    "BTC".to_string(),
);

// Create an option instrument
let btc_option = Instrument {
    symbol: "BTC-30JUN23-50000-C".to_string(),
    instrument_id: "BTC-30JUN23-50000-C".to_string(),
    instrument_type: InstrumentType::Option,
    underlying: "BTC".to_string(),
    quote_currency: "USD".to_string(),
    base_currency: "BTC".to_string(),
    contract_size: 1.0,
    tick_size: 0.5,
    min_qty: 0.001,
    max_qty: 1000.0,
    max_price: Some(100000.0),
    min_price: Some(0.01),
    expiration_timestamp: Some(1688169600), // June 30, 2023
    settlement_timestamp: Some(1688169600),
    strike: Some(50000.0),
    option_type: Some(OptionType::Call),
    is_active: true,
    risk_limits: RiskLimits::default(),
    trading_hours: TradingHours::default(),
    metadata: HashMap::new(),
};
```

### Instrument Validation

```rust
// Validate order against instrument constraints
let validation_result = btc_option.validate_order(&order);
match validation_result {
    Ok(()) => println!("Order is valid for this instrument"),
    Err(errors) => {
        for error in errors {
            eprintln!("Validation error: {:?}", error);
        }
    }
}

// Check if instrument is tradeable
if btc_option.is_tradeable() {
    println!("Instrument {} is available for trading", btc_option.symbol);
} else {
    println!("Instrument {} is not currently tradeable", btc_option.symbol);
}
```

### Price and Quantity Validation

```rust
// Get next valid price increment
let current_price = 50000.0;
let next_bid = btc_option.next_valid_price(current_price, OrderSide::Buy);
let next_ask = btc_option.next_valid_price(current_price, OrderSide::Sell);

println!("Next valid bid: {}", next_bid);
println!("Next valid ask: {}", next_ask);

// Validate quantity
let qty = 1.5;
if btc_option.is_valid_quantity(qty) {
    println!("Quantity {} is valid for {}", qty, btc_option.symbol);
} else {
    println!("Quantity {} is outside valid range", qty);
}
```

### Risk Management

```rust
// Calculate margin requirements
let position_qty = 10.0;
let current_price = 50000.0;
let margin = btc_option.calculate_margin(position_qty, current_price);

println!("Margin required for {} {}: {}", 
    position_qty, btc_option.symbol, margin);

// Get risk parameters
let risk_params = btc_option.risk_parameters();
println!("Max position size: {}", risk_params.max_position_size);
println!("Max leverage: {}", risk_params.max_leverage);
```

## Module Dependencies

### Direct Dependencies

- **`types`**: `InstrumentType`, `OptionType`, `RiskLimits`, `TradingHours`
- **`model`**: `Order`, `OrderValidationError`
- **`chrono`**: `DateTime<Utc>`
- **`std::collections`**: `HashMap`

### Related Types

- **`InstrumentType`**: Defines the category of financial instrument
- **`OptionType`**: Specifies call/put for option instruments
- **`RiskLimits`**: Contains instrument-specific risk parameters
- **`TradingHours`**: Defines when the instrument can be traded
- **`Order`**: Orders that reference this instrument
- **`Position`**: Positions held in this instrument

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instrument_creation() {
        let instrument = Instrument::new(
            "TEST-INST".to_string(),
            InstrumentType::Perpetual,
            "TEST".to_string(),
            "USD".to_string(),
            "TEST".to_string(),
        );

        assert_eq!(instrument.symbol, "TEST-INST");
        assert_eq!(instrument.instrument_type, InstrumentType::Perpetual);
        assert_eq!(instrument.underlying, "TEST");
    }

    #[test]
    fn test_quantity_validation() {
        let instrument = Instrument::new(
            "TEST-INST".to_string(),
            InstrumentType::Perpetual,
            "TEST".to_string(),
            "USD".to_string(),
            "TEST".to_string(),
        );

        instrument.min_qty = 1.0;
        instrument.max_qty = 100.0;

        assert!(instrument.is_valid_quantity(50.0));
        assert!(!instrument.is_valid_quantity(0.5));
        assert!(!instrument.is_valid_quantity(150.0));
    }

    #[test]
    fn test_price_validation() {
        let instrument = Instrument::new(
            "TEST-INST".to_string(),
            InstrumentType::Perpetual,
            "TEST".to_string(),
            "USD".to_string(),
            "TEST".to_string(),
        );

        instrument.tick_size = 0.5;
        instrument.min_price = Some(10.0);
        instrument.max_price = Some(1000.0);

        let next_bid = instrument.next_valid_price(100.0, OrderSide::Buy);
        let next_ask = instrument.next_valid_price(100.0, OrderSide::Sell);

        assert_eq!(next_bid, 99.5);
        assert_eq!(next_ask, 100.5);
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_instrument_order_validation() {
    let instrument = create_test_instrument();
    let order = create_test_order(&instrument.symbol);

    let result = instrument.validate_order(&order);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_instrument_risk_calculation() {
    let instrument = create_test_instrument();
    let qty = 10.0;
    let price = 100.0;

    let margin = instrument.calculate_margin(qty, price);
    assert!(margin > 0.0);
    assert!(margin <= qty * price); // Margin should not exceed position value
}
```

## Performance Considerations

- **Validation Caching**: Consider caching validation results for frequently traded instruments
- **Price Calculations**: Use efficient algorithms for tick size calculations
- **Memory Usage**: Large metadata maps should be optimized for memory usage
- **Concurrent Access**: Ensure thread-safe access to instrument data

## Security Considerations

- **Input Validation**: Validate all instrument parameters during creation
- **Access Control**: Restrict modification of critical instrument parameters
- **Audit Logging**: Log changes to instrument specifications
- **Data Integrity**: Ensure instrument data consistency across the system
