# Advanced Usage Examples

## Overview

This guide demonstrates advanced usage patterns for the Deribit FIX Client, including complex order management, market data handling, position management, and risk control strategies.

## Prerequisites

Before running these examples, ensure you have:

1. **Configured Client**: A properly initialized `DeribitFixClient`
2. **Active Connection**: Established FIX session with Deribit
3. **Authentication**: Valid API credentials and successful logon
4. **Market Data Access**: Subscriptions to required instrument feeds

## Order Management Examples

### **1. Complex Order Lifecycle**

```rust
use deribit_fix::client::DeribitFixClient;
use deribit_fix::config::DeribitFixConfig;
use deribit_fix::message::orders::{NewOrderSingle, OrderCancelRequest, OrderCancelReplaceRequest};
use deribit_fix::message::orders::{OrderSide, OrderType, TimeInForce};
use log::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let config = DeribitFixConfig::new()
        .with_credentials("your_username".to_string(), "your_password".to_string())
        .with_session_ids("YOUR_SENDER_ID".to_string(), "DERIBITSERVER".to_string());

    let mut client = DeribitFixClient::new(config).await?;
    client.connect().await?;

    // Place initial limit order
    let order = NewOrderSingle::new(
        "ord_001".to_string(),
        "BTC-PERPETUAL".to_string(),
        OrderSide::Buy,
        OrderType::Limit,
        TimeInForce::GoodTillCancelled,
        10.0, // quantity
    ).with_price(45000.0);

    let order_id = client.place_order(order).await?;
    info!("Placed order: {}", order_id);

    // Wait and then modify the order
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let modify_request = OrderCancelReplaceRequest::new(
        order_id.clone(),
        "ord_001_mod".to_string(),
        "BTC-PERPETUAL".to_string(),
        OrderSide::Buy,
        OrderType::Limit,
        TimeInForce::GoodTillCancelled,
        15.0, // new quantity
    ).with_price(44500.0); // new price

    client.modify_order(modify_request).await?;
    info!("Modified order: {}", order_id);

    // Wait and cancel if not filled
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    let cancel_request = OrderCancelRequest::new(
        order_id.clone(),
        "BTC-PERPETUAL".to_string(),
    );

    client.cancel_order(cancel_request).await?;
    info!("Cancelled order: {}", order_id);

    Ok(())
}
```

### **2. Bulk Order Operations**

```rust
use deribit_fix::client::DeribitFixClient;
use deribit_fix::config::DeribitFixConfig;
use deribit_fix::message::orders::{NewOrderSingle, MassOrderCancelRequest};
use deribit_fix::message::orders::{OrderSide, OrderType, TimeInForce};
use std::collections::HashMap;
use log::{info, warn};

struct OrderManager {
    client: DeribitFixClient,
    active_orders: HashMap<String, String>, // order_id -> instrument
}

impl OrderManager {
    async fn new(config: DeribitFixConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let client = DeribitFixClient::new(config).await?;
        Ok(Self {
            client,
            active_orders: HashMap::new(),
        })
    }

    async fn place_bracket_orders(&mut self, instrument: &str, base_price: f64, quantity: f64) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut order_ids = Vec::new();

        // Place buy order below market
        let buy_order = NewOrderSingle::new(
            format!("buy_{}", chrono::Utc::now().timestamp()),
            instrument.to_string(),
            OrderSide::Buy,
            OrderType::Limit,
            TimeInForce::GoodTillCancelled,
            quantity,
        ).with_price(base_price * 0.98);

        let buy_order_id = self.client.place_order(buy_order).await?;
        self.active_orders.insert(buy_order_id.clone(), instrument.to_string());
        order_ids.push(buy_order_id.clone());
        info!("Placed buy order: {} at {}", buy_order_id, base_price * 0.98);

        // Place sell order above market
        let sell_order = NewOrderSingle::new(
            format!("sell_{}", chrono::Utc::now().timestamp()),
            instrument.to_string(),
            OrderSide::Sell,
            OrderType::Limit,
            TimeInForce::GoodTillCancelled,
            quantity,
        ).with_price(base_price * 1.02);

        let sell_order_id = self.client.place_order(sell_order).await?;
        self.active_orders.insert(sell_order_id.clone(), instrument.to_string());
        order_ids.push(sell_order_id.clone());
        info!("Placed sell order: {} at {}", sell_order_id, base_price * 1.02);

        Ok(order_ids)
    }

    async fn cancel_all_orders(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.active_orders.is_empty() {
            info!("No active orders to cancel");
            return Ok(());
        }

        let cancel_request = MassOrderCancelRequest::new(
            format!("mass_cancel_{}", chrono::Utc::now().timestamp()),
        );

        match self.client.mass_cancel_orders(cancel_request).await {
            Ok(_) => {
                info!("Mass cancelled {} orders", self.active_orders.len());
                self.active_orders.clear();
            }
            Err(e) => {
                warn!("Mass cancel failed: {}, attempting individual cancels", e);
                // Fallback to individual cancellations
                for (order_id, instrument) in &self.active_orders {
                    let cancel_request = OrderCancelRequest::new(
                        order_id.clone(),
                        instrument.clone(),
                    );
                    if let Err(e) = self.client.cancel_order(cancel_request).await {
                        warn!("Failed to cancel order {}: {}", order_id, e);
                    }
                }
                self.active_orders.clear();
            }
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let config = DeribitFixConfig::new()
        .with_credentials("your_username".to_string(), "your_password".to_string())
        .with_session_ids("YOUR_SENDER_ID".to_string(), "DERIBITSERVER".to_string());

    let mut order_manager = OrderManager::new(config).await?;
    order_manager.client.connect().await?;

    // Place bracket orders around current price
    let order_ids = order_manager.place_bracket_orders("BTC-PERPETUAL", 50000.0, 5.0).await?;
    info!("Placed bracket orders: {:?}", order_ids);

    // Wait for some time
    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

    // Cancel all orders
    order_manager.cancel_all_orders().await?;

    Ok(())
}
```

## Market Data Snapshot with Additional Fields

```rust
use deribit_fix::client::DeribitFixClient;
use deribit_fix::config::DeribitFixConfig;
use deribit_fix::message::market_data::{MarketDataRequest, MdEntryType, MdSubscriptionRequestType};
use deribit_fix::message::market_data::MarketDataSnapshotFullRefresh;
use log::{info, warn};
use std::collections::HashMap;

struct SnapshotHandler {
    client: DeribitFixClient,
    subscriptions: HashMap<String, bool>,
}

impl SnapshotHandler {
    fn new(client: DeribitFixClient) -> Self {
        Self {
            client,
            subscriptions: HashMap::new(),
        }
    }

    async fn subscribe_and_process_snapshots(&mut self, instrument: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Subscribe to market data snapshot
        let md_request = MarketDataRequest::new(
            format!("md_req_{}", instrument),
            MdSubscriptionRequestType::SnapshotAndUpdates,
            vec![instrument.to_string()],
            vec![
                MdEntryType::Bid,
                MdEntryType::Offer,
                MdEntryType::Trade,
                MdEntryType::OpenInterest,
                MdEntryType::MarkPrice,
                MdEntryType::UnderlyingPrice,
                MdEntryType::Funding,
            ],
        );

        self.client.send_market_data_request(md_request).await?;
        self.subscriptions.insert(instrument.to_string(), true);
        info!("Subscribed to market data for {}", instrument);

        // Process incoming snapshots
        while let Some(snapshot) = self.client.receive_market_data_snapshot().await? {
            self.process_snapshot(snapshot).await?;
        }

        Ok(())
    }

    async fn process_snapshot(&self, snapshot: MarketDataSnapshotFullRefresh) -> Result<(), Box<dyn std::error::Error>> {
        info!("Processing snapshot for {}", snapshot.symbol);

        // Extract additional fields
        let mut underlying_px = None;
        let mut mark_price = None;
        let mut open_interest = None;
        let mut funding_rate = None;

        for entry in &snapshot.md_entries {
            match entry.md_entry_type {
                MdEntryType::UnderlyingPrice => underlying_px = Some(entry.md_entry_px),
                MdEntryType::MarkPrice => mark_price = Some(entry.md_entry_px),
                MdEntryType::OpenInterest => open_interest = Some(entry.md_entry_size),
                MdEntryType::Funding => funding_rate = Some(entry.md_entry_px),
                _ => {}
            }
        }

        // Log detailed market data
        info!("Market Data Snapshot:");
        info!("  Symbol: {}", snapshot.symbol);
        if let Some(price) = underlying_px {
            info!("  Underlying Price: {}", price);
        }
        if let Some(price) = mark_price {
            info!("  Mark Price: {}", price);
        }
        if let Some(oi) = open_interest {
            info!("  Open Interest: {}", oi);
        }
        if let Some(funding) = funding_rate {
            info!("  Current Funding: {:.6}%", funding * 100.0);
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let config = DeribitFixConfig::new()
        .with_credentials("your_username".to_string(), "your_password".to_string())
        .with_session_ids("YOUR_SENDER_ID".to_string(), "DERIBITSERVER".to_string());

    let client = DeribitFixClient::new(config).await?;
    let mut handler = SnapshotHandler::new(client);
    
    // Subscribe to market data snapshots for BTC-PERPETUAL
    handler.subscribe_and_process_snapshots("BTC-PERPETUAL").await?;

    Ok(())
}
```

## Market Data Snapshot and Analysis

```rust
use deribit_fix::client::DeribitFixClient;
use deribit_fix::config::DeribitFixConfig;
use deribit_fix::message::market_data::MarketDataSnapshotFullRefresh;
use log::{info, warn};
use std::collections::VecDeque;

struct OrderBookAnalyzer {
    bid_history: VecDeque<f64>,
    ask_history: VecDeque<f64>,
    max_history: usize,
}

impl OrderBookAnalyzer {
    fn new(max_history: usize) -> Self {
        Self {
            bid_history: VecDeque::with_capacity(max_history),
            ask_history: VecDeque::with_capacity(max_history),
            max_history,
        }
    }

    fn update(&mut self, snapshot: &MarketDataSnapshotFullRefresh) {
        let mut best_bid = None;
        let mut best_ask = None;

        for entry in &snapshot.md_entries {
            match entry.md_entry_type {
                MdEntryType::Bid => {
                    if best_bid.is_none() || entry.md_entry_px > best_bid.unwrap() {
                        best_bid = Some(entry.md_entry_px);
                    }
                }
                MdEntryType::Offer => {
                    if best_ask.is_none() || entry.md_entry_px < best_ask.unwrap() {
                        best_ask = Some(entry.md_entry_px);
                    }
                }
                _ => {}
            }
        }

        if let Some(bid) = best_bid {
            if self.bid_history.len() >= self.max_history {
                self.bid_history.pop_front();
            }
            self.bid_history.push_back(bid);
        }

        if let Some(ask) = best_ask {
            if self.ask_history.len() >= self.max_history {
                self.ask_history.pop_front();
            }
            self.ask_history.push_back(ask);
        }
    }

    fn calculate_spread_statistics(&self) -> Option<(f64, f64, f64)> {
        if self.bid_history.is_empty() || self.ask_history.is_empty() {
            return None;
        }

        let current_bid = *self.bid_history.back().unwrap();
        let current_ask = *self.ask_history.back().unwrap();
        let current_spread = current_ask - current_bid;

        let spreads: Vec<f64> = self.bid_history.iter()
            .zip(self.ask_history.iter())
            .map(|(bid, ask)| ask - bid)
            .collect();

        let avg_spread = spreads.iter().sum::<f64>() / spreads.len() as f64;
        let mid_price = (current_bid + current_ask) / 2.0;

        Some((current_spread, avg_spread, mid_price))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let config = DeribitFixConfig::new()
        .with_credentials("your_username".to_string(), "your_password".to_string())
        .with_session_ids("YOUR_SENDER_ID".to_string(), "DERIBITSERVER".to_string());

    let client = DeribitFixClient::new(config).await?;
    client.connect().await?;

    let mut analyzer = OrderBookAnalyzer::new(100);

    // Process market data snapshots
    while let Some(snapshot) = client.receive_market_data_snapshot().await? {
        analyzer.update(&snapshot);

        if let Some((current_spread, avg_spread, mid_price)) = analyzer.calculate_spread_statistics() {
            info!("Market Analysis for {}:", snapshot.symbol);
            info!("  Mid Price: {:.2}", mid_price);
            info!("  Current Spread: {:.2}", current_spread);
            info!("  Average Spread: {:.2}", avg_spread);
            info!("  Spread as % of mid: {:.4}%", (current_spread / mid_price) * 100.0);
        }
    }

    Ok(())
}
```

## Position Management

### **Position Report Parsing and Exposure**

```rust
use deribit_fix::client::DeribitFixClient;
use deribit_fix::config::DeribitFixConfig;
use deribit_fix::message::positions::{RequestForPositions, PositionReport};
use log::{info, warn};
use std::collections::HashMap;

struct PositionSnapshot {
    client: DeribitFixClient,
    positions: HashMap<String, PositionData>,
}

#[derive(Debug, Clone)]
struct PositionData {
    instrument: String,
    size: f64,
    side: String,
    avg_price: f64,
    unrealized_pnl: f64,
    realized_pnl: f64,
    margin: f64,
    leverage: f64,
    liquidation_price: Option<f64>,
    mark_price: f64,
    index_price: f64,
    timestamp: u64,
}

impl PositionSnapshot {
    fn new(client: DeribitFixClient) -> Self {
        Self {
            client,
            positions: HashMap::new(),
        }
    }

    async fn refresh_positions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let pos_request = RequestForPositions::new(
            format!("pos_req_{}", chrono::Utc::now().timestamp()),
        );

        self.client.send_position_request(pos_request).await?;

        // Process position reports
        while let Some(position_report) = self.client.receive_position_report().await? {
            self.update_position(position_report);
        }

        Ok(())
    }

    fn update_position(&mut self, report: PositionReport) {
        let position_data = PositionData {
            instrument: report.instrument.clone(),
            size: report.long_qty.unwrap_or(0.0) - report.short_qty.unwrap_or(0.0),
            side: if report.long_qty.unwrap_or(0.0) > report.short_qty.unwrap_or(0.0) {
                "Long".to_string()
            } else {
                "Short".to_string()
            },
            avg_price: report.average_price.unwrap_or(0.0),
            unrealized_pnl: report.unrealized_pnl.unwrap_or(0.0),
            realized_pnl: report.realized_pnl.unwrap_or(0.0),
            margin: report.margin_used.unwrap_or(0.0),
            leverage: report.leverage.unwrap_or(1.0),
            liquidation_price: report.liquidation_price,
            mark_price: report.mark_price.unwrap_or(0.0),
            index_price: report.index_price.unwrap_or(0.0),
            timestamp: chrono::Utc::now().timestamp() as u64,
        };

        self.positions.insert(report.instrument, position_data);
    }

    fn calculate_total_exposure(&self) -> (f64, f64, f64) {
        let mut total_unrealized_pnl = 0.0;
        let mut total_realized_pnl = 0.0;
        let mut total_margin = 0.0;

        for position in self.positions.values() {
            total_unrealized_pnl += position.unrealized_pnl;
            total_realized_pnl += position.realized_pnl;
            total_margin += position.margin;
        }

        (total_unrealized_pnl, total_realized_pnl, total_margin)
    }

    fn log_positions(&self) {
        info!("=== Position Summary ===");
        for (instrument, position) in &self.positions {
            info!("Position in {}:", instrument);
            info!("  Size: {} ({})", position.size, position.side);
            info!("  Avg Price: {:.2}", position.avg_price);
            info!("  Mark Price: {:.2}", position.mark_price);
            info!("  Unrealized PnL: {:.4}", position.unrealized_pnl);
            info!("  Realized PnL: {:.4}", position.realized_pnl);
            info!("  Margin Used: {:.4}", position.margin);
            info!("  Leverage: {:.1}x", position.leverage);
            if let Some(liq_price) = position.liquidation_price {
                info!("  Liquidation Price: {:.2}", liq_price);
            }
        }

        let (total_upnl, total_rpnl, total_margin) = self.calculate_total_exposure();
        info!("=== Total Exposure ===");
        info!("  Total Unrealized PnL: {:.4}", total_upnl);
        info!("  Total Realized PnL: {:.4}", total_rpnl);
        info!("  Total Margin Used: {:.4}", total_margin);
    }
}

struct PositionManager {
    snapshot: PositionSnapshot,
    risk_limits: RiskLimits,
}

#[derive(Debug)]
struct RiskLimits {
    max_total_exposure: f64,
    max_single_position: f64,
    max_leverage: f64,
    max_drawdown: f64,
}

impl PositionManager {
    fn new(client: DeribitFixClient) -> Self {
        Self {
            snapshot: PositionSnapshot::new(client),
            risk_limits: RiskLimits {
                max_total_exposure: 10000.0,
                max_single_position: 5000.0,
                max_leverage: 10.0,
                max_drawdown: -2000.0,
            },
        }
    }

    async fn monitor_positions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            self.snapshot.refresh_positions().await?;
            self.snapshot.log_positions();
            
            if self.check_risk_limits() {
                warn!("Risk limits breached!");
                // Implement risk management actions here
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
        }
    }

    fn check_risk_limits(&self) -> bool {
        let (total_upnl, _, total_margin) = self.snapshot.calculate_total_exposure();
        
        // Check total exposure
        if total_margin > self.risk_limits.max_total_exposure {
            warn!("Total exposure limit breached: {:.2} > {:.2}", 
                  total_margin, self.risk_limits.max_total_exposure);
            return true;
        }

        // Check drawdown limit
        if total_upnl < self.risk_limits.max_drawdown {
            warn!("Drawdown limit breached: {:.2} < {:.2}", 
                  total_upnl, self.risk_limits.max_drawdown);
            return true;
        }

        // Check individual position limits
        for position in self.snapshot.positions.values() {
            if position.margin > self.risk_limits.max_single_position {
                warn!("Single position limit breached for {}: {:.2} > {:.2}", 
                      position.instrument, position.margin, self.risk_limits.max_single_position);
                return true;
            }

            if position.leverage > self.risk_limits.max_leverage {
                warn!("Leverage limit breached for {}: {:.1}x > {:.1}x", 
                      position.instrument, position.leverage, self.risk_limits.max_leverage);
                return true;
            }
        }

        false
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let config = DeribitFixConfig::new()
        .with_credentials("your_username".to_string(), "your_password".to_string())
        .with_session_ids("YOUR_SENDER_ID".to_string(), "DERIBITSERVER".to_string());

    let client = DeribitFixClient::new(config).await?;
    client.connect().await?;

    let mut position_manager = PositionManager::new(client);
    position_manager.monitor_positions().await?;

    Ok(())
}
```

## Best Practices for Advanced Usage

### **Error Handling and Recovery**

```rust
use deribit_fix::error::DeribitFixError;
use log::{error, warn, info};

async fn robust_operation_with_retry<F, Fut, T>(
    operation: F,
    max_retries: usize,
    delay_ms: u64,
) -> Result<T, DeribitFixError>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, DeribitFixError>>,
{
    for attempt in 1..=max_retries {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                warn!("Attempt {} failed: {}", attempt, e);
                if attempt < max_retries {
                    tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
                } else {
                    error!("All {} attempts failed", max_retries);
                    return Err(e);
                }
            }
        }
    }
    unreachable!()
}
```

### **Concurrent Operations**

```rust
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

async fn concurrent_market_monitoring() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, mut rx) = mpsc::channel(100);
    
    // Spawn market data handler
    let market_data_handle: JoinHandle<Result<(), Box<dyn std::error::Error>>> = tokio::spawn({
        let tx = tx.clone();
        async move {
            // Market data processing logic
            // Send updates via tx.send(update).await
            Ok(())
        }
    });
    
    // Spawn order management handler
    let order_handle: JoinHandle<Result<(), Box<dyn std::error::Error>>> = tokio::spawn({
        async move {
            // Order management logic
            Ok(())
        }
    });
    
    // Main event loop
    while let Some(update) = rx.recv().await {
        // Process market updates and make trading decisions
        info!("Received market update: {:?}", update);
    }
    
    // Wait for all tasks to complete
    let _ = tokio::try_join!(market_data_handle, order_handle)?;
    
    Ok(())
}
```

These advanced examples demonstrate sophisticated usage patterns for professional trading applications, including proper error handling, concurrent operations, and comprehensive risk management.
