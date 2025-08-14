/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 10/8/25
******************************************************************************/

//! Quote Cancel FIX Message Implementation

use crate::error::Result as DeribitFixResult;
use crate::message::builder::MessageBuilder;
use crate::message::orders::OrderSide;
use crate::model::types::MsgType;
use chrono::Utc;
use deribit_base::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

/// Quote cancel type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuoteCancelType {
    /// Quit
    Quit,
    /// Cancel for symbol(s)
    CancelForSymbol,
    /// Cancel for security type
    CancelForSecurityType,
    /// Cancel for underlying symbol
    CancelForUnderlyingSymbol,
    /// Cancel all quotes
    CancelAll,
}

impl From<QuoteCancelType> for i32 {
    fn from(cancel_type: QuoteCancelType) -> Self {
        match cancel_type {
            QuoteCancelType::Quit => 1,
            QuoteCancelType::CancelForSymbol => 2,
            QuoteCancelType::CancelForSecurityType => 3,
            QuoteCancelType::CancelForUnderlyingSymbol => 4,
            QuoteCancelType::CancelAll => 5,
        }
    }
}

impl TryFrom<i32> for QuoteCancelType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(QuoteCancelType::Quit),
            2 => Ok(QuoteCancelType::CancelForSymbol),
            3 => Ok(QuoteCancelType::CancelForSecurityType),
            4 => Ok(QuoteCancelType::CancelForUnderlyingSymbol),
            5 => Ok(QuoteCancelType::CancelAll),
            _ => Err(format!("Invalid QuoteCancelType: {}", value)),
        }
    }
}

/// Quote entry for quote cancel
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct QuoteCancelEntry {
    /// Quote entry ID
    pub quote_entry_id: String,
    /// Instrument symbol
    pub symbol: String,
    /// Side (optional for two-sided quotes)
    pub side: Option<OrderSide>,
    /// Quote entry reject reason
    pub quote_entry_reject_reason: Option<i32>,
}

impl QuoteCancelEntry {
    /// Create a new quote cancel entry
    pub fn new(quote_entry_id: String, symbol: String) -> Self {
        Self {
            quote_entry_id,
            symbol,
            side: None,
            quote_entry_reject_reason: None,
        }
    }

    /// Create a quote cancel entry for a specific side
    pub fn with_side(quote_entry_id: String, symbol: String, side: OrderSide) -> Self {
        Self {
            quote_entry_id,
            symbol,
            side: Some(side),
            quote_entry_reject_reason: None,
        }
    }

    /// Set side
    pub fn set_side(mut self, side: OrderSide) -> Self {
        self.side = Some(side);
        self
    }

    /// Set quote entry reject reason
    pub fn set_reject_reason(mut self, reason: i32) -> Self {
        self.quote_entry_reject_reason = Some(reason);
        self
    }
}

/// Quote Cancel message (MsgType = 'Z')
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct QuoteCancel {
    /// Quote request ID
    pub quote_req_id: Option<String>,
    /// Quote ID
    pub quote_id: String,
    /// Quote cancel type
    pub quote_cancel_type: QuoteCancelType,
    /// Quote response level
    pub quote_resp_level: Option<i32>,
    /// Parties
    pub parties: Option<String>,
    /// Account
    pub account: Option<String>,
    /// Acc ID Source
    pub acc_id_source: Option<String>,
    /// Account Type
    pub account_type: Option<i32>,
    /// Quote set ID
    pub quote_set_id: Option<String>,
    /// Underlying symbol
    pub underlying_symbol: Option<String>,
    /// Total quote entries
    pub tot_quote_entries: Option<i32>,
    /// Quote cancel entries
    pub quote_cancel_entries: Vec<QuoteCancelEntry>,
    /// Trading session ID
    pub trading_session_id: Option<String>,
    /// Trading session sub ID
    pub trading_session_sub_id: Option<String>,
    /// Text
    pub text: Option<String>,
    /// Custom label
    pub deribit_label: Option<String>,
    /// Use standard FIX repeating groups instead of simplified custom tags
    pub use_standard_repeating_groups: bool,
}

impl QuoteCancel {
    /// Create a new quote cancel
    pub fn new(quote_id: String, quote_cancel_type: QuoteCancelType) -> Self {
        Self {
            quote_req_id: None,
            quote_id,
            quote_cancel_type,
            quote_resp_level: None,
            parties: None,
            account: None,
            acc_id_source: None,
            account_type: None,
            quote_set_id: None,
            underlying_symbol: None,
            tot_quote_entries: None,
            quote_cancel_entries: Vec::new(),
            trading_session_id: None,
            trading_session_sub_id: None,
            text: None,
            deribit_label: None,
            use_standard_repeating_groups: false, // Default to simplified custom tags for backward compatibility
        }
    }

    /// Create a quote cancel for all quotes
    pub fn cancel_all(quote_id: String) -> Self {
        Self::new(quote_id, QuoteCancelType::CancelAll)
    }

    /// Create a quote cancel for specific symbol
    pub fn cancel_for_symbol(quote_id: String, symbol: String) -> Self {
        let mut cancel = Self::new(quote_id, QuoteCancelType::CancelForSymbol);
        cancel
            .quote_cancel_entries
            .push(QuoteCancelEntry::new("".to_string(), symbol));
        cancel.tot_quote_entries = Some(1);
        cancel
    }

    /// Create a quote cancel with specific entries
    pub fn with_entries(
        quote_id: String,
        quote_cancel_type: QuoteCancelType,
        entries: Vec<QuoteCancelEntry>,
    ) -> Self {
        let tot_quote_entries = entries.len() as i32;

        Self {
            quote_req_id: None,
            quote_id,
            quote_cancel_type,
            quote_resp_level: None,
            parties: None,
            account: None,
            acc_id_source: None,
            account_type: None,
            quote_set_id: None,
            underlying_symbol: None,
            tot_quote_entries: Some(tot_quote_entries),
            quote_cancel_entries: entries,
            trading_session_id: None,
            trading_session_sub_id: None,
            text: None,
            deribit_label: None,
            use_standard_repeating_groups: false, // Default to simplified custom tags for backward compatibility
        }
    }

    /// Add a quote cancel entry
    pub fn add_quote_cancel_entry(mut self, entry: QuoteCancelEntry) -> Self {
        self.quote_cancel_entries.push(entry);
        self.tot_quote_entries = Some(self.quote_cancel_entries.len() as i32);
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

    /// Set quote set ID
    pub fn with_quote_set_id(mut self, quote_set_id: String) -> Self {
        self.quote_set_id = Some(quote_set_id);
        self
    }

    /// Set underlying symbol
    pub fn with_underlying_symbol(mut self, underlying_symbol: String) -> Self {
        self.underlying_symbol = Some(underlying_symbol);
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
            .msg_type(MsgType::QuoteCancel)
            .sender_comp_id(sender_comp_id.to_string())
            .target_comp_id(target_comp_id.to_string())
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now());

        // Required fields
        builder = builder
            .field(117, self.quote_id.clone()) // QuoteID
            .field(298, i32::from(self.quote_cancel_type).to_string()); // QuoteCancelType

        // Optional fields
        if let Some(quote_req_id) = &self.quote_req_id {
            builder = builder.field(131, quote_req_id.clone());
        }

        if let Some(quote_resp_level) = &self.quote_resp_level {
            builder = builder.field(301, quote_resp_level.to_string());
        }

        if let Some(account) = &self.account {
            builder = builder.field(1, account.clone());
        }

        if let Some(quote_set_id) = &self.quote_set_id {
            builder = builder.field(302, quote_set_id.clone());
        }

        if let Some(underlying_symbol) = &self.underlying_symbol {
            builder = builder.field(311, underlying_symbol.clone());
        }

        if let Some(tot_quote_entries) = &self.tot_quote_entries {
            builder = builder.field(295, tot_quote_entries.to_string());
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

        // Add quote cancel entries - support both standard FIX repeating groups and simplified custom tags
        if self.use_standard_repeating_groups {
            // Standard FIX repeating groups implementation
            builder = builder.field(295, self.quote_cancel_entries.len().to_string()); // NoQuoteEntries
            
            for entry in &self.quote_cancel_entries {
                builder = builder.field(299, entry.quote_entry_id.clone()); // QuoteEntryID
                builder = builder.field(55, entry.symbol.clone()); // Symbol

                if let Some(side) = &entry.side {
                    builder = builder.field(54, char::from(*side).to_string()); // Side
                }

                if let Some(quote_entry_reject_reason) = &entry.quote_entry_reject_reason {
                    builder = builder.field(368, quote_entry_reject_reason.to_string()); // QuoteEntryRejectReason
                }
            }
        } else {
            // Simplified custom tags implementation (backward compatibility)
            for (i, entry) in self.quote_cancel_entries.iter().enumerate() {
                let base_tag = 4000 + (i * 100); // Custom tag range for quote cancel entries

                if !entry.quote_entry_id.is_empty() {
                    builder = builder.field(base_tag as u32, entry.quote_entry_id.clone()); // QuoteEntryID
                }

                builder = builder.field((base_tag + 1) as u32, entry.symbol.clone()); // Symbol

                if let Some(side) = &entry.side {
                    builder = builder.field((base_tag + 2) as u32, char::from(*side).to_string());
                }

                if let Some(quote_entry_reject_reason) = &entry.quote_entry_reject_reason {
                    builder =
                        builder.field((base_tag + 3) as u32, quote_entry_reject_reason.to_string());
                }
            }
        }

        Ok(builder.build()?.to_string())
    }
}

impl_json_display!(QuoteCancel, QuoteCancelEntry);
impl_json_debug_pretty!(QuoteCancel, QuoteCancelEntry);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quote_cancel_entry_creation() {
        let entry = QuoteCancelEntry::new("QCE123".to_string(), "BTC-PERPETUAL".to_string());

        assert_eq!(entry.quote_entry_id, "QCE123");
        assert_eq!(entry.symbol, "BTC-PERPETUAL");
        assert!(entry.side.is_none());
        assert!(entry.quote_entry_reject_reason.is_none());
    }

    #[test]
    fn test_quote_cancel_entry_with_side() {
        let entry = QuoteCancelEntry::with_side(
            "QCE456".to_string(),
            "ETH-PERPETUAL".to_string(),
            OrderSide::Buy,
        );

        assert_eq!(entry.quote_entry_id, "QCE456");
        assert_eq!(entry.symbol, "ETH-PERPETUAL");
        assert_eq!(entry.side, Some(OrderSide::Buy));
    }

    #[test]
    fn test_quote_cancel_creation() {
        let cancel = QuoteCancel::new("QC123".to_string(), QuoteCancelType::CancelAll);

        assert_eq!(cancel.quote_id, "QC123");
        assert_eq!(cancel.quote_cancel_type, QuoteCancelType::CancelAll);
        assert!(cancel.quote_cancel_entries.is_empty());
    }

    #[test]
    fn test_quote_cancel_all() {
        let cancel = QuoteCancel::cancel_all("QC456".to_string());

        assert_eq!(cancel.quote_id, "QC456");
        assert_eq!(cancel.quote_cancel_type, QuoteCancelType::CancelAll);
    }

    #[test]
    fn test_quote_cancel_for_symbol() {
        let cancel =
            QuoteCancel::cancel_for_symbol("QC789".to_string(), "BTC-PERPETUAL".to_string());

        assert_eq!(cancel.quote_cancel_type, QuoteCancelType::CancelForSymbol);
        assert_eq!(cancel.tot_quote_entries, Some(1));
        assert_eq!(cancel.quote_cancel_entries.len(), 1);
        assert_eq!(cancel.quote_cancel_entries[0].symbol, "BTC-PERPETUAL");
    }

    #[test]
    fn test_quote_cancel_with_entries() {
        let entry1 = QuoteCancelEntry::new("QCE1".to_string(), "BTC-PERPETUAL".to_string());
        let entry2 = QuoteCancelEntry::new("QCE2".to_string(), "ETH-PERPETUAL".to_string());

        let cancel = QuoteCancel::with_entries(
            "QC999".to_string(),
            QuoteCancelType::CancelForSymbol,
            vec![entry1, entry2],
        );

        assert_eq!(cancel.tot_quote_entries, Some(2));
        assert_eq!(cancel.quote_cancel_entries.len(), 2);
    }

    #[test]
    fn test_quote_cancel_add_entry() {
        let entry = QuoteCancelEntry::new("QCE1".to_string(), "BTC-PERPETUAL".to_string());

        let cancel = QuoteCancel::new("QC123".to_string(), QuoteCancelType::CancelForSymbol)
            .add_quote_cancel_entry(entry);

        assert_eq!(cancel.tot_quote_entries, Some(1));
        assert_eq!(cancel.quote_cancel_entries.len(), 1);
    }

    #[test]
    fn test_quote_cancel_with_options() {
        let cancel = QuoteCancel::new("QC888".to_string(), QuoteCancelType::CancelAll)
            .with_quote_req_id("QR123".to_string())
            .with_quote_resp_level(1)
            .with_account("ACC123".to_string())
            .with_quote_set_id("QS456".to_string())
            .with_underlying_symbol("BTC".to_string())
            .with_trading_session_id("SESSION1".to_string())
            .with_text("Cancel all quotes".to_string())
            .with_label("test-cancel".to_string());

        assert_eq!(cancel.quote_req_id, Some("QR123".to_string()));
        assert_eq!(cancel.quote_resp_level, Some(1));
        assert_eq!(cancel.account, Some("ACC123".to_string()));
        assert_eq!(cancel.quote_set_id, Some("QS456".to_string()));
        assert_eq!(cancel.underlying_symbol, Some("BTC".to_string()));
        assert_eq!(cancel.trading_session_id, Some("SESSION1".to_string()));
        assert_eq!(cancel.text, Some("Cancel all quotes".to_string()));
        assert_eq!(cancel.deribit_label, Some("test-cancel".to_string()));
    }

    #[test]
    fn test_quote_cancel_to_fix_message() {
        let entry = QuoteCancelEntry::new("QCE1".to_string(), "BTC-PERPETUAL".to_string())
            .set_side(OrderSide::Buy);

        let cancel = QuoteCancel::with_entries(
            "QC123".to_string(),
            QuoteCancelType::CancelForSymbol,
            vec![entry],
        )
        .with_label("test-label".to_string());

        let fix_message = cancel.to_fix_message("SENDER", "TARGET", 1).unwrap();

        // Check that the message contains required fields
        assert!(fix_message.contains("35=Z")); // MsgType
        assert!(fix_message.contains("117=QC123")); // QuoteID
        assert!(fix_message.contains("298=2")); // QuoteCancelType=CancelForSymbol
        assert!(fix_message.contains("295=1")); // TotQuoteEntries
        assert!(fix_message.contains("100010=test-label")); // Custom label

        // Check quote cancel entry fields (simplified check)
        assert!(fix_message.contains("4000=QCE1")); // QuoteEntryID
        assert!(fix_message.contains("4001=BTC-PERPETUAL")); // Symbol
        assert!(fix_message.contains("4002=1")); // Side=Buy
    }

    #[test]
    fn test_quote_cancel_all_to_fix_message() {
        let cancel = QuoteCancel::cancel_all("QC456".to_string())
            .with_text("Cancel all active quotes".to_string());

        let fix_message = cancel.to_fix_message("SENDER", "TARGET", 2).unwrap();

        // Check required fields
        assert!(fix_message.contains("35=Z")); // MsgType
        assert!(fix_message.contains("117=QC456")); // QuoteID
        assert!(fix_message.contains("298=5")); // QuoteCancelType=CancelAll
        assert!(fix_message.contains("58=Cancel all active quotes")); // Text

        // Should not have entries for cancel all
        assert!(!fix_message.contains("295=")); // TotQuoteEntries not set
        assert!(!fix_message.contains("4000=")); // No quote entries
    }

    #[test]
    fn test_quote_cancel_type_conversions() {
        assert_eq!(i32::from(QuoteCancelType::Quit), 1);
        assert_eq!(i32::from(QuoteCancelType::CancelForSymbol), 2);
        assert_eq!(i32::from(QuoteCancelType::CancelForSecurityType), 3);
        assert_eq!(i32::from(QuoteCancelType::CancelForUnderlyingSymbol), 4);
        assert_eq!(i32::from(QuoteCancelType::CancelAll), 5);

        assert_eq!(QuoteCancelType::try_from(1).unwrap(), QuoteCancelType::Quit);
        assert_eq!(
            QuoteCancelType::try_from(2).unwrap(),
            QuoteCancelType::CancelForSymbol
        );
        assert_eq!(
            QuoteCancelType::try_from(3).unwrap(),
            QuoteCancelType::CancelForSecurityType
        );
        assert_eq!(
            QuoteCancelType::try_from(4).unwrap(),
            QuoteCancelType::CancelForUnderlyingSymbol
        );
        assert_eq!(
            QuoteCancelType::try_from(5).unwrap(),
            QuoteCancelType::CancelAll
        );

        assert!(QuoteCancelType::try_from(99).is_err());
    }

    #[test]
    fn test_quote_cancel_standard_repeating_groups() {
        let entries = vec![
            QuoteCancelEntry::with_side(
                "QCE123".to_string(),
                "BTC-PERPETUAL".to_string(),
                OrderSide::Buy,
            ),
            QuoteCancelEntry::new("QCE456".to_string(), "ETH-PERPETUAL".to_string()),
        ];

        let quote_cancel = QuoteCancel::with_entries(
            "QC789".to_string(),
            QuoteCancelType::CancelAll,
            entries,
        )
        .enable_standard_repeating_groups();

        let fix_message = quote_cancel
            .to_fix_message("SENDER", "TARGET", 123)
            .unwrap();

        // Should contain standard FIX tags
        assert!(fix_message.contains("295=2")); // NoQuoteEntries
        // In FIX repeating groups, the last entry's fields will appear in the message
        assert!(fix_message.contains("299=QCE456")); // Second QuoteEntryID (last one)
        assert!(fix_message.contains("55=ETH-PERPETUAL")); // Second symbol (last one)
        
        // Check that we're using standard tags, not custom ones
        assert!(fix_message.contains("299=")); // QuoteEntryID (standard tag)
        assert!(fix_message.contains("55=")); // Symbol (standard tag)

        // Should not contain custom tags
        assert!(!fix_message.contains("4000="));
        assert!(!fix_message.contains("4001="));
    }

    #[test]
    fn test_quote_cancel_simplified_custom_tags() {
        let entries = vec![
            QuoteCancelEntry::with_side(
                "QCE123".to_string(),
                "BTC-PERPETUAL".to_string(),
                OrderSide::Buy,
            ),
            QuoteCancelEntry::new("QCE456".to_string(), "ETH-PERPETUAL".to_string()),
        ];

        let quote_cancel = QuoteCancel::with_entries(
            "QC789".to_string(),
            QuoteCancelType::CancelAll,
            entries,
        )
        .disable_standard_repeating_groups(); // Explicitly disable (though it's default)

        let fix_message = quote_cancel
            .to_fix_message("SENDER", "TARGET", 123)
            .unwrap();

        // Should contain custom tags
        assert!(fix_message.contains("4000=QCE123")); // First entry ID
        assert!(fix_message.contains("4001=BTC-PERPETUAL")); // First symbol
        assert!(fix_message.contains("4002=1")); // First side (Buy)
        assert!(fix_message.contains("4100=QCE456")); // Second entry ID
        assert!(fix_message.contains("4101=ETH-PERPETUAL")); // Second symbol

        // Should not contain standard repeating group tags (except tot_quote_entries)
        assert!(!fix_message.contains("299=")); // QuoteEntryID (standard)
        assert!(!fix_message.contains("55=BTC-PERPETUAL")); // Symbol (standard)
    }

    #[test]
    fn test_quote_cancel_builder_methods() {
        let quote_cancel = QuoteCancel::new("QC123".to_string(), QuoteCancelType::CancelAll);

        // Test default
        assert!(!quote_cancel.use_standard_repeating_groups);

        // Test enable
        let enabled = quote_cancel.clone().enable_standard_repeating_groups();
        assert!(enabled.use_standard_repeating_groups);

        // Test disable
        let disabled = enabled.disable_standard_repeating_groups();
        assert!(!disabled.use_standard_repeating_groups);
    }
}
