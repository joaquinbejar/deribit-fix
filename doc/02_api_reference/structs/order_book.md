# OrderBook

## Overview

`OrderBook` represents the current order book for an instrument in the Deribit FIX system. It provides bid and ask depth information with price levels and quantities.

## Purpose

- **Market depth**: Shows available liquidity at different price levels
- **Price discovery**: Helps determine fair market prices
- **Trading decisions**: Supports order placement strategies
- **Market analysis**: Provides insights into market structure

## Public Interface

### Struct Definition

```rust
pub struct OrderBook {
    pub instrument: String,
    pub bids: Vec<OrderBookEntry>,
    pub asks: Vec<OrderBookEntry>,
    pub timestamp: DateTime<Utc>,
    pub sequence_number: u64,
}
```

### Nested Types

```rust
pub struct OrderBookEntry {
    pub price: f64,
    pub quantity: f64,
    pub order_count: Option<u32>,
}
```

### Key Methods

```rust
impl OrderBook {
    /// Creates new order book
    pub fn new(instrument: String) -> Self
    
    /// Gets best bid price
    pub fn best_bid(&self) -> Option<f64>
    
    /// Gets best ask price
    pub fn best_ask(&self) -> Option<f64>
    
    /// Gets spread
    pub fn spread(&self) -> Option<f64>
    
    /// Gets mid price
    pub fn mid_price(&self) -> Option<f64>
    
    /// Updates order book with new data
    pub fn update(&mut self, bids: Vec<OrderBookEntry>, asks: Vec<OrderBookEntry>)
    
    /// Gets total quantity at price level
    pub fn total_at_price(&self, price: f64, side: OrderSide) -> f64
}
```

## Usage Examples

### Creating and Updating Order Book

```rust
use deribit_fix::{OrderBook, OrderBookEntry, OrderSide};

// Create new order book
let mut order_book = OrderBook::new("BTC-PERPETUAL".to_string());

// Create bid entries
let bids = vec![
    OrderBookEntry { price: 50000.0, quantity: 1.5, order_count: Some(3) },
    OrderBookEntry { price: 49999.0, quantity: 2.0, order_count: Some(2) },
];

// Create ask entries
let asks = vec![
    OrderBookEntry { price: 50001.0, quantity: 1.0, order_count: Some(1) },
    OrderBookEntry { price: 50002.0, quantity: 2.5, order_count: Some(4) },
];

// Update order book
order_book.update(bids, asks);
```

### Order Book Analysis

```rust
fn analyze_order_book(book: &OrderBook) {
    println!("Instrument: {}", book.instrument);
    
    if let Some(best_bid) = book.best_bid() {
        println!("Best Bid: ${:.2}", best_bid);
    }
    
    if let Some(best_ask) = book.best_ask() {
        println!("Best Ask: ${:.2}", best_ask);
    }
    
    if let Some(spread) = book.spread() {
        println!("Spread: ${:.2}", spread);
    }
    
    if let Some(mid) = book.mid_price() {
        println!("Mid Price: ${:.2}", mid);
    }
    
    println!("Bid Levels: {}", book.bids.len());
    println!("Ask Levels: {}", book.asks.len());
}
```

## Module Dependencies

- **types**: For `OrderSide` enum
- **chrono**: For `DateTime<Utc>` timestamp handling

## Related Types

- **OrderBookEntry**: Individual price level entry
- **OrderSide**: Bid/Ask side
- **MarketData**: Aggregated market information
