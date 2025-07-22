/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 22/7/25
******************************************************************************/

//! New Order Single FIX Message Implementation

use super::*;
use crate::error::Result as DeribitFixResult;
use crate::message::builder::MessageBuilder;
use crate::model::types::MsgType;
use chrono::{DateTime, Utc};
use deribit_base::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

/// New Order Single message (MsgType = 'D')
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct NewOrderSingle {
    /// Unique identifier for the order, assigned by the client
    pub cl_ord_id: String,
    /// Side of order (Buy/Sell)
    pub side: OrderSide,
    /// Order quantity
    pub order_qty: f64,
    /// Price
    pub price: f64,
    /// Instrument symbol
    pub symbol: String,
    /// Indicates expiration time of indication message
    pub valid_until_time: Option<DateTime<Utc>>,
    /// Execution instructions (POST ONLY, REDUCE ONLY)
    pub exec_inst: Option<String>,
    /// Order type
    pub ord_type: Option<OrderType>,
    /// Time in force
    pub time_in_force: Option<TimeInForce>,
    /// Stop price
    pub stop_px: Option<f64>,
    /// Display quantity
    pub display_qty: Option<f64>,
    /// Refresh quantity
    pub refresh_qty: Option<f64>,
    /// Quantity type
    pub qty_type: Option<QuantityType>,
    /// Peg offset value
    pub peg_offset_value: Option<f64>,
    /// Peg price type
    pub peg_price_type: Option<i32>,
    /// Custom label for order
    pub deribit_label: Option<String>,
    /// Advanced order type for options
    pub deribit_adv_order_type: Option<char>,
    /// Market Maker Protection flag
    pub deribit_mm_protection: Option<bool>,
    /// Condition trigger method for algo orders
    pub deribit_condition_trigger_method: Option<i32>,
}

impl NewOrderSingle {
    /// Create a new market order
    pub fn market(cl_ord_id: String, side: OrderSide, order_qty: f64, symbol: String) -> Self {
        Self {
            cl_ord_id,
            side,
            order_qty,
            price: 0.0, // Market orders don't need price
            symbol,
            valid_until_time: None,
            exec_inst: None,
            ord_type: Some(OrderType::Market),
            time_in_force: None,
            stop_px: None,
            display_qty: None,
            refresh_qty: None,
            qty_type: None,
            peg_offset_value: None,
            peg_price_type: None,
            deribit_label: None,
            deribit_adv_order_type: None,
            deribit_mm_protection: None,
            deribit_condition_trigger_method: None,
        }
    }

    /// Create a new limit order
    pub fn limit(
        cl_ord_id: String,
        side: OrderSide,
        order_qty: f64,
        price: f64,
        symbol: String,
    ) -> Self {
        Self {
            cl_ord_id,
            side,
            order_qty,
            price,
            symbol,
            valid_until_time: None,
            exec_inst: None,
            ord_type: Some(OrderType::Limit),
            time_in_force: None,
            stop_px: None,
            display_qty: None,
            refresh_qty: None,
            qty_type: None,
            peg_offset_value: None,
            peg_price_type: None,
            deribit_label: None,
            deribit_adv_order_type: None,
            deribit_mm_protection: None,
            deribit_condition_trigger_method: None,
        }
    }

    /// Set custom label
    pub fn with_label(mut self, label: String) -> Self {
        self.deribit_label = Some(label);
        self
    }

    /// Set time in force
    pub fn with_time_in_force(mut self, tif: TimeInForce) -> Self {
        self.time_in_force = Some(tif);
        self
    }

    /// Set as post only order
    pub fn post_only(mut self) -> Self {
        self.exec_inst = Some("6".to_string());
        self
    }

    /// Set as reduce only order
    pub fn reduce_only(mut self) -> Self {
        self.exec_inst = Some("E".to_string());
        self
    }

    /// Set as post only and reduce only order
    pub fn post_only_reduce_only(mut self) -> Self {
        self.exec_inst = Some("6E".to_string());
        self
    }

    /// Set stop price for stop orders
    pub fn with_stop_price(mut self, stop_px: f64) -> Self {
        self.stop_px = Some(stop_px);
        self
    }

    /// Set display quantity for iceberg orders
    pub fn with_display_qty(mut self, display_qty: f64) -> Self {
        self.display_qty = Some(display_qty);
        self
    }

    /// Set quantity type
    pub fn with_qty_type(mut self, qty_type: QuantityType) -> Self {
        self.qty_type = Some(qty_type);
        self
    }

    /// Set Market Maker Protection flag
    pub fn with_mmp(mut self, enabled: bool) -> Self {
        self.deribit_mm_protection = Some(enabled);
        self
    }

    /// Convert to FIX message
    pub fn to_fix_message(
        &self,
        sender_comp_id: &str,
        target_comp_id: &str,
        msg_seq_num: u32,
    ) -> DeribitFixResult<String> {
        let mut builder = MessageBuilder::new()
            .msg_type(MsgType::NewOrderSingle)
            .sender_comp_id(sender_comp_id.to_string())
            .target_comp_id(target_comp_id.to_string())
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now());

        // Required fields
        builder = builder
            .field(11, self.cl_ord_id.clone()) // ClOrdID
            .field(54, char::from(self.side).to_string()) // Side
            .field(38, self.order_qty.to_string()) // OrderQty
            .field(44, self.price.to_string()) // Price
            .field(55, self.symbol.clone()); // Symbol

        // Optional fields
        if let Some(valid_until_time) = &self.valid_until_time {
            builder = builder.field(
                62,
                valid_until_time.format("%Y%m%d-%H:%M:%S%.3f").to_string(),
            );
        }

        if let Some(exec_inst) = &self.exec_inst {
            builder = builder.field(18, exec_inst.clone());
        }

        if let Some(ord_type) = &self.ord_type {
            builder = builder.field(40, char::from(*ord_type).to_string());
        }

        if let Some(time_in_force) = &self.time_in_force {
            builder = builder.field(59, char::from(*time_in_force).to_string());
        }

        if let Some(stop_px) = &self.stop_px {
            builder = builder.field(99, stop_px.to_string());
        }

        if let Some(display_qty) = &self.display_qty {
            builder = builder.field(1138, display_qty.to_string());
        }

        if let Some(refresh_qty) = &self.refresh_qty {
            builder = builder.field(1088, refresh_qty.to_string());
        }

        if let Some(qty_type) = &self.qty_type {
            builder = builder.field(854, i32::from(*qty_type).to_string());
        }

        if let Some(peg_offset_value) = &self.peg_offset_value {
            builder = builder.field(211, peg_offset_value.to_string());
        }

        if let Some(peg_price_type) = &self.peg_price_type {
            builder = builder.field(1094, peg_price_type.to_string());
        }

        if let Some(deribit_label) = &self.deribit_label {
            builder = builder.field(100010, deribit_label.clone());
        }

        if let Some(deribit_adv_order_type) = &self.deribit_adv_order_type {
            builder = builder.field(100012, deribit_adv_order_type.to_string());
        }

        if let Some(deribit_mm_protection) = &self.deribit_mm_protection {
            builder = builder.field(
                9008,
                if *deribit_mm_protection { "Y" } else { "N" }.to_string(),
            );
        }

        if let Some(deribit_condition_trigger_method) = &self.deribit_condition_trigger_method {
            builder = builder.field(5127, deribit_condition_trigger_method.to_string());
        }

        Ok(builder.build()?.to_string())
    }
}

impl_json_display!(NewOrderSingle);
impl_json_debug_pretty!(NewOrderSingle);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_order_single_market_creation() {
        let order = NewOrderSingle::market(
            "ORDER123".to_string(),
            OrderSide::Buy,
            10.0,
            "BTC-PERPETUAL".to_string(),
        );

        assert_eq!(order.cl_ord_id, "ORDER123");
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.order_qty, 10.0);
        assert_eq!(order.price, 0.0);
        assert_eq!(order.symbol, "BTC-PERPETUAL");
        assert_eq!(order.ord_type, Some(OrderType::Market));
    }

    #[test]
    fn test_new_order_single_limit_creation() {
        let order = NewOrderSingle::limit(
            "ORDER456".to_string(),
            OrderSide::Sell,
            5.0,
            50000.0,
            "BTC-PERPETUAL".to_string(),
        );

        assert_eq!(order.cl_ord_id, "ORDER456");
        assert_eq!(order.side, OrderSide::Sell);
        assert_eq!(order.order_qty, 5.0);
        assert_eq!(order.price, 50000.0);
        assert_eq!(order.symbol, "BTC-PERPETUAL");
        assert_eq!(order.ord_type, Some(OrderType::Limit));
    }

    #[test]
    fn test_new_order_single_with_label() {
        let order = NewOrderSingle::limit(
            "ORDER789".to_string(),
            OrderSide::Buy,
            1.0,
            45000.0,
            "BTC-PERPETUAL".to_string(),
        )
        .with_label("my-order".to_string());

        assert_eq!(order.deribit_label, Some("my-order".to_string()));
    }

    #[test]
    fn test_new_order_single_post_only() {
        let order = NewOrderSingle::limit(
            "ORDER101".to_string(),
            OrderSide::Buy,
            1.0,
            45000.0,
            "BTC-PERPETUAL".to_string(),
        )
        .post_only();

        assert_eq!(order.exec_inst, Some("6".to_string()));
    }

    #[test]
    fn test_new_order_single_reduce_only() {
        let order = NewOrderSingle::limit(
            "ORDER102".to_string(),
            OrderSide::Sell,
            1.0,
            55000.0,
            "BTC-PERPETUAL".to_string(),
        )
        .reduce_only();

        assert_eq!(order.exec_inst, Some("E".to_string()));
    }

    #[test]
    fn test_new_order_single_to_fix_message() {
        let order = NewOrderSingle::limit(
            "ORDER123".to_string(),
            OrderSide::Buy,
            10.0,
            50000.0,
            "BTC-PERPETUAL".to_string(),
        )
        .with_label("test-order".to_string());

        let fix_message = order.to_fix_message("CLIENT", "DERIBITSERVER", 1);
        assert!(fix_message.is_ok());

        let message = fix_message.unwrap();
        assert!(message.contains("35=D")); // MsgType = NewOrderSingle
        assert!(message.contains("11=ORDER123")); // ClOrdID
        assert!(message.contains("54=1")); // Side = Buy
        assert!(message.contains("38=10")); // OrderQty
        assert!(message.contains("44=50000")); // Price
        assert!(message.contains("55=BTC-PERPETUAL")); // Symbol
        assert!(message.contains("100010=test-order")); // DeribitLabel
    }
}
