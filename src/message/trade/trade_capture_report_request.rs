/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 12/8/25
******************************************************************************/

//! Trade Capture Report Request FIX Message Implementation

use crate::error::Result as DeribitFixResult;
use crate::message::builder::MessageBuilder;
use crate::message::orders::OrderSide;
use crate::model::types::MsgType;
use chrono::{DateTime, Utc};
use deribit_base::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

/// Trade capture report request type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeCaptureRequestType {
    /// All trades
    AllTrades,
    /// Matched trades matching criteria
    MatchedTradesMatchingCriteria,
    /// Unmatched trades that meet criteria
    UnmatchedTrades,
    /// Advisories that meet criteria
    Advisories,
}

impl From<TradeCaptureRequestType> for i32 {
    fn from(request_type: TradeCaptureRequestType) -> Self {
        match request_type {
            TradeCaptureRequestType::AllTrades => 0,
            TradeCaptureRequestType::MatchedTradesMatchingCriteria => 1,
            TradeCaptureRequestType::UnmatchedTrades => 2,
            TradeCaptureRequestType::Advisories => 3,
        }
    }
}

impl TryFrom<i32> for TradeCaptureRequestType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TradeCaptureRequestType::AllTrades),
            1 => Ok(TradeCaptureRequestType::MatchedTradesMatchingCriteria),
            2 => Ok(TradeCaptureRequestType::UnmatchedTrades),
            3 => Ok(TradeCaptureRequestType::Advisories),
            _ => Err(format!("Invalid TradeCaptureRequestType: {}", value)),
        }
    }
}

/// Subscription request type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubscriptionRequestType {
    /// Snapshot
    Snapshot,
    /// Snapshot plus updates
    SnapshotPlusUpdates,
    /// Disable previous snapshot plus update request
    DisablePrevious,
}

impl From<SubscriptionRequestType> for char {
    fn from(sub_type: SubscriptionRequestType) -> Self {
        match sub_type {
            SubscriptionRequestType::Snapshot => '0',
            SubscriptionRequestType::SnapshotPlusUpdates => '1',
            SubscriptionRequestType::DisablePrevious => '2',
        }
    }
}

impl TryFrom<char> for SubscriptionRequestType {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(SubscriptionRequestType::Snapshot),
            '1' => Ok(SubscriptionRequestType::SnapshotPlusUpdates),
            '2' => Ok(SubscriptionRequestType::DisablePrevious),
            _ => Err(format!("Invalid SubscriptionRequestType: {}", value)),
        }
    }
}

/// Trade Capture Report Request message (MsgType = 'AD')
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeCaptureReportRequest {
    /// Trade request ID
    pub trade_request_id: String,
    /// Trade request type
    pub trade_request_type: TradeCaptureRequestType,
    /// Subscription request type
    pub subscription_request_type: Option<SubscriptionRequestType>,
    /// Trade report ID
    pub trade_report_id: Option<String>,
    /// Instrument symbol
    pub symbol: Option<String>,
    /// Side
    pub side: Option<OrderSide>,
    /// Order quantity
    pub order_qty: Option<f64>,
    /// Transaction time from
    pub transact_time_from: Option<DateTime<Utc>>,
    /// Transaction time to
    pub transact_time_to: Option<DateTime<Utc>>,
    /// Clearing business date
    pub clearing_business_date: Option<String>,
    /// Trade date
    pub trade_date: Option<String>,
    /// Account
    pub account: Option<String>,
    /// Clearing account
    pub clearing_account: Option<String>,
    /// Market segment ID
    pub market_segment_id: Option<String>,
    /// Trading session ID
    pub trading_session_id: Option<String>,
    /// Trading session sub ID
    pub trading_session_sub_id: Option<String>,
    /// Text
    pub text: Option<String>,
    /// Custom label
    pub deribit_label: Option<String>,
}

impl TradeCaptureReportRequest {
    /// Create a new trade capture report request
    pub fn new(trade_request_id: String, trade_request_type: TradeCaptureRequestType) -> Self {
        Self {
            trade_request_id,
            trade_request_type,
            subscription_request_type: None,
            trade_report_id: None,
            symbol: None,
            side: None,
            order_qty: None,
            transact_time_from: None,
            transact_time_to: None,
            clearing_business_date: None,
            trade_date: None,
            account: None,
            clearing_account: None,
            market_segment_id: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            text: None,
            deribit_label: None,
        }
    }

    /// Create a request for all trades
    pub fn all_trades(trade_request_id: String) -> Self {
        Self::new(trade_request_id, TradeCaptureRequestType::AllTrades)
    }

    /// Create a request for matched trades
    pub fn matched_trades(trade_request_id: String) -> Self {
        Self::new(trade_request_id, TradeCaptureRequestType::MatchedTradesMatchingCriteria)
    }

    /// Create a request for a specific symbol
    pub fn for_symbol(trade_request_id: String, symbol: String) -> Self {
        let mut request = Self::new(trade_request_id, TradeCaptureRequestType::MatchedTradesMatchingCriteria);
        request.symbol = Some(symbol);
        request
    }

    /// Set subscription request type
    pub fn with_subscription_type(mut self, sub_type: SubscriptionRequestType) -> Self {
        self.subscription_request_type = Some(sub_type);
        self
    }

    /// Set trade report ID
    pub fn with_trade_report_id(mut self, trade_report_id: String) -> Self {
        self.trade_report_id = Some(trade_report_id);
        self
    }

    /// Set symbol
    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    /// Set side
    pub fn with_side(mut self, side: OrderSide) -> Self {
        self.side = Some(side);
        self
    }

    /// Set order quantity
    pub fn with_order_qty(mut self, order_qty: f64) -> Self {
        self.order_qty = Some(order_qty);
        self
    }

    /// Set transaction time range
    pub fn with_time_range(mut self, from: DateTime<Utc>, to: DateTime<Utc>) -> Self {
        self.transact_time_from = Some(from);
        self.transact_time_to = Some(to);
        self
    }

    /// Set account
    pub fn with_account(mut self, account: String) -> Self {
        self.account = Some(account);
        self
    }

    /// Set trading session ID
    pub fn with_trading_session_id(mut self, trading_session_id: String) -> Self {
        self.trading_session_id = Some(trading_session_id);
        self
    }

    /// Set text
    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    /// Set custom label
    pub fn with_label(mut self, label: String) -> Self {
        self.deribit_label = Some(label);
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
            .msg_type(MsgType::TradeCaptureReportRequest)
            .sender_comp_id(sender_comp_id.to_string())
            .target_comp_id(target_comp_id.to_string())
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now());

        // Required fields
        builder = builder
            .field(568, self.trade_request_id.clone()) // TradeRequestID
            .field(569, i32::from(self.trade_request_type).to_string()); // TradeRequestType

        // Optional fields
        if let Some(subscription_request_type) = &self.subscription_request_type {
            builder = builder.field(263, char::from(*subscription_request_type).to_string());
        }

        if let Some(trade_report_id) = &self.trade_report_id {
            builder = builder.field(571, trade_report_id.clone());
        }

        if let Some(symbol) = &self.symbol {
            builder = builder.field(55, symbol.clone());
        }

        if let Some(side) = &self.side {
            builder = builder.field(54, char::from(*side).to_string());
        }

        if let Some(order_qty) = &self.order_qty {
            builder = builder.field(38, order_qty.to_string());
        }

        if let Some(transact_time_from) = &self.transact_time_from {
            builder = builder.field(
                60, // Using TransactTime field for from time
                transact_time_from.format("%Y%m%d-%H:%M:%S%.3f").to_string(),
            );
        }

        if let Some(transact_time_to) = &self.transact_time_to {
            builder = builder.field(
                126, // Using ExpireTime field for to time
                transact_time_to.format("%Y%m%d-%H:%M:%S%.3f").to_string(),
            );
        }

        if let Some(clearing_business_date) = &self.clearing_business_date {
            builder = builder.field(715, clearing_business_date.clone());
        }

        if let Some(trade_date) = &self.trade_date {
            builder = builder.field(75, trade_date.clone());
        }

        if let Some(account) = &self.account {
            builder = builder.field(1, account.clone());
        }

        if let Some(clearing_account) = &self.clearing_account {
            builder = builder.field(440, clearing_account.clone());
        }

        if let Some(market_segment_id) = &self.market_segment_id {
            builder = builder.field(1300, market_segment_id.clone());
        }

        if let Some(trading_session_id) = &self.trading_session_id {
            builder = builder.field(336, trading_session_id.clone());
        }

        if let Some(trading_session_sub_id) = &self.trading_session_sub_id {
            builder = builder.field(625, trading_session_sub_id.clone());
        }

        if let Some(text) = &self.text {
            builder = builder.field(58, text.clone());
        }

        if let Some(deribit_label) = &self.deribit_label {
            builder = builder.field(100010, deribit_label.clone());
        }

        Ok(builder.build()?.to_string())
    }
}

impl_json_display!(TradeCaptureReportRequest);
impl_json_debug_pretty!(TradeCaptureReportRequest);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trade_capture_report_request_creation() {
        let request = TradeCaptureReportRequest::new(
            "TR123".to_string(),
            TradeCaptureRequestType::AllTrades,
        );

        assert_eq!(request.trade_request_id, "TR123");
        assert_eq!(request.trade_request_type, TradeCaptureRequestType::AllTrades);
        assert!(request.symbol.is_none());
        assert!(request.side.is_none());
    }

    #[test]
    fn test_trade_capture_report_request_all_trades() {
        let request = TradeCaptureReportRequest::all_trades("TR456".to_string());

        assert_eq!(request.trade_request_type, TradeCaptureRequestType::AllTrades);
        assert_eq!(request.trade_request_id, "TR456");
    }

    #[test]
    fn test_trade_capture_report_request_matched_trades() {
        let request = TradeCaptureReportRequest::matched_trades("TR789".to_string());

        assert_eq!(request.trade_request_type, TradeCaptureRequestType::MatchedTradesMatchingCriteria);
        assert_eq!(request.trade_request_id, "TR789");
    }

    #[test]
    fn test_trade_capture_report_request_for_symbol() {
        let request = TradeCaptureReportRequest::for_symbol(
            "TR999".to_string(),
            "BTC-PERPETUAL".to_string(),
        );

        assert_eq!(request.trade_request_type, TradeCaptureRequestType::MatchedTradesMatchingCriteria);
        assert_eq!(request.symbol, Some("BTC-PERPETUAL".to_string()));
    }

    #[test]
    fn test_trade_capture_report_request_with_options() {
        let from_time = Utc::now() - chrono::Duration::hours(1);
        let to_time = Utc::now();

        let request = TradeCaptureReportRequest::new(
            "TR111".to_string(),
            TradeCaptureRequestType::MatchedTradesMatchingCriteria,
        )
        .with_subscription_type(SubscriptionRequestType::SnapshotPlusUpdates)
        .with_trade_report_id("TRP123".to_string())
        .with_symbol("ETH-PERPETUAL".to_string())
        .with_side(OrderSide::Buy)
        .with_order_qty(10.0)
        .with_time_range(from_time, to_time)
        .with_account("ACC123".to_string())
        .with_trading_session_id("SESSION1".to_string())
        .with_text("Trade capture request".to_string())
        .with_label("test-trade-capture".to_string());

        assert_eq!(request.subscription_request_type, Some(SubscriptionRequestType::SnapshotPlusUpdates));
        assert_eq!(request.trade_report_id, Some("TRP123".to_string()));
        assert_eq!(request.symbol, Some("ETH-PERPETUAL".to_string()));
        assert_eq!(request.side, Some(OrderSide::Buy));
        assert_eq!(request.order_qty, Some(10.0));
        assert_eq!(request.transact_time_from, Some(from_time));
        assert_eq!(request.transact_time_to, Some(to_time));
        assert_eq!(request.account, Some("ACC123".to_string()));
        assert_eq!(request.trading_session_id, Some("SESSION1".to_string()));
        assert_eq!(request.text, Some("Trade capture request".to_string()));
        assert_eq!(request.deribit_label, Some("test-trade-capture".to_string()));
    }

    #[test]
    fn test_trade_capture_report_request_to_fix_message() {
        let request = TradeCaptureReportRequest::for_symbol(
            "TR123".to_string(),
            "BTC-PERPETUAL".to_string(),
        )
        .with_subscription_type(SubscriptionRequestType::Snapshot)
        .with_label("test-label".to_string());

        let fix_message = request.to_fix_message("SENDER", "TARGET", 1).unwrap();

        // Check that the message contains required fields
        assert!(fix_message.contains("35=AD")); // MsgType
        assert!(fix_message.contains("568=TR123")); // TradeRequestID
        assert!(fix_message.contains("569=1")); // TradeRequestType=MatchedTradesMatchingCriteria
        assert!(fix_message.contains("55=BTC-PERPETUAL")); // Symbol
        assert!(fix_message.contains("263=0")); // SubscriptionRequestType=Snapshot
        assert!(fix_message.contains("100010=test-label")); // Custom label
    }

    #[test]
    fn test_trade_capture_request_type_conversions() {
        assert_eq!(i32::from(TradeCaptureRequestType::AllTrades), 0);
        assert_eq!(i32::from(TradeCaptureRequestType::MatchedTradesMatchingCriteria), 1);
        assert_eq!(i32::from(TradeCaptureRequestType::UnmatchedTrades), 2);
        assert_eq!(i32::from(TradeCaptureRequestType::Advisories), 3);

        assert_eq!(TradeCaptureRequestType::try_from(0).unwrap(), TradeCaptureRequestType::AllTrades);
        assert_eq!(TradeCaptureRequestType::try_from(1).unwrap(), TradeCaptureRequestType::MatchedTradesMatchingCriteria);
        assert_eq!(TradeCaptureRequestType::try_from(2).unwrap(), TradeCaptureRequestType::UnmatchedTrades);
        assert_eq!(TradeCaptureRequestType::try_from(3).unwrap(), TradeCaptureRequestType::Advisories);

        assert!(TradeCaptureRequestType::try_from(99).is_err());
    }

    #[test]
    fn test_subscription_request_type_conversions() {
        assert_eq!(char::from(SubscriptionRequestType::Snapshot), '0');
        assert_eq!(char::from(SubscriptionRequestType::SnapshotPlusUpdates), '1');
        assert_eq!(char::from(SubscriptionRequestType::DisablePrevious), '2');

        assert_eq!(SubscriptionRequestType::try_from('0').unwrap(), SubscriptionRequestType::Snapshot);
        assert_eq!(SubscriptionRequestType::try_from('1').unwrap(), SubscriptionRequestType::SnapshotPlusUpdates);
        assert_eq!(SubscriptionRequestType::try_from('2').unwrap(), SubscriptionRequestType::DisablePrevious);

        assert!(SubscriptionRequestType::try_from('9').is_err());
    }
}