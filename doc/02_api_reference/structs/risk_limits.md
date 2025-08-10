# RiskLimits

## Overview

The `RiskLimits` struct defines risk management parameters and limits for trading instruments, positions, and accounts on Deribit.

## Purpose

- **Position Limits**: Controls maximum position sizes and leverage
- **Loss Protection**: Defines stop-loss and drawdown limits
- **Margin Requirements**: Specifies initial and maintenance margin ratios
- **Risk Monitoring**: Provides thresholds for risk alerts and automatic actions

## Public Interface

### Struct Definition

```rust
pub struct RiskLimits {
    pub max_position_size: f64,
    pub max_leverage: f64,
    pub initial_margin_ratio: f64,
    pub maintenance_margin_ratio: f64,
    pub max_drawdown: f64,
    pub max_daily_loss: f64,
    pub max_order_size: f64,
    pub max_order_value: f64,
    pub min_margin_balance: f64,
    pub liquidation_threshold: f64,
    pub risk_alert_thresholds: RiskAlertThresholds,
    pub auto_liquidation: bool,
    pub position_concentration_limit: f64,
    pub correlation_limits: CorrelationLimits,
    pub volatility_limits: VolatilityLimits,
}
```

### Key Methods

```rust
impl RiskLimits {
    /// Create default risk limits
    pub fn default() -> Self

    /// Create conservative risk limits
    pub fn conservative() -> Self

    /// Create aggressive risk limits
    pub fn aggressive() -> Self

    /// Check if position exceeds limits
    pub fn check_position_limit(&self, position_size: f64) -> Result<(), RiskLimitError>

    /// Calculate required margin for position
    pub fn calculate_required_margin(&self, position_value: f64, leverage: f64) -> f64

    /// Check if account is at risk
    pub fn is_account_at_risk(&self, current_margin: f64, position_value: f64) -> bool

    /// Validate order against risk limits
    pub fn validate_order(&self, order: &Order, current_positions: &[Position]) -> Result<(), Vec<RiskLimitError>>

    /// Get risk level (Low, Medium, High, Critical)
    pub fn get_risk_level(&self, margin_ratio: f64) -> RiskLevel

    /// Apply risk limits to configuration
    pub fn apply_to_config(&self, config: &mut Config) -> Result<(), RiskLimitError>
}
```

## Usage Examples

### Creating Risk Limits

```rust
use deribit_fix::types::{RiskLimits, RiskAlertThresholds, CorrelationLimits, VolatilityLimits};

// Create default risk limits
let default_limits = RiskLimits::default();

// Create conservative limits for retail clients
let conservative_limits = RiskLimits {
    max_position_size: 100.0,
    max_leverage: 5.0,
    initial_margin_ratio: 0.20, // 20% initial margin
    maintenance_margin_ratio: 0.15, // 15% maintenance margin
    max_drawdown: 0.25, // 25% max drawdown
    max_daily_loss: 1000.0,
    max_order_size: 10.0,
    max_order_value: 50000.0,
    min_margin_balance: 1000.0,
    liquidation_threshold: 0.10, // 10% liquidation threshold
    risk_alert_thresholds: RiskAlertThresholds::default(),
    auto_liquidation: true,
    position_concentration_limit: 0.50, // 50% max in single instrument
    correlation_limits: CorrelationLimits::default(),
    volatility_limits: VolatilityLimits::default(),
};

// Create aggressive limits for professional clients
let aggressive_limits = RiskLimits::aggressive();
```

### Risk Monitoring

```rust
// Check position against limits
let position_size = 50.0;
match risk_limits.check_position_limit(position_size) {
    Ok(()) => println!("Position within limits"),
    Err(error) => eprintln!("Position exceeds limits: {:?}", error),
}

// Calculate required margin
let position_value = 100000.0;
let leverage = 10.0;
let required_margin = risk_limits.calculate_required_margin(position_value, leverage);

println!("Required margin: {}", required_margin);

// Check account risk level
let current_margin = 15000.0;
let margin_ratio = current_margin / position_value;
let risk_level = risk_limits.get_risk_level(margin_ratio);

match risk_level {
    RiskLevel::Low => println!("Account risk: Low"),
    RiskLevel::Medium => println!("Account risk: Medium - Monitor closely"),
    RiskLevel::High => println!("Account risk: High - Consider reducing exposure"),
    RiskLevel::Critical => println!("Account risk: Critical - Immediate action required"),
}
```

### Order Validation

```rust
// Validate order against risk limits
let order = Order::new(
    "BTC-PERPETUAL".to_string(),
    OrderSide::Buy,
    OrderType::Market,
    5.0,
);

let current_positions = vec![
    Position::new("BTC-PERPETUAL".to_string(), PositionSide::Long),
];

let validation_result = risk_limits.validate_order(&order, &current_positions);
match validation_result {
    Ok(()) => println!("Order passes risk checks"),
    Err(errors) => {
        for error in errors {
            eprintln!("Risk limit violation: {:?}", error);
        }
    }
}
```

### Dynamic Risk Management

```rust
// Adjust limits based on market conditions
let mut dynamic_limits = RiskLimits::default();

if market_volatility > 0.5 {
    // Reduce limits during high volatility
    dynamic_limits.max_leverage = dynamic_limits.max_leverage * 0.5;
    dynamic_limits.max_position_size = dynamic_limits.max_position_size * 0.7;
    dynamic_limits.auto_liquidation = true;
}

// Apply limits to configuration
let mut config = Config::default();
dynamic_limits.apply_to_config(&mut config)?;
```

## Module Dependencies

### Direct Dependencies

- **`types`**: `RiskAlertThresholds`, `CorrelationLimits`, `VolatilityLimits`
- **`model`**: `Order`, `Position`, `Config`
- **`error`**: `RiskLimitError`

### Related Types

- **`RiskAlertThresholds`**: Defines warning levels for risk monitoring
- **`CorrelationLimits`**: Controls exposure to correlated instruments
- **`VolatilityLimits`**: Adjusts limits based on market volatility
- **`RiskLevel`**: Enum representing risk assessment levels
- **`RiskLimitError`**: Specific error types for risk limit violations

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_risk_limits() {
        let limits = RiskLimits::default();
        
        assert!(limits.max_leverage > 0.0);
        assert!(limits.initial_margin_ratio > 0.0);
        assert!(limits.maintenance_margin_ratio > 0.0);
        assert!(limits.initial_margin_ratio > limits.maintenance_margin_ratio);
    }

    #[test]
    fn test_position_limit_check() {
        let mut limits = RiskLimits::default();
        limits.max_position_size = 100.0;

        assert!(limits.check_position_limit(50.0).is_ok());
        assert!(limits.check_position_limit(150.0).is_err());
    }

    #[test]
    fn test_margin_calculation() {
        let limits = RiskLimits::default();
        let position_value = 100000.0;
        let leverage = 10.0;

        let required_margin = limits.calculate_required_margin(position_value, leverage);
        assert_eq!(required_margin, position_value / leverage);
    }

    #[test]
    fn test_risk_level_calculation() {
        let limits = RiskLimits::default();
        
        let low_risk = limits.get_risk_level(0.50);
        let high_risk = limits.get_risk_level(0.05);
        
        assert_eq!(low_risk, RiskLevel::Low);
        assert_eq!(high_risk, RiskLevel::Critical);
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_order_validation_with_positions() {
    let limits = RiskLimits::default();
    let order = create_test_order("BTC-PERPETUAL");
    let positions = create_test_positions();

    let result = limits.validate_order(&order, &positions);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_risk_limit_configuration() {
    let limits = RiskLimits::conservative();
    let mut config = Config::default();

    let result = limits.apply_to_config(&mut config);
    assert!(result.is_ok());
    
    // Verify limits were applied
    assert_eq!(config.trading.max_leverage, limits.max_leverage);
    assert_eq!(config.trading.max_position_size, limits.max_position_size);
}
```

## Performance Considerations

- **Limit Caching**: Cache frequently accessed risk limits
- **Batch Validation**: Validate multiple orders/positions in batch operations
- **Efficient Calculations**: Use optimized algorithms for margin and risk calculations
- **Memory Usage**: Minimize memory footprint for large position sets

## Security Considerations

- **Limit Enforcement**: Ensure risk limits are enforced at all levels
- **Audit Logging**: Log all risk limit violations and adjustments
- **Access Control**: Restrict modification of critical risk parameters
- **Data Validation**: Validate all risk limit inputs and calculations
- **Real-time Monitoring**: Implement continuous risk monitoring systems
