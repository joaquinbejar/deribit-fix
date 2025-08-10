# MarketData

## Overview

`MarketData` represents real-time market data for instruments in the Deribit FIX system. It provides current prices, order book information, and trading statistics.

## Purpose

- **Price information**: Current bid/ask prices and last trade price
- **Order book data**: Bid and ask depth information
- **Trading statistics**: Volume, high/low prices, and other metrics
- **Market monitoring**: Real-time market state tracking

## Public Interface

### Struct Definition

```rust
pub struct MarketData {
    pub instrument: String,
    pub bid_price: Option<f64>,
    pub ask_price: Option<f64>,
    pub last_price: Option<f64>,
    pub last_qty: Option<f64>,
    pub volume_24h: f64,
    pub high_24h: Option<f64>,
    pub low_24h: Option<f64>,
    pub open_interest: Option<f64>,
    pub funding_rate: Option<f64>,
    pub mark_price: Option<f64>,
    pub index_price: Option<f64>,
    pub timestamp: DateTime<Utc>,
}
```

### Key Methods

```rust
impl MarketData {
    /// Creates new market data
    pub fn new(instrument: String) -> Self
    
    /// Gets spread between bid and ask
    pub fn spread(&self) -> Option<f64>
    
    /// Gets mid price
    pub fn mid_price(&self) -> Option<f64>
    
    /// Updates with new price data
    pub fn update_price(&mut self, bid: Option<f64>, ask: Option<f64>, last: Option<f64>)
    
    /// Checks if data is stale
    pub fn is_stale(&self, max_age: Duration) -> bool
}
```

## Usage Examples

### Creating and Updating Market Data

```rust
use deribit_fix::MarketData;
use chrono::Duration;

// Create new market data
let mut market_data = MarketData::new("BTC-PERPETUAL".to_string());

// Update with new prices
market_data.update_price(Some(50000.0), Some(50001.0), Some(50000.5));

// Check spread
if let Some(spread) = market_data.spread() {
    println!("Spread: ${:.2}", spread);
}
```

### Market Data Analysis

```rust
fn analyze_market_data(data: &MarketData) {
    println!("Instrument: {}", data.instrument);
    
    if let Some(mid) = data.mid_price() {
        println!("Mid Price: ${:.2}", mid);
    }
    
    if let Some(spread) = data.spread() {
        println!("Spread: ${:.2}", spread);
    }
    
    println!("24h Volume: {:.2}", data.volume_24h);
    
    if data.is_stale(Duration::seconds(5)) {
        println!("Warning: Data is stale");
    }
}
```

## Module Dependencies

- **chrono**: For `DateTime<Utc>` and `Duration` handling

## Related Types

- **Instrument**: The instrument being tracked
- **OrderBook**: Detailed order book information
- **Trade**: Individual trade data
