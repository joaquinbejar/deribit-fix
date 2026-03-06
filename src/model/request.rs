/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 6/3/26
******************************************************************************/

//! Order request model types for Deribit API compatibility
//!
//! This module provides order-related types that were previously imported from
//! deribit-base. These types represent order requests and their parameters
//! in API-style format (not FIX protocol format).

use serde::{Deserialize, Serialize};

/// Time in force enumeration (API style)
///
/// Specifies how long an order remains active before it is executed or expires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeInForce {
    /// Order remains active until explicitly cancelled
    #[serde(rename = "good_til_cancelled")]
    GoodTilCancelled,
    /// Order expires at the end of the trading day
    #[serde(rename = "good_til_day")]
    GoodTilDay,
    /// Order must be filled immediately and completely or cancelled
    #[serde(rename = "fill_or_kill")]
    FillOrKill,
    /// Order must be filled immediately, partial fills allowed, remaining cancelled
    #[serde(rename = "immediate_or_cancel")]
    ImmediateOrCancel,
}

impl TimeInForce {
    /// Returns the string representation of the time in force value
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            TimeInForce::GoodTilCancelled => "good_til_cancelled",
            TimeInForce::GoodTilDay => "good_til_day",
            TimeInForce::FillOrKill => "fill_or_kill",
            TimeInForce::ImmediateOrCancel => "immediate_or_cancel",
        }
    }
}

/// Order side enumeration (API style)
///
/// Indicates whether an order is a buy or sell order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    /// Buy order
    Buy,
    /// Sell order
    Sell,
}

/// Order type enumeration (API style)
///
/// Specifies the type of order execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    /// Limit order - executes at specified price or better
    #[serde(rename = "limit")]
    Limit,
    /// Market order - executes immediately at best available price
    #[serde(rename = "market")]
    Market,
    /// Stop limit order - becomes limit order when stop price is reached
    #[serde(rename = "stop_limit")]
    StopLimit,
    /// Stop market order - becomes market order when stop price is reached
    #[serde(rename = "stop_market")]
    StopMarket,
    /// Take limit order - limit order to take profit
    #[serde(rename = "take_limit")]
    TakeLimit,
    /// Take market order - market order to take profit
    #[serde(rename = "take_market")]
    TakeMarket,
    /// Market limit order - market order with limit price protection
    #[serde(rename = "market_limit")]
    MarketLimit,
    /// Trailing stop order - stop order that trails the market price
    #[serde(rename = "trailing_stop")]
    TrailingStop,
}

impl OrderType {
    /// Returns the string representation of the order type
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderType::Limit => "limit",
            OrderType::Market => "market",
            OrderType::StopLimit => "stop_limit",
            OrderType::StopMarket => "stop_market",
            OrderType::TakeLimit => "take_limit",
            OrderType::TakeMarket => "take_market",
            OrderType::MarketLimit => "market_limit",
            OrderType::TrailingStop => "trailing_stop",
        }
    }
}

/// Trigger type for stop orders
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TriggerType {
    /// Index price trigger
    #[serde(rename = "index_price")]
    IndexPrice,
    /// Mark price trigger
    #[serde(rename = "mark_price")]
    MarkPrice,
    /// Last price trigger
    #[serde(rename = "last_price")]
    LastPrice,
}

/// Advanced order type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdvancedOrderType {
    /// USD order
    #[serde(rename = "usd")]
    Usd,
    /// Implv order (implied volatility)
    #[serde(rename = "implv")]
    Implv,
}

/// Generic request for creating new orders (API style)
///
/// This structure represents an order request in the API format used by
/// deribit-base. It contains all parameters needed to place a new order.
#[derive(Clone, Serialize, Deserialize)]
pub struct NewOrderRequest {
    /// Instrument name (e.g., "BTC-PERPETUAL")
    pub instrument_name: String,
    /// Order amount
    pub amount: f64,
    /// Order type
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// Order side (buy/sell)
    pub side: OrderSide,
    /// Order price (required for limit orders)
    pub price: Option<f64>,
    /// Time in force
    pub time_in_force: TimeInForce,
    /// Post-only flag
    pub post_only: Option<bool>,
    /// Reduce-only flag
    pub reduce_only: Option<bool>,
    /// Order label
    pub label: Option<String>,
    /// Stop price for stop orders
    pub stop_price: Option<f64>,
    /// Trigger type for stop orders
    pub trigger: Option<TriggerType>,
    /// Advanced order type
    pub advanced: Option<AdvancedOrderType>,
    /// Maximum show amount (iceberg orders)
    pub max_show: Option<f64>,
    /// Reject post-only flag
    pub reject_post_only: Option<bool>,
    /// Valid until timestamp
    pub valid_until: Option<i64>,
    /// Client order ID for tracking
    pub client_order_id: Option<String>,
}

impl_json_display!(NewOrderRequest);
impl_json_debug_pretty!(NewOrderRequest);

impl NewOrderRequest {
    /// Create a new market buy order
    #[must_use]
    pub fn market_buy(instrument_name: String, amount: f64) -> Self {
        Self {
            instrument_name,
            amount,
            order_type: OrderType::Market,
            side: OrderSide::Buy,
            price: None,
            time_in_force: TimeInForce::ImmediateOrCancel,
            post_only: None,
            reduce_only: None,
            label: None,
            stop_price: None,
            trigger: None,
            advanced: None,
            max_show: None,
            reject_post_only: None,
            valid_until: None,
            client_order_id: None,
        }
    }

    /// Create a new market sell order
    #[must_use]
    pub fn market_sell(instrument_name: String, amount: f64) -> Self {
        Self {
            instrument_name,
            amount,
            order_type: OrderType::Market,
            side: OrderSide::Sell,
            price: None,
            time_in_force: TimeInForce::ImmediateOrCancel,
            post_only: None,
            reduce_only: None,
            label: None,
            stop_price: None,
            trigger: None,
            advanced: None,
            max_show: None,
            reject_post_only: None,
            valid_until: None,
            client_order_id: None,
        }
    }

    /// Create a new limit buy order
    #[must_use]
    pub fn limit_buy(instrument_name: String, amount: f64, price: f64) -> Self {
        Self {
            instrument_name,
            amount,
            order_type: OrderType::Limit,
            side: OrderSide::Buy,
            price: Some(price),
            time_in_force: TimeInForce::GoodTilCancelled,
            post_only: None,
            reduce_only: None,
            label: None,
            stop_price: None,
            trigger: None,
            advanced: None,
            max_show: None,
            reject_post_only: None,
            valid_until: None,
            client_order_id: None,
        }
    }

    /// Create a new limit sell order
    #[must_use]
    pub fn limit_sell(instrument_name: String, amount: f64, price: f64) -> Self {
        Self {
            instrument_name,
            amount,
            order_type: OrderType::Limit,
            side: OrderSide::Sell,
            price: Some(price),
            time_in_force: TimeInForce::GoodTilCancelled,
            post_only: None,
            reduce_only: None,
            label: None,
            stop_price: None,
            trigger: None,
            advanced: None,
            max_show: None,
            reject_post_only: None,
            valid_until: None,
            client_order_id: None,
        }
    }

    /// Set the order as post-only
    #[must_use]
    pub fn with_post_only(mut self, post_only: bool) -> Self {
        self.post_only = Some(post_only);
        self
    }

    /// Set the order as reduce-only
    #[must_use]
    pub fn with_reduce_only(mut self, reduce_only: bool) -> Self {
        self.reduce_only = Some(reduce_only);
        self
    }

    /// Set order label
    #[must_use]
    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    /// Set time in force
    #[must_use]
    pub fn with_time_in_force(mut self, tif: TimeInForce) -> Self {
        self.time_in_force = tif;
        self
    }

    /// Set client order ID
    #[must_use]
    pub fn with_client_order_id(mut self, client_order_id: String) -> Self {
        self.client_order_id = Some(client_order_id);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_in_force_as_str() {
        assert_eq!(TimeInForce::GoodTilCancelled.as_str(), "good_til_cancelled");
        assert_eq!(TimeInForce::GoodTilDay.as_str(), "good_til_day");
        assert_eq!(TimeInForce::FillOrKill.as_str(), "fill_or_kill");
        assert_eq!(
            TimeInForce::ImmediateOrCancel.as_str(),
            "immediate_or_cancel"
        );
    }

    #[test]
    fn test_order_type_as_str() {
        assert_eq!(OrderType::Limit.as_str(), "limit");
        assert_eq!(OrderType::Market.as_str(), "market");
        assert_eq!(OrderType::StopLimit.as_str(), "stop_limit");
        assert_eq!(OrderType::StopMarket.as_str(), "stop_market");
    }

    #[test]
    fn test_new_order_request_market_buy() {
        let order = NewOrderRequest::market_buy("BTC-PERPETUAL".to_string(), 1.0);
        assert_eq!(order.instrument_name, "BTC-PERPETUAL");
        assert_eq!(order.amount, 1.0);
        assert_eq!(order.order_type, OrderType::Market);
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.price, None);
        assert_eq!(order.time_in_force, TimeInForce::ImmediateOrCancel);
    }

    #[test]
    fn test_new_order_request_limit_sell() {
        let order = NewOrderRequest::limit_sell("ETH-PERPETUAL".to_string(), 2.0, 3500.0);
        assert_eq!(order.instrument_name, "ETH-PERPETUAL");
        assert_eq!(order.amount, 2.0);
        assert_eq!(order.order_type, OrderType::Limit);
        assert_eq!(order.side, OrderSide::Sell);
        assert_eq!(order.price, Some(3500.0));
        assert_eq!(order.time_in_force, TimeInForce::GoodTilCancelled);
    }

    #[test]
    fn test_new_order_request_builder_pattern() {
        let order = NewOrderRequest::limit_buy("BTC-PERPETUAL".to_string(), 1.0, 50000.0)
            .with_post_only(true)
            .with_reduce_only(false)
            .with_label("test_order".to_string())
            .with_client_order_id("CLIENT_123".to_string());

        assert_eq!(order.post_only, Some(true));
        assert_eq!(order.reduce_only, Some(false));
        assert_eq!(order.label, Some("test_order".to_string()));
        assert_eq!(order.client_order_id, Some("CLIENT_123".to_string()));
    }

    #[test]
    fn test_serialization_roundtrip() {
        let order = NewOrderRequest::limit_buy("BTC-PERPETUAL".to_string(), 1.0, 50000.0)
            .with_post_only(true)
            .with_label("test_order".to_string());

        let json = serde_json::to_string(&order).unwrap();
        let deserialized: NewOrderRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(order.instrument_name, deserialized.instrument_name);
        assert_eq!(order.amount, deserialized.amount);
        assert_eq!(order.order_type, deserialized.order_type);
        assert_eq!(order.side, deserialized.side);
        assert_eq!(order.price, deserialized.price);
        assert_eq!(order.post_only, deserialized.post_only);
        assert_eq!(order.label, deserialized.label);
    }
}
