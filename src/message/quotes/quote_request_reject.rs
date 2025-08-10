/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 10/8/25
******************************************************************************/

//! Quote Request Reject FIX Message Implementation

use crate::error::Result as DeribitFixResult;
use crate::message::builder::MessageBuilder;
use crate::model::types::MsgType;
use chrono::Utc;
use deribit_base::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

/// Quote request reject reason enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuoteRequestRejectReason {
    /// Unknown symbol
    UnknownSymbol,
    /// Exchange closed
    ExchangeClosed,
    /// Quote request exceeds limit
    QuoteRequestExceedsLimit,
    /// Too late to enter
    TooLateToEnter,
    /// Invalid price
    InvalidPrice,
    /// Not authorized to request quote
    NotAuthorizedToRequestQuote,
    /// No matching quote
    NoMatchingQuote,
    /// Other
    Other,
}

impl From<QuoteRequestRejectReason> for i32 {
    fn from(reason: QuoteRequestRejectReason) -> Self {
        match reason {
            QuoteRequestRejectReason::UnknownSymbol => 1,
            QuoteRequestRejectReason::ExchangeClosed => 2,
            QuoteRequestRejectReason::QuoteRequestExceedsLimit => 3,
            QuoteRequestRejectReason::TooLateToEnter => 4,
            QuoteRequestRejectReason::InvalidPrice => 6,
            QuoteRequestRejectReason::NotAuthorizedToRequestQuote => 7,
            QuoteRequestRejectReason::NoMatchingQuote => 8,
            QuoteRequestRejectReason::Other => 99,
        }
    }
}

impl TryFrom<i32> for QuoteRequestRejectReason {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(QuoteRequestRejectReason::UnknownSymbol),
            2 => Ok(QuoteRequestRejectReason::ExchangeClosed),
            3 => Ok(QuoteRequestRejectReason::QuoteRequestExceedsLimit),
            4 => Ok(QuoteRequestRejectReason::TooLateToEnter),
            6 => Ok(QuoteRequestRejectReason::InvalidPrice),
            7 => Ok(QuoteRequestRejectReason::NotAuthorizedToRequestQuote),
            8 => Ok(QuoteRequestRejectReason::NoMatchingQuote),
            99 => Ok(QuoteRequestRejectReason::Other),
            _ => Err(format!("Invalid QuoteRequestRejectReason: {}", value)),
        }
    }
}

/// Quote Request Reject message (MsgType = 'AG')
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct QuoteRequestReject {
    /// Quote request ID
    pub quote_req_id: String,
    /// Reject reason
    pub quote_request_reject_reason: QuoteRequestRejectReason,
    /// Reject text
    pub text: Option<String>,
    /// Instrument symbol (optional)
    pub symbol: Option<String>,
    /// No related symbols (group)
    pub no_related_sym: Option<i32>,
    /// Custom label
    pub deribit_label: Option<String>,
}

impl QuoteRequestReject {
    /// Create a new quote request reject
    pub fn new(quote_req_id: String, reject_reason: QuoteRequestRejectReason) -> Self {
        Self {
            quote_req_id,
            quote_request_reject_reason: reject_reason,
            text: None,
            symbol: None,
            no_related_sym: None,
            deribit_label: None,
        }
    }

    /// Create a reject for unknown symbol
    pub fn unknown_symbol(quote_req_id: String, symbol: String) -> Self {
        Self {
            quote_req_id,
            quote_request_reject_reason: QuoteRequestRejectReason::UnknownSymbol,
            text: Some(format!("Unknown symbol: {}", symbol)),
            symbol: Some(symbol),
            no_related_sym: None,
            deribit_label: None,
        }
    }

    /// Create a reject for exchange closed
    pub fn exchange_closed(quote_req_id: String) -> Self {
        Self {
            quote_req_id,
            quote_request_reject_reason: QuoteRequestRejectReason::ExchangeClosed,
            text: Some("Exchange is closed".to_string()),
            symbol: None,
            no_related_sym: None,
            deribit_label: None,
        }
    }

    /// Create a reject for exceeding limits
    pub fn exceeds_limit(quote_req_id: String) -> Self {
        Self {
            quote_req_id,
            quote_request_reject_reason: QuoteRequestRejectReason::QuoteRequestExceedsLimit,
            text: Some("Quote request exceeds limit".to_string()),
            symbol: None,
            no_related_sym: None,
            deribit_label: None,
        }
    }

    /// Set reject text
    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    /// Set symbol
    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    /// Set number of related symbols
    pub fn with_no_related_sym(mut self, no_related_sym: i32) -> Self {
        self.no_related_sym = Some(no_related_sym);
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
            .msg_type(MsgType::QuoteRequestReject)
            .sender_comp_id(sender_comp_id.to_string())
            .target_comp_id(target_comp_id.to_string())
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now());

        // Required fields
        builder = builder
            .field(131, self.quote_req_id.clone()) // QuoteReqID
            .field(658, i32::from(self.quote_request_reject_reason).to_string()); // QuoteRequestRejectReason

        // Optional fields
        if let Some(text) = &self.text {
            builder = builder.field(58, text.clone());
        }

        if let Some(symbol) = &self.symbol {
            builder = builder.field(55, symbol.clone());
        }

        if let Some(no_related_sym) = &self.no_related_sym {
            builder = builder.field(146, no_related_sym.to_string());
        }

        if let Some(deribit_label) = &self.deribit_label {
            builder = builder.field(100010, deribit_label.clone());
        }

        Ok(builder.build()?.to_string())
    }
}

impl_json_display!(QuoteRequestReject);
impl_json_debug_pretty!(QuoteRequestReject);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quote_request_reject_creation() {
        let reject =
            QuoteRequestReject::new("QR123".to_string(), QuoteRequestRejectReason::UnknownSymbol);

        assert_eq!(reject.quote_req_id, "QR123");
        assert_eq!(
            reject.quote_request_reject_reason,
            QuoteRequestRejectReason::UnknownSymbol
        );
        assert!(reject.text.is_none());
        assert!(reject.symbol.is_none());
    }

    #[test]
    fn test_quote_request_reject_unknown_symbol() {
        let reject =
            QuoteRequestReject::unknown_symbol("QR456".to_string(), "INVALID-SYMBOL".to_string());

        assert_eq!(
            reject.quote_request_reject_reason,
            QuoteRequestRejectReason::UnknownSymbol
        );
        assert_eq!(
            reject.text,
            Some("Unknown symbol: INVALID-SYMBOL".to_string())
        );
        assert_eq!(reject.symbol, Some("INVALID-SYMBOL".to_string()));
    }

    #[test]
    fn test_quote_request_reject_exchange_closed() {
        let reject = QuoteRequestReject::exchange_closed("QR789".to_string());

        assert_eq!(
            reject.quote_request_reject_reason,
            QuoteRequestRejectReason::ExchangeClosed
        );
        assert_eq!(reject.text, Some("Exchange is closed".to_string()));
    }

    #[test]
    fn test_quote_request_reject_exceeds_limit() {
        let reject = QuoteRequestReject::exceeds_limit("QR999".to_string());

        assert_eq!(
            reject.quote_request_reject_reason,
            QuoteRequestRejectReason::QuoteRequestExceedsLimit
        );
        assert_eq!(reject.text, Some("Quote request exceeds limit".to_string()));
    }

    #[test]
    fn test_quote_request_reject_with_options() {
        let reject =
            QuoteRequestReject::new("QR111".to_string(), QuoteRequestRejectReason::InvalidPrice)
                .with_text("Price is invalid".to_string())
                .with_symbol("BTC-PERPETUAL".to_string())
                .with_no_related_sym(1)
                .with_label("test-reject".to_string());

        assert_eq!(reject.text, Some("Price is invalid".to_string()));
        assert_eq!(reject.symbol, Some("BTC-PERPETUAL".to_string()));
        assert_eq!(reject.no_related_sym, Some(1));
        assert_eq!(reject.deribit_label, Some("test-reject".to_string()));
    }

    #[test]
    fn test_quote_request_reject_to_fix_message() {
        let reject =
            QuoteRequestReject::unknown_symbol("QR123".to_string(), "BTC-PERPETUAL".to_string())
                .with_label("test-label".to_string());

        let fix_message = reject.to_fix_message("SENDER", "TARGET", 1).unwrap();

        // Check that the message contains required fields
        assert!(fix_message.contains("35=AG")); // MsgType
        assert!(fix_message.contains("131=QR123")); // QuoteReqID
        assert!(fix_message.contains("658=1")); // QuoteRequestRejectReason=UnknownSymbol
        assert!(fix_message.contains("58=Unknown symbol: BTC-PERPETUAL")); // Text
        assert!(fix_message.contains("55=BTC-PERPETUAL")); // Symbol
        assert!(fix_message.contains("100010=test-label")); // Custom label
    }

    #[test]
    fn test_quote_request_reject_reason_conversions() {
        assert_eq!(i32::from(QuoteRequestRejectReason::UnknownSymbol), 1);
        assert_eq!(i32::from(QuoteRequestRejectReason::ExchangeClosed), 2);
        assert_eq!(
            i32::from(QuoteRequestRejectReason::QuoteRequestExceedsLimit),
            3
        );
        assert_eq!(i32::from(QuoteRequestRejectReason::TooLateToEnter), 4);
        assert_eq!(i32::from(QuoteRequestRejectReason::InvalidPrice), 6);
        assert_eq!(
            i32::from(QuoteRequestRejectReason::NotAuthorizedToRequestQuote),
            7
        );
        assert_eq!(i32::from(QuoteRequestRejectReason::NoMatchingQuote), 8);
        assert_eq!(i32::from(QuoteRequestRejectReason::Other), 99);

        assert_eq!(
            QuoteRequestRejectReason::try_from(1).unwrap(),
            QuoteRequestRejectReason::UnknownSymbol
        );
        assert_eq!(
            QuoteRequestRejectReason::try_from(2).unwrap(),
            QuoteRequestRejectReason::ExchangeClosed
        );
        assert_eq!(
            QuoteRequestRejectReason::try_from(3).unwrap(),
            QuoteRequestRejectReason::QuoteRequestExceedsLimit
        );
        assert_eq!(
            QuoteRequestRejectReason::try_from(4).unwrap(),
            QuoteRequestRejectReason::TooLateToEnter
        );
        assert_eq!(
            QuoteRequestRejectReason::try_from(6).unwrap(),
            QuoteRequestRejectReason::InvalidPrice
        );
        assert_eq!(
            QuoteRequestRejectReason::try_from(7).unwrap(),
            QuoteRequestRejectReason::NotAuthorizedToRequestQuote
        );
        assert_eq!(
            QuoteRequestRejectReason::try_from(8).unwrap(),
            QuoteRequestRejectReason::NoMatchingQuote
        );
        assert_eq!(
            QuoteRequestRejectReason::try_from(99).unwrap(),
            QuoteRequestRejectReason::Other
        );

        assert!(QuoteRequestRejectReason::try_from(50).is_err());
    }

    #[test]
    fn test_quote_request_reject_minimal_fix_message() {
        let reject = QuoteRequestReject::new("QR456".to_string(), QuoteRequestRejectReason::Other);

        let fix_message = reject.to_fix_message("SENDER", "TARGET", 2).unwrap();

        // Check required fields only
        assert!(fix_message.contains("35=AG")); // MsgType
        assert!(fix_message.contains("131=QR456")); // QuoteReqID
        assert!(fix_message.contains("658=99")); // QuoteRequestRejectReason=Other

        // Check optional fields are not present when not set
        // Use SOH character (\x01) to be more precise and avoid false matches
        assert!(!fix_message.contains("\x0158=")); // Text field not set
        assert!(!fix_message.contains("\x0155=")); // Symbol field not set  
        assert!(!fix_message.contains("\x01146=")); // NoRelatedSym field not set
    }
}
