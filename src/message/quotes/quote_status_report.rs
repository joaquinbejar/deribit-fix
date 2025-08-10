/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 10/8/25
******************************************************************************/

//! Quote Status Report FIX Message Implementation

use crate::error::Result as DeribitFixResult;
use crate::message::builder::MessageBuilder;
use crate::message::orders::OrderSide;
use crate::model::types::MsgType;
use chrono::{DateTime, Utc};
use deribit_base::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

/// Quote status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuoteStatus {
    /// Quote accepted
    Accepted,
    /// Quote canceled for symbol
    CanceledForSymbol,
    /// Quote canceled for security type
    CanceledForSecurityType,
    /// Quote canceled for underlying
    CanceledForUnderlying,
    /// Quote canceled - all
    CanceledAll,
    /// Quote rejected
    Rejected,
    /// Quote removed from market
    RemovedFromMarket,
    /// Quote expired
    Expired,
    /// Query
    Query,
    /// Quote not found
    QuoteNotFound,
    /// Pending
    Pending,
    /// Pass
    Pass,
    /// Locked market warning
    LockedMarketWarning,
    /// Cross market warning
    CrossMarketWarning,
    /// Canceled due to lock market
    CanceledDueToLockMarket,
    /// Canceled due to cross market
    CanceledDueToCrossMarket,
}

impl From<QuoteStatus> for i32 {
    fn from(status: QuoteStatus) -> Self {
        match status {
            QuoteStatus::Accepted => 0,
            QuoteStatus::CanceledForSymbol => 1,
            QuoteStatus::CanceledForSecurityType => 2,
            QuoteStatus::CanceledForUnderlying => 3,
            QuoteStatus::CanceledAll => 4,
            QuoteStatus::Rejected => 5,
            QuoteStatus::RemovedFromMarket => 6,
            QuoteStatus::Expired => 7,
            QuoteStatus::Query => 8,
            QuoteStatus::QuoteNotFound => 9,
            QuoteStatus::Pending => 10,
            QuoteStatus::Pass => 11,
            QuoteStatus::LockedMarketWarning => 12,
            QuoteStatus::CrossMarketWarning => 13,
            QuoteStatus::CanceledDueToLockMarket => 14,
            QuoteStatus::CanceledDueToCrossMarket => 15,
        }
    }
}

impl TryFrom<i32> for QuoteStatus {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(QuoteStatus::Accepted),
            1 => Ok(QuoteStatus::CanceledForSymbol),
            2 => Ok(QuoteStatus::CanceledForSecurityType),
            3 => Ok(QuoteStatus::CanceledForUnderlying),
            4 => Ok(QuoteStatus::CanceledAll),
            5 => Ok(QuoteStatus::Rejected),
            6 => Ok(QuoteStatus::RemovedFromMarket),
            7 => Ok(QuoteStatus::Expired),
            8 => Ok(QuoteStatus::Query),
            9 => Ok(QuoteStatus::QuoteNotFound),
            10 => Ok(QuoteStatus::Pending),
            11 => Ok(QuoteStatus::Pass),
            12 => Ok(QuoteStatus::LockedMarketWarning),
            13 => Ok(QuoteStatus::CrossMarketWarning),
            14 => Ok(QuoteStatus::CanceledDueToLockMarket),
            15 => Ok(QuoteStatus::CanceledDueToCrossMarket),
            _ => Err(format!("Invalid QuoteStatus: {}", value)),
        }
    }
}

/// Quote Status Report message (MsgType = 'AI')
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct QuoteStatusReport {
    /// Quote status report ID
    pub quote_status_report_id: String,
    /// Quote request ID
    pub quote_req_id: Option<String>,
    /// Quote ID
    pub quote_id: Option<String>,
    /// Quote response level
    pub quote_resp_level: Option<i32>,
    /// Quote status
    pub quote_status: QuoteStatus,
    /// Quote reject reason
    pub quote_reject_reason: Option<i32>,
    /// Instrument symbol
    pub symbol: String,
    /// Side
    pub side: Option<OrderSide>,
    /// Bid price
    pub bid_px: Option<f64>,
    /// Offer price
    pub offer_px: Option<f64>,
    /// Bid size
    pub bid_size: Option<f64>,
    /// Offer size
    pub offer_size: Option<f64>,
    /// Valid until time
    pub valid_until_time: Option<DateTime<Utc>>,
    /// Bid spot rate
    pub bid_spot_rate: Option<f64>,
    /// Offer spot rate
    pub offer_spot_rate: Option<f64>,
    /// Bid forward points
    pub bid_forward_points: Option<f64>,
    /// Offer forward points
    pub offer_forward_points: Option<f64>,
    /// Mid price
    pub mid_px: Option<f64>,
    /// Bid yield
    pub bid_yield: Option<f64>,
    /// Mid yield
    pub mid_yield: Option<f64>,
    /// Offer yield
    pub offer_yield: Option<f64>,
    /// Transaction time
    pub transact_time: DateTime<Utc>,
    /// Trading session ID
    pub trading_session_id: Option<String>,
    /// Text
    pub text: Option<String>,
    /// Custom label
    pub deribit_label: Option<String>,
}

impl QuoteStatusReport {
    /// Create a new quote status report
    pub fn new(quote_status_report_id: String, quote_status: QuoteStatus, symbol: String) -> Self {
        Self {
            quote_status_report_id,
            quote_req_id: None,
            quote_id: None,
            quote_resp_level: None,
            quote_status,
            quote_reject_reason: None,
            symbol,
            side: None,
            bid_px: None,
            offer_px: None,
            bid_size: None,
            offer_size: None,
            valid_until_time: None,
            bid_spot_rate: None,
            offer_spot_rate: None,
            bid_forward_points: None,
            offer_forward_points: None,
            mid_px: None,
            bid_yield: None,
            mid_yield: None,
            offer_yield: None,
            transact_time: Utc::now(),
            trading_session_id: None,
            text: None,
            deribit_label: None,
        }
    }

    /// Create an accepted quote status report
    pub fn accepted(
        quote_status_report_id: String,
        symbol: String,
        bid_px: f64,
        offer_px: f64,
        bid_size: f64,
        offer_size: f64,
    ) -> Self {
        Self {
            quote_status_report_id,
            quote_req_id: None,
            quote_id: None,
            quote_resp_level: None,
            quote_status: QuoteStatus::Accepted,
            quote_reject_reason: None,
            symbol,
            side: None,
            bid_px: Some(bid_px),
            offer_px: Some(offer_px),
            bid_size: Some(bid_size),
            offer_size: Some(offer_size),
            valid_until_time: None,
            bid_spot_rate: None,
            offer_spot_rate: None,
            bid_forward_points: None,
            offer_forward_points: None,
            mid_px: Some((bid_px + offer_px) / 2.0),
            bid_yield: None,
            mid_yield: None,
            offer_yield: None,
            transact_time: Utc::now(),
            trading_session_id: None,
            text: None,
            deribit_label: None,
        }
    }

    /// Create a rejected quote status report
    pub fn rejected(
        quote_status_report_id: String,
        symbol: String,
        reject_reason: i32,
        text: Option<String>,
    ) -> Self {
        Self {
            quote_status_report_id,
            quote_req_id: None,
            quote_id: None,
            quote_resp_level: None,
            quote_status: QuoteStatus::Rejected,
            quote_reject_reason: Some(reject_reason),
            symbol,
            side: None,
            bid_px: None,
            offer_px: None,
            bid_size: None,
            offer_size: None,
            valid_until_time: None,
            bid_spot_rate: None,
            offer_spot_rate: None,
            bid_forward_points: None,
            offer_forward_points: None,
            mid_px: None,
            bid_yield: None,
            mid_yield: None,
            offer_yield: None,
            transact_time: Utc::now(),
            trading_session_id: None,
            text,
            deribit_label: None,
        }
    }

    /// Set quote request ID
    pub fn with_quote_req_id(mut self, quote_req_id: String) -> Self {
        self.quote_req_id = Some(quote_req_id);
        self
    }

    /// Set quote ID
    pub fn with_quote_id(mut self, quote_id: String) -> Self {
        self.quote_id = Some(quote_id);
        self
    }

    /// Set side
    pub fn with_side(mut self, side: OrderSide) -> Self {
        self.side = Some(side);
        self
    }

    /// Set valid until time
    pub fn with_valid_until(mut self, valid_until: DateTime<Utc>) -> Self {
        self.valid_until_time = Some(valid_until);
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
            .msg_type(MsgType::QuoteStatusReport)
            .sender_comp_id(sender_comp_id.to_string())
            .target_comp_id(target_comp_id.to_string())
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now());

        // Required fields
        builder = builder
            .field(649, self.quote_status_report_id.clone()) // QuoteStatusReportID
            .field(297, i32::from(self.quote_status).to_string()) // QuoteStatus
            .field(55, self.symbol.clone()) // Symbol
            .field(
                60,
                self.transact_time.format("%Y%m%d-%H:%M:%S%.3f").to_string(),
            ); // TransactTime

        // Optional fields
        if let Some(quote_req_id) = &self.quote_req_id {
            builder = builder.field(131, quote_req_id.clone());
        }

        if let Some(quote_id) = &self.quote_id {
            builder = builder.field(117, quote_id.clone());
        }

        if let Some(quote_resp_level) = &self.quote_resp_level {
            builder = builder.field(301, quote_resp_level.to_string());
        }

        if let Some(quote_reject_reason) = &self.quote_reject_reason {
            builder = builder.field(300, quote_reject_reason.to_string());
        }

        if let Some(side) = &self.side {
            builder = builder.field(54, char::from(*side).to_string());
        }

        if let Some(bid_px) = &self.bid_px {
            builder = builder.field(132, bid_px.to_string());
        }

        if let Some(offer_px) = &self.offer_px {
            builder = builder.field(133, offer_px.to_string());
        }

        if let Some(bid_size) = &self.bid_size {
            builder = builder.field(134, bid_size.to_string());
        }

        if let Some(offer_size) = &self.offer_size {
            builder = builder.field(135, offer_size.to_string());
        }

        if let Some(valid_until_time) = &self.valid_until_time {
            builder = builder.field(
                62,
                valid_until_time.format("%Y%m%d-%H:%M:%S%.3f").to_string(),
            );
        }

        if let Some(mid_px) = &self.mid_px {
            builder = builder.field(631, mid_px.to_string());
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

impl_json_display!(QuoteStatusReport);
impl_json_debug_pretty!(QuoteStatusReport);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::QuoteCancelType;

    #[test]
    fn test_quote_status_report_creation() {
        let report = QuoteStatusReport::new(
            "QSR123".to_string(),
            QuoteStatus::Accepted,
            "BTC-PERPETUAL".to_string(),
        );

        assert_eq!(report.quote_status_report_id, "QSR123");
        assert_eq!(report.quote_status, QuoteStatus::Accepted);
        assert_eq!(report.symbol, "BTC-PERPETUAL");
    }

    #[test]
    fn test_quote_status_report_accepted() {
        let report = QuoteStatusReport::accepted(
            "QSR456".to_string(),
            "ETH-PERPETUAL".to_string(),
            3200.0,
            3205.0,
            10.0,
            8.0,
        );

        assert_eq!(report.quote_status, QuoteStatus::Accepted);
        assert_eq!(report.bid_px, Some(3200.0));
        assert_eq!(report.offer_px, Some(3205.0));
        assert_eq!(report.bid_size, Some(10.0));
        assert_eq!(report.offer_size, Some(8.0));
        assert_eq!(report.mid_px, Some(3202.5));
    }

    #[test]
    fn test_quote_status_report_rejected() {
        let report = QuoteStatusReport::rejected(
            "QSR789".to_string(),
            "BTC-PERPETUAL".to_string(),
            5,
            Some("Invalid price".to_string()),
        );

        assert_eq!(report.quote_status, QuoteStatus::Rejected);
        assert_eq!(report.quote_reject_reason, Some(5));
        assert_eq!(report.text, Some("Invalid price".to_string()));
    }

    #[test]
    fn test_quote_status_report_with_options() {
        let valid_until = Utc::now() + chrono::Duration::hours(1);
        let report = QuoteStatusReport::new(
            "QSR999".to_string(),
            QuoteStatus::Pending,
            "ETH-PERPETUAL".to_string(),
        )
        .with_quote_req_id("QR123".to_string())
        .with_quote_id("Q456".to_string())
        .with_side(OrderSide::Buy)
        .with_valid_until(valid_until)
        .with_label("test-quote-status".to_string());

        assert_eq!(report.quote_req_id, Some("QR123".to_string()));
        assert_eq!(report.quote_id, Some("Q456".to_string()));
        assert_eq!(report.side, Some(OrderSide::Buy));
        assert_eq!(report.valid_until_time, Some(valid_until));
        assert_eq!(report.deribit_label, Some("test-quote-status".to_string()));
    }

    #[test]
    fn test_quote_status_report_to_fix_message() {
        let report = QuoteStatusReport::accepted(
            "QSR123".to_string(),
            "BTC-PERPETUAL".to_string(),
            50000.0,
            50010.0,
            5.0,
            3.0,
        )
        .with_quote_req_id("QR123".to_string())
        .with_label("test-label".to_string());

        let fix_message = report.to_fix_message("SENDER", "TARGET", 1).unwrap();

        // Check that the message contains required fields
        assert!(fix_message.contains("35=AI")); // MsgType
        assert!(fix_message.contains("649=QSR123")); // QuoteStatusReportID
        assert!(fix_message.contains("297=0")); // QuoteStatus=Accepted
        assert!(fix_message.contains("55=BTC-PERPETUAL")); // Symbol
        assert!(fix_message.contains("131=QR123")); // QuoteReqID
        assert!(fix_message.contains("132=50000")); // BidPx
        assert!(fix_message.contains("133=50010")); // OfferPx
        assert!(fix_message.contains("134=5")); // BidSize
        assert!(fix_message.contains("135=3")); // OfferSize
        assert!(fix_message.contains("631=50005")); // MidPx
        assert!(fix_message.contains("100010=test-label")); // Custom label
    }

    #[test]
    fn test_quote_status_conversions() {
        assert_eq!(i32::from(QuoteStatus::Accepted), 0);
        assert_eq!(i32::from(QuoteStatus::CanceledAll), 4);
        assert_eq!(i32::from(QuoteStatus::Rejected), 5);
        assert_eq!(i32::from(QuoteStatus::Expired), 7);

        assert_eq!(QuoteStatus::try_from(0).unwrap(), QuoteStatus::Accepted);
        assert_eq!(QuoteStatus::try_from(4).unwrap(), QuoteStatus::CanceledAll);
        assert_eq!(QuoteStatus::try_from(5).unwrap(), QuoteStatus::Rejected);
        assert_eq!(QuoteStatus::try_from(7).unwrap(), QuoteStatus::Expired);

        assert!(QuoteStatus::try_from(99).is_err());
    }

    #[test]
    fn test_quote_cancel_type_conversions() {
        assert_eq!(i32::from(QuoteCancelType::Quit), 1);
        assert_eq!(i32::from(QuoteCancelType::CancelForSymbol), 2);
        assert_eq!(i32::from(QuoteCancelType::CancelAll), 5);

        assert_eq!(QuoteCancelType::try_from(1).unwrap(), QuoteCancelType::Quit);
        assert_eq!(
            QuoteCancelType::try_from(2).unwrap(),
            QuoteCancelType::CancelForSymbol
        );
        assert_eq!(
            QuoteCancelType::try_from(5).unwrap(),
            QuoteCancelType::CancelAll
        );

        assert!(QuoteCancelType::try_from(99).is_err());
    }
}
