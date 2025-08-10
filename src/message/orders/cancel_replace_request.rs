/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 10/8/25
******************************************************************************/

//! Order Cancel/Replace Request FIX Message Implementation

use super::*;
use crate::error::Result as DeribitFixResult;
use crate::message::builder::MessageBuilder;
use crate::model::types::MsgType;
use chrono::{DateTime, Utc};
use deribit_base::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

/// Order Cancel/Replace Request message (MsgType = 'G')
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderCancelReplaceRequest {
    /// Original client order ID
    pub orig_cl_ord_id: String,
    /// New client order ID
    pub cl_ord_id: String,
    /// Instrument symbol
    pub symbol: String,
    /// Side of order
    pub side: OrderSide,
    /// Transaction time
    pub transact_time: DateTime<Utc>,
    /// New order quantity
    pub order_qty: Option<f64>,
    /// New price
    pub price: Option<f64>,
    /// Order type
    pub ord_type: Option<OrderType>,
    /// Time in force
    pub time_in_force: Option<TimeInForce>,
    /// Stop price
    pub stop_px: Option<f64>,
    /// Display quantity
    pub display_qty: Option<f64>,
    /// Quantity type
    pub qty_type: Option<QuantityType>,
    /// Custom label
    pub deribit_label: Option<String>,
    /// Market Maker Protection flag
    pub deribit_mm_protection: Option<bool>,
}

impl OrderCancelReplaceRequest {
    /// Create a new cancel/replace request
    pub fn new(orig_cl_ord_id: String, cl_ord_id: String, symbol: String, side: OrderSide) -> Self {
        Self {
            orig_cl_ord_id,
            cl_ord_id,
            symbol,
            side,
            transact_time: Utc::now(),
            order_qty: None,
            price: None,
            ord_type: None,
            time_in_force: None,
            stop_px: None,
            display_qty: None,
            qty_type: None,
            deribit_label: None,
            deribit_mm_protection: None,
        }
    }

    /// Set new order quantity
    pub fn with_qty(mut self, qty: f64) -> Self {
        self.order_qty = Some(qty);
        self
    }

    /// Set new price
    pub fn with_price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }

    /// Set order type
    pub fn with_order_type(mut self, ord_type: OrderType) -> Self {
        self.ord_type = Some(ord_type);
        self
    }

    /// Set time in force
    pub fn with_time_in_force(mut self, tif: TimeInForce) -> Self {
        self.time_in_force = Some(tif);
        self
    }

    /// Set stop price
    pub fn with_stop_price(mut self, stop_px: f64) -> Self {
        self.stop_px = Some(stop_px);
        self
    }

    /// Set display quantity
    pub fn with_display_qty(mut self, display_qty: f64) -> Self {
        self.display_qty = Some(display_qty);
        self
    }

    /// Set quantity type
    pub fn with_qty_type(mut self, qty_type: QuantityType) -> Self {
        self.qty_type = Some(qty_type);
        self
    }

    /// Set custom label
    pub fn with_label(mut self, label: String) -> Self {
        self.deribit_label = Some(label);
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
            .msg_type(MsgType::OrderCancelReplaceRequest)
            .sender_comp_id(sender_comp_id.to_string())
            .target_comp_id(target_comp_id.to_string())
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now());

        // Required fields
        builder = builder
            .field(41, self.orig_cl_ord_id.clone()) // OrigClOrdID
            .field(11, self.cl_ord_id.clone()) // ClOrdID
            .field(55, self.symbol.clone()) // Symbol
            .field(54, char::from(self.side).to_string()) // Side
            .field(
                60,
                self.transact_time.format("%Y%m%d-%H:%M:%S%.3f").to_string(),
            ); // TransactTime

        // Optional fields
        if let Some(order_qty) = &self.order_qty {
            builder = builder.field(38, order_qty.to_string());
        }

        if let Some(price) = &self.price {
            builder = builder.field(44, price.to_string());
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

        if let Some(qty_type) = &self.qty_type {
            builder = builder.field(854, i32::from(*qty_type).to_string());
        }

        if let Some(deribit_label) = &self.deribit_label {
            builder = builder.field(100010, deribit_label.clone());
        }

        if let Some(deribit_mm_protection) = &self.deribit_mm_protection {
            builder = builder.field(
                9008,
                if *deribit_mm_protection { "Y" } else { "N" }.to_string(),
            );
        }

        Ok(builder.build()?.to_string())
    }
}

impl_json_display!(OrderCancelReplaceRequest);
impl_json_debug_pretty!(OrderCancelReplaceRequest);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_replace_request_creation() {
        let request = OrderCancelReplaceRequest::new(
            "ORIG123".to_string(),
            "NEW123".to_string(),
            "BTC-PERPETUAL".to_string(),
            OrderSide::Buy,
        );

        assert_eq!(request.orig_cl_ord_id, "ORIG123");
        assert_eq!(request.cl_ord_id, "NEW123");
        assert_eq!(request.symbol, "BTC-PERPETUAL");
        assert_eq!(request.side, OrderSide::Buy);
        assert!(request.order_qty.is_none());
        assert!(request.price.is_none());
        assert!(request.ord_type.is_none());
    }

    #[test]
    fn test_cancel_replace_request_with_quantity_and_price() {
        let request = OrderCancelReplaceRequest::new(
            "ORIG123".to_string(),
            "NEW123".to_string(),
            "BTC-PERPETUAL".to_string(),
            OrderSide::Sell,
        )
        .with_qty(15.0)
        .with_price(48000.0);

        assert_eq!(request.order_qty, Some(15.0));
        assert_eq!(request.price, Some(48000.0));
    }

    #[test]
    fn test_cancel_replace_request_with_all_options() {
        let request = OrderCancelReplaceRequest::new(
            "ORIG123".to_string(),
            "NEW123".to_string(),
            "ETH-PERPETUAL".to_string(),
            OrderSide::Buy,
        )
        .with_qty(20.0)
        .with_price(3200.0)
        .with_order_type(OrderType::Limit)
        .with_time_in_force(TimeInForce::GoodTillCancelled)
        .with_stop_price(3100.0)
        .with_display_qty(10.0)
        .with_qty_type(QuantityType::Contracts)
        .with_label("updated-order".to_string())
        .with_mmp(true);

        assert_eq!(request.order_qty, Some(20.0));
        assert_eq!(request.price, Some(3200.0));
        assert_eq!(request.ord_type, Some(OrderType::Limit));
        assert_eq!(request.time_in_force, Some(TimeInForce::GoodTillCancelled));
        assert_eq!(request.stop_px, Some(3100.0));
        assert_eq!(request.display_qty, Some(10.0));
        assert_eq!(request.qty_type, Some(QuantityType::Contracts));
        assert_eq!(request.deribit_label, Some("updated-order".to_string()));
        assert_eq!(request.deribit_mm_protection, Some(true));
    }

    #[test]
    fn test_cancel_replace_request_to_fix_message() {
        let request = OrderCancelReplaceRequest::new(
            "ORIG123".to_string(),
            "NEW123".to_string(),
            "BTC-PERPETUAL".to_string(),
            OrderSide::Buy,
        )
        .with_qty(10.0)
        .with_price(51000.0)
        .with_order_type(OrderType::Limit);

        let fix_message = request.to_fix_message("SENDER", "TARGET", 1).unwrap();

        // Check that the message contains required fields
        assert!(fix_message.contains("35=G")); // MsgType
        assert!(fix_message.contains("41=ORIG123")); // OrigClOrdID
        assert!(fix_message.contains("11=NEW123")); // ClOrdID
        assert!(fix_message.contains("55=BTC-PERPETUAL")); // Symbol
        assert!(fix_message.contains("54=1")); // Side=Buy
        assert!(fix_message.contains("38=10")); // OrderQty
        assert!(fix_message.contains("44=51000")); // Price
        assert!(fix_message.contains("40=2")); // OrdType=Limit
    }

    #[test]
    fn test_cancel_replace_request_minimal_fix_message() {
        let request = OrderCancelReplaceRequest::new(
            "ORIG456".to_string(),
            "NEW456".to_string(),
            "ETH-PERPETUAL".to_string(),
            OrderSide::Sell,
        );

        let fix_message = request.to_fix_message("SENDER", "TARGET", 2).unwrap();

        // Check required fields only
        assert!(fix_message.contains("35=G")); // MsgType
        assert!(fix_message.contains("41=ORIG456")); // OrigClOrdID
        assert!(fix_message.contains("11=NEW456")); // ClOrdID
        assert!(fix_message.contains("55=ETH-PERPETUAL")); // Symbol
        assert!(fix_message.contains("54=2")); // Side=Sell

        // Check optional fields are not present when not set
        assert!(!fix_message.contains("38=")); // OrderQty not set
        assert!(!fix_message.contains("44=")); // Price not set
        assert!(!fix_message.contains("40=")); // OrdType not set
    }

    #[test]
    fn test_cancel_replace_request_with_label_and_mmp() {
        let request = OrderCancelReplaceRequest::new(
            "ORIG789".to_string(),
            "NEW789".to_string(),
            "BTC-PERPETUAL".to_string(),
            OrderSide::Buy,
        )
        .with_label("strategy-v2".to_string())
        .with_mmp(false);

        let fix_message = request.to_fix_message("SENDER", "TARGET", 3).unwrap();

        assert!(fix_message.contains("100010=strategy-v2")); // Custom label
        assert!(fix_message.contains("9008=N")); // MMP disabled
    }
}
