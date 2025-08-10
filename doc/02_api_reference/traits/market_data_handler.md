# MarketDataHandler Trait

## Overview

The `MarketDataHandler` trait defines the interface for managing market data within the Deribit FIX client. It provides methods for requesting, receiving, processing, and distributing real-time market data including order books, trades, tick data, and market statistics.

## Purpose

- **Market Data Requests**: Request specific market data from the exchange
- **Real-time Data Processing**: Handle incoming market data streams
- **Order Book Management**: Maintain and update order book snapshots
- **Trade Data Processing**: Process and store trade execution data
- **Market Statistics**: Calculate and provide market analytics
- **Data Distribution**: Distribute market data to subscribers
- **Historical Data**: Access historical market data when available

## Public Interface

### Trait Definition

```rust
#[async_trait]
pub trait MarketDataHandler: Send + Sync {
    async fn request_market_data(&mut self, request: MarketDataRequest) -> Result<String, MarketDataHandlerError>;
    async fn cancel_market_data_request(&mut self, request_id: &str) -> Result<(), MarketDataHandlerError>;
    async fn subscribe_to_order_book(&mut self, instrument: &str, depth: u32) -> Result<(), MarketDataHandlerError>;
    async fn unsubscribe_from_order_book(&mut self, instrument: &str) -> Result<(), MarketDataHandlerError>;
    async fn get_order_book(&mut self, instrument: &str) -> Result<OrderBook, MarketDataHandlerError>;
    async fn get_trade_history(&mut self, instrument: &str, limit: Option<u32>) -> Result<Vec<Trade>, MarketDataHandlerError>;
    async fn get_ticker(&mut self, instrument: &str) -> Result<Ticker, MarketDataHandlerError>;
    async fn get_market_depth(&mut self, instrument: &str, levels: u32) -> Result<MarketDepth, MarketDataHandlerError>;
    async fn handle_market_data_snapshot(&mut self, snapshot: MarketDataSnapshot) -> Result<(), MarketDataHandlerError>;
    async fn handle_market_data_incremental(&mut self, update: MarketDataIncremental) -> Result<(), MarketDataHandlerError>;
    async fn handle_trade_report(&mut self, trade: TradeReport) -> Result<(), MarketDataHandlerError>;
    async fn calculate_market_statistics(&mut self, instrument: &str) -> Result<MarketStatistics, MarketDataHandlerError>;
    fn get_market_data_stats(&self) -> MarketDataHandlerStats;
    async fn get_instruments_info(&mut self, filter: InstrumentFilter) -> Result<Vec<InstrumentInfo>, MarketDataHandlerError>;
    async fn validate_market_data_request(&mut self, request: &MarketDataRequest) -> Result<(), Vec<MarketDataValidationError>>;
    async fn get_market_data_subscriptions(&mut self) -> Result<Vec<MarketDataSubscription>, MarketDataHandlerError>;
}
```

### Associated Types

```rust
pub enum MarketDataHandlerError {
    InstrumentNotFound(String),
    InvalidRequest(Vec<MarketDataValidationError>),
    SubscriptionLimitExceeded(u32),
    RequestTimeout(Duration),
    ExchangeError(String),
    NetworkError(String),
    DataUnavailable(String),
    InvalidDepth(u32),
    RateLimitExceeded(String),
}

pub enum MarketDataValidationError {
    InvalidInstrument(String),
    InvalidDepth(u32),
    InvalidUpdateType(String),
    InvalidSnapshotType(String),
    UnsupportedMarketDataType(String),
    InvalidSubscriptionParameters(String),
}

pub struct MarketDataRequest {
    pub instrument: String,
    pub market_data_type: MarketDataType,
    pub depth: Option<u32>,
    pub update_type: MarketDataUpdateType,
    pub snapshot_type: Option<MarketDataSnapshotType>,
    pub subscription_id: Option<String>,
}

pub enum MarketDataType {
    OrderBook,
    Trades,
    Ticker,
    MarketDepth,
    OHLCV,
    FundingRate,
    OpenInterest,
    ImpliedVolatility,
}

pub enum MarketDataUpdateType {
    FullRefresh,
    IncrementalRefresh,
    Snapshot,
    SnapshotAndUpdates,
}

pub enum MarketDataSnapshotType {
    Full,
    Incremental,
    TopOfBook,
    Depth,
}

pub struct MarketDataSnapshot {
    pub instrument: String,
    pub snapshot_type: MarketDataSnapshotType,
    pub timestamp: DateTime<Utc>,
    pub data: MarketDataContent,
    pub sequence_number: u64,
}

pub enum MarketDataContent {
    OrderBook(OrderBook),
    Trades(Vec<Trade>),
    Ticker(Ticker),
    MarketDepth(MarketDepth),
    OHLCV(OHLCV),
}

pub struct MarketDataIncremental {
    pub instrument: String,
    pub update_type: MarketDataUpdateType,
    pub timestamp: DateTime<Utc>,
    pub updates: Vec<MarketDataUpdate>,
    pub sequence_number: u64,
}

pub enum MarketDataUpdate {
    OrderBookUpdate(OrderBookUpdate),
    TradeUpdate(TradeUpdate),
    TickerUpdate(TickerUpdate),
    DepthUpdate(DepthUpdate),
}

pub struct OrderBookUpdate {
    pub side: OrderSide,
    pub price: f64,
    pub quantity: f64,
    pub update_type: OrderBookUpdateType,
}

pub enum OrderBookUpdateType {
    New,
    Change,
    Delete,
}

pub struct TradeReport {
    pub instrument: String,
    pub trade_id: String,
    pub price: f64,
    pub quantity: f64,
    pub side: OrderSide,
    pub timestamp: DateTime<Utc>,
    pub maker_order_id: Option<String>,
    pub taker_order_id: Option<String>,
}

pub struct MarketStatistics {
    pub instrument: String,
    pub timestamp: DateTime<Utc>,
    pub volume_24h: f64,
    pub high_24h: f64,
    pub low_24h: f64,
    pub price_change_24h: f64,
    pub price_change_percent_24h: f64,
    pub open_interest: Option<f64>,
    pub funding_rate: Option<f64>,
    pub implied_volatility: Option<f64>,
}

pub struct InstrumentFilter {
    pub instrument_type: Option<InstrumentType>,
    pub underlying: Option<String>,
    pub quote_currency: Option<String>,
    pub base_currency: Option<String>,
    pub active_only: bool,
}

pub struct InstrumentInfo {
    pub symbol: String,
    pub instrument_type: InstrumentType,
    pub underlying: String,
    pub quote_currency: String,
    pub base_currency: String,
    pub contract_size: f64,
    pub tick_size: f64,
    pub min_order_size: f64,
    pub max_order_size: f64,
    pub is_active: bool,
    pub expiration: Option<DateTime<Utc>>,
}

pub struct MarketDataSubscription {
    pub subscription_id: String,
    pub instrument: String,
    pub market_data_type: MarketDataType,
    pub depth: Option<u32>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub last_update: Option<DateTime<Utc>>,
}

pub struct MarketDataHandlerStats {
    pub total_requests: u64,
    pub active_subscriptions: u64,
    pub total_data_updates: u64,
    pub average_latency: Duration,
    pub error_rate: f64,
    pub last_update_time: Option<DateTime<Utc>>,
    pub bandwidth_usage: u64,
}
```

## Usage Examples

### Basic Market Data Request

```rust
use deribit_fix::traits::MarketDataHandler;
use deribit_fix::types::{MarketDataRequest, MarketDataType, MarketDataUpdateType};

struct MyMarketDataHandler;

#[async_trait]
impl MarketDataHandler for MyMarketDataHandler {
    async fn request_market_data(&mut self, request: MarketDataRequest) -> Result<String, MarketDataHandlerError> {
        // Validate the request
        self.validate_market_data_request(&request).await?;
        
        // Check subscription limits
        if self.would_exceed_subscription_limit(&request).await? {
            return Err(MarketDataHandlerError::SubscriptionLimitExceeded(self.get_max_subscriptions()));
        }
        
        // Submit request to exchange
        let request_id = self.submit_market_data_request(&request).await?;
        
        // Store subscription locally
        self.store_subscription(request_id.clone(), request).await?;
        
        Ok(request_id)
    }
    
    // ... implement other required methods
}
```

### Order Book Subscription

```rust
impl MyMarketDataHandler {
    async fn subscribe_to_full_order_book(&mut self, instrument: &str) -> Result<(), MarketDataHandlerError> {
        let request = MarketDataRequest {
            instrument: instrument.to_string(),
            market_data_type: MarketDataType::OrderBook,
            depth: Some(100), // Full depth
            update_type: MarketDataUpdateType::SnapshotAndUpdates,
            snapshot_type: Some(MarketDataSnapshotType::Full),
            subscription_id: None,
        };
        
        let request_id = self.request_market_data(request).await?;
        
        // Subscribe to order book updates
        self.subscribe_to_order_book(instrument, 100).await?;
        
        log::info!("Subscribed to full order book for {}", instrument);
        Ok(())
    }
}
```

### Market Data Processing

```rust
impl MyMarketDataHandler {
    async fn process_market_data_snapshot(&mut self, snapshot: MarketDataSnapshot) -> Result<(), MarketDataHandlerError> {
        let instrument = &snapshot.instrument;
        
        match &snapshot.data {
            MarketDataContent::OrderBook(order_book) => {
                // Update local order book
                self.update_order_book(instrument, order_book.clone()).await?;
                
                // Notify subscribers
                self.notify_order_book_subscribers(instrument, order_book).await?;
            }
            
            MarketDataContent::Trades(trades) => {
                // Store trade history
                self.store_trades(instrument, trades).await?;
                
                // Update ticker
                self.update_ticker_from_trades(instrument, trades).await?;
            }
            
            MarketDataContent::Ticker(ticker) => {
                // Update ticker
                self.update_ticker(instrument, ticker.clone()).await?;
                
                // Notify ticker subscribers
                self.notify_ticker_subscribers(instrument, ticker).await?;
            }
            
            _ => {
                log::warn!("Unhandled market data content type for {}", instrument);
            }
        }
        
        // Update statistics
        self.update_market_statistics(instrument).await?;
        
        Ok(())
    }
}
```

### Incremental Updates

```rust
impl MyMarketDataHandler {
    async fn process_incremental_update(&mut self, update: MarketDataIncremental) -> Result<(), MarketDataHandlerError> {
        let instrument = &update.instrument;
        
        for data_update in &update.updates {
            match data_update {
                MarketDataUpdate::OrderBookUpdate(book_update) => {
                    self.apply_order_book_update(instrument, book_update).await?;
                }
                
                MarketDataUpdate::TradeUpdate(trade_update) => {
                    self.apply_trade_update(instrument, trade_update).await?;
                }
                
                MarketDataUpdate::TickerUpdate(ticker_update) => {
                    self.apply_ticker_update(instrument, ticker_update).await?;
                }
                
                MarketDataUpdate::DepthUpdate(depth_update) => {
                    self.apply_depth_update(instrument, depth_update).await?;
                }
            }
        }
        
        // Notify subscribers of changes
        self.notify_incremental_update_subscribers(&update).await?;
        
        Ok(())
    }
}
```

### Market Statistics Calculation

```rust
impl MyMarketDataHandler {
    async fn calculate_comprehensive_statistics(&mut self, instrument: &str) -> Result<MarketStatistics, MarketDataHandlerError> {
        let now = Utc::now();
        let day_ago = now - Duration::hours(24);
        
        // Get trade history for last 24 hours
        let trades = self.get_trade_history(instrument, Some(1000)).await?;
        let recent_trades: Vec<_> = trades.into_iter()
            .filter(|trade| trade.timestamp >= day_ago)
            .collect();
        
        if recent_trades.is_empty() {
            return Err(MarketDataHandlerError::DataUnavailable("No recent trades".to_string()));
        }
        
        // Calculate statistics
        let volume_24h: f64 = recent_trades.iter().map(|t| t.quantity).sum();
        let prices: Vec<f64> = recent_trades.iter().map(|t| t.price).collect();
        let high_24h = prices.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let low_24h = prices.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        
        let first_price = recent_trades.first().unwrap().price;
        let last_price = recent_trades.last().unwrap().price;
        let price_change_24h = last_price - first_price;
        let price_change_percent_24h = if first_price > 0.0 {
            (price_change_24h / first_price) * 100.0
        } else {
            0.0
        };
        
        // Get additional data
        let open_interest = self.get_open_interest(instrument).await.ok();
        let funding_rate = self.get_funding_rate(instrument).await.ok();
        let implied_volatility = self.get_implied_volatility(instrument).await.ok();
        
        Ok(MarketStatistics {
            instrument: instrument.to_string(),
            timestamp: now,
            volume_24h,
            high_24h,
            low_24h,
            price_change_24h,
            price_change_percent_24h,
            open_interest,
            funding_rate,
            implied_volatility,
        })
    }
}
```

### Order Book Management

```rust
impl MyMarketDataHandler {
    async fn maintain_order_book(&mut self, instrument: &str) -> Result<(), MarketDataHandlerError> {
        let mut order_book = self.get_order_book(instrument).await?;
        
        // Sort bids (descending) and asks (ascending)
        order_book.bids.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
        order_book.asks.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
        
        // Remove zero quantities
        order_book.bids.retain(|entry| entry.quantity > 0.0);
        order_book.asks.retain(|entry| entry.quantity > 0.0);
        
        // Calculate spread
        if let (Some(best_bid), Some(best_ask)) = (order_book.bids.first(), order_book.asks.first()) {
            order_book.spread = Some(best_ask.price - best_bid.price);
            order_book.mid_price = Some((best_bid.price + best_ask.price) / 2.0);
        }
        
        // Update stored order book
        self.update_stored_order_book(instrument, order_book).await?;
        
        Ok(())
    }
}
```

## Module Dependencies

- **`types`**: Uses `OrderBook`, `Trade`, `Ticker`, and related market data types
- **`error`**: Uses `MarketDataHandlerError` and validation error types
- **`message`**: May interact with FIX message handling for market data
- **`session`**: May use session management for data requests

## Related Types

- **`OrderBook`**: The core order book structure being managed
- **`MarketDataRequest`**: Requests for specific market data
- **`MarketDataSnapshot`**: Complete market data snapshots
- **`MarketDataIncremental`**: Incremental market data updates
- **`MarketStatistics`**: Calculated market statistics and analytics

## Testing

### Unit Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_market_data_request_validation() {
        let handler = MyMarketDataHandler::new();
        let request = create_test_market_data_request();
        
        let result = handler.validate_market_data_request(&request).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_order_book_subscription() {
        let mut handler = MyMarketDataHandler::new();
        
        let result = handler.subscribe_to_order_book("BTC-PERPETUAL", 100).await;
        assert!(result.is_ok());
    }
}
```

### Integration Testing

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_market_data_lifecycle() {
        let mut handler = MyMarketDataHandler::new();
        
        // Subscribe to market data
        handler.subscribe_to_order_book("BTC-PERPETUAL", 100).await.unwrap();
        
        // Request market data
        let request = create_test_market_data_request();
        let request_id = handler.request_market_data(request).await.unwrap();
        
        // Wait for data
        tokio::time::sleep(Duration::from_millis(1000)).await;
        
        // Get order book
        let order_book = handler.get_order_book("BTC-PERPETUAL").await.unwrap();
        assert!(!order_book.bids.is_empty() || !order_book.asks.is_empty());
        
        // Cancel subscription
        handler.unsubscribe_from_order_book("BTC-PERPETUAL").await.unwrap();
    }
}
```

## Performance Considerations

- **Async Operations**: All operations are async to avoid blocking
- **Data Caching**: Cache frequently accessed market data
- **Incremental Updates**: Use incremental updates to minimize data transfer
- **Connection Pooling**: Use connection pools for exchange communication
- **Rate Limiting**: Respect exchange rate limits for market data requests

## Security Considerations

- **Request Validation**: Validate all market data requests
- **Subscription Limits**: Enforce subscription limits to prevent abuse
- **Data Sanitization**: Sanitize all incoming market data
- **Access Control**: Restrict market data access to authorized users
- **Audit Logging**: Log all market data requests and subscriptions
