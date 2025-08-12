/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 12/8/25
******************************************************************************/

//! FIX Position messages implementation
//!
//! This module provides functionality for creating and parsing FIX position
//! messages used in communication with Deribit, including:
//! - RequestForPositions (MsgType = "AN")
//! - PositionReport (MsgType = "AP")

use crate::error::{DeribitFixError, Result as DeribitFixResult};
use crate::message::MessageBuilder;
use crate::model::message::FixMessage;
use crate::model::types::MsgType;
use chrono::{DateTime, Utc};
use deribit_base::prelude::Position;
use serde::{Deserialize, Serialize};

/// Position request type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PosReqType {
    /// Positions (0)
    Positions,
    /// Trades (1)
    Trades,
    /// Exercises (2)
    Exercises,
    /// Assignments (3)
    Assignments,
}

impl From<PosReqType> for i32 {
    fn from(value: PosReqType) -> Self {
        match value {
            PosReqType::Positions => 0,
            PosReqType::Trades => 1,
            PosReqType::Exercises => 2,
            PosReqType::Assignments => 3,
        }
    }
}

impl TryFrom<i32> for PosReqType {
    type Error = DeribitFixError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PosReqType::Positions),
            1 => Ok(PosReqType::Trades),
            2 => Ok(PosReqType::Exercises),
            3 => Ok(PosReqType::Assignments),
            _ => Err(DeribitFixError::MessageParsing(format!(
                "Invalid PosReqType: {}",
                value
            ))),
        }
    }
}

/// Subscription request type for positions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubscriptionRequestType {
    /// Snapshot (0)
    Snapshot,
    /// Snapshot + Updates (1)
    SnapshotPlusUpdates,
    /// Disable previous snapshot + updates (2)
    DisablePreviousSnapshotPlusUpdates,
}

impl From<SubscriptionRequestType> for i32 {
    fn from(value: SubscriptionRequestType) -> Self {
        match value {
            SubscriptionRequestType::Snapshot => 0,
            SubscriptionRequestType::SnapshotPlusUpdates => 1,
            SubscriptionRequestType::DisablePreviousSnapshotPlusUpdates => 2,
        }
    }
}

impl TryFrom<i32> for SubscriptionRequestType {
    type Error = DeribitFixError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SubscriptionRequestType::Snapshot),
            1 => Ok(SubscriptionRequestType::SnapshotPlusUpdates),
            2 => Ok(SubscriptionRequestType::DisablePreviousSnapshotPlusUpdates),
            _ => Err(DeribitFixError::MessageParsing(format!(
                "Invalid SubscriptionRequestType: {}",
                value
            ))),
        }
    }
}

/// Request For Positions message (MsgType = "AN")
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestForPositions {
    /// Position Request ID (710)
    pub pos_req_id: String,
    /// Position Request Type (724)
    pub pos_req_type: PosReqType,
    /// Subscription Request Type (263) - optional
    pub subscription_request_type: Option<SubscriptionRequestType>,
    /// Clearing Business Date (715) - optional
    pub clearing_business_date: Option<String>,
    /// Symbols filter - optional
    pub symbols: Vec<String>,
}

impl RequestForPositions {
    /// Create a new position request for all positions
    pub fn all_positions(pos_req_id: String) -> Self {
        Self {
            pos_req_id,
            pos_req_type: PosReqType::Positions,
            subscription_request_type: Some(SubscriptionRequestType::Snapshot),
            clearing_business_date: None,
            symbols: Vec::new(),
        }
    }

    /// Create a new position request with subscription for updates
    pub fn positions_with_updates(pos_req_id: String) -> Self {
        Self {
            pos_req_id,
            pos_req_type: PosReqType::Positions,
            subscription_request_type: Some(SubscriptionRequestType::SnapshotPlusUpdates),
            clearing_business_date: None,
            symbols: Vec::new(),
        }
    }

    /// Add symbols filter
    pub fn with_symbols(mut self, symbols: Vec<String>) -> Self {
        self.symbols = symbols;
        self
    }

    /// Add clearing business date
    pub fn with_clearing_date(mut self, date: String) -> Self {
        self.clearing_business_date = Some(date);
        self
    }

    /// Convert to FIX message
    pub fn to_fix_message(
        &self,
        sender_comp_id: String,
        target_comp_id: String,
        msg_seq_num: u32,
    ) -> DeribitFixResult<FixMessage> {
        let mut builder = MessageBuilder::new()
            .msg_type(MsgType::RequestForPositions)
            .sender_comp_id(sender_comp_id)
            .target_comp_id(target_comp_id)
            .msg_seq_num(msg_seq_num)
            .field(710, self.pos_req_id.clone()) // PosReqID
            .field(724, i32::from(self.pos_req_type).to_string()); // PosReqType

        // Add optional subscription request type
        if let Some(subscription_type) = self.subscription_request_type {
            builder = builder.field(263, i32::from(subscription_type).to_string());
        }

        // Add optional clearing business date
        if let Some(ref date) = self.clearing_business_date {
            builder = builder.field(715, date.clone());
        }

        // Add symbols if present
        if !self.symbols.is_empty() {
            builder = builder.field(146, self.symbols.len().to_string()); // NoRelatedSym
            for symbol in &self.symbols {
                builder = builder.field(55, symbol.clone()); // Symbol
            }
        }

        builder.build()
    }
}

/// Position Report message (MsgType = "AP")
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PositionReport {
    /// Position Request ID (710)
    pub pos_req_id: String,
    /// Symbol (55)
    pub symbol: String,
    /// Position Quantity (703)
    pub position_qty: Option<f64>,
    /// Average Price (6)
    pub average_price: Option<f64>,
    /// Unrealized PnL (1247)
    pub unrealized_pnl: Option<f64>,
    /// Realized PnL (1248)
    pub realized_pnl: Option<f64>,
    /// Position date (704) - optional
    pub position_date: Option<String>,
    /// Last update time
    pub last_update_time: Option<DateTime<Utc>>,
}

impl PositionReport {
    /// Parse from FIX message
    pub fn from_fix_message(message: &FixMessage) -> DeribitFixResult<Self> {
        let pos_req_id = message
            .get_field(710)
            .ok_or_else(|| DeribitFixError::MessageParsing("Missing PosReqID (710)".to_string()))?
            .clone();

        let symbol = message
            .get_field(55)
            .ok_or_else(|| DeribitFixError::MessageParsing("Missing Symbol (55)".to_string()))?
            .clone();

        let position_qty = message
            .get_field(703)
            .and_then(|s| s.parse::<f64>().ok());

        let average_price = message
            .get_field(6)
            .and_then(|s| s.parse::<f64>().ok());

        let unrealized_pnl = message
            .get_field(1247)
            .and_then(|s| s.parse::<f64>().ok());

        let realized_pnl = message
            .get_field(1248)
            .and_then(|s| s.parse::<f64>().ok());

        let position_date = message.get_field(704).cloned();

        Ok(Self {
            pos_req_id,
            symbol,
            position_qty,
            average_price,
            unrealized_pnl,
            realized_pnl,
            position_date,
            last_update_time: Some(Utc::now()),
        })
    }

    /// Convert to deribit_base Position
    pub fn to_position(&self) -> Position {
        Position {
            symbol: self.symbol.clone(),
            quantity: self.position_qty.unwrap_or(0.0),
            average_price: self.average_price.unwrap_or(0.0),
            realized_pnl: self.realized_pnl.unwrap_or(0.0),
            unrealized_pnl: self.unrealized_pnl.unwrap_or(0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pos_req_type_conversion() {
        assert_eq!(i32::from(PosReqType::Positions), 0);
        assert_eq!(i32::from(PosReqType::Trades), 1);
        
        assert_eq!(PosReqType::try_from(0).unwrap(), PosReqType::Positions);
        assert_eq!(PosReqType::try_from(1).unwrap(), PosReqType::Trades);
        
        assert!(PosReqType::try_from(99).is_err());
    }

    #[test]
    fn test_subscription_request_type_conversion() {
        assert_eq!(i32::from(SubscriptionRequestType::Snapshot), 0);
        assert_eq!(i32::from(SubscriptionRequestType::SnapshotPlusUpdates), 1);
        
        assert_eq!(SubscriptionRequestType::try_from(0).unwrap(), SubscriptionRequestType::Snapshot);
        assert_eq!(SubscriptionRequestType::try_from(1).unwrap(), SubscriptionRequestType::SnapshotPlusUpdates);
        
        assert!(SubscriptionRequestType::try_from(99).is_err());
    }

    #[test]
    fn test_request_for_positions_creation() {
        let request = RequestForPositions::all_positions("POS_123".to_string());
        assert_eq!(request.pos_req_id, "POS_123");
        assert_eq!(request.pos_req_type, PosReqType::Positions);
        assert_eq!(request.subscription_request_type, Some(SubscriptionRequestType::Snapshot));
    }

    #[test]
    fn test_request_for_positions_with_symbols() {
        let request = RequestForPositions::all_positions("POS_123".to_string())
            .with_symbols(vec!["BTC-PERPETUAL".to_string(), "ETH-PERPETUAL".to_string()]);
        
        assert_eq!(request.symbols.len(), 2);
        assert!(request.symbols.contains(&"BTC-PERPETUAL".to_string()));
    }

    #[test]
    fn test_request_for_positions_to_fix_message() {
        let request = RequestForPositions::all_positions("POS_123".to_string());
        let fix_message = request.to_fix_message(
            "SENDER".to_string(),
            "TARGET".to_string(),
            1,
        ).unwrap();
        
        // Test field values directly
        assert_eq!(fix_message.get_field(35), Some(&"AN".to_string())); // MsgType
        assert_eq!(fix_message.get_field(710), Some(&"POS_123".to_string())); // PosReqID
        assert_eq!(fix_message.get_field(724), Some(&"0".to_string())); // PosReqType
        assert_eq!(fix_message.get_field(263), Some(&"0".to_string())); // SubscriptionRequestType
    }

    #[test]
    fn test_position_report_from_fix_message() {
        // Create a FixMessage manually by setting fields
        let mut fix_message = FixMessage::new();
        fix_message.set_field(710, "POS_123".to_string());
        fix_message.set_field(55, "BTC-PERPETUAL".to_string());
        fix_message.set_field(703, "1.5".to_string());
        fix_message.set_field(6, "50000.0".to_string());
        fix_message.set_field(1247, "100.0".to_string());
        fix_message.set_field(1248, "50.0".to_string());

        let position_report = PositionReport::from_fix_message(&fix_message).unwrap();

        assert_eq!(position_report.pos_req_id, "POS_123");
        assert_eq!(position_report.symbol, "BTC-PERPETUAL");
        assert_eq!(position_report.position_qty, Some(1.5));
        assert_eq!(position_report.average_price, Some(50000.0));
        assert_eq!(position_report.unrealized_pnl, Some(100.0));
        assert_eq!(position_report.realized_pnl, Some(50.0));
    }

    #[test]
    fn test_position_report_to_position() {
        let position_report = PositionReport {
            pos_req_id: "POS_123".to_string(),
            symbol: "BTC-PERPETUAL".to_string(),
            position_qty: Some(1.5),
            average_price: Some(50000.0),
            unrealized_pnl: Some(100.0),
            realized_pnl: Some(50.0),
            position_date: None,
            last_update_time: None,
        };

        let position = position_report.to_position();
        assert_eq!(position.symbol, "BTC-PERPETUAL");
        assert_eq!(position.quantity, 1.5);
        assert_eq!(position.average_price, 50000.0);
        assert_eq!(position.unrealized_pnl, 100.0);
        assert_eq!(position.realized_pnl, 50.0);
    }
}