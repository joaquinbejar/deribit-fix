/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 6/3/26
******************************************************************************/

//! Position model types for Deribit API compatibility
//!
//! This module provides position-related types that were previously imported from
//! deribit-base. These types represent trading positions and their associated data.

use serde::{Deserialize, Serialize};

/// Position direction enumeration
///
/// Indicates whether a position is long (buy) or short (sell).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    /// Buy direction (long position)
    Buy,
    /// Sell direction (short position)
    Sell,
}

/// Trading position structure
///
/// Represents a trading position with all associated metrics including
/// size, direction, prices, margins, and Greeks for options.
#[derive(Clone, Serialize, Deserialize)]
pub struct Position {
    /// Name of the instrument (e.g., "BTC-PERPETUAL")
    pub instrument_name: String,
    /// Position size (positive for long, negative for short)
    pub size: f64,
    /// Direction of the position (buy/sell)
    pub direction: Direction,
    /// Average entry price of the position
    pub average_price: f64,
    /// Average price in USD
    pub average_price_usd: Option<f64>,
    /// Delta (price sensitivity) of the position
    pub delta: Option<f64>,
    /// Estimated liquidation price
    pub estimated_liquidation_price: Option<f64>,
    /// Floating (unrealized) profit/loss
    pub floating_profit_loss: Option<f64>,
    /// Floating profit/loss in USD
    pub floating_profit_loss_usd: Option<f64>,
    /// Gamma (delta sensitivity) of the position
    pub gamma: Option<f64>,
    /// Current index price
    pub index_price: Option<f64>,
    /// Initial margin requirement
    pub initial_margin: Option<f64>,
    /// Interest value
    pub interest_value: Option<f64>,
    /// Instrument kind (future, option, etc.)
    pub kind: Option<String>,
    /// Leverage used for the position
    pub leverage: Option<i32>,
    /// Maintenance margin requirement
    pub maintenance_margin: Option<f64>,
    /// Current mark price
    pub mark_price: Option<f64>,
    /// Margin used by open orders
    pub open_orders_margin: Option<f64>,
    /// Realized funding payments
    pub realized_funding: Option<f64>,
    /// Realized profit/loss
    pub realized_profit_loss: Option<f64>,
    /// Settlement price
    pub settlement_price: Option<f64>,
    /// Position size in currency units
    pub size_currency: Option<f64>,
    /// Theta (time decay) of the position
    pub theta: Option<f64>,
    /// Total profit/loss
    pub total_profit_loss: Option<f64>,
    /// Vega (volatility sensitivity) of the position
    pub vega: Option<f64>,
    /// Unrealized profit/loss
    pub unrealized_profit_loss: Option<f64>,
}

impl_json_display!(Position);
impl_json_debug_pretty!(Position);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_serialization() {
        let buy = Direction::Buy;
        let sell = Direction::Sell;

        let buy_json = serde_json::to_string(&buy).unwrap();
        let sell_json = serde_json::to_string(&sell).unwrap();

        assert_eq!(buy_json, "\"buy\"");
        assert_eq!(sell_json, "\"sell\"");

        let buy_deserialized: Direction = serde_json::from_str(&buy_json).unwrap();
        let sell_deserialized: Direction = serde_json::from_str(&sell_json).unwrap();

        assert_eq!(buy_deserialized, Direction::Buy);
        assert_eq!(sell_deserialized, Direction::Sell);
    }

    #[test]
    fn test_position_creation() {
        let position = Position {
            instrument_name: "BTC-PERPETUAL".to_string(),
            size: 1.5,
            direction: Direction::Buy,
            average_price: 50000.0,
            average_price_usd: None,
            delta: Some(0.5),
            estimated_liquidation_price: None,
            floating_profit_loss: Some(100.0),
            floating_profit_loss_usd: None,
            gamma: Some(0.001),
            index_price: Some(50100.0),
            initial_margin: Some(500.0),
            interest_value: None,
            kind: Some("future".to_string()),
            leverage: Some(10),
            maintenance_margin: Some(250.0),
            mark_price: Some(50050.0),
            open_orders_margin: None,
            realized_funding: None,
            realized_profit_loss: Some(50.0),
            settlement_price: Some(50000.0),
            size_currency: None,
            theta: None,
            total_profit_loss: Some(150.0),
            vega: None,
            unrealized_profit_loss: Some(100.0),
        };

        assert_eq!(position.instrument_name, "BTC-PERPETUAL");
        assert_eq!(position.size, 1.5);
        assert!(matches!(position.direction, Direction::Buy));
        assert_eq!(position.average_price, 50000.0);
    }

    #[test]
    fn test_position_serialization_roundtrip() {
        let position = Position {
            instrument_name: "ETH-PERPETUAL".to_string(),
            size: -2.0,
            direction: Direction::Sell,
            average_price: 3500.0,
            average_price_usd: Some(3500.0),
            delta: None,
            estimated_liquidation_price: Some(4000.0),
            floating_profit_loss: Some(-50.0),
            floating_profit_loss_usd: Some(-50.0),
            gamma: None,
            index_price: Some(3550.0),
            initial_margin: Some(350.0),
            interest_value: None,
            kind: Some("future".to_string()),
            leverage: Some(5),
            maintenance_margin: Some(175.0),
            mark_price: Some(3525.0),
            open_orders_margin: None,
            realized_funding: None,
            realized_profit_loss: None,
            settlement_price: None,
            size_currency: Some(-2.0),
            theta: None,
            total_profit_loss: Some(-50.0),
            vega: None,
            unrealized_profit_loss: Some(-50.0),
        };

        let json = serde_json::to_string(&position).unwrap();
        let deserialized: Position = serde_json::from_str(&json).unwrap();

        assert_eq!(position.instrument_name, deserialized.instrument_name);
        assert_eq!(position.size, deserialized.size);
        assert_eq!(position.direction, deserialized.direction);
        assert_eq!(position.average_price, deserialized.average_price);
    }
}
