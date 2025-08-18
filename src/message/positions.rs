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

use crate::error::Result;
use crate::error::{DeribitFixError, Result as DeribitFixResult};
use crate::message::MessageBuilder;
use crate::model::message::FixMessage;
use crate::model::types::MsgType;

use deribit_base::model::position::Direction;
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

    fn try_from(value: i32) -> Result<Self> {
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

    fn try_from(value: i32) -> Result<Self> {
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

/// Represents a FIX Position Report message (MsgType = AP).
pub struct PositionReport;

impl PositionReport {
    /// Parse a Position from a FIX message (Position Report-like payload).
    ///
    /// This function extracts Deribit position information from a FixMessage by
    /// reading standard FIX tags and Deribit extensions. It computes derived fields
    /// such as net size and direction, and maps several optional numeric fields into
    /// greeks and margin metrics.
    ///
    /// Behavior:
    /// - Instrument name (tag 55) is mandatory; absence results in an error.
    /// - LongQty (704) and ShortQty (705) are read and netted as `size = long - short`.
    /// - `direction` is Buy if `size > 0.0`, otherwise Sell.
    /// - Many numeric fields are optional; if missing or unparsable, they default to:
    ///   - f64 options: None
    ///   - Aggregated numeric values used directly (e.g., `average_price`) default to 0.0
    /// - Settlement price (730) is reused as both `average_price` and `settlement_price`.
    ///
    /// Tag mapping:
    /// - 55 (Symbol) -> Position.instrument_name
    /// - 704 (LongQty) and 705 (ShortQty) -> Position.size (long - short)
    /// - 730 (SettlPx) -> Position.average_price and Position.settlement_price
    /// - 731 (IndexPx) -> Position.index_price
    /// - 732 (MarkPx) -> Position.mark_price
    /// - 898 (MaintenanceMargin) -> Position.maintenance_margin
    /// - 899 (InitialMargin) -> Position.initial_margin
    /// - 706 (RealizedPnL) -> Position.realized_profit_loss
    /// - 707 (UnrealizedPnL) -> Position.floating_profit_loss and Position.unrealized_profit_loss
    /// - 708 (TotalPnL) -> Position.total_profit_loss
    /// - 811 (Delta), 812 (Gamma), 813 (Theta), 814 (Vega) -> Position greeks
    /// - 461 (CFICode) -> Position.kind
    ///
    /// Errors:
    /// - Returns DeribitFixError::Generic when tag 55 (Symbol) is missing.
    ///
    /// Note:
    /// - Fields like `average_price_usd`, `interest_value`, `leverage`, `open_orders_margin`,
    ///   `realized_funding`, and `size_currency` are not provided by this parser and remain `None`.
    ///
    /// Returns:
    /// - Ok(Position) when parsing succeeds
    /// - Err(DeribitFixError) if the required symbol (tag 55) is missing
    pub fn try_from_fix_message(message: &FixMessage) -> Result<Position> {
        let get_f64 = |tag| message.get_field(tag).and_then(|s| s.parse::<f64>().ok());
        let get_string = |tag| message.get_field(tag).map(|s| s.to_string());

        let instrument_name = get_string(55).ok_or_else(|| {
            DeribitFixError::Generic("Missing instrument name (tag 55)".to_string())
        })?;
        let long_qty = get_f64(704).unwrap_or(0.0);
        let short_qty = get_f64(705).unwrap_or(0.0);
        let size = long_qty - short_qty;
        let direction = if size > 0.0 {
            Direction::Buy
        } else {
            Direction::Sell
        };
        let average_price = get_f64(730).unwrap_or(0.0);

        Ok(Position {
            instrument_name,
            size,
            direction,
            average_price,
            average_price_usd: None,
            delta: get_f64(811), // Greeks delta
            estimated_liquidation_price: None,
            floating_profit_loss: get_f64(707), // Unrealized PnL
            floating_profit_loss_usd: None,
            gamma: get_f64(812),          // Greeks gamma
            index_price: get_f64(731),    // Index price
            initial_margin: get_f64(899), // Initial margin
            interest_value: None,
            kind: get_string(461), // CFICode for instrument type
            leverage: None,
            maintenance_margin: get_f64(898), // Maintenance margin
            mark_price: get_f64(732),         // Mark price
            open_orders_margin: None,
            realized_funding: None,
            realized_profit_loss: get_f64(706), // Realized PnL
            settlement_price: get_f64(730),     // Settlement price (same as avg price for now)
            size_currency: None,
            theta: get_f64(813),                  // Greeks theta
            total_profit_loss: get_f64(708),      // Total PnL
            vega: get_f64(814),                   // Greeks vega
            unrealized_profit_loss: get_f64(707), // Same as floating PnL
        })
    }

    /// Builds a FIX Position Report (MsgType=AP) from a Deribit `Position`.
    ///
    /// This function converts a Deribit position into a FIX message using
    /// the provided message metadata (sender/target IDs and sequence number).
    /// Only fields present in the input position are included in the resulting
    /// message, and position-side determines whether LongQty or ShortQty is used.
    ///
    /// FIX tags populated:
    /// - 35 (MsgType): AP (PositionReport)
    /// - 49 (SenderCompID): from `sender_comp_id`
    /// - 56 (TargetCompID): from `target_comp_id`
    /// - 34 (MsgSeqNum): from `msg_seq_num`
    /// - 55 (Symbol): from `position.instrument_name`
    /// - 730 (SettlPx): from `position.average_price`
    /// - 704 (LongQty): if `position.direction` is Buy, with `position.size`
    /// - 705 (ShortQty): if `position.direction` is Sell, with `abs(position.size)`
    /// - 706 (PosAmt Realized PnL): from `position.realized_profit_loss` (if set)
    /// - 707 (PosAmt Floating/Unrealized PnL): from `position.floating_profit_loss` (if set)
    /// - 708 (PosAmt Total PnL): from `position.total_profit_loss` (if set)
    /// - 811 (Delta): from `position.delta` (if set)
    /// - 812 (Gamma): from `position.gamma` (if set)
    /// - 813 (Theta): from `position.theta` (if set)
    /// - 814 (Vega): from `position.vega` (if set)
    /// - 731 (IndexPx): from `position.index_price` (if set)
    /// - 732 (MarkPx): from `position.mark_price` (if set)
    /// - 899 (InitialMargin): from `position.initial_margin` (if set)
    /// - 898 (MaintenanceMargin): from `position.maintenance_margin` (if set)
    /// - 979 (PosAmtType): constant "FMTM"
    ///
    /// Parameters:
    /// - `position`: Source Deribit position to translate.
    /// - `sender_comp_id`: Value for FIX tag 49 (SenderCompID).
    /// - `target_comp_id`: Value for FIX tag 56 (TargetCompID).
    /// - `msg_seq_num`: Value for FIX tag 34 (MsgSeqNum).
    ///
    /// Returns:
    /// - `Ok(String)`: The serialized FIX message string when message building succeeds.
    /// - `Err(DeribitFixError)`: If the underlying builder fails to construct the message.
    ///
    /// Notes:
    /// - Quantity tag selection depends on `position.direction`:
    ///   - Buy -> 704=LongQty
    ///   - Sell -> 705=ShortQty (absolute value)
    /// - Optional numeric fields are only included when present in `position`.
    pub fn from_deribit_position(
        position: &Position,
        sender_comp_id: String,
        target_comp_id: String,
        msg_seq_num: u32,
    ) -> Result<String> {
        let msg = MessageBuilder::new()
            .msg_type(MsgType::PositionReport)
            .sender_comp_id(sender_comp_id)
            .target_comp_id(target_comp_id)
            .msg_seq_num(msg_seq_num);

        // Add position-specific fields
        let msg = msg.field(55, position.instrument_name.clone()); // Symbol
        let msg = msg.field(730, position.average_price.to_string()); // SettlPx

        // Add position quantity based on direction
        let msg = match position.direction {
            Direction::Buy => msg.field(704, position.size.to_string()), // LongQty
            Direction::Sell => msg.field(705, position.size.abs().to_string()), // ShortQty (absolute value)
        };

        // Add other position fields (only if they exist)
        let msg = if let Some(realized_pnl) = position.realized_profit_loss {
            msg.field(706, realized_pnl.to_string())
        } else {
            msg
        };

        let msg = if let Some(floating_pnl) = position.floating_profit_loss {
            msg.field(707, floating_pnl.to_string())
        } else {
            msg
        };

        let msg = if let Some(total_pnl) = position.total_profit_loss {
            msg.field(708, total_pnl.to_string())
        } else {
            msg
        };

        // Add Greeks if available
        let msg = if let Some(delta) = position.delta {
            msg.field(811, delta.to_string())
        } else {
            msg
        };

        let msg = if let Some(gamma) = position.gamma {
            msg.field(812, gamma.to_string())
        } else {
            msg
        };

        let msg = if let Some(theta) = position.theta {
            msg.field(813, theta.to_string())
        } else {
            msg
        };

        let msg = if let Some(vega) = position.vega {
            msg.field(814, vega.to_string())
        } else {
            msg
        };

        // Add other optional fields
        let msg = if let Some(index_price) = position.index_price {
            msg.field(731, index_price.to_string())
        } else {
            msg
        };

        let msg = if let Some(mark_price) = position.mark_price {
            msg.field(732, mark_price.to_string())
        } else {
            msg
        };

        let msg = if let Some(initial_margin) = position.initial_margin {
            msg.field(899, initial_margin.to_string())
        } else {
            msg
        };

        let msg = if let Some(maintenance_margin) = position.maintenance_margin {
            msg.field(898, maintenance_margin.to_string())
        } else {
            msg
        };

        let msg = msg.field(979, "FMTM".to_string()); // PosAmtType

        Ok(msg.build()?.to_string())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::message::FixMessage;

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

        assert_eq!(
            SubscriptionRequestType::try_from(0).unwrap(),
            SubscriptionRequestType::Snapshot
        );
        assert_eq!(
            SubscriptionRequestType::try_from(1).unwrap(),
            SubscriptionRequestType::SnapshotPlusUpdates
        );

        assert!(SubscriptionRequestType::try_from(99).is_err());
    }

    #[test]
    fn test_request_for_positions_creation() {
        let request = RequestForPositions::all_positions("POS_123".to_string());
        assert_eq!(request.pos_req_id, "POS_123");
        assert_eq!(request.pos_req_type, PosReqType::Positions);
        assert_eq!(
            request.subscription_request_type,
            Some(SubscriptionRequestType::Snapshot)
        );
    }

    #[test]
    fn test_request_for_positions_with_symbols() {
        let request = RequestForPositions::all_positions("POS_123".to_string()).with_symbols(vec![
            "BTC-PERPETUAL".to_string(),
            "ETH-PERPETUAL".to_string(),
        ]);

        assert_eq!(request.symbols.len(), 2);
        assert!(request.symbols.contains(&"BTC-PERPETUAL".to_string()));
    }

    #[test]
    fn test_request_for_positions_to_fix_message() {
        let request = RequestForPositions::all_positions("POS_123".to_string());
        let fix_message = request
            .to_fix_message("SENDER".to_string(), "TARGET".to_string(), 1)
            .unwrap();

        // Test field values directly
        assert_eq!(fix_message.get_field(35), Some(&"AN".to_string())); // MsgType
        assert_eq!(fix_message.get_field(710), Some(&"POS_123".to_string())); // PosReqID
        assert_eq!(fix_message.get_field(724), Some(&"0".to_string())); // PosReqType
        assert_eq!(fix_message.get_field(263), Some(&"0".to_string())); // SubscriptionRequestType
    }

    #[test]
    fn test_position_report_try_from_fix_message() {
        // Create a FixMessage manually by setting fields
        let mut fix_message = FixMessage::new();
        fix_message.set_field(55, "BTC-PERPETUAL".to_string()); // Symbol
        fix_message.set_field(704, "1.5".to_string()); // LongQty
        fix_message.set_field(705, "0.0".to_string()); // ShortQty
        fix_message.set_field(730, "50000.0".to_string()); // SettlPx (average price)
        fix_message.set_field(707, "100.0".to_string()); // Unrealized PnL
        fix_message.set_field(706, "50.0".to_string()); // Realized PnL

        let position = PositionReport::try_from_fix_message(&fix_message).unwrap();

        assert_eq!(position.instrument_name, "BTC-PERPETUAL");
        assert_eq!(position.size, 1.5);
        assert_eq!(position.average_price, 50000.0);
        assert!(matches!(position.direction, Direction::Buy));
        assert_eq!(position.floating_profit_loss, Some(100.0));
        assert_eq!(position.realized_profit_loss, Some(50.0));
    }

    #[test]
    fn test_position_report_from_deribit_position() {
        // Create a Position struct
        let position = Position {
            instrument_name: "ETH-PERPETUAL".to_string(),
            size: 2.0,
            direction: Direction::Buy,
            average_price: 3500.0,
            average_price_usd: None,
            delta: Some(0.5),
            estimated_liquidation_price: None,
            floating_profit_loss: Some(150.0),
            floating_profit_loss_usd: None,
            gamma: Some(0.001),
            index_price: Some(3520.0),
            initial_margin: Some(100.0),
            interest_value: None,
            kind: Some("future".to_string()),
            leverage: None,
            maintenance_margin: Some(50.0),
            mark_price: Some(3510.0),
            open_orders_margin: None,
            realized_funding: None,
            realized_profit_loss: Some(50.0),
            settlement_price: Some(3500.0),
            size_currency: None,
            theta: Some(-0.1),
            total_profit_loss: Some(200.0),
            vega: Some(0.05),
            unrealized_profit_loss: Some(150.0),
        };

        let fix_message = PositionReport::from_deribit_position(
            &position,
            "SENDER".to_string(),
            "TARGET".to_string(),
            1,
        )
        .unwrap();

        // Verify the FIX message contains expected fields
        assert!(fix_message.contains("55=ETH-PERPETUAL")); // Symbol
        assert!(fix_message.contains("704=2")); // LongQty
        assert!(fix_message.contains("730=3500")); // SettlPx
    }

    #[test]
    fn test_position_direction_sell() {
        // Create a FixMessage with negative position (sell)
        let mut fix_message = FixMessage::new();
        fix_message.set_field(55, "BTC-PERPETUAL".to_string()); // Symbol
        fix_message.set_field(704, "0.0".to_string()); // LongQty
        fix_message.set_field(705, "1.0".to_string()); // ShortQty
        fix_message.set_field(730, "45000.0".to_string()); // SettlPx

        let position = PositionReport::try_from_fix_message(&fix_message).unwrap();

        assert_eq!(position.instrument_name, "BTC-PERPETUAL");
        assert_eq!(position.size, -1.0); // Negative size for sell
        assert!(matches!(position.direction, Direction::Sell));
        assert_eq!(position.average_price, 45000.0);
    }
}
