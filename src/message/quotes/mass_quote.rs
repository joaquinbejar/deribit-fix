/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 10/8/25
******************************************************************************/

//! Mass Quote FIX Message Implementation

use crate::error::Result as DeribitFixResult;
use crate::message::builder::MessageBuilder;
use crate::message::orders::{OrderSide, TimeInForce};
use crate::model::types::MsgType;
use chrono::{DateTime, Utc};
use deribit_base::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

/// Quote entry for mass quote
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct QuoteEntry {
    /// Quote entry ID
    pub quote_entry_id: String,
    /// Instrument symbol
    pub symbol: String,
    /// Side (optional for two-sided quotes)
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
    /// Trading session ID
    pub trading_session_id: Option<String>,
}

impl QuoteEntry {
    /// Create a new quote entry
    pub fn new(quote_entry_id: String, symbol: String) -> Self {
        Self {
            quote_entry_id,
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
            trading_session_id: None,
        }
    }

    /// Create a two-sided quote entry
    pub fn two_sided(
        quote_entry_id: String,
        symbol: String,
        bid_px: f64,
        offer_px: f64,
        bid_size: f64,
        offer_size: f64,
    ) -> Self {
        Self {
            quote_entry_id,
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
            trading_session_id: None,
        }
    }

    /// Create a one-sided quote entry
    pub fn one_sided(
        quote_entry_id: String,
        symbol: String,
        side: OrderSide,
        price: f64,
        size: f64,
    ) -> Self {
        let mut entry = Self::new(quote_entry_id, symbol);
        entry.side = Some(side);

        match side {
            OrderSide::Buy => {
                entry.bid_px = Some(price);
                entry.bid_size = Some(size);
            }
            OrderSide::Sell => {
                entry.offer_px = Some(price);
                entry.offer_size = Some(size);
            }
        }

        entry
    }

    /// Set valid until time
    pub fn with_valid_until(mut self, valid_until: DateTime<Utc>) -> Self {
        self.valid_until_time = Some(valid_until);
        self
    }

    /// Set trading session ID
    pub fn with_trading_session_id(mut self, session_id: String) -> Self {
        self.trading_session_id = Some(session_id);
        self
    }
}

/// Mass quote response type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MassQuoteResponseType {
    /// No acknowledgement required
    NoAckRequired,
    /// Acknowledgement required
    AckRequired,
}

impl From<MassQuoteResponseType> for i32 {
    fn from(response_type: MassQuoteResponseType) -> Self {
        match response_type {
            MassQuoteResponseType::NoAckRequired => 0,
            MassQuoteResponseType::AckRequired => 1,
        }
    }
}

impl TryFrom<i32> for MassQuoteResponseType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MassQuoteResponseType::NoAckRequired),
            1 => Ok(MassQuoteResponseType::AckRequired),
            _ => Err(format!("Invalid MassQuoteResponseType: {}", value)),
        }
    }
}

/// Mass Quote message (MsgType = 'i')
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct MassQuote {
    /// Quote ID
    pub quote_id: String,
    /// Quote request ID
    pub quote_req_id: Option<String>,
    /// Quote response level
    pub quote_resp_level: Option<i32>,
    /// Defaul bid size
    pub defaul_bid_size: Option<f64>,
    /// Default offer size
    pub default_offer_size: Option<f64>,
    /// Quote set ID
    pub quote_set_id: String,
    /// Quote set valid until time
    pub quote_set_valid_until_time: Option<DateTime<Utc>>,
    /// Total quote entries
    pub tot_quote_entries: i32,
    /// Quote entries
    pub quote_entries: Vec<QuoteEntry>,
    /// Account
    pub account: Option<String>,
    /// Clearing account
    pub clearing_account: Option<String>,
    /// Settlement type
    pub settlement_type: Option<char>,
    /// Settlement date
    pub settlement_date: Option<String>,
    /// Clearing business date
    pub clearing_business_date: Option<String>,
    /// Time in force
    pub time_in_force: Option<TimeInForce>,
    /// Custom label
    pub deribit_label: Option<String>,
    /// Mass quote response type
    pub mass_quote_response_type: Option<MassQuoteResponseType>,
}

impl MassQuote {
    /// Create a new mass quote
    pub fn new(quote_id: String, quote_set_id: String, quote_entries: Vec<QuoteEntry>) -> Self {
        let tot_quote_entries = quote_entries.len() as i32;

        Self {
            quote_id,
            quote_req_id: None,
            quote_resp_level: None,
            defaul_bid_size: None,
            default_offer_size: None,
            quote_set_id,
            quote_set_valid_until_time: None,
            tot_quote_entries,
            quote_entries,
            account: None,
            clearing_account: None,
            settlement_type: None,
            settlement_date: None,
            clearing_business_date: None,
            time_in_force: None,
            deribit_label: None,
            mass_quote_response_type: None,
        }
    }

    /// Create a mass quote with default sizes
    pub fn with_default_sizes(
        quote_id: String,
        quote_set_id: String,
        quote_entries: Vec<QuoteEntry>,
        default_bid_size: f64,
        default_offer_size: f64,
    ) -> Self {
        let mut mass_quote = Self::new(quote_id, quote_set_id, quote_entries);
        mass_quote.defaul_bid_size = Some(default_bid_size);
        mass_quote.default_offer_size = Some(default_offer_size);
        mass_quote
    }

    /// Add a quote entry
    pub fn add_quote_entry(mut self, entry: QuoteEntry) -> Self {
        self.quote_entries.push(entry);
        self.tot_quote_entries = self.quote_entries.len() as i32;
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

    /// Set quote set valid until time
    pub fn with_quote_set_valid_until(mut self, valid_until: DateTime<Utc>) -> Self {
        self.quote_set_valid_until_time = Some(valid_until);
        self
    }

    /// Set account
    pub fn with_account(mut self, account: String) -> Self {
        self.account = Some(account);
        self
    }

    /// Set time in force
    pub fn with_time_in_force(mut self, tif: TimeInForce) -> Self {
        self.time_in_force = Some(tif);
        self
    }

    /// Set custom label
    pub fn with_label(mut self, label: String) -> Self {
        self.deribit_label = Some(label);
        self
    }

    /// Set mass quote response type
    pub fn with_response_type(mut self, response_type: MassQuoteResponseType) -> Self {
        self.mass_quote_response_type = Some(response_type);
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
            .msg_type(MsgType::MassQuote)
            .sender_comp_id(sender_comp_id.to_string())
            .target_comp_id(target_comp_id.to_string())
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now());

        // Required fields
        builder = builder
            .field(117, self.quote_id.clone()) // QuoteID
            .field(302, self.quote_set_id.clone()) // QuoteSetID
            .field(295, self.tot_quote_entries.to_string()); // TotQuoteEntries

        // Optional fields
        if let Some(quote_req_id) = &self.quote_req_id {
            builder = builder.field(131, quote_req_id.clone());
        }

        if let Some(quote_resp_level) = &self.quote_resp_level {
            builder = builder.field(301, quote_resp_level.to_string());
        }

        if let Some(default_bid_size) = &self.defaul_bid_size {
            builder = builder.field(293, default_bid_size.to_string());
        }

        if let Some(default_offer_size) = &self.default_offer_size {
            builder = builder.field(294, default_offer_size.to_string());
        }

        if let Some(quote_set_valid_until_time) = &self.quote_set_valid_until_time {
            builder = builder.field(
                367,
                quote_set_valid_until_time
                    .format("%Y%m%d-%H:%M:%S%.3f")
                    .to_string(),
            );
        }

        if let Some(account) = &self.account {
            builder = builder.field(1, account.clone());
        }

        if let Some(time_in_force) = &self.time_in_force {
            builder = builder.field(59, char::from(*time_in_force).to_string());
        }

        if let Some(deribit_label) = &self.deribit_label {
            builder = builder.field(100010, deribit_label.clone());
        }

        if let Some(mass_quote_response_type) = &self.mass_quote_response_type {
            builder = builder.field(296, i32::from(*mass_quote_response_type).to_string());
        }

        // Add quote entries (simplified - in real implementation would need repeating groups)
        for (i, entry) in self.quote_entries.iter().enumerate() {
            let base_tag = 2000 + (i * 100); // Custom tag range for quote entries

            builder = builder
                .field(base_tag as u32, entry.quote_entry_id.clone()) // QuoteEntryID
                .field((base_tag + 1) as u32, entry.symbol.clone()); // Symbol

            if let Some(side) = &entry.side {
                builder = builder.field((base_tag + 2) as u32, char::from(*side).to_string());
            }

            if let Some(bid_px) = &entry.bid_px {
                builder = builder.field((base_tag + 10) as u32, bid_px.to_string());
            }

            if let Some(offer_px) = &entry.offer_px {
                builder = builder.field((base_tag + 11) as u32, offer_px.to_string());
            }

            if let Some(bid_size) = &entry.bid_size {
                builder = builder.field((base_tag + 12) as u32, bid_size.to_string());
            }

            if let Some(offer_size) = &entry.offer_size {
                builder = builder.field((base_tag + 13) as u32, offer_size.to_string());
            }
        }

        Ok(builder.build()?.to_string())
    }
}

impl_json_display!(MassQuote, QuoteEntry);
impl_json_debug_pretty!(MassQuote, QuoteEntry);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quote_entry_creation() {
        let entry = QuoteEntry::new("QE123".to_string(), "BTC-PERPETUAL".to_string());

        assert_eq!(entry.quote_entry_id, "QE123");
        assert_eq!(entry.symbol, "BTC-PERPETUAL");
        assert!(entry.bid_px.is_none());
        assert!(entry.offer_px.is_none());
    }

    #[test]
    fn test_quote_entry_two_sided() {
        let entry = QuoteEntry::two_sided(
            "QE456".to_string(),
            "ETH-PERPETUAL".to_string(),
            3200.0,
            3205.0,
            10.0,
            8.0,
        );

        assert_eq!(entry.bid_px, Some(3200.0));
        assert_eq!(entry.offer_px, Some(3205.0));
        assert_eq!(entry.bid_size, Some(10.0));
        assert_eq!(entry.offer_size, Some(8.0));
        assert_eq!(entry.mid_px, Some(3202.5));
    }

    #[test]
    fn test_quote_entry_one_sided_buy() {
        let entry = QuoteEntry::one_sided(
            "QE789".to_string(),
            "BTC-PERPETUAL".to_string(),
            OrderSide::Buy,
            50000.0,
            5.0,
        );

        assert_eq!(entry.side, Some(OrderSide::Buy));
        assert_eq!(entry.bid_px, Some(50000.0));
        assert_eq!(entry.bid_size, Some(5.0));
        assert!(entry.offer_px.is_none());
        assert!(entry.offer_size.is_none());
    }

    #[test]
    fn test_quote_entry_one_sided_sell() {
        let entry = QuoteEntry::one_sided(
            "QE999".to_string(),
            "BTC-PERPETUAL".to_string(),
            OrderSide::Sell,
            50010.0,
            3.0,
        );

        assert_eq!(entry.side, Some(OrderSide::Sell));
        assert_eq!(entry.offer_px, Some(50010.0));
        assert_eq!(entry.offer_size, Some(3.0));
        assert!(entry.bid_px.is_none());
        assert!(entry.bid_size.is_none());
    }

    #[test]
    fn test_mass_quote_creation() {
        let entry1 = QuoteEntry::two_sided(
            "QE1".to_string(),
            "BTC-PERPETUAL".to_string(),
            50000.0,
            50010.0,
            5.0,
            3.0,
        );
        let entry2 = QuoteEntry::two_sided(
            "QE2".to_string(),
            "ETH-PERPETUAL".to_string(),
            3200.0,
            3205.0,
            10.0,
            8.0,
        );

        let mass_quote = MassQuote::new(
            "MQ123".to_string(),
            "QS456".to_string(),
            vec![entry1, entry2],
        );

        assert_eq!(mass_quote.quote_id, "MQ123");
        assert_eq!(mass_quote.quote_set_id, "QS456");
        assert_eq!(mass_quote.tot_quote_entries, 2);
        assert_eq!(mass_quote.quote_entries.len(), 2);
    }

    #[test]
    fn test_mass_quote_with_default_sizes() {
        let entry = QuoteEntry::new("QE1".to_string(), "BTC-PERPETUAL".to_string());

        let mass_quote = MassQuote::with_default_sizes(
            "MQ456".to_string(),
            "QS789".to_string(),
            vec![entry],
            10.0,
            8.0,
        );

        assert_eq!(mass_quote.defaul_bid_size, Some(10.0));
        assert_eq!(mass_quote.default_offer_size, Some(8.0));
    }

    #[test]
    fn test_mass_quote_add_entry() {
        let entry1 = QuoteEntry::new("QE1".to_string(), "BTC-PERPETUAL".to_string());
        let entry2 = QuoteEntry::new("QE2".to_string(), "ETH-PERPETUAL".to_string());

        let mass_quote = MassQuote::new("MQ123".to_string(), "QS456".to_string(), vec![entry1])
            .add_quote_entry(entry2);

        assert_eq!(mass_quote.tot_quote_entries, 2);
        assert_eq!(mass_quote.quote_entries.len(), 2);
    }

    #[test]
    fn test_mass_quote_with_options() {
        let entry = QuoteEntry::new("QE1".to_string(), "BTC-PERPETUAL".to_string());
        let valid_until = Utc::now() + chrono::Duration::hours(1);

        let mass_quote = MassQuote::new("MQ789".to_string(), "QS999".to_string(), vec![entry])
            .with_quote_req_id("QR123".to_string())
            .with_quote_resp_level(1)
            .with_quote_set_valid_until(valid_until)
            .with_account("ACC123".to_string())
            .with_time_in_force(TimeInForce::GoodTillCancelled)
            .with_label("test-mass-quote".to_string())
            .with_response_type(MassQuoteResponseType::AckRequired);

        assert_eq!(mass_quote.quote_req_id, Some("QR123".to_string()));
        assert_eq!(mass_quote.quote_resp_level, Some(1));
        assert_eq!(mass_quote.quote_set_valid_until_time, Some(valid_until));
        assert_eq!(mass_quote.account, Some("ACC123".to_string()));
        assert_eq!(
            mass_quote.time_in_force,
            Some(TimeInForce::GoodTillCancelled)
        );
        assert_eq!(
            mass_quote.deribit_label,
            Some("test-mass-quote".to_string())
        );
        assert_eq!(
            mass_quote.mass_quote_response_type,
            Some(MassQuoteResponseType::AckRequired)
        );
    }

    #[test]
    fn test_mass_quote_to_fix_message() {
        let entry = QuoteEntry::two_sided(
            "QE1".to_string(),
            "BTC-PERPETUAL".to_string(),
            50000.0,
            50010.0,
            5.0,
            3.0,
        );

        let mass_quote = MassQuote::new("MQ123".to_string(), "QS456".to_string(), vec![entry])
            .with_label("test-label".to_string());

        let fix_message = mass_quote.to_fix_message("SENDER", "TARGET", 1).unwrap();

        // Check that the message contains required fields
        assert!(fix_message.contains("35=i")); // MsgType
        assert!(fix_message.contains("117=MQ123")); // QuoteID
        assert!(fix_message.contains("302=QS456")); // QuoteSetID
        assert!(fix_message.contains("295=1")); // TotQuoteEntries
        assert!(fix_message.contains("100010=test-label")); // Custom label

        // Check quote entry fields (simplified check)
        assert!(fix_message.contains("2000=QE1")); // QuoteEntryID
        assert!(fix_message.contains("2001=BTC-PERPETUAL")); // Symbol
        assert!(fix_message.contains("2010=50000")); // BidPx
        assert!(fix_message.contains("2011=50010")); // OfferPx
    }

    #[test]
    fn test_mass_quote_response_type_conversions() {
        assert_eq!(i32::from(MassQuoteResponseType::NoAckRequired), 0);
        assert_eq!(i32::from(MassQuoteResponseType::AckRequired), 1);

        assert_eq!(
            MassQuoteResponseType::try_from(0).unwrap(),
            MassQuoteResponseType::NoAckRequired
        );
        assert_eq!(
            MassQuoteResponseType::try_from(1).unwrap(),
            MassQuoteResponseType::AckRequired
        );

        assert!(MassQuoteResponseType::try_from(99).is_err());
    }
}
