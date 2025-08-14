/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 12/8/25
******************************************************************************/

//! Trade Capture Report Request Ack FIX Message Implementation

use crate::error::Result as DeribitFixResult;
use crate::message::builder::MessageBuilder;
use crate::model::types::MsgType;
use chrono::Utc;
use deribit_base::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

/// Trade capture report request result enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeCaptureRequestResult {
    /// Successful (default)
    Successful,
    /// Invalid or unknown instrument
    InvalidOrUnknownInstrument,
    /// Invalid type of trade requested
    InvalidTypeOfTradeRequested,
    /// Invalid parties
    InvalidParties,
    /// Invalid transport type requested
    InvalidTransportTypeRequested,
    /// Invalid destination requested
    InvalidDestinationRequested,
    /// TradeRequestType not supported
    TradeRequestTypeNotSupported,
    /// Unauthorized for trade capture report request
    UnauthorizedForTradeCaptureReportRequest,
    /// Other
    Other,
}

impl From<TradeCaptureRequestResult> for i32 {
    fn from(result: TradeCaptureRequestResult) -> Self {
        match result {
            TradeCaptureRequestResult::Successful => 0,
            TradeCaptureRequestResult::InvalidOrUnknownInstrument => 1,
            TradeCaptureRequestResult::InvalidTypeOfTradeRequested => 2,
            TradeCaptureRequestResult::InvalidParties => 3,
            TradeCaptureRequestResult::InvalidTransportTypeRequested => 4,
            TradeCaptureRequestResult::InvalidDestinationRequested => 5,
            TradeCaptureRequestResult::TradeRequestTypeNotSupported => 8,
            TradeCaptureRequestResult::UnauthorizedForTradeCaptureReportRequest => 9,
            TradeCaptureRequestResult::Other => 99,
        }
    }
}

impl TryFrom<i32> for TradeCaptureRequestResult {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TradeCaptureRequestResult::Successful),
            1 => Ok(TradeCaptureRequestResult::InvalidOrUnknownInstrument),
            2 => Ok(TradeCaptureRequestResult::InvalidTypeOfTradeRequested),
            3 => Ok(TradeCaptureRequestResult::InvalidParties),
            4 => Ok(TradeCaptureRequestResult::InvalidTransportTypeRequested),
            5 => Ok(TradeCaptureRequestResult::InvalidDestinationRequested),
            8 => Ok(TradeCaptureRequestResult::TradeRequestTypeNotSupported),
            9 => Ok(TradeCaptureRequestResult::UnauthorizedForTradeCaptureReportRequest),
            99 => Ok(TradeCaptureRequestResult::Other),
            _ => Err(format!("Invalid TradeCaptureRequestResult: {}", value)),
        }
    }
}

/// Trade capture request status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeCaptureRequestStatus {
    /// Accepted
    Accepted,
    /// Completed
    Completed,
    /// Rejected
    Rejected,
}

impl From<TradeCaptureRequestStatus> for i32 {
    fn from(status: TradeCaptureRequestStatus) -> Self {
        match status {
            TradeCaptureRequestStatus::Accepted => 0,
            TradeCaptureRequestStatus::Completed => 1,
            TradeCaptureRequestStatus::Rejected => 2,
        }
    }
}

impl TryFrom<i32> for TradeCaptureRequestStatus {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TradeCaptureRequestStatus::Accepted),
            1 => Ok(TradeCaptureRequestStatus::Completed),
            2 => Ok(TradeCaptureRequestStatus::Rejected),
            _ => Err(format!("Invalid TradeCaptureRequestStatus: {}", value)),
        }
    }
}

/// Trade Capture Report Request Ack message (MsgType = 'AQ')
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeCaptureReportRequestAck {
    /// Trade request ID
    pub trade_request_id: String,
    /// Trade request status
    pub trade_request_status: TradeCaptureRequestStatus,
    /// Trade request result
    pub trade_request_result: Option<TradeCaptureRequestResult>,
    /// Trade report ID
    pub trade_report_id: Option<String>,
    /// Instrument symbol
    pub symbol: Option<String>,
    /// Total number of trade reports
    pub tot_num_trade_reports: Option<i32>,
    /// Multi leg reporting type
    pub multi_leg_reporting_type: Option<char>,
    /// Response transport type
    pub response_transport_type: Option<i32>,
    /// Response destination
    pub response_destination: Option<String>,
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
    /// Encoded text
    pub encoded_text: Option<Vec<u8>>,
    /// Custom label
    pub deribit_label: Option<String>,
}

impl TradeCaptureReportRequestAck {
    /// Create a new trade capture report request acknowledgement
    pub fn new(trade_request_id: String, trade_request_status: TradeCaptureRequestStatus) -> Self {
        Self {
            trade_request_id,
            trade_request_status,
            trade_request_result: None,
            trade_report_id: None,
            symbol: None,
            tot_num_trade_reports: None,
            multi_leg_reporting_type: None,
            response_transport_type: None,
            response_destination: None,
            account: None,
            clearing_account: None,
            market_segment_id: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            text: None,
            encoded_text: None,
            deribit_label: None,
        }
    }

    /// Create an accepted acknowledgement
    pub fn accepted(trade_request_id: String) -> Self {
        Self::new(trade_request_id, TradeCaptureRequestStatus::Accepted)
    }

    /// Create a completed acknowledgement
    pub fn completed(trade_request_id: String, tot_num_trade_reports: i32) -> Self {
        let mut ack = Self::new(trade_request_id, TradeCaptureRequestStatus::Completed);
        ack.tot_num_trade_reports = Some(tot_num_trade_reports);
        ack
    }

    /// Create a rejected acknowledgement
    pub fn rejected(
        trade_request_id: String,
        result: TradeCaptureRequestResult,
        text: Option<String>,
    ) -> Self {
        let mut ack = Self::new(trade_request_id, TradeCaptureRequestStatus::Rejected);
        ack.trade_request_result = Some(result);
        ack.text = text;
        ack
    }

    /// Set trade request result
    pub fn with_result(mut self, result: TradeCaptureRequestResult) -> Self {
        self.trade_request_result = Some(result);
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

    /// Set total number of trade reports
    pub fn with_total_trade_reports(mut self, total: i32) -> Self {
        self.tot_num_trade_reports = Some(total);
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
            .msg_type(MsgType::TradeCaptureReportRequestAck)
            .sender_comp_id(sender_comp_id.to_string())
            .target_comp_id(target_comp_id.to_string())
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now());

        // Required fields
        builder = builder
            .field(568, self.trade_request_id.clone()) // TradeRequestID
            .field(749, i32::from(self.trade_request_status).to_string()); // TradeRequestStatus

        // Optional fields
        if let Some(trade_request_result) = &self.trade_request_result {
            builder = builder.field(750, i32::from(*trade_request_result).to_string());
        }

        if let Some(trade_report_id) = &self.trade_report_id {
            builder = builder.field(571, trade_report_id.clone());
        }

        if let Some(symbol) = &self.symbol {
            builder = builder.field(55, symbol.clone());
        }

        if let Some(tot_num_trade_reports) = &self.tot_num_trade_reports {
            builder = builder.field(748, tot_num_trade_reports.to_string());
        }

        if let Some(multi_leg_reporting_type) = &self.multi_leg_reporting_type {
            builder = builder.field(442, multi_leg_reporting_type.to_string());
        }

        if let Some(response_transport_type) = &self.response_transport_type {
            builder = builder.field(725, response_transport_type.to_string());
        }

        if let Some(response_destination) = &self.response_destination {
            builder = builder.field(726, response_destination.clone());
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

impl_json_display!(TradeCaptureReportRequestAck);
impl_json_debug_pretty!(TradeCaptureReportRequestAck);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trade_capture_report_request_ack_creation() {
        let ack = TradeCaptureReportRequestAck::new(
            "TR123".to_string(),
            TradeCaptureRequestStatus::Accepted,
        );

        assert_eq!(ack.trade_request_id, "TR123");
        assert_eq!(
            ack.trade_request_status,
            TradeCaptureRequestStatus::Accepted
        );
        assert!(ack.trade_request_result.is_none());
        assert!(ack.symbol.is_none());
    }

    #[test]
    fn test_trade_capture_report_request_ack_accepted() {
        let ack = TradeCaptureReportRequestAck::accepted("TR456".to_string());

        assert_eq!(
            ack.trade_request_status,
            TradeCaptureRequestStatus::Accepted
        );
        assert_eq!(ack.trade_request_id, "TR456");
    }

    #[test]
    fn test_trade_capture_report_request_ack_completed() {
        let ack = TradeCaptureReportRequestAck::completed("TR789".to_string(), 25);

        assert_eq!(
            ack.trade_request_status,
            TradeCaptureRequestStatus::Completed
        );
        assert_eq!(ack.tot_num_trade_reports, Some(25));
    }

    #[test]
    fn test_trade_capture_report_request_ack_rejected() {
        let ack = TradeCaptureReportRequestAck::rejected(
            "TR999".to_string(),
            TradeCaptureRequestResult::InvalidOrUnknownInstrument,
            Some("Invalid symbol".to_string()),
        );

        assert_eq!(
            ack.trade_request_status,
            TradeCaptureRequestStatus::Rejected
        );
        assert_eq!(
            ack.trade_request_result,
            Some(TradeCaptureRequestResult::InvalidOrUnknownInstrument)
        );
        assert_eq!(ack.text, Some("Invalid symbol".to_string()));
    }

    #[test]
    fn test_trade_capture_report_request_ack_with_options() {
        let ack = TradeCaptureReportRequestAck::new(
            "TR111".to_string(),
            TradeCaptureRequestStatus::Completed,
        )
        .with_result(TradeCaptureRequestResult::Successful)
        .with_trade_report_id("TRP123".to_string())
        .with_symbol("BTC-PERPETUAL".to_string())
        .with_total_trade_reports(10)
        .with_account("ACC123".to_string())
        .with_trading_session_id("SESSION1".to_string())
        .with_text("Trade capture completed".to_string())
        .with_label("test-trade-ack".to_string());

        assert_eq!(
            ack.trade_request_result,
            Some(TradeCaptureRequestResult::Successful)
        );
        assert_eq!(ack.trade_report_id, Some("TRP123".to_string()));
        assert_eq!(ack.symbol, Some("BTC-PERPETUAL".to_string()));
        assert_eq!(ack.tot_num_trade_reports, Some(10));
        assert_eq!(ack.account, Some("ACC123".to_string()));
        assert_eq!(ack.trading_session_id, Some("SESSION1".to_string()));
        assert_eq!(ack.text, Some("Trade capture completed".to_string()));
        assert_eq!(ack.deribit_label, Some("test-trade-ack".to_string()));
    }

    #[test]
    fn test_trade_capture_report_request_ack_to_fix_message() {
        let ack = TradeCaptureReportRequestAck::completed("TR123".to_string(), 5)
            .with_symbol("BTC-PERPETUAL".to_string())
            .with_label("test-label".to_string());

        let fix_message = ack.to_fix_message("SENDER", "TARGET", 1).unwrap();

        // Check that the message contains required fields
        assert!(fix_message.contains("35=AQ")); // MsgType
        assert!(fix_message.contains("568=TR123")); // TradeRequestID
        assert!(fix_message.contains("749=1")); // TradeRequestStatus=Completed
        assert!(fix_message.contains("748=5")); // TotNumTradeReports
        assert!(fix_message.contains("55=BTC-PERPETUAL")); // Symbol
        assert!(fix_message.contains("100010=test-label")); // Custom label
    }

    #[test]
    fn test_trade_capture_request_result_conversions() {
        assert_eq!(i32::from(TradeCaptureRequestResult::Successful), 0);
        assert_eq!(
            i32::from(TradeCaptureRequestResult::InvalidOrUnknownInstrument),
            1
        );
        assert_eq!(
            i32::from(TradeCaptureRequestResult::InvalidTypeOfTradeRequested),
            2
        );
        assert_eq!(i32::from(TradeCaptureRequestResult::Other), 99);

        assert_eq!(
            TradeCaptureRequestResult::try_from(0).unwrap(),
            TradeCaptureRequestResult::Successful
        );
        assert_eq!(
            TradeCaptureRequestResult::try_from(1).unwrap(),
            TradeCaptureRequestResult::InvalidOrUnknownInstrument
        );
        assert_eq!(
            TradeCaptureRequestResult::try_from(2).unwrap(),
            TradeCaptureRequestResult::InvalidTypeOfTradeRequested
        );
        assert_eq!(
            TradeCaptureRequestResult::try_from(99).unwrap(),
            TradeCaptureRequestResult::Other
        );

        assert!(TradeCaptureRequestResult::try_from(50).is_err());
    }

    #[test]
    fn test_trade_capture_request_status_conversions() {
        assert_eq!(i32::from(TradeCaptureRequestStatus::Accepted), 0);
        assert_eq!(i32::from(TradeCaptureRequestStatus::Completed), 1);
        assert_eq!(i32::from(TradeCaptureRequestStatus::Rejected), 2);

        assert_eq!(
            TradeCaptureRequestStatus::try_from(0).unwrap(),
            TradeCaptureRequestStatus::Accepted
        );
        assert_eq!(
            TradeCaptureRequestStatus::try_from(1).unwrap(),
            TradeCaptureRequestStatus::Completed
        );
        assert_eq!(
            TradeCaptureRequestStatus::try_from(2).unwrap(),
            TradeCaptureRequestStatus::Rejected
        );

        assert!(TradeCaptureRequestStatus::try_from(99).is_err());
    }
}
