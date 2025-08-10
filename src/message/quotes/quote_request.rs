/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 10/8/25
******************************************************************************/

//! Quote Request FIX Message Implementation

use crate::error::Result as DeribitFixResult;
use crate::message::builder::MessageBuilder;
use crate::message::orders::{OrderSide, TimeInForce};
use crate::model::types::MsgType;
use chrono::{DateTime, Utc};
use deribit_base::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

/// Quote type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuoteType {
    /// Indicative quote
    Indicative,
    /// Tradeable quote
    Tradeable,
    /// Restricted tradeable quote
    RestrictedTradeable,
    /// Counter quote
    Counter,
}

impl From<QuoteType> for i32 {
    fn from(quote_type: QuoteType) -> Self {
        match quote_type {
            QuoteType::Indicative => 0,
            QuoteType::Tradeable => 1,
            QuoteType::RestrictedTradeable => 2,
            QuoteType::Counter => 3,
        }
    }
}

impl TryFrom<i32> for QuoteType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(QuoteType::Indicative),
            1 => Ok(QuoteType::Tradeable),
            2 => Ok(QuoteType::RestrictedTradeable),
            3 => Ok(QuoteType::Counter),
            _ => Err(format!("Invalid QuoteType: {}", value)),
        }
    }
}

/// Quote Request message (MsgType = 'R')
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct QuoteRequest {
    /// Quote request ID
    pub quote_req_id: String,
    /// Instrument symbol
    pub symbol: String,
    /// Quote type
    pub quote_type: QuoteType,
    /// Side of quote request
    pub side: OrderSide,
    /// Order quantity
    pub order_qty: f64,
    /// Valid until time
    pub valid_until_time: Option<DateTime<Utc>>,
    /// Quote request type (0=Manual, 1=Automatic)
    pub quote_request_type: Option<i32>,
    /// Time in force
    pub time_in_force: Option<TimeInForce>,
    /// Minimum quantity
    pub min_qty: Option<f64>,
    /// Settlement type
    pub settlement_type: Option<char>,
    /// Custom label
    pub deribit_label: Option<String>,
    /// Market segment ID
    pub market_segment_id: Option<String>,
}

impl QuoteRequest {
    /// Create a new quote request
    pub fn new(
        quote_req_id: String,
        symbol: String,
        quote_type: QuoteType,
        side: OrderSide,
        order_qty: f64,
    ) -> Self {
        Self {
            quote_req_id,
            symbol,
            quote_type,
            side,
            order_qty,
            valid_until_time: None,
            quote_request_type: None,
            time_in_force: None,
            min_qty: None,
            settlement_type: None,
            deribit_label: None,
            market_segment_id: None,
        }
    }

    /// Create a tradeable quote request
    pub fn tradeable(
        quote_req_id: String,
        symbol: String,
        side: OrderSide,
        order_qty: f64,
    ) -> Self {
        Self::new(quote_req_id, symbol, QuoteType::Tradeable, side, order_qty)
    }

    /// Create an indicative quote request
    pub fn indicative(
        quote_req_id: String,
        symbol: String,
        side: OrderSide,
        order_qty: f64,
    ) -> Self {
        Self::new(quote_req_id, symbol, QuoteType::Indicative, side, order_qty)
    }

    /// Set valid until time
    pub fn with_valid_until(mut self, valid_until: DateTime<Utc>) -> Self {
        self.valid_until_time = Some(valid_until);
        self
    }

    /// Set quote request type
    pub fn with_quote_request_type(mut self, request_type: i32) -> Self {
        self.quote_request_type = Some(request_type);
        self
    }

    /// Set time in force
    pub fn with_time_in_force(mut self, tif: TimeInForce) -> Self {
        self.time_in_force = Some(tif);
        self
    }

    /// Set minimum quantity
    pub fn with_min_qty(mut self, min_qty: f64) -> Self {
        self.min_qty = Some(min_qty);
        self
    }

    /// Set settlement type
    pub fn with_settlement_type(mut self, settlement_type: char) -> Self {
        self.settlement_type = Some(settlement_type);
        self
    }

    /// Set custom label
    pub fn with_label(mut self, label: String) -> Self {
        self.deribit_label = Some(label);
        self
    }

    /// Set market segment ID
    pub fn with_market_segment_id(mut self, segment_id: String) -> Self {
        self.market_segment_id = Some(segment_id);
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
            .msg_type(MsgType::QuoteRequest)
            .sender_comp_id(sender_comp_id.to_string())
            .target_comp_id(target_comp_id.to_string())
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now());

        // Required fields
        builder = builder
            .field(131, self.quote_req_id.clone()) // QuoteReqID
            .field(55, self.symbol.clone()) // Symbol
            .field(537, i32::from(self.quote_type).to_string()) // QuoteType
            .field(54, char::from(self.side).to_string()) // Side
            .field(38, self.order_qty.to_string()); // OrderQty

        // Optional fields
        if let Some(valid_until_time) = &self.valid_until_time {
            builder = builder.field(
                62,
                valid_until_time.format("%Y%m%d-%H:%M:%S%.3f").to_string(),
            );
        }

        if let Some(quote_request_type) = &self.quote_request_type {
            builder = builder.field(303, quote_request_type.to_string());
        }

        if let Some(time_in_force) = &self.time_in_force {
            builder = builder.field(59, char::from(*time_in_force).to_string());
        }

        if let Some(min_qty) = &self.min_qty {
            builder = builder.field(110, min_qty.to_string());
        }

        if let Some(settlement_type) = &self.settlement_type {
            builder = builder.field(63, settlement_type.to_string());
        }

        if let Some(deribit_label) = &self.deribit_label {
            builder = builder.field(100010, deribit_label.clone());
        }

        if let Some(market_segment_id) = &self.market_segment_id {
            builder = builder.field(1300, market_segment_id.clone());
        }

        Ok(builder.build()?.to_string())
    }
}

impl_json_display!(QuoteRequest);
impl_json_debug_pretty!(QuoteRequest);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quote_request_creation() {
        let request = QuoteRequest::new(
            "QR123".to_string(),
            "BTC-PERPETUAL".to_string(),
            QuoteType::Tradeable,
            OrderSide::Buy,
            10.0,
        );

        assert_eq!(request.quote_req_id, "QR123");
        assert_eq!(request.symbol, "BTC-PERPETUAL");
        assert_eq!(request.quote_type, QuoteType::Tradeable);
        assert_eq!(request.side, OrderSide::Buy);
        assert_eq!(request.order_qty, 10.0);
    }

    #[test]
    fn test_quote_request_tradeable() {
        let request = QuoteRequest::tradeable(
            "QR456".to_string(),
            "ETH-PERPETUAL".to_string(),
            OrderSide::Sell,
            5.0,
        );

        assert_eq!(request.quote_type, QuoteType::Tradeable);
        assert_eq!(request.side, OrderSide::Sell);
        assert_eq!(request.order_qty, 5.0);
    }

    #[test]
    fn test_quote_request_indicative() {
        let request = QuoteRequest::indicative(
            "QR789".to_string(),
            "BTC-PERPETUAL".to_string(),
            OrderSide::Buy,
            15.0,
        );

        assert_eq!(request.quote_type, QuoteType::Indicative);
        assert_eq!(request.order_qty, 15.0);
    }

    #[test]
    fn test_quote_request_with_options() {
        let valid_until = Utc::now() + chrono::Duration::hours(1);
        let request = QuoteRequest::new(
            "QR999".to_string(),
            "ETH-PERPETUAL".to_string(),
            QuoteType::RestrictedTradeable,
            OrderSide::Buy,
            20.0,
        )
        .with_valid_until(valid_until)
        .with_quote_request_type(1)
        .with_time_in_force(TimeInForce::GoodTillCancelled)
        .with_min_qty(5.0)
        .with_settlement_type('0')
        .with_label("test-quote".to_string())
        .with_market_segment_id("DERIBIT".to_string());

        assert_eq!(request.valid_until_time, Some(valid_until));
        assert_eq!(request.quote_request_type, Some(1));
        assert_eq!(request.time_in_force, Some(TimeInForce::GoodTillCancelled));
        assert_eq!(request.min_qty, Some(5.0));
        assert_eq!(request.settlement_type, Some('0'));
        assert_eq!(request.deribit_label, Some("test-quote".to_string()));
        assert_eq!(request.market_segment_id, Some("DERIBIT".to_string()));
    }

    #[test]
    fn test_quote_request_to_fix_message() {
        let request = QuoteRequest::tradeable(
            "QR123".to_string(),
            "BTC-PERPETUAL".to_string(),
            OrderSide::Buy,
            10.0,
        )
        .with_label("test-label".to_string());

        let fix_message = request.to_fix_message("SENDER", "TARGET", 1).unwrap();

        // Check that the message contains required fields
        assert!(fix_message.contains("35=R")); // MsgType
        assert!(fix_message.contains("131=QR123")); // QuoteReqID
        assert!(fix_message.contains("55=BTC-PERPETUAL")); // Symbol
        assert!(fix_message.contains("537=1")); // QuoteType=Tradeable
        assert!(fix_message.contains("54=1")); // Side=Buy
        assert!(fix_message.contains("38=10")); // OrderQty
        assert!(fix_message.contains("100010=test-label")); // Custom label
    }

    #[test]
    fn test_quote_type_conversions() {
        assert_eq!(i32::from(QuoteType::Indicative), 0);
        assert_eq!(i32::from(QuoteType::Tradeable), 1);
        assert_eq!(i32::from(QuoteType::RestrictedTradeable), 2);
        assert_eq!(i32::from(QuoteType::Counter), 3);

        assert_eq!(QuoteType::try_from(0).unwrap(), QuoteType::Indicative);
        assert_eq!(QuoteType::try_from(1).unwrap(), QuoteType::Tradeable);
        assert_eq!(
            QuoteType::try_from(2).unwrap(),
            QuoteType::RestrictedTradeable
        );
        assert_eq!(QuoteType::try_from(3).unwrap(), QuoteType::Counter);

        assert!(QuoteType::try_from(99).is_err());
    }
}
