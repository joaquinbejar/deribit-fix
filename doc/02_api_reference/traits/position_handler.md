# PositionHandler Trait

## Overview

The `PositionHandler` trait defines the interface for managing trading positions within the Deribit FIX client. It provides methods for tracking, updating, and managing positions across different instruments, including position calculations, risk management, and position-related FIX messages.

## Purpose

- **Position Tracking**: Monitor current positions across all instruments
- **Position Updates**: Update positions based on trades and executions
- **Risk Management**: Calculate position risk metrics and enforce limits
- **Margin Management**: Track margin requirements and available margin
- **Position Analytics**: Provide position statistics and performance metrics
- **Position Reconciliation**: Ensure position accuracy with exchange data
- **Hedging Support**: Support position hedging and risk mitigation strategies

## Public Interface

### Trait Definition

```rust
#[async_trait]
pub trait PositionHandler: Send + Sync {
    async fn get_position(&mut self, instrument: &str) -> Result<Position, PositionHandlerError>;
    async fn get_all_positions(&mut self) -> Result<Vec<Position>, PositionHandlerError>;
    async fn update_position(&mut self, instrument: &str, update: PositionUpdate) -> Result<(), PositionHandlerError>;
    async fn calculate_position_pnl(&mut self, instrument: &str) -> Result<PositionPnl, PositionHandlerError>;
    async fn calculate_portfolio_pnl(&mut self) -> Result<PortfolioPnl, PositionHandlerError>;
    async fn get_position_risk_metrics(&mut self, instrument: &str) -> Result<PositionRiskMetrics, PositionHandlerError>;
    async fn get_portfolio_risk_metrics(&mut self) -> Result<PortfolioRiskMetrics, PositionHandlerError>;
    async fn validate_position_change(&mut self, change: PositionChange) -> Result<(), Vec<PositionValidationError>>;
    async fn handle_position_report(&mut self, report: PositionReport) -> Result<(), PositionHandlerError>;
    async fn reconcile_positions(&mut self) -> Result<ReconciliationResult, PositionHandlerError>;
    async fn get_margin_requirement(&mut self, instrument: &str) -> Result<MarginRequirement, PositionHandlerError>;
    async fn get_total_margin_requirement(&mut self) -> Result<f64, PositionHandlerError>;
    async fn get_available_margin(&mut self) -> Result<f64, PositionHandlerError>;
    async fn get_margin_ratio(&mut self) -> Result<f64, PositionHandlerError>;
    fn get_position_stats(&self) -> PositionHandlerStats;
    async fn get_position_history(&mut self, filter: PositionHistoryFilter) -> Result<Vec<PositionSnapshot>, PositionHandlerError>;
    async fn close_position(&mut self, instrument: &str, reason: Option<&str>) -> Result<(), PositionHandlerError>;
    async fn partial_close_position(&mut self, instrument: &str, quantity: f64) -> Result<(), PositionHandlerError>;
}
```

### Associated Types

```rust
pub enum PositionHandlerError {
    PositionNotFound(String),
    InvalidPositionUpdate(PositionUpdateError),
    InsufficientMargin(f64),
    RiskLimitExceeded(String),
    InvalidPositionChange(Vec<PositionValidationError>),
    ExchangeError(String),
    NetworkError(String),
    ValidationError(Vec<PositionValidationError>),
    ReconciliationFailed(String),
}

pub enum PositionUpdateError {
    InvalidQuantity(f64),
    InvalidPrice(f64),
    InvalidSide(PositionSide),
    InvalidInstrument(String),
    UpdateWouldExceedLimit(f64, f64),
}

pub enum PositionValidationError {
    InvalidSize(f64),
    InvalidSide(PositionSide),
    InvalidInstrument(String),
    RiskLimitViolation(String),
    MarginInsufficient(f64, f64),
    PositionLimitExceeded(f64, f64),
}

pub struct PositionUpdate {
    pub quantity_change: f64,
    pub price: f64,
    pub side: PositionSide,
    pub timestamp: DateTime<Utc>,
    pub trade_id: Option<String>,
    pub order_id: Option<String>,
    pub fee: Option<f64>,
    pub fee_currency: Option<String>,
}

pub struct PositionChange {
    pub instrument: String,
    pub current_size: f64,
    pub new_size: f64,
    pub side: PositionSide,
    pub price: f64,
    pub timestamp: DateTime<Utc>,
}

pub struct PositionPnl {
    pub instrument: String,
    pub unrealized_pnl: f64,
    pub realized_pnl: f64,
    pub total_pnl: f64,
    pub pnl_currency: String,
    pub mark_price: f64,
    pub entry_price: f64,
    pub timestamp: DateTime<Utc>,
}

pub struct PortfolioPnl {
    pub total_unrealized_pnl: f64,
    pub total_realized_pnl: f64,
    pub total_pnl: f64,
    pub pnl_currency: String,
    pub timestamp: DateTime<Utc>,
    pub instrument_breakdown: Vec<PositionPnl>,
}

pub struct PositionRiskMetrics {
    pub instrument: String,
    pub var_95: f64,
    pub var_99: f64,
    pub max_drawdown: f64,
    pub sharpe_ratio: Option<f64>,
    pub beta: Option<f64>,
    pub correlation: Option<f64>,
    pub timestamp: DateTime<Utc>,
}

pub struct PortfolioRiskMetrics {
    pub total_var_95: f64,
    pub total_var_99: f64,
    pub portfolio_beta: f64,
    pub diversification_ratio: f64,
    pub concentration_risk: f64,
    pub timestamp: DateTime<Utc>,
}

pub struct PositionReport {
    pub instrument: String,
    pub size: f64,
    pub side: PositionSide,
    pub average_price: f64,
    pub mark_price: f64,
    pub unrealized_pnl: f64,
    pub realized_pnl: f64,
    pub timestamp: DateTime<Utc>,
    pub sequence_number: u64,
}

pub struct ReconciliationResult {
    pub total_positions: u32,
    pub reconciled_positions: u32,
    pub discrepancies: Vec<PositionDiscrepancy>,
    pub timestamp: DateTime<Utc>,
}

pub struct PositionDiscrepancy {
    pub instrument: String,
    pub local_size: f64,
    pub exchange_size: f64,
    pub difference: f64,
    pub severity: DiscrepancySeverity,
}

pub enum DiscrepancySeverity {
    Minor,
    Moderate,
    Critical,
}

pub struct MarginRequirement {
    pub instrument: String,
    pub initial_margin: f64,
    pub maintenance_margin: f64,
    pub current_margin: f64,
    pub margin_currency: String,
    pub leverage: f64,
    pub timestamp: DateTime<Utc>,
}

pub struct PositionHistoryFilter {
    pub instrument: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub min_size: Option<f64>,
    pub max_size: Option<f64>,
    pub side: Option<PositionSide>,
    pub limit: Option<u32>,
}

pub struct PositionSnapshot {
    pub instrument: String,
    pub size: f64,
    pub side: PositionSide,
    pub average_price: f64,
    pub mark_price: f64,
    pub unrealized_pnl: f64,
    pub timestamp: DateTime<Utc>,
}

pub struct PositionHandlerStats {
    pub total_positions: u64,
    pub active_positions: u64,
    pub total_pnl: f64,
    pub total_margin_used: f64,
    pub last_update_time: Option<DateTime<Utc>>,
    pub reconciliation_count: u64,
    pub error_count: u64,
}
```

## Usage Examples

### Basic Position Management

```rust
use deribit_fix::traits::PositionHandler;
use deribit_fix::types::{Position, PositionUpdate, PositionSide};

struct MyPositionHandler;

#[async_trait]
impl PositionHandler for MyPositionHandler {
    async fn get_position(&mut self, instrument: &str) -> Result<Position, PositionHandlerError> {
        // Check local cache first
        if let Some(position) = self.get_cached_position(instrument).await? {
            return Ok(position);
        }
        
        // Fetch from exchange if not cached
        let position = self.fetch_position_from_exchange(instrument).await?;
        
        // Cache the position
        self.cache_position(instrument, &position).await?;
        
        Ok(position)
    }
    
    // ... implement other required methods
}
```

### Position Updates

```rust
impl MyPositionHandler {
    async fn process_trade_update(&mut self, trade: &Trade) -> Result<(), PositionHandlerError> {
        let instrument = &trade.instrument;
        let current_position = self.get_position(instrument).await?;
        
        // Calculate position update
        let update = PositionUpdate {
            quantity_change: if trade.side == OrderSide::Buy { trade.quantity } else { -trade.quantity },
            price: trade.price,
            side: current_position.side,
            timestamp: trade.timestamp,
            trade_id: Some(trade.trade_id.clone()),
            order_id: None,
            fee: trade.fee,
            fee_currency: Some(trade.fee_currency.clone()),
        };
        
        // Validate the update
        self.validate_position_update(&update).await?;
        
        // Apply the update
        self.update_position(instrument, update).await?;
        
        // Update PnL
        self.update_position_pnl(instrument).await?;
        
        Ok(())
    }
}
```

### PnL Calculation

```rust
impl MyPositionHandler {
    async fn calculate_realized_pnl(&mut self, instrument: &str) -> Result<f64, PositionHandlerError> {
        let position = self.get_position(instrument).await?;
        let trades = self.get_position_trades(instrument).await?;
        
        if position.size == 0.0 {
            return Ok(0.0);
        }
        
        let mut realized_pnl = 0.0;
        let mut remaining_size = position.size.abs();
        
        for trade in trades.iter().rev() {
            let trade_quantity = trade.quantity.min(remaining_size);
            let trade_pnl = if position.side == PositionSide::Long {
                (trade.price - position.average_price) * trade_quantity
            } else {
                (position.average_price - trade.price) * trade_quantity
            };
            
            realized_pnl += trade_pnl;
            remaining_size -= trade_quantity;
            
            if remaining_size <= 0.0 {
                break;
            }
        }
        
        Ok(realized_pnl)
    }
    
    async fn calculate_unrealized_pnl(&mut self, instrument: &str) -> Result<f64, PositionHandlerError> {
        let position = self.get_position(instrument).await?;
        let mark_price = self.get_mark_price(instrument).await?;
        
        if position.size == 0.0 {
            return Ok(0.0);
        }
        
        let unrealized_pnl = if position.side == PositionSide::Long {
            (mark_price - position.average_price) * position.size
        } else {
            (position.average_price - mark_price) * position.size.abs()
        };
        
        Ok(unrealized_pnl)
    }
}
```

### Risk Management

```rust
impl MyPositionHandler {
    async fn calculate_position_var(&mut self, instrument: &str, confidence: f64) -> Result<f64, PositionHandlerError> {
        let position = self.get_position(instrument).await?;
        let price_history = self.get_price_history(instrument, 252).await?; // 1 year of data
        
        if price_history.len() < 30 {
            return Err(PositionHandlerError::DataUnavailable("Insufficient price history".to_string()));
        }
        
        // Calculate price returns
        let returns: Vec<f64> = price_history.windows(2)
            .map(|window| (window[1] - window[0]) / window[0])
            .collect();
        
        // Calculate VaR using historical simulation
        let sorted_returns: Vec<f64> = returns.into_iter().collect();
        sorted_returns.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let var_index = ((1.0 - confidence) * sorted_returns.len() as f64) as usize;
        let var_return = sorted_returns.get(var_index).unwrap_or(&0.0);
        
        // Calculate VaR in currency terms
        let position_value = position.size.abs() * self.get_mark_price(instrument).await?;
        let var = position_value * var_return.abs();
        
        Ok(var)
    }
    
    async fn check_risk_limits(&mut self, instrument: &str) -> Result<(), PositionHandlerError> {
        let position = self.get_position(instrument).await?;
        let risk_metrics = self.get_position_risk_metrics(instrument).await?;
        
        // Check position size limits
        let max_position_size = self.get_max_position_size(instrument).await?;
        if position.size.abs() > max_position_size {
            return Err(PositionHandlerError::RiskLimitExceeded(
                format!("Position size {} exceeds limit {}", position.size, max_position_size)
            ));
        }
        
        // Check VaR limits
        let max_var = self.get_max_var_limit(instrument).await?;
        if risk_metrics.var_95 > max_var {
            return Err(PositionHandlerError::RiskLimitExceeded(
                format!("VaR {} exceeds limit {}", risk_metrics.var_95, max_var)
            ));
        }
        
        // Check margin requirements
        let margin_requirement = self.get_margin_requirement(instrument).await?;
        let available_margin = self.get_available_margin().await?;
        
        if margin_requirement.current_margin > available_margin {
            return Err(PositionHandlerError::InsufficientMargin(
                margin_requirement.current_margin - available_margin
            ));
        }
        
        Ok(())
    }
}
```

### Position Reconciliation

```rust
impl MyPositionHandler {
    async fn reconcile_with_exchange(&mut self) -> Result<ReconciliationResult, PositionHandlerError> {
        let local_positions = self.get_all_positions().await?;
        let exchange_positions = self.fetch_all_positions_from_exchange().await?;
        
        let mut reconciled_count = 0;
        let mut discrepancies = Vec::new();
        
        for local_pos in &local_positions {
            if let Some(exchange_pos) = exchange_positions.iter().find(|ep| ep.instrument == local_pos.instrument) {
                let size_diff = (local_pos.size - exchange_pos.size).abs();
                let price_diff = (local_pos.average_price - exchange_pos.average_price).abs();
                
                if size_diff > 0.001 || price_diff > 0.01 {
                    let severity = if size_diff > 1.0 || price_diff > 10.0 {
                        DiscrepancySeverity::Critical
                    } else if size_diff > 0.1 || price_diff > 1.0 {
                        DiscrepancySeverity::Moderate
                    } else {
                        DiscrepancySeverity::Minor
                    };
                    
                    discrepancies.push(PositionDiscrepancy {
                        instrument: local_pos.instrument.clone(),
                        local_size: local_pos.size,
                        exchange_size: exchange_pos.size,
                        difference: size_diff,
                        severity,
                    });
                } else {
                    reconciled_count += 1;
                }
            } else {
                // Position exists locally but not on exchange
                discrepancies.push(PositionDiscrepancy {
                    instrument: local_pos.instrument.clone(),
                    local_size: local_pos.size,
                    exchange_size: 0.0,
                    difference: local_pos.size.abs(),
                    severity: DiscrepancySeverity::Moderate,
                });
            }
        }
        
        // Check for positions that exist on exchange but not locally
        for exchange_pos in &exchange_positions {
            if !local_positions.iter().any(|lp| lp.instrument == exchange_pos.instrument) {
                discrepancies.push(PositionDiscrepancy {
                    instrument: exchange_pos.instrument.clone(),
                    local_size: 0.0,
                    exchange_size: exchange_pos.size,
                    difference: exchange_pos.size.abs(),
                    severity: DiscrepancySeverity::Moderate,
                });
            }
        }
        
        Ok(ReconciliationResult {
            total_positions: local_positions.len() as u32,
            reconciled_positions: reconciled_count,
            discrepancies,
            timestamp: Utc::now(),
        })
    }
}
```

## Module Dependencies

- **`types`**: Uses `Position`, `PositionSide`, and related position types
- **`error`**: Uses `PositionHandlerError` and validation error types
- **`message`**: May interact with FIX message handling for position reports
- **`session`**: May use session management for position queries

## Related Types

- **`Position`**: The core position structure being managed
- **`PositionUpdate`**: Updates to position data
- **`PositionPnl`**: Profit and loss calculations for positions
- **`PositionRiskMetrics`**: Risk metrics for individual positions
- **`MarginRequirement`**: Margin requirements for positions

## Testing

### Unit Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_position_update() {
        let mut handler = MyPositionHandler::new();
        let update = create_test_position_update();
        
        let result = handler.update_position("BTC-PERPETUAL", update).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_pnl_calculation() {
        let mut handler = MyPositionHandler::new();
        
        let pnl = handler.calculate_position_pnl("BTC-PERPETUAL").await.unwrap();
        assert!(pnl.total_pnl.is_finite());
    }
}
```

### Integration Testing

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_position_lifecycle() {
        let mut handler = MyPositionHandler::new();
        
        // Get initial position
        let initial_position = handler.get_position("BTC-PERPETUAL").await.unwrap();
        
        // Apply trade update
        let update = create_test_trade_update();
        handler.update_position("BTC-PERPETUAL", update).await.unwrap();
        
        // Verify position change
        let updated_position = handler.get_position("BTC-PERPETUAL").await.unwrap();
        assert_ne!(initial_position.size, updated_position.size);
        
        // Check PnL
        let pnl = handler.calculate_position_pnl("BTC-PERPETUAL").await.unwrap();
        assert!(pnl.total_pnl.is_finite());
    }
}
```

## Performance Considerations

- **Async Operations**: All operations are async to avoid blocking
- **Position Caching**: Cache frequently accessed position data
- **Batch Updates**: Consider batching multiple position updates
- **Incremental Calculations**: Use incremental updates for PnL calculations
- **Connection Pooling**: Use connection pools for exchange communication

## Security Considerations

- **Position Validation**: Validate all position updates and changes
- **Risk Limits**: Enforce risk management rules and position limits
- **Margin Monitoring**: Continuously monitor margin requirements
- **Access Control**: Restrict position operations to authorized users
- **Audit Logging**: Log all position changes and updates
