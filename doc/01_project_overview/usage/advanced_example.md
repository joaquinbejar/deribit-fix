# Advanced Usage Examples

## Overview

This guide provides advanced examples for complex trading operations with the Deribit FIX Client. These examples cover order management, market data subscriptions, position management, and sophisticated trading strategies.

## Prerequisites

Before running these examples, ensure you have:

1. **Basic Knowledge**: Understanding of the [Basic Examples](basic_example.md)
2. **API Credentials**: Production-ready Deribit API credentials
3. **Risk Management**: Proper risk controls and position limits
4. **Testing Environment**: Thorough testing on testnet before production

## Order Management Examples

### **1. Basic Order Placement**

```rust
use deribit_fix::client::FixClient;
use deribit_fix::config::FixConfig;
use deribit_fix::message::orders::{
    NewOrderSingle, OrderSide, OrderType, TimeInForce, OrderCapacity
};
use log::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let config = FixConfig::new()
        .with_sender_comp_id("YOUR_SENDER_ID".to_string())
        .with_target_comp_id("DERIBIT".to_string())
        .with_api_key("YOUR_API_KEY".to_string())
        .with_secret_key("YOUR_SECRET_KEY".to_string());

    let mut client = FixClient::new(config);
    client.connect().await?;
    
    // Create a market buy order for BTC-PERPETUAL
    let market_order = NewOrderSingle::new(
        "BTC-PERPETUAL".to_string(),
        OrderSide::Buy,
        OrderType::Market,
        TimeInForce::ImmediateOrCancel,
        0.1, // quantity in BTC
    )
    .with_order_capacity(OrderCapacity::Agency)
    .with_text("Market buy order example".to_string());
    
    // Place the order
    match client.place_order(market_order).await {
        Ok(order_id) => {
            info!("Market order placed successfully with ID: {}", order_id);
        }
        Err(e) => {
            error!("Failed to place market order: {}", e);
        }
    }
    
    Ok(())
}
```

### **2. Limit Order with Price Management**

```rust
use deribit_fix::client::FixClient;
use deribit_fix::config::FixConfig;
use deribit_fix::message::orders::{
    NewOrderSingle, OrderSide, OrderType, TimeInForce, OrderCapacity
};
use log::{info, warn, error};

struct LimitOrderManager {
    client: FixClient,
    min_spread: f64,
    max_quantity: f64,
}

impl LimitOrderManager {
    fn new(config: FixConfig) -> Self {
        Self {
            client: FixClient::new(config),
            min_spread: 0.5, // 0.5% minimum spread
            max_quantity: 1.0, // Maximum 1 BTC per order
        }
    }
    
    async fn place_limit_order(
        &mut self,
        symbol: String,
        side: OrderSide,
        quantity: f64,
        price: f64,
        current_market_price: f64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Validate quantity
        if quantity > self.max_quantity {
            return Err("Quantity exceeds maximum allowed".into());
        }
        
        // Validate price based on side
        let spread = match side {
            OrderSide::Buy => (current_market_price - price) / current_market_price * 100.0,
            OrderSide::Sell => (price - current_market_price) / current_market_price * 100.0,
        };
        
        if spread < self.min_spread {
            warn!("Spread {}% is below minimum {}%", spread, self.min_spread);
        }
        
        // Create and place the order
        let order = NewOrderSingle::new(
            symbol,
            side,
            OrderType::Limit,
            TimeInForce::GoodTillCancelled,
            quantity,
        )
        .with_price(price)
        .with_order_capacity(OrderCapacity::Agency)
        .with_text(format!("Limit order with {}% spread", spread));
        
        let order_id = self.client.place_order(order).await?;
        info!("Limit order placed: {} {} {} @ {}", side, quantity, symbol, price);
        
        Ok(order_id)
    }
    
    async fn cancel_order(&mut self, order_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.client.cancel_order(order_id).await?;
        info!("Order {} cancelled successfully", order_id);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let config = FixConfig::new()
        .with_sender_comp_id("YOUR_SENDER_ID".to_string())
        .with_target_comp_id("DERIBIT".to_string())
        .with_api_key("YOUR_API_KEY".to_string())
        .with_secret_key("YOUR_SECRET_KEY".to_string());

    let mut manager = LimitOrderManager::new(config);
    manager.client.connect().await?;
    
    // Place a limit buy order
    let order_id = manager.place_limit_order(
        "BTC-PERPETUAL".to_string(),
        OrderSide::Buy,
        0.1,
        45000.0, // Limit price
        46000.0,  // Current market price
    ).await?;
    
    // Wait a bit and then cancel the order
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    manager.cancel_order(&order_id).await?;
    
    Ok(())
}
```

### **3. Order Modification and Replacement**

```rust
use deribit_fix::client::FixClient;
use deribit_fix::config::FixConfig;
use deribit_fix::message::orders::{
    NewOrderSingle, OrderCancelReplaceRequest, OrderSide, OrderType, TimeInForce
};
use log::{info, warn, error};

struct OrderModifier {
    client: FixClient,
}

impl OrderModifier {
    fn new(config: FixConfig) -> Self {
        Self {
            client: FixClient::new(config),
        }
    }
    
    async fn modify_order(
        &mut self,
        original_order_id: &str,
        new_quantity: Option<f64>,
        new_price: Option<f64>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Create modification request
        let mut modify_request = OrderCancelReplaceRequest::new(
            original_order_id.to_string(),
            "BTC-PERPETUAL".to_string(),
            OrderSide::Buy,
            OrderType::Limit,
            TimeInForce::GoodTillCancelled,
        );
        
        if let Some(quantity) = new_quantity {
            modify_request = modify_request.with_order_qty(quantity);
        }
        
        if let Some(price) = new_price {
            modify_request = modify_request.with_price(price);
        }
        
        // Send modification request
        let new_order_id = self.client.modify_order(modify_request).await?;
        info!("Order {} modified successfully, new ID: {}", original_order_id, new_order_id);
        
        Ok(new_order_id)
    }
    
    async fn place_and_modify_example(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Place initial order
        let initial_order = NewOrderSingle::new(
            "BTC-PERPETUAL".to_string(),
            OrderSide::Buy,
            OrderType::Limit,
            TimeInForce::GoodTillCancelled,
            0.1,
        )
        .with_price(45000.0);
        
        let order_id = self.client.place_order(initial_order).await?;
        info!("Initial order placed with ID: {}", order_id);
        
        // Wait a bit
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        
        // Modify the order
        let new_order_id = self.modify_order(
            &order_id,
            Some(0.2),    // Double the quantity
            Some(44000.0), // Lower the price
        ).await?;
        
        info!("Order modified: {} -> {}", order_id, new_order_id);
        
        // Cancel the modified order
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        self.client.cancel_order(&new_order_id).await?;
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let config = FixConfig::new()
        .with_sender_comp_id("YOUR_SENDER_ID".to_string())
        .with_target_comp_id("DERIBIT".to_string())
        .with_api_key("YOUR_API_KEY".to_string())
        .with_secret_key("YOUR_SECRET_KEY".to_string());

    let mut modifier = OrderModifier::new(config);
    modifier.client.connect().await?;
    
    modifier.place_and_modify_example().await?;
    
    Ok(())
}
```

## Market Data Examples

### **1. Real-Time Market Data Subscription**

```rust
use deribit_fix::client::FixClient;
use deribit_fix::config::FixConfig;
use deribit_fix::message::market_data::{
    MarketDataRequest, MdEntryType, MdSubscriptionRequestType, MarketDataIncrementalRefresh
};
use log::{info, warn, error};
use tokio::sync::mpsc;

struct MarketDataSubscriber {
    client: FixClient,
    data_receiver: mpsc::Receiver<MarketDataIncrementalRefresh>,
}

impl MarketDataSubscriber {
    fn new(config: FixConfig) -> (Self, mpsc::Sender<MarketDataIncrementalRefresh>) {
        let (tx, rx) = mpsc::channel(1000);
        
        (Self {
            client: FixClient::new(config),
            data_receiver: rx,
        }, tx)
    }
    
    async fn subscribe_to_orderbook(
        &mut self,
        symbol: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let md_request = MarketDataRequest::subscription(
            "md_req_001".to_string(),
            vec![symbol.to_string()],
            vec![
                MdEntryType::Bid,
                MdEntryType::Offer,
                MdEntryType::Trade,
            ],
            MdSubscriptionRequestType::SnapshotAndUpdates,
        );
        
        self.client.send_market_data_request(md_request).await?;
        info!("Subscribed to market data for {}", symbol);
        
        Ok(())
    }
    
    async fn process_market_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while let Some(md_update) = self.data_receiver.recv().await {
            match md_update.md_entry_type {
                MdEntryType::Bid => {
                    info!("Bid update: {} @ {}", md_update.md_entry_size, md_update.md_entry_px);
                }
                MdEntryType::Offer => {
                    info!("Offer update: {} @ {}", md_update.md_entry_size, md_update.md_entry_px);
                }
                MdEntryType::Trade => {
                    info!("Trade: {} @ {}", md_update.md_entry_size, md_update.md_entry_px);
                }
                _ => {
                    warn!("Unknown market data entry type: {:?}", md_update.md_entry_type);
                }
            }
        }
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let config = FixConfig::new()
        .with_sender_comp_id("YOUR_SENDER_ID".to_string())
        .with_target_comp_id("DERIBIT".to_string())
        .with_api_key("YOUR_API_KEY".to_string())
        .with_secret_key("YOUR_SECRET_KEY".to_string());

    let (mut subscriber, data_sender) = MarketDataSubscriber::new(config);
    subscriber.client.connect().await?;
    
    // Subscribe to BTC-PERPETUAL market data
    subscriber.subscribe_to_orderbook("BTC-PERPETUAL").await?;
    
    // Process market data for 30 seconds
    let timeout = tokio::time::sleep(tokio::time::Duration::from_secs(30));
    tokio::select! {
        _ = subscriber.process_market_data() => {
            info!("Market data processing completed");
        }
        _ = timeout => {
            info!("Market data subscription timeout");
        }
    }
    
    Ok(())
}
```

### **2. Market Data Snapshot with Additional Fields**

```rust
use deribit_fix::{DeribitFixClient, DeribitFixConfig};
use deribit_fix::model::message::FixMessage;
use log::info;
use tokio::time::{sleep, Duration};

struct SnapshotHandler {
    client: DeribitFixClient,
}

impl SnapshotHandler {
    fn new(client: DeribitFixClient) -> Self {
        Self { client }
    }
    
    async fn subscribe_and_process_snapshots(
        &mut self,
        symbol: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Connect to Deribit
        self.client.connect().await?;
        
        // Subscribe to market data
        self.client.subscribe_market_data(symbol.to_string()).await?;
        info!("Subscribed to market data for {}", symbol);
        
        // Process incoming messages
        for _ in 0..50 {  // Process 50 messages maximum
            match self.client.receive_message().await? {
                Some(message) => {
                    if self.is_market_data_snapshot(&message) {
                        self.process_snapshot(&message, symbol).await?;
                    }
                }
                None => {
                    sleep(Duration::from_millis(100)).await;
                }
            }
        }
        
        Ok(())
    }
    
    fn is_market_data_snapshot(&self, message: &FixMessage) -> bool {
        if let Some(msg_type) = message.get_field(35) {
            msg_type == "W" // MarketDataSnapshotFullRefresh
        } else {
            false
        }
    }
    
    async fn process_snapshot(
        &self,
        message: &FixMessage,
        symbol: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Extract key fields from the snapshot
        let underlying_px = message.get_field(810)
            .and_then(|s| s.parse::<f64>().ok());
        
        let mark_price = message.get_field(100090)
            .and_then(|s| s.parse::<f64>().ok());
        
        let current_funding = message.get_field(100092)
            .and_then(|s| s.parse::<f64>().ok());
        
        let funding_8h = message.get_field(100093)
            .and_then(|s| s.parse::<f64>().ok());
        
        let open_interest = message.get_field(746)
            .and_then(|s| s.parse::<f64>().ok());
        
        let trade_volume_24h = message.get_field(100087)
            .and_then(|s| s.parse::<f64>().ok());
        
        info!("=== Market Data Snapshot for {} ===", symbol);
        
        if let Some(price) = underlying_px {
            info!("Underlying Price: ${:.2}", price);
        }
        
        if let Some(mark) = mark_price {
            info!("Mark Price: ${:.2}", mark);
            
            // Calculate premium/discount
            if let Some(underlying) = underlying_px {
                let premium = ((mark - underlying) / underlying) * 100.0;
                info!("Premium/Discount: {:.2}%", premium);
            }
        }
        
        if let Some(funding) = current_funding {
            info!("Current Funding Rate: {:.6}%", funding * 100.0);
        }
        
        if let Some(funding_8h) = funding_8h {
            info!("8h Funding Rate: {:.6}%", funding_8h * 100.0);
        }
        
        if let Some(oi) = open_interest {
            info!("Open Interest: {:.0} contracts", oi);
        }
        
        if let Some(volume) = trade_volume_24h {
            info!("24h Volume: {:.2} BTC", volume);
        }
        
        info!("=====================================");
        
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
    
    // Process snapshots for BTC-PERPETUAL
    handler.subscribe_and_process_snapshots("BTC-PERPETUAL").await?;
    
    Ok(())
}
```

### **3. Market Data Snapshot and Analysis**

```rust
use deribit_fix::client::FixClient;
use deribit_fix::config::FixConfig;
use deribit_fix::message::market_data::{
    MarketDataRequest, MdEntryType, MarketDataSnapshotFullRefresh
};
use log::{info, warn};
use std::collections::HashMap;

struct OrderBookAnalyzer {
    client: FixClient,
    bids: HashMap<f64, f64>, // price -> quantity
    offers: HashMap<f64, f64>, // price -> quantity
}

impl OrderBookAnalyzer {
    fn new(config: FixConfig) -> Self {
        Self {
            client: FixClient::new(config),
            bids: HashMap::new(),
            offers: HashMap::new(),
        }
    }
    
    async fn get_orderbook_snapshot(
        &mut self,
        symbol: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let md_request = MarketDataRequest::snapshot(
            "md_snapshot_001".to_string(),
            vec![symbol.to_string()],
            vec![MdEntryType::Bid, MdEntryType::Offer],
        );
        
        self.client.send_market_data_request(md_request).await?;
        info!("Requested orderbook snapshot for {}", symbol);
        
        Ok(())
    }
    
    fn update_orderbook(&mut self, entry_type: MdEntryType, price: f64, quantity: f64) {
        match entry_type {
            MdEntryType::Bid => {
                if quantity > 0.0 {
                    self.bids.insert(price, quantity);
                } else {
                    self.bids.remove(&price);
                }
            }
            MdEntryType::Offer => {
                if quantity > 0.0 {
                    self.offers.insert(price, quantity);
                } else {
                    self.offers.remove(&price);
                }
            }
            _ => {}
        }
    }
    
    fn analyze_orderbook(&self) -> OrderBookAnalysis {
        let best_bid = self.bids.keys().max().copied().unwrap_or(0.0);
        let best_offer = self.offers.keys().min().copied().unwrap_or(f64::MAX);
        let spread = best_offer - best_bid;
        let spread_percentage = if best_bid > 0.0 { (spread / best_bid) * 100.0 } else { 0.0 };
        
        let total_bid_volume: f64 = self.bids.values().sum();
        let total_offer_volume: f64 = self.offers.values().sum();
        
        OrderBookAnalysis {
            best_bid,
            best_offer,
            spread,
            spread_percentage,
            total_bid_volume,
            total_offer_volume,
            bid_levels: self.bids.len(),
            offer_levels: self.offers.len(),
        }
    }
}

#[derive(Debug)]
struct OrderBookAnalysis {
    best_bid: f64,
    best_offer: f64,
    spread: f64,
    spread_percentage: f64,
    total_bid_volume: f64,
    total_offer_volume: f64,
    bid_levels: usize,
    offer_levels: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let config = FixConfig::new()
        .with_sender_comp_id("YOUR_SENDER_ID".to_string())
        .with_target_comp_id("DERIBIT".to_string())
        .with_api_key("YOUR_API_KEY".to_string())
        .with_secret_key("YOUR_SECRET_KEY".to_string());

    let mut analyzer = OrderBookAnalyzer::new(config);
    analyzer.client.connect().await?;
    
    // Get orderbook snapshot
    analyzer.get_orderbook_snapshot("BTC-PERPETUAL").await?;
    
    // Wait for data to arrive
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    
    // Analyze the orderbook
    let analysis = analyzer.analyze_orderbook();
    info!("OrderBook Analysis: {:?}", analysis);
    
    Ok(())
}
```

## Position Management Examples

### **1. Position Monitoring and Risk Management**

```rust
use deribit_fix::client::FixClient;
use deribit_fix::config::FixConfig;
use deribit_fix::message::orders::RequestForPositions;
use log::{info, warn, error};
use tokio::time::{sleep, Duration};

struct PositionManager {
    client: FixClient,
    max_position_size: f64,
    max_drawdown: f64,
    positions: HashMap<String, Position>,
}

#[derive(Debug, Clone)]
struct Position {
    symbol: String,
    size: f64,
    avg_price: f64,
    unrealized_pnl: f64,
    realized_pnl: f64,
}

impl PositionManager {
    fn new(config: FixConfig) -> Self {
        Self {
            client: FixClient::new(config),
            max_position_size: 10.0, // Maximum 10 BTC
            max_drawdown: 0.1,       // Maximum 10% drawdown
            positions: HashMap::new(),
        }
    }
    
    async fn request_positions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let pos_request = RequestForPositions::new(
            "pos_req_001".to_string(),
            "BTC-PERPETUAL".to_string(),
        );
        
        self.client.send_position_request(pos_request).await?;
        info!("Position request sent");
        
        Ok(())
    }
    
    fn update_position(&mut self, position: Position) {
        self.positions.insert(position.symbol.clone(), position);
    }
    
    fn check_risk_limits(&self) -> Vec<RiskViolation> {
        let mut violations = Vec::new();
        
        for (symbol, position) in &self.positions {
            // Check position size
            if position.size.abs() > self.max_position_size {
                violations.push(RiskViolation::PositionSizeExceeded {
                    symbol: symbol.clone(),
                    current: position.size.abs(),
                    limit: self.max_position_size,
                });
            }
            
            // Check drawdown
            let total_pnl = position.unrealized_pnl + position.realized_pnl;
            let initial_value = position.avg_price * position.size.abs();
            if initial_value > 0.0 {
                let drawdown = -total_pnl / initial_value;
                if drawdown > self.max_drawdown {
                    violations.push(RiskViolation::DrawdownExceeded {
                        symbol: symbol.clone(),
                        current: drawdown,
                        limit: self.max_drawdown,
                    });
                }
            }
        }
        
        violations
    }
    
    async fn risk_monitoring_loop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut interval = tokio::time::interval(Duration::from_secs(30));
        
        loop {
            interval.tick().await;
            
            // Request updated positions
            self.request_positions().await?;
            
            // Wait for position updates
            sleep(Duration::from_secs(2)).await;
            
            // Check risk limits
            let violations = self.check_risk_limits();
            
            for violation in violations {
                match violation {
                    RiskViolation::PositionSizeExceeded { symbol, current, limit } => {
                        warn!("Position size limit exceeded for {}: {} > {}", symbol, current, limit);

### 2. Position Report Parsing and Exposure

```rust
use deribit_fix::{DeribitFixClient, DeribitFixConfig};
use deribit_fix::message::PositionReport;
use deribit_fix::model::message::FixMessage;
use log::{info, warn};
use tokio::time::{sleep, Duration};

struct PositionSnapshot {
    client: DeribitFixClient,
}

impl PositionSnapshot {
    fn new(config: DeribitFixConfig) -> Self {
        Self {
            client: DeribitFixClient::new(config).expect("Failed to create client"),
        }
    }
    
    async fn refresh_positions(&mut self) -> Result<Vec<deribit_base::prelude::Position>, Box<dyn std::error::Error>> {
        self.client.connect().await?;
        let positions = self.client.get_positions().await?;
        Ok(positions)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let config = DeribitFixConfig::new()
        .with_credentials("your_username".to_string(), "your_password".to_string())
        .with_session_ids("YOUR_SENDER_ID".to_string(), "DERIBITSERVER".to_string());

    let mut snapshot = PositionSnapshot::new(config);
    let positions = snapshot.refresh_positions().await?;

    for p in positions {
        info!(
            "Position: {} | qty: {} | avg_px: {} | mark: {} | unrealized: {}",
            p.symbol, p.quantity, p.average_price, p.mark_price, p.unrealized_pnl
        );
    }

    Ok(())
}
```
                        // Implement position reduction logic
                    }
                    RiskViolation::DrawdownExceeded { symbol, current, limit } => {
                        error!("Drawdown limit exceeded for {}: {:.2}% > {:.2}%", symbol, current * 100.0, limit * 100.0);
                        // Implement emergency stop logic
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
enum RiskViolation {
    PositionSizeExceeded { symbol: String, current: f64, limit: f64 },
    DrawdownExceeded { symbol: String, current: f64, limit: f64 },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let config = FixConfig::new()
        .with_sender_comp_id("YOUR_SENDER_ID".to_string())
        .with_target_comp_id("DERIBIT".to_string())
        .with_api_key("YOUR_API_KEY".to_string())
        .with_secret_key("YOUR_SECRET_KEY".to_string());

    let mut manager = PositionManager::new(config);
    manager.client.connect().await?;
    
    // Start risk monitoring
    manager.risk_monitoring_loop().await?;
    
    Ok(())
}
```

## Trading Strategy Examples

### **1. Simple Market Making Strategy**

```rust
use deribit_fix::client::FixClient;
use deribit_fix::config::FixConfig;
use deribit_fix::message::orders::{
    NewOrderSingle, OrderSide, OrderType, TimeInForce
};
use log::{info, warn, error};
use tokio::time::{sleep, Duration};
use std::collections::HashMap;

struct MarketMaker {
    client: FixClient,
    symbol: String,
    spread: f64,           // Spread in percentage
    order_size: f64,       // Size per order
    max_orders: usize,     // Maximum active orders per side
    active_orders: HashMap<String, ActiveOrder>,
}

#[derive(Debug, Clone)]
struct ActiveOrder {
    order_id: String,
    side: OrderSide,
    price: f64,
    quantity: f64,
}

impl MarketMaker {
    fn new(config: FixConfig, symbol: String) -> Self {
        Self {
            client: FixClient::new(config),
            symbol,
            spread: 0.001,      // 0.1% spread
            order_size: 0.01,   // 0.01 BTC per order
            max_orders: 3,      // 3 orders per side
            active_orders: HashMap::new(),
        }
    }
    
    async fn get_market_price(&self) -> Result<f64, Box<dyn std::error::Error>> {
        // This would typically come from market data subscription
        // For this example, we'll use a placeholder
        Ok(50000.0)
    }
    
    async fn place_market_making_orders(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let market_price = self.get_market_price().await?;
        let half_spread = self.spread / 2.0;
        
        // Calculate bid and ask prices
        let bid_price = market_price * (1.0 - half_spread);
        let ask_price = market_price * (1.0 + half_spread);
        
        // Place bid orders
        for i in 0..self.max_orders {
            let price = bid_price * (1.0 - (i as f64 * 0.0001)); // Slightly lower prices
            
            let bid_order = NewOrderSingle::new(
                self.symbol.clone(),
                OrderSide::Buy,
                OrderType::Limit,
                TimeInForce::GoodTillCancelled,
                self.order_size,
            )
            .with_price(price)
            .with_text(format!("Market making bid {}", i + 1));
            
            match self.client.place_order(bid_order).await {
                Ok(order_id) => {
                    let active_order = ActiveOrder {
                        order_id: order_id.clone(),
                        side: OrderSide::Buy,
                        price,
                        quantity: self.order_size,
                    };
                    self.active_orders.insert(order_id, active_order);
                    info!("Bid order placed: {} @ {}", self.order_size, price);
                }
                Err(e) => {
                    warn!("Failed to place bid order: {}", e);
                }
            }
        }
        
        // Place ask orders
        for i in 0..self.max_orders {
            let price = ask_price * (1.0 + (i as f64 * 0.0001)); // Slightly higher prices
            
            let ask_order = NewOrderSingle::new(
                self.symbol.clone(),
                OrderSide::Sell,
                OrderType::Limit,
                TimeInForce::GoodTillCancelled,
                self.order_size,
            )
            .with_price(price)
            .with_text(format!("Market making ask {}", i + 1));
            
            match self.client.place_order(ask_order).await {
                Ok(order_id) => {
                    let active_order = ActiveOrder {
                        order_id: order_id.clone(),
                        side: OrderSide::Sell,
                        price,
                        quantity: self.order_size,
                    };
                    self.active_orders.insert(order_id, active_order);
                    info!("Ask order placed: {} @ {}", self.order_size, price);
                }
                Err(e) => {
                    warn!("Failed to place ask order: {}", e);
                }
            }
        }
        
        Ok(())
    }
    
    async fn cancel_all_orders(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for (order_id, _) in &self.active_orders {
            if let Err(e) = self.client.cancel_order(order_id).await {
                warn!("Failed to cancel order {}: {}", order_id, e);
            } else {
                info!("Order {} cancelled successfully", order_id);
            }
        }
        
        self.active_orders.clear();
        Ok(())
    }
    
    async fn run_strategy(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting market making strategy for {}", self.symbol);
        
        // Place initial orders
        self.place_market_making_orders().await?;
        
        // Run strategy loop
        let mut interval = tokio::time::interval(Duration::from_secs(60));
        
        for _ in 0..10 { // Run for 10 minutes
            interval.tick().await;
            
            info!("Refreshing market making orders...");
            
            // Cancel existing orders
            self.cancel_all_orders().await?;
            
            // Place new orders
            self.place_market_making_orders().await?;
        }
        
        // Clean up
        self.cancel_all_orders().await?;
        info!("Market making strategy completed");
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let config = FixConfig::new()
        .with_sender_comp_id("YOUR_SENDER_ID".to_string())
        .with_target_comp_id("DERIBIT".to_string())
        .with_api_key("YOUR_API_KEY".to_string())
        .with_secret_key("YOUR_SECRET_KEY".to_string());

    let mut market_maker = MarketMaker::new(config, "BTC-PERPETUAL".to_string());
    market_maker.client.connect().await?;
    
    // Run the strategy
    market_maker.run_strategy().await?;
    
    Ok(())
}
```

### **2. Mean Reversion Strategy**

```rust
use deribit_fix::client::FixClient;
use deribit_fix::config::FixConfig;
use deribit_fix::message::orders::{
    NewOrderSingle, OrderSide, OrderType, TimeInForce
};
use log::{info, warn, error};
use tokio::time::{sleep, Duration};

struct MeanReversionStrategy {
    client: FixClient,
    symbol: String,
    lookback_period: usize,
    price_history: Vec<f64>,
    entry_threshold: f64,    // Standard deviations from mean
    position_size: f64,
    max_positions: usize,
    active_positions: Vec<Position>,
}

#[derive(Debug, Clone)]
struct Position {
    side: OrderSide,
    entry_price: f64,
    quantity: f64,
    order_id: String,
}

impl MeanReversionStrategy {
    fn new(config: FixConfig, symbol: String) -> Self {
        Self {
            client: FixClient::new(config),
            symbol,
            lookback_period: 100,
            price_history: Vec::new(),
            entry_threshold: 2.0,    // 2 standard deviations
            position_size: 0.01,
            max_positions: 5,
            active_positions: Vec::new(),
        }
    }
    
    fn update_price_history(&mut self, price: f64) {
        self.price_history.push(price);
        if self.price_history.len() > self.lookback_period {
            self.price_history.remove(0);
        }
    }
    
    fn calculate_statistics(&self) -> Option<(f64, f64)> {
        if self.price_history.len() < 20 {
            return None;
        }
        
        let mean = self.price_history.iter().sum::<f64>() / self.price_history.len() as f64;
        let variance = self.price_history.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / self.price_history.len() as f64;
        let std_dev = variance.sqrt();
        
        Some((mean, std_dev))
    }
    
    fn should_enter_long(&self, current_price: f64) -> bool {
        if let Some((mean, std_dev)) = self.calculate_statistics() {
            let z_score = (current_price - mean) / std_dev;
            z_score < -self.entry_threshold
        } else {
            false
        }
    }
    
    fn should_enter_short(&self, current_price: f64) -> bool {
        if let Some((mean, std_dev)) = self.calculate_statistics() {
            let z_score = (current_price - mean) / std_dev;
            z_score > self.entry_threshold
        } else {
            false
        }
    }
    
    async fn enter_long(&mut self, price: f64) -> Result<(), Box<dyn std::error::Error>> {
        if self.active_positions.len() >= self.max_positions {
            warn!("Maximum positions reached, skipping long entry");
            return Ok(());
        }
        
        let order = NewOrderSingle::new(
            self.symbol.clone(),
            OrderSide::Buy,
            OrderType::Limit,
            TimeInForce::GoodTillCancelled,
            self.position_size,
        )
        .with_price(price)
        .with_text("Mean reversion long entry".to_string());
        
        match self.client.place_order(order).await {
            Ok(order_id) => {
                let position = Position {
                    side: OrderSide::Buy,
                    entry_price: price,
                    quantity: self.position_size,
                    order_id,
                };
                self.active_positions.push(position);
                info!("Long position entered at {}", price);
            }
            Err(e) => {
                warn!("Failed to enter long position: {}", e);
            }
        }
        
        Ok(())
    }
    
    async fn enter_short(&mut self, price: f64) -> Result<(), Box<dyn std::error::Error>> {
        if self.active_positions.len() >= self.max_positions {
            warn!("Maximum positions reached, skipping short entry");
            return Ok(());
        }
        
        let order = NewOrderSingle::new(
            self.symbol.clone(),
            OrderSide::Sell,
            OrderType::Limit,
            TimeInForce::GoodTillCancelled,
            self.position_size,
        )
        .with_price(price)
        .with_text("Mean reversion short entry".to_string());
        
        match self.client.place_order(order).await {
            Ok(order_id) => {
                let position = Position {
                    side: OrderSide::Sell,
                    entry_price: price,
                    quantity: self.position_size,
                    order_id,
                };
                self.active_positions.push(position);
                info!("Short position entered at {}", price);
            }
            Err(e) => {
                warn!("Failed to enter short position: {}", e);
            }
        }
        
        Ok(())
    }
    
    async fn run_strategy(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting mean reversion strategy for {}", self.symbol);
        
        let mut interval = tokio::time::interval(Duration::from_secs(10));
        
        for _ in 0..60 { // Run for 10 minutes
            interval.tick().await;
            
            // Simulate getting current price (in real implementation, this would come from market data)
            let current_price = 50000.0 + (rand::random::<f64>() - 0.5) * 1000.0;
            self.update_price_history(current_price);
            
            info!("Current price: {}, Active positions: {}", current_price, self.active_positions.len());
            
            // Check for entry signals
            if self.should_enter_long(current_price) {
                info!("Long entry signal detected");
                self.enter_long(current_price).await?;
            } else if self.should_enter_short(current_price) {
                info!("Short entry signal detected");
                self.enter_short(current_price).await?;
            }
            
            // Simple exit logic: exit when price returns to mean
            if let Some((mean, _)) = self.calculate_statistics() {
                let mean_distance = (current_price - mean).abs() / mean;
                if mean_distance < 0.001 { // Within 0.1% of mean
                    info!("Price near mean, closing positions");
                    self.close_all_positions().await?;
                }
            }
        }
        
        info!("Mean reversion strategy completed");
        Ok(())
    }
    
    async fn close_all_positions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for position in &self.active_positions {
            if let Err(e) = self.client.cancel_order(&position.order_id).await {
                warn!("Failed to cancel order {}: {}", position.order_id, e);
            }
        }
        
        self.active_positions.clear();
        info!("All positions closed");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let config = FixConfig::new()
        .with_sender_comp_id("YOUR_SENDER_ID".to_string())
        .with_target_comp_id("DERIBIT".to_string())
        .with_api_key("YOUR_API_KEY".to_string())
        .with_secret_key("YOUR_SECRET_KEY".to_string());

    let mut strategy = MeanReversionStrategy::new(config, "BTC-PERPETUAL".to_string());
    strategy.client.connect().await?;
    
    // Run the strategy
    strategy.run_strategy().await?;
    
    Ok(())
}
```

## Performance Optimization Examples

### **1. Connection Pooling for High-Frequency Trading**

```rust
use deribit_fix::client::FixClient;
use deribit_fix::config::FixConfig;
use std::sync::Arc;
use tokio::sync::Mutex;
use log::info;

struct ConnectionPool {
    connections: Vec<Arc<Mutex<FixClient>>>,
    current_index: usize,
    max_connections: usize,
}

impl ConnectionPool {
    fn new(config: FixConfig, max_connections: usize) -> Self {
        let mut connections = Vec::new();
        
        for i in 0..max_connections {
            let mut config = config.clone();
            config = config.with_sender_comp_id(format!("{}_{}", config.sender_comp_id, i));
            
            let client = FixClient::new(config);
            connections.push(Arc::new(Mutex::new(client)));
        }
        
        Self {
            connections,
            current_index: 0,
            max_connections,
        }
    }
    
    async fn initialize_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut handles = Vec::new();
        
        for connection in &self.connections {
            let connection = Arc::clone(connection);
            let handle = tokio::spawn(async move {
                let mut client = connection.lock().await;
                client.connect().await
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.await??;
        }
        
        info!("All {} connections initialized successfully", self.max_connections);
        Ok(())
    }
    
    async fn get_connection(&mut self) -> Arc<Mutex<FixClient>> {
        let connection = Arc::clone(&self.connections[self.current_index]);
        self.current_index = (self.current_index + 1) % self.max_connections;
        connection
    }
    
    async fn execute_with_connection<F, Fut, T>(
        &mut self,
        operation: F,
    ) -> Result<T, Box<dyn std::error::Error>>
    where
        F: FnOnce(&mut FixClient) -> Fut,
        Fut: std::future::Future<Output = Result<T, Box<dyn std::error::Error>>>,
    {
        let connection = self.get_connection().await;
        let mut client = connection.lock().await;
        
        if !client.is_connected().await? {
            warn!("Connection lost, reconnecting...");
            client.reconnect().await?;
        }
        
        operation(&mut *client).await
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let config = FixConfig::new()
        .with_sender_comp_id("YOUR_SENDER_ID".to_string())
        .with_target_comp_id("DERIBIT".to_string())
        .with_api_key("YOUR_API_KEY".to_string())
        .with_secret_key("YOUR_SECRET_KEY".to_string());

    let mut pool = ConnectionPool::new(config, 5);
    pool.initialize_all().await?;
    
    // Example: Execute operations using the pool
    for i in 0..10 {
        let result = pool.execute_with_connection(|client| async move {
            // Your trading operation here
            info!("Executing operation {} on connection", i);
            Ok(())
        }).await?;
        
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    
    Ok(())
}
```

## Next Steps

After mastering these advanced examples:

1. **[API Reference](../../02_api_reference/main.md)** - Explore all available functionality
2. **[Architecture](../architecture/main.md)** - Understand internal design
3. **[Development Guide](../../03_development_guide/main.md)** - Contribute to the project

---

**Ready to explore the full API?** Check out the [API Reference](../../02_api_reference/main.md) to discover all available functionality!
