/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 10/8/25
******************************************************************************/

//! Mass Quote Acknowledgement FIX Message Implementation

use crate::error::Result as DeribitFixResult;
use crate::message::builder::MessageBuilder;
use crate::message::orders::OrderSide;
use crate::model::types::MsgType;
use chrono::Utc;
use deribit_base::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

/// Quote acknowledgement status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuoteAckStatus {
    /// Received - not yet processed
    Received,
    /// Accepted
    Accepted,
    /// Rejected
    Rejected,
}

impl From<QuoteAckStatus> for i32 {
    fn from(status: QuoteAckStatus) -> Self {
        match status {
            QuoteAckStatus::Received => 0,
            QuoteAckStatus::Accepted => 1,
            QuoteAckStatus::Rejected => 5,
        }
    }
}

impl TryFrom<i32> for QuoteAckStatus {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(QuoteAckStatus::Received),
            1 => Ok(QuoteAckStatus::Accepted),
            5 => Ok(QuoteAckStatus::Rejected),
            _ => Err(format!("Invalid QuoteAckStatus: {}", value)),
        }
    }
}

/// Quote reject reason enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuoteRejectReason {
    /// Unknown symbol
    UnknownSymbol,
    /// Exchange closed
    ExchangeClosed,
    /// Quote exceeds limit
    QuoteExceedsLimit,
    /// Too late to enter
    TooLateToEnter,
    /// Unknown quote
    UnknownQuote,
    /// Duplicate quote
    DuplicateQuote,
    /// Invalid bid/ask spread
    InvalidBidAskSpread,
    /// Invalid price
    InvalidPrice,
    /// Not authorized to quote security
    NotAuthorizedToQuoteSecurity,
    /// Price exceeds current price band
    PriceExceedsPriceBand,
    /// Quote locked - Try again
    QuoteLockedTryAgain,
    /// Invalid or unknown security issuer
    InvalidOrUnknownSecurityIssuer,
    /// Invalid or unknown issuer of underlying security
    InvalidOrUnknownIssuerOfUnderlyingSecurity,
    /// Other
    Other,
}

impl From<QuoteRejectReason> for i32 {
    fn from(reason: QuoteRejectReason) -> Self {
        match reason {
            QuoteRejectReason::UnknownSymbol => 1,
            QuoteRejectReason::ExchangeClosed => 2,
            QuoteRejectReason::QuoteExceedsLimit => 3,
            QuoteRejectReason::TooLateToEnter => 4,
            QuoteRejectReason::UnknownQuote => 5,
            QuoteRejectReason::DuplicateQuote => 6,
            QuoteRejectReason::InvalidBidAskSpread => 7,
            QuoteRejectReason::InvalidPrice => 8,
            QuoteRejectReason::NotAuthorizedToQuoteSecurity => 9,
            QuoteRejectReason::PriceExceedsPriceBand => 10,
            QuoteRejectReason::QuoteLockedTryAgain => 11,
            QuoteRejectReason::InvalidOrUnknownSecurityIssuer => 12,
            QuoteRejectReason::InvalidOrUnknownIssuerOfUnderlyingSecurity => 13,
            QuoteRejectReason::Other => 99,
        }
    }
}

impl TryFrom<i32> for QuoteRejectReason {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(QuoteRejectReason::UnknownSymbol),
            2 => Ok(QuoteRejectReason::ExchangeClosed),
            3 => Ok(QuoteRejectReason::QuoteExceedsLimit),
            4 => Ok(QuoteRejectReason::TooLateToEnter),
            5 => Ok(QuoteRejectReason::UnknownQuote),
            6 => Ok(QuoteRejectReason::DuplicateQuote),
            7 => Ok(QuoteRejectReason::InvalidBidAskSpread),
            8 => Ok(QuoteRejectReason::InvalidPrice),
            9 => Ok(QuoteRejectReason::NotAuthorizedToQuoteSecurity),
            10 => Ok(QuoteRejectReason::PriceExceedsPriceBand),
            11 => Ok(QuoteRejectReason::QuoteLockedTryAgain),
            12 => Ok(QuoteRejectReason::InvalidOrUnknownSecurityIssuer),
            13 => Ok(QuoteRejectReason::InvalidOrUnknownIssuerOfUnderlyingSecurity),
            99 => Ok(QuoteRejectReason::Other),
            _ => Err(format!("Invalid QuoteRejectReason: {}", value)),
        }
    }
}

/// Quote entry acknowledgement for mass quote ack
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct QuoteEntryAck {
    /// Quote entry ID
    pub quote_entry_id: String,
    /// Instrument symbol
    pub symbol: String,
    /// Quote acknowledgement status
    pub quote_ack_status: QuoteAckStatus,
    /// Quote reject reason
    pub quote_reject_reason: Option<QuoteRejectReason>,
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
    /// Text
    pub text: Option<String>,
}

impl QuoteEntryAck {
    /// Create a new quote entry acknowledgement
    pub fn new(quote_entry_id: String, symbol: String, quote_ack_status: QuoteAckStatus) -> Self {
        Self {
            quote_entry_id,
            symbol,
            quote_ack_status,
            quote_reject_reason: None,
            side: None,
            bid_px: None,
            offer_px: None,
            bid_size: None,
            offer_size: None,
            text: None,
        }
    }

    /// Create an accepted quote entry acknowledgement
    pub fn accepted(
        quote_entry_id: String,
        symbol: String,
        bid_px: Option<f64>,
        offer_px: Option<f64>,
        bid_size: Option<f64>,
        offer_size: Option<f64>,
    ) -> Self {
        Self {
            quote_entry_id,
            symbol,
            quote_ack_status: QuoteAckStatus::Accepted,
            quote_reject_reason: None,
            side: None,
            bid_px,
            offer_px,
            bid_size,
            offer_size,
            text: None,
        }
    }

    /// Create a rejected quote entry acknowledgement
    pub fn rejected(
        quote_entry_id: String,
        symbol: String,
        reject_reason: QuoteRejectReason,
        text: Option<String>,
    ) -> Self {
        Self {
            quote_entry_id,
            symbol,
            quote_ack_status: QuoteAckStatus::Rejected,
            quote_reject_reason: Some(reject_reason),
            side: None,
            bid_px: None,
            offer_px: None,
            bid_size: None,
            offer_size: None,
            text,
        }
    }
}

/// Mass Quote Acknowledgement message (MsgType = 'b')
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct MassQuoteAcknowledgement {
    /// Quote request ID
    pub quote_req_id: Option<String>,
    /// Quote ID
    pub quote_id: String,
    /// Quote acknowledgement status
    pub quote_ack_status: QuoteAckStatus,
    /// Quote reject reason
    pub quote_reject_reason: Option<QuoteRejectReason>,
    /// Quote response level
    pub quote_resp_level: Option<i32>,
    /// Quote set ID
    pub quote_set_id: Option<String>,
    /// Total quote entries
    pub tot_quote_entries: Option<i32>,
    /// Quote entry acknowledgements
    pub quote_entry_acks: Vec<QuoteEntryAck>,
    /// Account
    pub account: Option<String>,
    /// Clearing account
    pub clearing_account: Option<String>,
    /// Text
    pub text: Option<String>,
    /// Custom label
    pub deribit_label: Option<String>,
    /// Use standard FIX repeating groups instead of simplified custom tags
    pub use_standard_repeating_groups: bool,
}

impl MassQuoteAcknowledgement {
    /// Create a new mass quote acknowledgement
    pub fn new(quote_id: String, quote_ack_status: QuoteAckStatus) -> Self {
        Self {
            quote_req_id: None,
            quote_id,
            quote_ack_status,
            quote_reject_reason: None,
            quote_resp_level: None,
            quote_set_id: None,
            tot_quote_entries: None,
            quote_entry_acks: Vec::new(),
            account: None,
            clearing_account: None,
            text: None,
            deribit_label: None,
            use_standard_repeating_groups: false, // Default to simplified custom tags for backward compatibility
        }
    }

    /// Create an accepted mass quote acknowledgement
    pub fn accepted(
        quote_id: String,
        quote_set_id: String,
        quote_entry_acks: Vec<QuoteEntryAck>,
    ) -> Self {
        let tot_quote_entries = quote_entry_acks.len() as i32;

        Self {
            quote_req_id: None,
            quote_id,
            quote_ack_status: QuoteAckStatus::Accepted,
            quote_reject_reason: None,
            quote_resp_level: None,
            quote_set_id: Some(quote_set_id),
            tot_quote_entries: Some(tot_quote_entries),
            quote_entry_acks,
            account: None,
            clearing_account: None,
            text: None,
            deribit_label: None,
            use_standard_repeating_groups: false, // Default to simplified custom tags for backward compatibility
        }
    }

    /// Create a rejected mass quote acknowledgement
    pub fn rejected(
        quote_id: String,
        reject_reason: QuoteRejectReason,
        text: Option<String>,
    ) -> Self {
        Self {
            quote_req_id: None,
            quote_id,
            quote_ack_status: QuoteAckStatus::Rejected,
            quote_reject_reason: Some(reject_reason),
            quote_resp_level: None,
            quote_set_id: None,
            tot_quote_entries: None,
            quote_entry_acks: Vec::new(),
            account: None,
            clearing_account: None,
            text,
            deribit_label: None,
            use_standard_repeating_groups: false, // Default to simplified custom tags for backward compatibility
        }
    }

    /// Add a quote entry acknowledgement
    pub fn add_quote_entry_ack(mut self, entry_ack: QuoteEntryAck) -> Self {
        self.quote_entry_acks.push(entry_ack);
        self.tot_quote_entries = Some(self.quote_entry_acks.len() as i32);
        self
    }

    /// Set quote request ID
    pub fn with_quote_req_id(mut self, quote_req_id: String) -> Self {
        self.quote_req_id = Some(quote_req_id);
        self
    }

    /// Set quote response level
    pub fn with_quote_resp_level(mut self, quote_resp_level: i32) -> Self {
        self.quote_resp_level = Some(quote_resp_level);
        self
    }

    /// Set account
    pub fn with_account(mut self, account: String) -> Self {
        self.account = Some(account);
        self
    }

    /// Set custom label
    pub fn with_label(mut self, label: String) -> Self {
        self.deribit_label = Some(label);
        self
    }

    /// Enable standard FIX repeating groups (instead of simplified custom tags)
    pub fn enable_standard_repeating_groups(mut self) -> Self {
        self.use_standard_repeating_groups = true;
        self
    }

    /// Disable standard FIX repeating groups (use simplified custom tags for backward compatibility)
    pub fn disable_standard_repeating_groups(mut self) -> Self {
        self.use_standard_repeating_groups = false;
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
            .msg_type(MsgType::MassQuoteAcknowledgement)
            .sender_comp_id(sender_comp_id.to_string())
            .target_comp_id(target_comp_id.to_string())
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now());

        // Required fields
        builder = builder
            .field(117, self.quote_id.clone()) // QuoteID
            .field(297, i32::from(self.quote_ack_status).to_string()); // QuoteAckStatus

        // Optional fields
        if let Some(quote_req_id) = &self.quote_req_id {
            builder = builder.field(131, quote_req_id.clone());
        }

        if let Some(quote_reject_reason) = &self.quote_reject_reason {
            builder = builder.field(300, i32::from(*quote_reject_reason).to_string());
        }

        if let Some(quote_resp_level) = &self.quote_resp_level {
            builder = builder.field(301, quote_resp_level.to_string());
        }

        if let Some(quote_set_id) = &self.quote_set_id {
            builder = builder.field(302, quote_set_id.clone());
        }

        if let Some(tot_quote_entries) = &self.tot_quote_entries {
            builder = builder.field(295, tot_quote_entries.to_string());
        }

        if let Some(account) = &self.account {
            builder = builder.field(1, account.clone());
        }

        if let Some(text) = &self.text {
            builder = builder.field(58, text.clone());
        }

        if let Some(deribit_label) = &self.deribit_label {
            builder = builder.field(100010, deribit_label.clone());
        }

        // Add quote entry acknowledgements - support both standard FIX repeating groups and simplified custom tags
        if self.use_standard_repeating_groups {
            // Standard FIX repeating groups implementation
            builder = builder.field(295, self.quote_entry_acks.len().to_string()); // NoQuoteEntries
            
            for entry_ack in &self.quote_entry_acks {
                builder = builder
                    .field(299, entry_ack.quote_entry_id.clone()) // QuoteEntryID
                    .field(9020, i32::from(entry_ack.quote_ack_status).to_string()); // QuoteEntryType (0 = order, 1 = trade, 2 = error)

                if let Some(quote_set_id) = &self.quote_set_id {
                    builder = builder.field(302, quote_set_id.clone()); // QuoteSetID
                }

                builder = builder.field(1167, i32::from(entry_ack.quote_ack_status).to_string()); // QuoteEntryStatus

                builder = builder.field(55, entry_ack.symbol.clone()); // Symbol

                if let Some(side) = &entry_ack.side {
                    builder = builder.field(54, char::from(*side).to_string()); // Side
                }

                if let Some(bid_px) = &entry_ack.bid_px {
                    builder = builder.field(132, bid_px.to_string()); // BidPx
                }

                if let Some(offer_px) = &entry_ack.offer_px {
                    builder = builder.field(133, offer_px.to_string()); // OfferPx
                }

                if let Some(bid_size) = &entry_ack.bid_size {
                    builder = builder.field(134, bid_size.to_string()); // BidSize
                }

                if let Some(offer_size) = &entry_ack.offer_size {
                    builder = builder.field(135, offer_size.to_string()); // OfferSize
                }

                if let Some(quote_reject_reason) = &entry_ack.quote_reject_reason {
                    builder = builder.field(368, i32::from(*quote_reject_reason).to_string()); // QuoteEntryRejectReason
                }

                if let Some(text) = &entry_ack.text {
                    builder = builder.field(58, text.clone()); // Text
                }
            }
        } else {
            // Simplified custom tags implementation (backward compatibility)
            for (i, entry_ack) in self.quote_entry_acks.iter().enumerate() {
                let base_tag = 3000 + (i * 100); // Custom tag range for quote entry acks

                builder = builder
                    .field(base_tag as u32, entry_ack.quote_entry_id.clone()) // QuoteEntryID
                    .field((base_tag + 1) as u32, entry_ack.symbol.clone()) // Symbol
                    .field(
                        (base_tag + 2) as u32,
                        i32::from(entry_ack.quote_ack_status).to_string(),
                    ); // QuoteAckStatus

                if let Some(quote_reject_reason) = &entry_ack.quote_reject_reason {
                    builder = builder.field(
                        (base_tag + 3) as u32,
                        i32::from(*quote_reject_reason).to_string(),
                    );
                }

                if let Some(side) = &entry_ack.side {
                    builder = builder.field((base_tag + 4) as u32, char::from(*side).to_string());
                }

                if let Some(bid_px) = &entry_ack.bid_px {
                    builder = builder.field((base_tag + 10) as u32, bid_px.to_string());
                }

                if let Some(offer_px) = &entry_ack.offer_px {
                    builder = builder.field((base_tag + 11) as u32, offer_px.to_string());
                }

                if let Some(bid_size) = &entry_ack.bid_size {
                    builder = builder.field((base_tag + 12) as u32, bid_size.to_string());
                }

                if let Some(offer_size) = &entry_ack.offer_size {
                    builder = builder.field((base_tag + 13) as u32, offer_size.to_string());
                }

                if let Some(text) = &entry_ack.text {
                    builder = builder.field((base_tag + 20) as u32, text.clone());
                }
            }
        }

        Ok(builder.build()?.to_string())
    }
}

impl_json_display!(MassQuoteAcknowledgement, QuoteEntryAck);
impl_json_debug_pretty!(MassQuoteAcknowledgement, QuoteEntryAck);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quote_entry_ack_creation() {
        let entry_ack = QuoteEntryAck::new(
            "QEA123".to_string(),
            "BTC-PERPETUAL".to_string(),
            QuoteAckStatus::Accepted,
        );

        assert_eq!(entry_ack.quote_entry_id, "QEA123");
        assert_eq!(entry_ack.symbol, "BTC-PERPETUAL");
        assert_eq!(entry_ack.quote_ack_status, QuoteAckStatus::Accepted);
        assert!(entry_ack.quote_reject_reason.is_none());
    }

    #[test]
    fn test_quote_entry_ack_accepted() {
        let entry_ack = QuoteEntryAck::accepted(
            "QEA456".to_string(),
            "ETH-PERPETUAL".to_string(),
            Some(3200.0),
            Some(3205.0),
            Some(10.0),
            Some(8.0),
        );

        assert_eq!(entry_ack.quote_ack_status, QuoteAckStatus::Accepted);
        assert_eq!(entry_ack.bid_px, Some(3200.0));
        assert_eq!(entry_ack.offer_px, Some(3205.0));
        assert_eq!(entry_ack.bid_size, Some(10.0));
        assert_eq!(entry_ack.offer_size, Some(8.0));
    }

    #[test]
    fn test_quote_entry_ack_rejected() {
        let entry_ack = QuoteEntryAck::rejected(
            "QEA789".to_string(),
            "BTC-PERPETUAL".to_string(),
            QuoteRejectReason::InvalidPrice,
            Some("Price out of range".to_string()),
        );

        assert_eq!(entry_ack.quote_ack_status, QuoteAckStatus::Rejected);
        assert_eq!(
            entry_ack.quote_reject_reason,
            Some(QuoteRejectReason::InvalidPrice)
        );
        assert_eq!(entry_ack.text, Some("Price out of range".to_string()));
    }

    #[test]
    fn test_mass_quote_acknowledgement_creation() {
        let mass_quote_ack =
            MassQuoteAcknowledgement::new("MQA123".to_string(), QuoteAckStatus::Received);

        assert_eq!(mass_quote_ack.quote_id, "MQA123");
        assert_eq!(mass_quote_ack.quote_ack_status, QuoteAckStatus::Received);
        assert!(mass_quote_ack.quote_entry_acks.is_empty());
    }

    #[test]
    fn test_mass_quote_acknowledgement_accepted() {
        let entry_ack1 = QuoteEntryAck::accepted(
            "QEA1".to_string(),
            "BTC-PERPETUAL".to_string(),
            Some(50000.0),
            Some(50010.0),
            Some(5.0),
            Some(3.0),
        );
        let entry_ack2 = QuoteEntryAck::accepted(
            "QEA2".to_string(),
            "ETH-PERPETUAL".to_string(),
            Some(3200.0),
            Some(3205.0),
            Some(10.0),
            Some(8.0),
        );

        let mass_quote_ack = MassQuoteAcknowledgement::accepted(
            "MQA456".to_string(),
            "QS789".to_string(),
            vec![entry_ack1, entry_ack2],
        );

        assert_eq!(mass_quote_ack.quote_ack_status, QuoteAckStatus::Accepted);
        assert_eq!(mass_quote_ack.quote_set_id, Some("QS789".to_string()));
        assert_eq!(mass_quote_ack.tot_quote_entries, Some(2));
        assert_eq!(mass_quote_ack.quote_entry_acks.len(), 2);
    }

    #[test]
    fn test_mass_quote_acknowledgement_rejected() {
        let mass_quote_ack = MassQuoteAcknowledgement::rejected(
            "MQA999".to_string(),
            QuoteRejectReason::ExchangeClosed,
            Some("Exchange is closed".to_string()),
        );

        assert_eq!(mass_quote_ack.quote_ack_status, QuoteAckStatus::Rejected);
        assert_eq!(
            mass_quote_ack.quote_reject_reason,
            Some(QuoteRejectReason::ExchangeClosed)
        );
        assert_eq!(mass_quote_ack.text, Some("Exchange is closed".to_string()));
        assert!(mass_quote_ack.quote_entry_acks.is_empty());
    }

    #[test]
    fn test_mass_quote_acknowledgement_add_entry() {
        let entry_ack = QuoteEntryAck::new(
            "QEA1".to_string(),
            "BTC-PERPETUAL".to_string(),
            QuoteAckStatus::Accepted,
        );

        let mass_quote_ack =
            MassQuoteAcknowledgement::new("MQA123".to_string(), QuoteAckStatus::Accepted)
                .add_quote_entry_ack(entry_ack);

        assert_eq!(mass_quote_ack.tot_quote_entries, Some(1));
        assert_eq!(mass_quote_ack.quote_entry_acks.len(), 1);
    }

    #[test]
    fn test_mass_quote_acknowledgement_to_fix_message() {
        let entry_ack = QuoteEntryAck::accepted(
            "QEA1".to_string(),
            "BTC-PERPETUAL".to_string(),
            Some(50000.0),
            Some(50010.0),
            Some(5.0),
            Some(3.0),
        );

        let mass_quote_ack = MassQuoteAcknowledgement::accepted(
            "MQA123".to_string(),
            "QS456".to_string(),
            vec![entry_ack],
        )
        .with_label("test-label".to_string());

        let fix_message = mass_quote_ack
            .to_fix_message("SENDER", "TARGET", 1)
            .unwrap();

        // Check that the message contains required fields
        assert!(fix_message.contains("35=b")); // MsgType
        assert!(fix_message.contains("117=MQA123")); // QuoteID
        assert!(fix_message.contains("297=1")); // QuoteAckStatus=Accepted
        assert!(fix_message.contains("302=QS456")); // QuoteSetID
        assert!(fix_message.contains("295=1")); // TotQuoteEntries
        assert!(fix_message.contains("100010=test-label")); // Custom label

        // Check quote entry ack fields (simplified check)
        assert!(fix_message.contains("3000=QEA1")); // QuoteEntryID
        assert!(fix_message.contains("3001=BTC-PERPETUAL")); // Symbol
        assert!(fix_message.contains("3002=1")); // QuoteAckStatus=Accepted
    }

    #[test]
    fn test_quote_ack_status_conversions() {
        assert_eq!(i32::from(QuoteAckStatus::Received), 0);
        assert_eq!(i32::from(QuoteAckStatus::Accepted), 1);
        assert_eq!(i32::from(QuoteAckStatus::Rejected), 5);

        assert_eq!(
            QuoteAckStatus::try_from(0).unwrap(),
            QuoteAckStatus::Received
        );
        assert_eq!(
            QuoteAckStatus::try_from(1).unwrap(),
            QuoteAckStatus::Accepted
        );
        assert_eq!(
            QuoteAckStatus::try_from(5).unwrap(),
            QuoteAckStatus::Rejected
        );

        assert!(QuoteAckStatus::try_from(99).is_err());
    }

    #[test]
    fn test_quote_reject_reason_conversions() {
        assert_eq!(i32::from(QuoteRejectReason::UnknownSymbol), 1);
        assert_eq!(i32::from(QuoteRejectReason::ExchangeClosed), 2);
        assert_eq!(i32::from(QuoteRejectReason::Other), 99);

        assert_eq!(
            QuoteRejectReason::try_from(1).unwrap(),
            QuoteRejectReason::UnknownSymbol
        );
        assert_eq!(
            QuoteRejectReason::try_from(2).unwrap(),
            QuoteRejectReason::ExchangeClosed
        );
        assert_eq!(
            QuoteRejectReason::try_from(99).unwrap(),
            QuoteRejectReason::Other
        );

        assert!(QuoteRejectReason::try_from(50).is_err());
    }
}
