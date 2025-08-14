# Position

## Overview

`Position` represents a trading position in the Deribit FIX system. It tracks the current state of a position for a specific instrument, including size, P&L, and margin information.

## Purpose

- **Position tracking**: Monitors current position size and status
- **Risk management**: Provides P&L and margin information
- **Portfolio overview**: Shows exposure across instruments
- **Margin calculations**: Supports margin requirement calculations
- **Builder integration**: Emission and parsing supported via dedicated PositionReport builder (35=AP)

## Public Interface

### Struct Definition

```rust
pub struct Position {
    pub instrument: String,
    pub size: f64,
    pub side: PositionSide,
    pub avg_price: f64,
    pub unrealized_pnl: f64,
    pub realized_pnl: f64,
    pub margin: f64,
    pub leverage: f64,
    pub liquidation_price: Option<f64>,
    pub mark_price: f64,
    pub index_price: f64,
    pub timestamp: DateTime<Utc>,
}
```

### Key Methods

```rust
impl Position {
    /// Creates a new position
    pub fn new(instrument: String, side: PositionSide) -> Self
    
    /// Updates position with new trade
    pub fn update_with_trade(&mut self, qty: f64, price: f64)
    
    /// Calculates current P&L
    pub fn calculate_pnl(&self, current_price: f64) -> f64
    
    /// Gets position value
    pub fn position_value(&self) -> f64
    
    /// Checks if position is long
    pub fn is_long(&self) -> bool
    
    /// Checks if position is short
    pub fn is_short(&self) -> bool
    
    /// Gets position size as absolute value
    pub fn absolute_size(&self) -> f64
}
```

## Usage Examples

### Creating and Updating Positions

```rust
use deribit_fix::{Position, PositionSide};

// Create new position
let mut position = Position::new(
    "BTC-PERPETUAL".to_string(),
    PositionSide::Long
);

// Update with trade
position.update_with_trade(1.0, 50000.0);

// Check position status
if position.is_long() {
    println!("Long position: {} BTC", position.size);
}
```

### Position Monitoring

```rust
fn monitor_position(position: &Position) {
    println!("Instrument: {}", position.instrument);
    println!("Size: {} ({} side)", position.absolute_size(), 
        if position.is_long() { "long" } else { "short" });
    println!("Average Price: ${:.2}", position.avg_price);
    println!("Unrealized P&L: ${:.2}", position.unrealized_pnl);
    println!("Margin: ${:.2}", position.margin);
    
    if let Some(liq_price) = position.liquidation_price {
        println!("Liquidation Price: ${:.2}", liq_price);
    }
}
```

## Module Dependencies

- **types**: For `PositionSide` enum
- **chrono**: For `DateTime<Utc>` timestamp handling

## Related Types

- **PositionSide**: Long/Short position direction
- **Instrument**: The traded instrument
- **Order**: Orders that affect position
