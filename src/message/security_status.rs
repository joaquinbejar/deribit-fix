//! Security Status Request and Security Status messages
//!
//! This module implements Security Status Request (35=e) and Security Status (35=f) messages
//! for requesting and receiving security status information from the Deribit FIX API.
//!
//! Security Status Request allows clients to request the current status of a security,
//! while Security Status provides the response with trading status and market data.

#![warn(missing_docs)]

use crate::error::{DeribitFixError, Result as DeribitFixResult};
use crate::message::MessageBuilder;
use crate::model::message::FixMessage;
use crate::model::types::MsgType;
use serde::{Deserialize, Serialize};

/// Security Status Request message (35=e)
///
/// Provides the ability to request the status of a security including
/// trading status, volumes, and price information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityStatusRequest {
    /// ID of the request (tag 324)
    pub security_status_req_id: String,
    /// Instrument symbol (tag 55)
    pub symbol: String,
    /// Subscription request type (tag 263)
    /// 0 = Snapshot, 1 = Snapshot + Updates (Subscribe), 2 = Unsubscribe
    pub subscription_request_type: char,
}

impl SecurityStatusRequest {
    /// Create a new Security Status Request
    pub fn new(
        security_status_req_id: String,
        symbol: String,
        subscription_request_type: char,
    ) -> Self {
        Self {
            security_status_req_id,
            symbol,
            subscription_request_type,
        }
    }

    /// Create a snapshot request (no subscription)
    pub fn snapshot(security_status_req_id: String, symbol: String) -> Self {
        Self::new(security_status_req_id, symbol, '0')
    }

    /// Create a subscription request (snapshot + updates)
    pub fn subscribe(security_status_req_id: String, symbol: String) -> Self {
        Self::new(security_status_req_id, symbol, '1')
    }

    /// Create an unsubscribe request
    pub fn unsubscribe(security_status_req_id: String, symbol: String) -> Self {
        Self::new(security_status_req_id, symbol, '2')
    }

    /// Convert to FIX message
    pub fn to_fix_message(
        &self,
        sender_comp_id: String,
        target_comp_id: String,
        msg_seq_num: u32,
    ) -> DeribitFixResult<FixMessage> {
        let builder = MessageBuilder::new()
            .msg_type(MsgType::SecurityStatusRequest)
            .sender_comp_id(sender_comp_id)
            .target_comp_id(target_comp_id)
            .msg_seq_num(msg_seq_num)
            .field(324, self.security_status_req_id.clone()) // SecurityStatusReqID
            .field(55, self.symbol.clone()) // Symbol
            .field(263, self.subscription_request_type.to_string()); // SubscriptionRequestType

        builder.build()
    }
}

/// Security Status message (35=f)
///
/// Provides information about the current status of a security including
/// trading status, volumes, and various price metrics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityStatus {
    /// ID of the request (tag 324) - optional in response
    pub security_status_req_id: Option<String>,
    /// Instrument symbol (tag 55)
    pub symbol: String,
    /// Security trading status (tag 326)
    /// 7 = Ready to trade, 8 = Not Available for trading, 20 = Unknown or Invalid
    pub security_trading_status: Option<i32>,
    /// Volume in buy contracts (tag 330)
    pub buy_volume: Option<f64>,
    /// Volume in sell contracts (tag 331)
    pub sell_volume: Option<f64>,
    /// Price of the 24h highest trade (tag 332)
    pub high_px: Option<f64>,
    /// Price of the 24h lowest trade (tag 333)
    pub low_px: Option<f64>,
    /// The price of the latest trade (tag 31)
    pub last_px: Option<f64>,
    /// Explanatory text (tag 58)
    pub text: Option<String>,
}

impl SecurityStatus {
    /// Create a new Security Status message
    pub fn new(symbol: String) -> Self {
        Self {
            security_status_req_id: None,
            symbol,
            security_trading_status: None,
            buy_volume: None,
            sell_volume: None,
            high_px: None,
            low_px: None,
            last_px: None,
            text: None,
        }
    }

    /// Set the security status request ID
    pub fn with_security_status_req_id(mut self, req_id: String) -> Self {
        self.security_status_req_id = Some(req_id);
        self
    }

    /// Set the trading status
    pub fn with_trading_status(mut self, status: i32) -> Self {
        self.security_trading_status = Some(status);
        self
    }

    /// Set buy volume
    pub fn with_buy_volume(mut self, volume: f64) -> Self {
        self.buy_volume = Some(volume);
        self
    }

    /// Set sell volume
    pub fn with_sell_volume(mut self, volume: f64) -> Self {
        self.sell_volume = Some(volume);
        self
    }

    /// Set high price
    pub fn with_high_px(mut self, price: f64) -> Self {
        self.high_px = Some(price);
        self
    }

    /// Set low price
    pub fn with_low_px(mut self, price: f64) -> Self {
        self.low_px = Some(price);
        self
    }

    /// Set last price
    pub fn with_last_px(mut self, price: f64) -> Self {
        self.last_px = Some(price);
        self
    }

    /// Set explanatory text
    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    /// Parse from FIX message
    pub fn from_fix_message(message: &FixMessage) -> DeribitFixResult<Self> {
        let symbol = message
            .get_field(55)
            .ok_or_else(|| DeribitFixError::MessageParsing("Symbol (55) is required".to_string()))?
            .clone();

        let mut security_status = Self::new(symbol);

        // Optional fields
        if let Some(req_id) = message.get_field(324) {
            security_status.security_status_req_id = Some(req_id.clone());
        }

        if let Some(status_str) = message.get_field(326)
            && let Ok(status) = status_str.parse::<i32>()
        {
            security_status.security_trading_status = Some(status);
        }

        if let Some(buy_vol_str) = message.get_field(330)
            && let Ok(buy_vol) = buy_vol_str.parse::<f64>()
        {
            security_status.buy_volume = Some(buy_vol);
        }

        if let Some(sell_vol_str) = message.get_field(331)
            && let Ok(sell_vol) = sell_vol_str.parse::<f64>()
        {
            security_status.sell_volume = Some(sell_vol);
        }

        if let Some(high_str) = message.get_field(332)
            && let Ok(high) = high_str.parse::<f64>()
        {
            security_status.high_px = Some(high);
        }

        if let Some(low_str) = message.get_field(333)
            && let Ok(low) = low_str.parse::<f64>()
        {
            security_status.low_px = Some(low);
        }

        if let Some(last_str) = message.get_field(31)
            && let Ok(last) = last_str.parse::<f64>()
        {
            security_status.last_px = Some(last);
        }

        if let Some(text) = message.get_field(58) {
            security_status.text = Some(text.clone());
        }

        Ok(security_status)
    }

    /// Convert to FIX message
    pub fn to_fix_message(
        &self,
        sender_comp_id: String,
        target_comp_id: String,
        msg_seq_num: u32,
    ) -> DeribitFixResult<FixMessage> {
        let mut builder = MessageBuilder::new()
            .msg_type(MsgType::SecurityStatus)
            .sender_comp_id(sender_comp_id)
            .target_comp_id(target_comp_id)
            .msg_seq_num(msg_seq_num)
            .field(55, self.symbol.clone()); // Symbol

        // Optional fields
        if let Some(ref req_id) = self.security_status_req_id {
            builder = builder.field(324, req_id.clone()); // SecurityStatusReqID
        }

        if let Some(status) = self.security_trading_status {
            builder = builder.field(326, status.to_string()); // SecurityTradingStatus
        }

        if let Some(buy_vol) = self.buy_volume {
            builder = builder.field(330, buy_vol.to_string()); // BuyVolume
        }

        if let Some(sell_vol) = self.sell_volume {
            builder = builder.field(331, sell_vol.to_string()); // SellVolume
        }

        if let Some(high) = self.high_px {
            builder = builder.field(332, high.to_string()); // HighPx
        }

        if let Some(low) = self.low_px {
            builder = builder.field(333, low.to_string()); // LowPx
        }

        if let Some(last) = self.last_px {
            builder = builder.field(31, last.to_string()); // LastPx
        }

        if let Some(ref text) = self.text {
            builder = builder.field(58, text.clone()); // Text
        }

        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_status_request_creation() {
        let req =
            SecurityStatusRequest::new("REQ123".to_string(), "BTC-PERPETUAL".to_string(), '0');
        assert_eq!(req.security_status_req_id, "REQ123");
        assert_eq!(req.symbol, "BTC-PERPETUAL");
        assert_eq!(req.subscription_request_type, '0');
    }

    #[test]
    fn test_security_status_request_builders() {
        let snapshot =
            SecurityStatusRequest::snapshot("REQ1".to_string(), "BTC-PERPETUAL".to_string());
        assert_eq!(snapshot.subscription_request_type, '0');

        let subscribe =
            SecurityStatusRequest::subscribe("REQ2".to_string(), "BTC-PERPETUAL".to_string());
        assert_eq!(subscribe.subscription_request_type, '1');

        let unsubscribe =
            SecurityStatusRequest::unsubscribe("REQ3".to_string(), "BTC-PERPETUAL".to_string());
        assert_eq!(unsubscribe.subscription_request_type, '2');
    }

    #[test]
    fn test_security_status_request_to_fix_message() {
        let req =
            SecurityStatusRequest::snapshot("REQ123".to_string(), "BTC-PERPETUAL".to_string());
        let fix_msg = req
            .to_fix_message("CLIENT".to_string(), "DERIBIT".to_string(), 1)
            .unwrap();

        assert_eq!(fix_msg.get_field(35).unwrap(), "e"); // MsgType
        assert_eq!(fix_msg.get_field(324).unwrap(), "REQ123"); // SecurityStatusReqID
        assert_eq!(fix_msg.get_field(55).unwrap(), "BTC-PERPETUAL"); // Symbol
        assert_eq!(fix_msg.get_field(263).unwrap(), "0"); // SubscriptionRequestType
    }

    #[test]
    fn test_security_status_creation() {
        let status = SecurityStatus::new("BTC-PERPETUAL".to_string())
            .with_trading_status(7)
            .with_buy_volume(100.0)
            .with_sell_volume(200.0)
            .with_high_px(50000.0)
            .with_low_px(49000.0)
            .with_last_px(49500.0)
            .with_text("success".to_string());

        assert_eq!(status.symbol, "BTC-PERPETUAL");
        assert_eq!(status.security_trading_status, Some(7));
        assert_eq!(status.buy_volume, Some(100.0));
        assert_eq!(status.sell_volume, Some(200.0));
        assert_eq!(status.high_px, Some(50000.0));
        assert_eq!(status.low_px, Some(49000.0));
        assert_eq!(status.last_px, Some(49500.0));
        assert_eq!(status.text, Some("success".to_string()));
    }

    #[test]
    fn test_security_status_to_fix_message() {
        let status = SecurityStatus::new("BTC-PERPETUAL".to_string())
            .with_security_status_req_id("REQ123".to_string())
            .with_trading_status(7)
            .with_last_px(49500.0);

        let fix_msg = status
            .to_fix_message("DERIBIT".to_string(), "CLIENT".to_string(), 2)
            .unwrap();

        assert_eq!(fix_msg.get_field(35).unwrap(), "f"); // MsgType
        assert_eq!(fix_msg.get_field(55).unwrap(), "BTC-PERPETUAL"); // Symbol
        assert_eq!(fix_msg.get_field(324).unwrap(), "REQ123"); // SecurityStatusReqID
        assert_eq!(fix_msg.get_field(326).unwrap(), "7"); // SecurityTradingStatus
        assert_eq!(fix_msg.get_field(31).unwrap(), "49500"); // LastPx
    }
}
