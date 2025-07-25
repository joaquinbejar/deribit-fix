/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 22/7/25
******************************************************************************/

//! # Market Data FIX Messages
//!
//! This module implements the Market Data FIX protocol messages for Deribit according to the
//! official FIX API specification. It includes:
//!
//! - Market Data Request (MsgType = 'V')
//! - Market Data Request Reject (MsgType = 'Y')
//! - Market Data Snapshot/Full Refresh (MsgType = 'W')
//! - Market Data Incremental Refresh (MsgType = 'X')

use crate::error::Result as DeribitFixResult;
use crate::message::MessageBuilder;
use crate::model::types::MsgType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Market Data Subscription Request Type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MdSubscriptionRequestType {
    /// Snapshot only
    Snapshot = 0,
    /// Snapshot + Updates (Subscribe)
    SnapshotPlusUpdates = 1,
    /// Disable previous Snapshot + Update Request (Unsubscribe)
    Unsubscribe = 2,
}

impl From<MdSubscriptionRequestType> for i32 {
    fn from(value: MdSubscriptionRequestType) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for MdSubscriptionRequestType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MdSubscriptionRequestType::Snapshot),
            1 => Ok(MdSubscriptionRequestType::SnapshotPlusUpdates),
            2 => Ok(MdSubscriptionRequestType::Unsubscribe),
            _ => Err(format!("Invalid MdSubscriptionRequestType: {value}")),
        }
    }
}

/// MD Update Type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MdUpdateType {
    /// Full refresh
    FullRefresh = 0,
    /// Incremental refresh
    IncrementalRefresh = 1,
}

impl From<MdUpdateType> for i32 {
    fn from(value: MdUpdateType) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for MdUpdateType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MdUpdateType::FullRefresh),
            1 => Ok(MdUpdateType::IncrementalRefresh),
            _ => Err(format!("Invalid MdUpdateType: {value}")),
        }
    }
}

/// MD Entry Type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MdEntryType {
    /// Bid (Bid side of the order book)
    Bid = 0,
    /// Offer (Ask side of the order book)
    Offer = 1,
    /// Trade (Info about recent trades)
    Trade = 2,
    /// Index Value (value of Index for INDEX instruments)
    IndexValue = 3,
    /// Settlement Price (Estimated Delivery Price for INDEX instruments)
    SettlementPrice = 6,
}

impl From<MdEntryType> for i32 {
    fn from(value: MdEntryType) -> Self {
        value as i32
    }
}

impl TryFrom<i32> for MdEntryType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MdEntryType::Bid),
            1 => Ok(MdEntryType::Offer),
            2 => Ok(MdEntryType::Trade),
            3 => Ok(MdEntryType::IndexValue),
            6 => Ok(MdEntryType::SettlementPrice),
            _ => Err(format!("Invalid MdEntryType: {value}")),
        }
    }
}

/// MD Update Action enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MdUpdateAction {
    /// New entry
    New = 0,
    /// Change existing entry
    Change = 1,
    /// Delete entry
    Delete = 2,
}

impl From<MdUpdateAction> for char {
    fn from(value: MdUpdateAction) -> Self {
        match value {
            MdUpdateAction::New => '0',
            MdUpdateAction::Change => '1',
            MdUpdateAction::Delete => '2',
        }
    }
}

impl TryFrom<char> for MdUpdateAction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(MdUpdateAction::New),
            '1' => Ok(MdUpdateAction::Change),
            '2' => Ok(MdUpdateAction::Delete),
            _ => Err(format!("Invalid MdUpdateAction: {value}")),
        }
    }
}

/// MD Request Reject Reason enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MdReqRejReason {
    /// Unknown symbol
    UnknownSymbol = 0,
    /// Duplicate MDReqID
    DuplicateMdReqId = 1,
    /// Insufficient Bandwidth
    InsufficientBandwidth = 2,
    /// Insufficient Permissions
    InsufficientPermissions = 3,
    /// Unsupported SubscriptionRequestType
    UnsupportedSubscriptionRequestType = 4,
    /// Unsupported MarketDepth
    UnsupportedMarketDepth = 5,
    /// Unsupported MDUpdateType
    UnsupportedMdUpdateType = 6,
    /// Unsupported AggregatedBook
    UnsupportedAggregatedBook = 7,
    /// Unsupported MDEntryType
    UnsupportedMdEntryType = 8,
    /// Unsupported TradingSessionID
    UnsupportedTradingSessionId = 9,
    /// Unsupported Scope
    UnsupportedScope = 10,
    /// Unsupported OpenCloseSettlFlag
    UnsupportedOpenCloseSettlFlag = 11,
    /// Unsupported MDImplicitDelete
    UnsupportedMdImplicitDelete = 12,
    /// Insufficient credit
    InsufficientCredit = 13,
}

impl From<MdReqRejReason> for char {
    fn from(value: MdReqRejReason) -> Self {
        match value {
            MdReqRejReason::UnknownSymbol => '0',
            MdReqRejReason::DuplicateMdReqId => '1',
            MdReqRejReason::InsufficientBandwidth => '2',
            MdReqRejReason::InsufficientPermissions => '3',
            MdReqRejReason::UnsupportedSubscriptionRequestType => '4',
            MdReqRejReason::UnsupportedMarketDepth => '5',
            MdReqRejReason::UnsupportedMdUpdateType => '6',
            MdReqRejReason::UnsupportedAggregatedBook => '7',
            MdReqRejReason::UnsupportedMdEntryType => '8',
            MdReqRejReason::UnsupportedTradingSessionId => '9',
            MdReqRejReason::UnsupportedScope => 'A',
            MdReqRejReason::UnsupportedOpenCloseSettlFlag => 'B',
            MdReqRejReason::UnsupportedMdImplicitDelete => 'C',
            MdReqRejReason::InsufficientCredit => 'D',
        }
    }
}

impl TryFrom<char> for MdReqRejReason {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(MdReqRejReason::UnknownSymbol),
            '1' => Ok(MdReqRejReason::DuplicateMdReqId),
            '2' => Ok(MdReqRejReason::InsufficientBandwidth),
            '3' => Ok(MdReqRejReason::InsufficientPermissions),
            '4' => Ok(MdReqRejReason::UnsupportedSubscriptionRequestType),
            '5' => Ok(MdReqRejReason::UnsupportedMarketDepth),
            '6' => Ok(MdReqRejReason::UnsupportedMdUpdateType),
            '7' => Ok(MdReqRejReason::UnsupportedAggregatedBook),
            '8' => Ok(MdReqRejReason::UnsupportedMdEntryType),
            '9' => Ok(MdReqRejReason::UnsupportedTradingSessionId),
            'A' => Ok(MdReqRejReason::UnsupportedScope),
            'B' => Ok(MdReqRejReason::UnsupportedOpenCloseSettlFlag),
            'C' => Ok(MdReqRejReason::UnsupportedMdImplicitDelete),
            'D' => Ok(MdReqRejReason::InsufficientCredit),
            _ => Err(format!("Invalid MdReqRejReason: {value}")),
        }
    }
}

/// Market Data Request message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDataRequest {
    /// Unique ID assigned to this request
    pub md_req_id: String,
    /// Subscription Request Type
    pub subscription_request_type: MdSubscriptionRequestType,
    /// Market depth (optional)
    pub market_depth: Option<i32>,
    /// MD Update Type (when SubscriptionRequestType=1)
    pub md_update_type: Option<MdUpdateType>,
    /// Skip block trades flag
    pub skip_block_trades: Option<bool>,
    /// Show block trade ID flag
    pub show_block_trade_id: Option<bool>,
    /// Amount of trades returned in snapshot (default 20, max 1000)
    pub trade_amount: Option<i32>,
    /// UTC timestamp in milliseconds for trades since timestamp
    pub since_timestamp: Option<i64>,
    /// Entry types requested
    pub entry_types: Vec<MdEntryType>,
    /// Symbols requested
    pub symbols: Vec<String>,
}

impl MarketDataRequest {
    /// Create a new snapshot request
    pub fn snapshot(
        md_req_id: String,
        symbols: Vec<String>,
        entry_types: Vec<MdEntryType>,
    ) -> Self {
        Self {
            md_req_id,
            subscription_request_type: MdSubscriptionRequestType::Snapshot,
            market_depth: None,
            md_update_type: None,
            skip_block_trades: None,
            show_block_trade_id: None,
            trade_amount: None,
            since_timestamp: None,
            entry_types,
            symbols,
        }
    }

    /// Create a new subscription request
    pub fn subscription(
        md_req_id: String,
        symbols: Vec<String>,
        entry_types: Vec<MdEntryType>,
        md_update_type: MdUpdateType,
    ) -> Self {
        Self {
            md_req_id,
            subscription_request_type: MdSubscriptionRequestType::SnapshotPlusUpdates,
            market_depth: None,
            md_update_type: Some(md_update_type),
            skip_block_trades: None,
            show_block_trade_id: None,
            trade_amount: None,
            since_timestamp: None,
            entry_types,
            symbols,
        }
    }

    /// Create an unsubscribe request
    pub fn unsubscribe(md_req_id: String) -> Self {
        Self {
            md_req_id,
            subscription_request_type: MdSubscriptionRequestType::Unsubscribe,
            market_depth: None,
            md_update_type: None,
            skip_block_trades: None,
            show_block_trade_id: None,
            trade_amount: None,
            since_timestamp: None,
            entry_types: Vec::new(),
            symbols: Vec::new(),
        }
    }

    /// Convert to FIX message
    pub fn to_fix_message(
        &self,
        sender_comp_id: String,
        target_comp_id: String,
        msg_seq_num: u32,
    ) -> DeribitFixResult<String> {
        let mut builder = MessageBuilder::new()
            .msg_type(MsgType::MarketDataRequest)
            .sender_comp_id(sender_comp_id)
            .target_comp_id(target_comp_id)
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now())
            .field(262, self.md_req_id.clone()) // MDReqID
            .field(263, i32::from(self.subscription_request_type).to_string()); // SubscriptionRequestType

        // Add optional fields
        if let Some(depth) = self.market_depth {
            builder = builder.field(264, depth.to_string()); // MarketDepth
        }

        if let Some(update_type) = self.md_update_type {
            builder = builder.field(265, i32::from(update_type).to_string()); // MDUpdateType
        }

        // Add entry types group
        builder = builder.field(267, self.entry_types.len().to_string()); // NoMDEntryTypes
        for entry_type in &self.entry_types {
            builder = builder.field(269, i32::from(*entry_type).to_string()); // MDEntryType
        }

        // Add symbols group
        if !self.symbols.is_empty() {
            builder = builder.field(146, self.symbols.len().to_string()); // NoRelatedSym
            for symbol in &self.symbols {
                builder = builder.field(55, symbol.clone()); // Symbol
            }
        }

        Ok(builder.build()?.to_string())
    }
}

/// Market Data Request Reject message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDataRequestReject {
    /// ID of the original request
    pub md_req_id: String,
    /// Reason for rejection
    pub md_req_rej_reason: MdReqRejReason,
    /// Free format text string
    pub text: Option<String>,
}

impl MarketDataRequestReject {
    /// Create a new reject message
    pub fn new(md_req_id: String, reason: MdReqRejReason) -> Self {
        Self {
            md_req_id,
            md_req_rej_reason: reason,
            text: None,
        }
    }

    /// Create a reject message with text
    pub fn with_text(md_req_id: String, reason: MdReqRejReason, text: String) -> Self {
        Self {
            md_req_id,
            md_req_rej_reason: reason,
            text: Some(text),
        }
    }

    /// Convert to FIX message
    pub fn to_fix_message(
        &self,
        sender_comp_id: String,
        target_comp_id: String,
        msg_seq_num: u32,
    ) -> DeribitFixResult<String> {
        let mut builder = MessageBuilder::new()
            .msg_type(MsgType::MarketDataRequestReject)
            .sender_comp_id(sender_comp_id)
            .target_comp_id(target_comp_id)
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now())
            .field(262, self.md_req_id.clone()) // MDReqID
            .field(281, char::from(self.md_req_rej_reason).to_string()); // MDReqRejReason

        if let Some(ref text) = self.text {
            builder = builder.field(58, text.clone()); // Text
        }

        Ok(builder.build()?.to_string())
    }
}

/// Market Data Entry for snapshot and incremental messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdEntry {
    /// Entry type
    pub md_entry_type: MdEntryType,
    /// Price of entry (optional)
    pub md_entry_px: Option<f64>,
    /// Size of entry (optional)
    pub md_entry_size: Option<f64>,
    /// Timestamp for entry (optional)
    pub md_entry_date: Option<DateTime<Utc>>,
    /// Update action (for incremental refresh)
    pub md_update_action: Option<MdUpdateAction>,
    /// Trade ID (for trades)
    pub trade_id: Option<String>,
    /// Side (for trades)
    pub side: Option<char>,
    /// Order ID (for trades)
    pub order_id: Option<String>,
    /// Secondary order ID (for trades)
    pub secondary_order_id: Option<String>,
}

impl MdEntry {
    /// Create a new bid entry
    pub fn bid(price: f64, size: f64) -> Self {
        Self {
            md_entry_type: MdEntryType::Bid,
            md_entry_px: Some(price),
            md_entry_size: Some(size),
            md_entry_date: None,
            md_update_action: None,
            trade_id: None,
            side: None,
            order_id: None,
            secondary_order_id: None,
        }
    }

    /// Create a new offer entry
    pub fn offer(price: f64, size: f64) -> Self {
        Self {
            md_entry_type: MdEntryType::Offer,
            md_entry_px: Some(price),
            md_entry_size: Some(size),
            md_entry_date: None,
            md_update_action: None,
            trade_id: None,
            side: None,
            order_id: None,
            secondary_order_id: None,
        }
    }

    /// Create a new trade entry
    pub fn trade(
        price: f64,
        size: f64,
        side: char,
        trade_id: String,
        timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            md_entry_type: MdEntryType::Trade,
            md_entry_px: Some(price),
            md_entry_size: Some(size),
            md_entry_date: Some(timestamp),
            md_update_action: None,
            trade_id: Some(trade_id),
            side: Some(side),
            order_id: None,
            secondary_order_id: None,
        }
    }

    /// Set update action for incremental refresh
    pub fn with_update_action(mut self, action: MdUpdateAction) -> Self {
        self.md_update_action = Some(action);
        self
    }
}

/// Market Data Snapshot/Full Refresh message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDataSnapshotFullRefresh {
    /// Instrument symbol
    pub symbol: String,
    /// ID of the original request (optional)
    pub md_req_id: Option<String>,
    /// Market data entries
    pub entries: Vec<MdEntry>,
}

impl MarketDataSnapshotFullRefresh {
    /// Create a new snapshot message
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            md_req_id: None,
            entries: Vec::new(),
        }
    }

    /// Set request ID
    pub fn with_request_id(mut self, md_req_id: String) -> Self {
        self.md_req_id = Some(md_req_id);
        self
    }

    /// Add entries
    pub fn with_entries(mut self, entries: Vec<MdEntry>) -> Self {
        self.entries = entries;
        self
    }

    /// Convert to FIX message
    pub fn to_fix_message(
        &self,
        sender_comp_id: String,
        target_comp_id: String,
        msg_seq_num: u32,
    ) -> DeribitFixResult<String> {
        let mut builder = MessageBuilder::new()
            .msg_type(MsgType::MarketDataSnapshotFullRefresh)
            .sender_comp_id(sender_comp_id)
            .target_comp_id(target_comp_id)
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now())
            .field(55, self.symbol.clone()); // Symbol

        if let Some(ref md_req_id) = self.md_req_id {
            builder = builder.field(262, md_req_id.clone()); // MDReqID
        }

        // Add entries group
        builder = builder.field(268, self.entries.len().to_string()); // NoMDEntries

        for entry in &self.entries {
            builder = builder.field(269, i32::from(entry.md_entry_type).to_string()); // MDEntryType

            if let Some(px) = entry.md_entry_px {
                builder = builder.field(270, px.to_string()); // MDEntryPx
            }

            if let Some(size) = entry.md_entry_size {
                builder = builder.field(271, size.to_string()); // MDEntrySize
            }

            if let Some(date) = entry.md_entry_date {
                builder = builder.field(272, date.timestamp_millis().to_string()); // MDEntryDate
            }

            if let Some(ref trade_id) = entry.trade_id {
                builder = builder.field(100009, trade_id.clone()); // DeribitTradeId
            }

            if let Some(side) = entry.side {
                builder = builder.field(54, side.to_string()); // Side
            }
        }

        Ok(builder.build()?.to_string())
    }
}

/// Market Data Incremental Refresh message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDataIncrementalRefresh {
    /// Instrument symbol
    pub symbol: String,
    /// ID of the original request (optional)
    pub md_req_id: Option<String>,
    /// Market data entries with update actions
    pub entries: Vec<MdEntry>,
}

impl MarketDataIncrementalRefresh {
    /// Create a new incremental refresh message
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            md_req_id: None,
            entries: Vec::new(),
        }
    }

    /// Set request ID
    pub fn with_request_id(mut self, md_req_id: String) -> Self {
        self.md_req_id = Some(md_req_id);
        self
    }

    /// Add entries
    pub fn with_entries(mut self, entries: Vec<MdEntry>) -> Self {
        self.entries = entries;
        self
    }

    /// Convert to FIX message
    pub fn to_fix_message(
        &self,
        sender_comp_id: String,
        target_comp_id: String,
        msg_seq_num: u32,
    ) -> DeribitFixResult<String> {
        let mut builder = MessageBuilder::new()
            .msg_type(MsgType::MarketDataIncrementalRefresh)
            .sender_comp_id(sender_comp_id)
            .target_comp_id(target_comp_id)
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now())
            .field(55, self.symbol.clone()); // Symbol

        if let Some(ref md_req_id) = self.md_req_id {
            builder = builder.field(262, md_req_id.clone()); // MDReqID
        }

        // Add entries group
        builder = builder.field(268, self.entries.len().to_string()); // NoMDEntries

        for entry in &self.entries {
            if let Some(action) = entry.md_update_action {
                builder = builder.field(279, char::from(action).to_string()); // MDUpdateAction
            }

            builder = builder.field(269, i32::from(entry.md_entry_type).to_string()); // MDEntryType

            if let Some(px) = entry.md_entry_px {
                builder = builder.field(270, px.to_string()); // MDEntryPx
            }

            if let Some(size) = entry.md_entry_size {
                builder = builder.field(271, size.to_string()); // MDEntrySize
            }

            if let Some(date) = entry.md_entry_date {
                builder = builder.field(272, date.timestamp_millis().to_string()); // MDEntryDate
            }

            if let Some(ref trade_id) = entry.trade_id {
                builder = builder.field(100009, trade_id.clone()); // DeribitTradeId
            }

            if let Some(side) = entry.side {
                builder = builder.field(54, side.to_string()); // Side
            }
        }

        Ok(builder.build()?.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md_subscription_request_type_conversion() {
        assert_eq!(i32::from(MdSubscriptionRequestType::Snapshot), 0);
        assert_eq!(i32::from(MdSubscriptionRequestType::SnapshotPlusUpdates), 1);
        assert_eq!(i32::from(MdSubscriptionRequestType::Unsubscribe), 2);

        assert_eq!(
            MdSubscriptionRequestType::try_from(0).unwrap(),
            MdSubscriptionRequestType::Snapshot
        );
        assert_eq!(
            MdSubscriptionRequestType::try_from(1).unwrap(),
            MdSubscriptionRequestType::SnapshotPlusUpdates
        );
        assert_eq!(
            MdSubscriptionRequestType::try_from(2).unwrap(),
            MdSubscriptionRequestType::Unsubscribe
        );
        assert!(MdSubscriptionRequestType::try_from(99).is_err());
    }

    #[test]
    fn test_md_entry_type_conversion() {
        assert_eq!(i32::from(MdEntryType::Bid), 0);
        assert_eq!(i32::from(MdEntryType::Offer), 1);
        assert_eq!(i32::from(MdEntryType::Trade), 2);

        assert_eq!(MdEntryType::try_from(0).unwrap(), MdEntryType::Bid);
        assert_eq!(MdEntryType::try_from(1).unwrap(), MdEntryType::Offer);
        assert_eq!(MdEntryType::try_from(2).unwrap(), MdEntryType::Trade);
        assert!(MdEntryType::try_from(99).is_err());
    }

    #[test]
    fn test_market_data_request_creation() {
        let request = MarketDataRequest::snapshot(
            "REQ123".to_string(),
            vec!["BTC-PERPETUAL".to_string()],
            vec![MdEntryType::Bid, MdEntryType::Offer],
        );
        assert_eq!(request.md_req_id, "REQ123");
        assert_eq!(
            request.subscription_request_type,
            MdSubscriptionRequestType::Snapshot
        );
        assert_eq!(request.symbols.len(), 1);
        assert_eq!(request.entry_types.len(), 2);
    }

    #[test]
    fn test_market_data_request_reject_creation() {
        let reject =
            MarketDataRequestReject::new("REQ123".to_string(), MdReqRejReason::UnknownSymbol);
        assert_eq!(reject.md_req_id, "REQ123");
        assert_eq!(reject.md_req_rej_reason, MdReqRejReason::UnknownSymbol);
        assert!(reject.text.is_none());
    }

    #[test]
    fn test_md_req_rej_reason_conversion() {
        assert_eq!(char::from(MdReqRejReason::UnknownSymbol), '0');
        assert_eq!(char::from(MdReqRejReason::InsufficientCredit), 'D');

        assert_eq!(
            MdReqRejReason::try_from('0').unwrap(),
            MdReqRejReason::UnknownSymbol
        );
        assert_eq!(
            MdReqRejReason::try_from('D').unwrap(),
            MdReqRejReason::InsufficientCredit
        );
        assert!(MdReqRejReason::try_from('Z').is_err());
    }

    #[test]
    fn test_md_entry_creation() {
        let bid = MdEntry::bid(50000.0, 1.5);
        assert_eq!(bid.md_entry_type, MdEntryType::Bid);
        assert_eq!(bid.md_entry_px, Some(50000.0));
        assert_eq!(bid.md_entry_size, Some(1.5));
        assert!(bid.md_update_action.is_none());

        let offer = MdEntry::offer(50100.0, 2.0);
        assert_eq!(offer.md_entry_type, MdEntryType::Offer);
        assert_eq!(offer.md_entry_px, Some(50100.0));
        assert_eq!(offer.md_entry_size, Some(2.0));
    }

    #[test]
    fn test_market_data_snapshot_creation() {
        let snapshot = MarketDataSnapshotFullRefresh::new("BTC-PERPETUAL".to_string())
            .with_request_id("REQ123".to_string())
            .with_entries(vec![
                MdEntry::bid(50000.0, 1.0),
                MdEntry::offer(50100.0, 1.5),
            ]);

        assert_eq!(snapshot.symbol, "BTC-PERPETUAL");
        assert_eq!(snapshot.md_req_id, Some("REQ123".to_string()));
        assert_eq!(snapshot.entries.len(), 2);
    }

    #[test]
    fn test_market_data_incremental_creation() {
        let incremental = MarketDataIncrementalRefresh::new("BTC-PERPETUAL".to_string())
            .with_entries(vec![
                MdEntry::bid(50000.0, 1.0).with_update_action(MdUpdateAction::New),
                MdEntry::offer(50100.0, 1.5).with_update_action(MdUpdateAction::Change),
            ]);

        assert_eq!(incremental.symbol, "BTC-PERPETUAL");
        assert_eq!(incremental.entries.len(), 2);
        assert_eq!(
            incremental.entries[0].md_update_action,
            Some(MdUpdateAction::New)
        );
        assert_eq!(
            incremental.entries[1].md_update_action,
            Some(MdUpdateAction::Change)
        );
    }

    #[test]
    fn test_md_update_action_conversion() {
        assert_eq!(char::from(MdUpdateAction::New), '0');
        assert_eq!(char::from(MdUpdateAction::Change), '1');
        assert_eq!(char::from(MdUpdateAction::Delete), '2');

        assert_eq!(MdUpdateAction::try_from('0').unwrap(), MdUpdateAction::New);
        assert_eq!(
            MdUpdateAction::try_from('1').unwrap(),
            MdUpdateAction::Change
        );
        assert_eq!(
            MdUpdateAction::try_from('2').unwrap(),
            MdUpdateAction::Delete
        );
        assert!(MdUpdateAction::try_from('9').is_err());
    }
}
