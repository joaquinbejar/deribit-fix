/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 12/8/25
******************************************************************************/

//! Trade Capture Report FIX Message Implementation

use crate::error::Result as DeribitFixResult;
use crate::message::builder::MessageBuilder;
use crate::message::orders::OrderSide;
use crate::model::types::MsgType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Trade Capture Report Leg for multi-leg trades (NoLegs group)
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeCaptureReportLeg {
    /// Leg symbol (tag 600)
    pub leg_symbol: String,
    /// Leg quantity (tag 687)
    pub leg_qty: f64,
    /// Leg price (tag 566)
    pub leg_price: f64,
    /// Leg side (tag 624)
    pub leg_side: OrderSide,
}

impl TradeCaptureReportLeg {
    /// Create a new trade capture report leg
    pub fn new(leg_symbol: String, leg_qty: f64, leg_price: f64, leg_side: OrderSide) -> Self {
        Self {
            leg_symbol,
            leg_qty,
            leg_price,
            leg_side,
        }
    }
}

impl_json_display!(TradeCaptureReportLeg);
impl_json_debug_pretty!(TradeCaptureReportLeg);

/// Trade Capture Report Side for NoSides group
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeCaptureReportSide {
    /// Side (tag 54)
    pub side: OrderSide,
    /// Order ID (tag 37)
    pub order_id: String,
    /// Commission (tag 12)
    pub commission: Option<f64>,
    /// Commission currency (tag 479)
    pub comm_currency: Option<String>,
}

impl TradeCaptureReportSide {
    /// Create a new trade capture report side
    pub fn new(side: OrderSide, order_id: String) -> Self {
        Self {
            side,
            order_id,
            commission: None,
            comm_currency: None,
        }
    }

    /// Set commission
    #[must_use]
    pub fn with_commission(mut self, commission: f64, currency: String) -> Self {
        self.commission = Some(commission);
        self.comm_currency = Some(currency);
        self
    }
}

impl_json_display!(TradeCaptureReportSide);
impl_json_debug_pretty!(TradeCaptureReportSide);

/// Trade capture report type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeCaptureReportType {
    /// Submit
    Submit,
    /// Alleged
    Alleged,
    /// Accept
    Accept,
    /// Decline
    Decline,
    /// Addendum
    Addendum,
    /// No/Was
    NoWas,
    /// Trade Report Cancel
    TradeReportCancel,
    /// Locked In Trade Break
    LockedInTradeBreak,
    /// Restated
    Restated,
}

impl From<TradeCaptureReportType> for i32 {
    fn from(report_type: TradeCaptureReportType) -> Self {
        match report_type {
            TradeCaptureReportType::Submit => 0,
            TradeCaptureReportType::Alleged => 1,
            TradeCaptureReportType::Accept => 2,
            TradeCaptureReportType::Decline => 3,
            TradeCaptureReportType::Addendum => 4,
            TradeCaptureReportType::NoWas => 5,
            TradeCaptureReportType::TradeReportCancel => 6,
            TradeCaptureReportType::LockedInTradeBreak => 7,
            TradeCaptureReportType::Restated => 8,
        }
    }
}

impl TryFrom<i32> for TradeCaptureReportType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TradeCaptureReportType::Submit),
            1 => Ok(TradeCaptureReportType::Alleged),
            2 => Ok(TradeCaptureReportType::Accept),
            3 => Ok(TradeCaptureReportType::Decline),
            4 => Ok(TradeCaptureReportType::Addendum),
            5 => Ok(TradeCaptureReportType::NoWas),
            6 => Ok(TradeCaptureReportType::TradeReportCancel),
            7 => Ok(TradeCaptureReportType::LockedInTradeBreak),
            8 => Ok(TradeCaptureReportType::Restated),
            _ => Err(format!("Invalid TradeCaptureReportType: {}", value)),
        }
    }
}

/// Trade report transaction type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeReportTransType {
    /// New
    New,
    /// Cancel
    Cancel,
    /// Replace
    Replace,
    /// Release
    Release,
    /// Reverse
    Reverse,
    /// Cancel Due To Back Out Of Trade
    CancelDueToBackOutOfTrade,
}

impl From<TradeReportTransType> for i32 {
    fn from(trans_type: TradeReportTransType) -> Self {
        match trans_type {
            TradeReportTransType::New => 0,
            TradeReportTransType::Cancel => 1,
            TradeReportTransType::Replace => 2,
            TradeReportTransType::Release => 3,
            TradeReportTransType::Reverse => 4,
            TradeReportTransType::CancelDueToBackOutOfTrade => 5,
        }
    }
}

impl TryFrom<i32> for TradeReportTransType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TradeReportTransType::New),
            1 => Ok(TradeReportTransType::Cancel),
            2 => Ok(TradeReportTransType::Replace),
            3 => Ok(TradeReportTransType::Release),
            4 => Ok(TradeReportTransType::Reverse),
            5 => Ok(TradeReportTransType::CancelDueToBackOutOfTrade),
            _ => Err(format!("Invalid TradeReportTransType: {}", value)),
        }
    }
}

/// Trade Capture Report message (MsgType = 'AE')
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeCaptureReport {
    /// Trade report ID
    pub trade_report_id: String,
    /// Trade ID
    pub trade_id: Option<String>,
    /// Secondary trade ID
    pub secondary_trade_id: Option<String>,
    /// Firm trade ID
    pub firm_trade_id: Option<String>,
    /// Secondary firm trade ID
    pub secondary_firm_trade_id: Option<String>,
    /// Trade report transaction type
    pub trade_report_trans_type: Option<TradeReportTransType>,
    /// Trade report type
    pub trade_report_type: Option<TradeCaptureReportType>,
    /// Trade request ID
    pub trade_request_id: Option<String>,
    /// TrdType
    pub trd_type: Option<i32>,
    /// Trade sub type
    pub trade_sub_type: Option<i32>,
    /// Transfer reason
    pub transfer_reason: Option<String>,
    /// Instrument symbol
    pub symbol: String,
    /// Side
    pub side: OrderSide,
    /// Order quantity
    pub order_qty: Option<f64>,
    /// Quantity
    pub quantity: f64,
    /// Last quantity
    pub last_qty: f64,
    /// Last price
    pub last_px: f64,
    /// Gross trade amount
    pub gross_trade_amt: Option<f64>,
    /// Execute time
    pub exec_time: Option<DateTime<Utc>>,
    /// Settlement date
    pub settlement_date: Option<String>,
    /// Trade date
    pub trade_date: String,
    /// Transaction time
    pub transact_time: DateTime<Utc>,
    /// Multi leg reporting type
    pub multi_leg_reporting_type: Option<char>,
    /// Previously reported
    pub previously_reported: Option<bool>,
    /// Price type
    pub price_type: Option<i32>,
    /// Underlying price
    pub underlying_price: Option<f64>,
    /// Underlying start value
    pub underlying_start_value: Option<f64>,
    /// Underlying current value
    pub underlying_current_value: Option<f64>,
    /// Underlying end value
    pub underlying_end_value: Option<f64>,
    /// Account
    pub account: Option<String>,
    /// Clearing account
    pub clearing_account: Option<String>,
    /// Account type
    pub account_type: Option<i32>,
    /// Position effect
    pub position_effect: Option<char>,
    /// Prealloc method
    pub prealloc_method: Option<char>,
    /// Clearing business date
    pub clearing_business_date: Option<String>,
    /// Trading session ID
    pub trading_session_id: Option<String>,
    /// Trading session sub ID
    pub trading_session_sub_id: Option<String>,
    /// Market segment ID
    pub market_segment_id: Option<String>,
    /// Text
    pub text: Option<String>,
    /// Encoded text
    pub encoded_text: Option<Vec<u8>>,
    /// Custom label
    pub deribit_label: Option<String>,
    /// Block trade match ID (tag 880) - Block Trade ID or Combo Trade ID
    pub trd_match_id: Option<String>,
    /// Legs for multi-leg trades (NoLegs group, tag 555)
    pub legs: Vec<TradeCaptureReportLeg>,
    /// Sides for trade reporting (NoSides group, tag 552)
    pub sides: Vec<TradeCaptureReportSide>,
}

impl TradeCaptureReport {
    /// Create a new trade capture report
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        trade_report_id: String,
        symbol: String,
        side: OrderSide,
        quantity: f64,
        last_qty: f64,
        last_px: f64,
        trade_date: String,
    ) -> Self {
        Self {
            trade_report_id,
            trade_id: None,
            secondary_trade_id: None,
            firm_trade_id: None,
            secondary_firm_trade_id: None,
            trade_report_trans_type: None,
            trade_report_type: None,
            trade_request_id: None,
            trd_type: None,
            trade_sub_type: None,
            transfer_reason: None,
            symbol,
            side,
            order_qty: None,
            quantity,
            last_qty,
            last_px,
            gross_trade_amt: None,
            exec_time: None,
            settlement_date: None,
            trade_date,
            transact_time: Utc::now(),
            multi_leg_reporting_type: None,
            previously_reported: None,
            price_type: None,
            underlying_price: None,
            underlying_start_value: None,
            underlying_current_value: None,
            underlying_end_value: None,
            account: None,
            clearing_account: None,
            account_type: None,
            position_effect: None,
            prealloc_method: None,
            clearing_business_date: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            market_segment_id: None,
            text: None,
            encoded_text: None,
            deribit_label: None,
            trd_match_id: None,
            legs: Vec::new(),
            sides: Vec::new(),
        }
    }

    /// Create a new trade report
    #[allow(clippy::too_many_arguments)]
    pub fn new_trade(
        trade_report_id: String,
        trade_id: String,
        symbol: String,
        side: OrderSide,
        quantity: f64,
        last_qty: f64,
        last_px: f64,
        trade_date: String,
    ) -> Self {
        let mut report = Self::new(
            trade_report_id,
            symbol,
            side,
            quantity,
            last_qty,
            last_px,
            trade_date,
        );
        report.trade_id = Some(trade_id);
        report.trade_report_trans_type = Some(TradeReportTransType::New);
        report.trade_report_type = Some(TradeCaptureReportType::Submit);
        report
    }

    /// Create a trade cancellation report
    pub fn cancel_trade(
        trade_report_id: String,
        original_trade_id: String,
        symbol: String,
        side: OrderSide,
        quantity: f64,
        trade_date: String,
    ) -> Self {
        let mut report = Self::new(
            trade_report_id,
            symbol,
            side,
            quantity,
            quantity,
            0.0,
            trade_date,
        );
        report.trade_id = Some(original_trade_id);
        report.trade_report_trans_type = Some(TradeReportTransType::Cancel);
        report.trade_report_type = Some(TradeCaptureReportType::TradeReportCancel);
        report
    }

    /// Set trade ID
    pub fn with_trade_id(mut self, trade_id: String) -> Self {
        self.trade_id = Some(trade_id);
        self
    }

    /// Set trade report transaction type
    pub fn with_trans_type(mut self, trans_type: TradeReportTransType) -> Self {
        self.trade_report_trans_type = Some(trans_type);
        self
    }

    /// Set trade report type
    pub fn with_report_type(mut self, report_type: TradeCaptureReportType) -> Self {
        self.trade_report_type = Some(report_type);
        self
    }

    /// Set trade request ID
    pub fn with_trade_request_id(mut self, trade_request_id: String) -> Self {
        self.trade_request_id = Some(trade_request_id);
        self
    }

    /// Set order quantity
    pub fn with_order_qty(mut self, order_qty: f64) -> Self {
        self.order_qty = Some(order_qty);
        self
    }

    /// Set gross trade amount
    pub fn with_gross_trade_amount(mut self, amount: f64) -> Self {
        self.gross_trade_amt = Some(amount);
        self
    }

    /// Set execution time
    pub fn with_exec_time(mut self, exec_time: DateTime<Utc>) -> Self {
        self.exec_time = Some(exec_time);
        self
    }

    /// Set settlement date
    pub fn with_settlement_date(mut self, settlement_date: String) -> Self {
        self.settlement_date = Some(settlement_date);
        self
    }

    /// Set account
    pub fn with_account(mut self, account: String) -> Self {
        self.account = Some(account);
        self
    }

    /// Set position effect
    pub fn with_position_effect(mut self, position_effect: char) -> Self {
        self.position_effect = Some(position_effect);
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

    /// Set block trade match ID (tag 880)
    #[must_use]
    pub fn with_trd_match_id(mut self, trd_match_id: String) -> Self {
        self.trd_match_id = Some(trd_match_id);
        self
    }

    /// Add a leg for multi-leg trade
    #[must_use]
    pub fn with_leg(mut self, leg: TradeCaptureReportLeg) -> Self {
        self.legs.push(leg);
        self
    }

    /// Set legs for multi-leg trade
    #[must_use]
    pub fn with_legs(mut self, legs: Vec<TradeCaptureReportLeg>) -> Self {
        self.legs = legs;
        self
    }

    /// Add a side for trade reporting
    #[must_use]
    pub fn with_side(mut self, side: TradeCaptureReportSide) -> Self {
        self.sides.push(side);
        self
    }

    /// Set sides for trade reporting
    #[must_use]
    pub fn with_sides(mut self, sides: Vec<TradeCaptureReportSide>) -> Self {
        self.sides = sides;
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
            .msg_type(MsgType::TradeCaptureReport)
            .sender_comp_id(sender_comp_id.to_string())
            .target_comp_id(target_comp_id.to_string())
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now());

        // Required fields
        builder = builder
            .field(571, self.trade_report_id.clone()) // TradeReportID
            .field(55, self.symbol.clone()) // Symbol
            .field(54, char::from(self.side).to_string()) // Side
            .field(53, self.quantity.to_string()) // Quantity
            .field(32, self.last_qty.to_string()) // LastQty
            .field(31, self.last_px.to_string()) // LastPx
            .field(75, self.trade_date.clone()) // TradeDate
            .field(
                60,
                self.transact_time.format("%Y%m%d-%H:%M:%S%.3f").to_string(),
            ); // TransactTime

        // Optional fields
        if let Some(trade_id) = &self.trade_id {
            builder = builder.field(1003, trade_id.clone());
        }

        if let Some(secondary_trade_id) = &self.secondary_trade_id {
            builder = builder.field(1040, secondary_trade_id.clone());
        }

        if let Some(firm_trade_id) = &self.firm_trade_id {
            builder = builder.field(1041, firm_trade_id.clone());
        }

        if let Some(trade_report_trans_type) = &self.trade_report_trans_type {
            builder = builder.field(487, i32::from(*trade_report_trans_type).to_string());
        }

        if let Some(trade_report_type) = &self.trade_report_type {
            builder = builder.field(856, i32::from(*trade_report_type).to_string());
        }

        if let Some(trade_request_id) = &self.trade_request_id {
            builder = builder.field(568, trade_request_id.clone());
        }

        if let Some(trd_type) = &self.trd_type {
            builder = builder.field(828, trd_type.to_string());
        }

        if let Some(trade_sub_type) = &self.trade_sub_type {
            builder = builder.field(829, trade_sub_type.to_string());
        }

        if let Some(order_qty) = &self.order_qty {
            builder = builder.field(38, order_qty.to_string());
        }

        if let Some(gross_trade_amt) = &self.gross_trade_amt {
            builder = builder.field(381, gross_trade_amt.to_string());
        }

        if let Some(exec_time) = &self.exec_time {
            builder = builder.field(126, exec_time.format("%Y%m%d-%H:%M:%S%.3f").to_string());
        }

        if let Some(settlement_date) = &self.settlement_date {
            builder = builder.field(64, settlement_date.clone());
        }

        if let Some(multi_leg_reporting_type) = &self.multi_leg_reporting_type {
            builder = builder.field(442, multi_leg_reporting_type.to_string());
        }

        if let Some(previously_reported) = &self.previously_reported {
            builder = builder.field(
                570,
                if *previously_reported { "Y" } else { "N" }.to_string(),
            );
        }

        if let Some(price_type) = &self.price_type {
            builder = builder.field(423, price_type.to_string());
        }

        if let Some(underlying_price) = &self.underlying_price {
            builder = builder.field(810, underlying_price.to_string());
        }

        if let Some(account) = &self.account {
            builder = builder.field(1, account.clone());
        }

        if let Some(clearing_account) = &self.clearing_account {
            builder = builder.field(440, clearing_account.clone());
        }

        if let Some(position_effect) = &self.position_effect {
            builder = builder.field(77, position_effect.to_string());
        }

        if let Some(clearing_business_date) = &self.clearing_business_date {
            builder = builder.field(715, clearing_business_date.clone());
        }

        if let Some(trading_session_id) = &self.trading_session_id {
            builder = builder.field(336, trading_session_id.clone());
        }

        if let Some(trading_session_sub_id) = &self.trading_session_sub_id {
            builder = builder.field(625, trading_session_sub_id.clone());
        }

        if let Some(market_segment_id) = &self.market_segment_id {
            builder = builder.field(1300, market_segment_id.clone());
        }

        if let Some(text) = &self.text {
            builder = builder.field(58, text.clone());
        }

        if let Some(deribit_label) = &self.deribit_label {
            builder = builder.field(100010, deribit_label.clone());
        }

        // Block trade match ID (tag 880)
        if let Some(trd_match_id) = &self.trd_match_id {
            builder = builder.field(880, trd_match_id.clone());
        }

        // Legs group (NoLegs, tag 555)
        if !self.legs.is_empty() {
            builder = builder.field(555, self.legs.len().to_string());
            for leg in &self.legs {
                builder = builder
                    .field(600, leg.leg_symbol.clone()) // LegSymbol
                    .field(687, leg.leg_qty.to_string()) // LegQty
                    .field(566, leg.leg_price.to_string()) // LegPrice
                    .field(624, char::from(leg.leg_side).to_string()); // LegSide
            }
        }

        // Sides group (NoSides, tag 552)
        if !self.sides.is_empty() {
            builder = builder.field(552, self.sides.len().to_string());
            for side in &self.sides {
                builder = builder
                    .field(54, char::from(side.side).to_string()) // Side
                    .field(37, side.order_id.clone()); // OrderId

                if let Some(commission) = side.commission {
                    builder = builder.field(12, commission.to_string());
                }

                if let Some(comm_currency) = &side.comm_currency {
                    builder = builder.field(479, comm_currency.clone());
                }
            }
        }

        Ok(builder.build()?.to_string())
    }
}

impl_json_display!(TradeCaptureReport);
impl_json_debug_pretty!(TradeCaptureReport);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trade_capture_report_creation() {
        let report = TradeCaptureReport::new(
            "TR123".to_string(),
            "BTC-PERPETUAL".to_string(),
            OrderSide::Buy,
            10.0,
            10.0,
            50000.0,
            "20250812".to_string(),
        );

        assert_eq!(report.trade_report_id, "TR123");
        assert_eq!(report.symbol, "BTC-PERPETUAL");
        assert_eq!(report.side, OrderSide::Buy);
        assert_eq!(report.quantity, 10.0);
        assert_eq!(report.last_qty, 10.0);
        assert_eq!(report.last_px, 50000.0);
        assert_eq!(report.trade_date, "20250812");
    }

    #[test]
    fn test_trade_capture_report_new_trade() {
        let report = TradeCaptureReport::new_trade(
            "TR456".to_string(),
            "TRADE456".to_string(),
            "ETH-PERPETUAL".to_string(),
            OrderSide::Sell,
            5.0,
            5.0,
            3200.0,
            "20250812".to_string(),
        );

        assert_eq!(report.trade_id, Some("TRADE456".to_string()));
        assert_eq!(
            report.trade_report_trans_type,
            Some(TradeReportTransType::New)
        );
        assert_eq!(
            report.trade_report_type,
            Some(TradeCaptureReportType::Submit)
        );
        assert_eq!(report.side, OrderSide::Sell);
    }

    #[test]
    fn test_trade_capture_report_cancel_trade() {
        let report = TradeCaptureReport::cancel_trade(
            "TR789".to_string(),
            "TRADE789".to_string(),
            "BTC-PERPETUAL".to_string(),
            OrderSide::Buy,
            10.0,
            "20250812".to_string(),
        );

        assert_eq!(report.trade_id, Some("TRADE789".to_string()));
        assert_eq!(
            report.trade_report_trans_type,
            Some(TradeReportTransType::Cancel)
        );
        assert_eq!(
            report.trade_report_type,
            Some(TradeCaptureReportType::TradeReportCancel)
        );
        assert_eq!(report.last_px, 0.0);
    }

    #[test]
    fn test_trade_capture_report_with_options() {
        let exec_time = Utc::now();

        let report = TradeCaptureReport::new(
            "TR999".to_string(),
            "BTC-PERPETUAL".to_string(),
            OrderSide::Buy,
            20.0,
            20.0,
            51000.0,
            "20250812".to_string(),
        )
        .with_trade_id("TRADE999".to_string())
        .with_trans_type(TradeReportTransType::New)
        .with_report_type(TradeCaptureReportType::Submit)
        .with_trade_request_id("REQ999".to_string())
        .with_order_qty(20.0)
        .with_gross_trade_amount(1020000.0)
        .with_exec_time(exec_time)
        .with_settlement_date("20250815".to_string())
        .with_account("ACC123".to_string())
        .with_position_effect('O')
        .with_trading_session_id("SESSION1".to_string())
        .with_text("Trade execution report".to_string())
        .with_label("test-trade".to_string());

        assert_eq!(report.trade_id, Some("TRADE999".to_string()));
        assert_eq!(
            report.trade_report_trans_type,
            Some(TradeReportTransType::New)
        );
        assert_eq!(
            report.trade_report_type,
            Some(TradeCaptureReportType::Submit)
        );
        assert_eq!(report.trade_request_id, Some("REQ999".to_string()));
        assert_eq!(report.order_qty, Some(20.0));
        assert_eq!(report.gross_trade_amt, Some(1020000.0));
        assert_eq!(report.exec_time, Some(exec_time));
        assert_eq!(report.settlement_date, Some("20250815".to_string()));
        assert_eq!(report.account, Some("ACC123".to_string()));
        assert_eq!(report.position_effect, Some('O'));
        assert_eq!(report.trading_session_id, Some("SESSION1".to_string()));
        assert_eq!(report.text, Some("Trade execution report".to_string()));
        assert_eq!(report.deribit_label, Some("test-trade".to_string()));
    }

    #[test]
    fn test_trade_capture_report_to_fix_message() {
        let report = TradeCaptureReport::new_trade(
            "TR123".to_string(),
            "TRADE123".to_string(),
            "BTC-PERPETUAL".to_string(),
            OrderSide::Buy,
            10.0,
            10.0,
            50000.0,
            "20250812".to_string(),
        )
        .with_label("test-label".to_string());

        let fix_message = report.to_fix_message("SENDER", "TARGET", 1).unwrap();

        // Check that the message contains required fields
        assert!(fix_message.contains("35=AE")); // MsgType
        assert!(fix_message.contains("571=TR123")); // TradeReportID
        assert!(fix_message.contains("55=BTC-PERPETUAL")); // Symbol
        assert!(fix_message.contains("54=1")); // Side=Buy
        assert!(fix_message.contains("53=10")); // Quantity
        assert!(fix_message.contains("32=10")); // LastQty
        assert!(fix_message.contains("31=50000")); // LastPx
        assert!(fix_message.contains("75=20250812")); // TradeDate
        assert!(fix_message.contains("1003=TRADE123")); // TradeID
        assert!(fix_message.contains("487=0")); // TradeReportTransType=New
        assert!(fix_message.contains("856=0")); // TradeReportType=Submit
        assert!(fix_message.contains("100010=test-label")); // Custom label
    }

    #[test]
    fn test_trade_capture_report_type_conversions() {
        assert_eq!(i32::from(TradeCaptureReportType::Submit), 0);
        assert_eq!(i32::from(TradeCaptureReportType::Alleged), 1);
        assert_eq!(i32::from(TradeCaptureReportType::Accept), 2);
        assert_eq!(i32::from(TradeCaptureReportType::TradeReportCancel), 6);

        assert_eq!(
            TradeCaptureReportType::try_from(0).unwrap(),
            TradeCaptureReportType::Submit
        );
        assert_eq!(
            TradeCaptureReportType::try_from(1).unwrap(),
            TradeCaptureReportType::Alleged
        );
        assert_eq!(
            TradeCaptureReportType::try_from(2).unwrap(),
            TradeCaptureReportType::Accept
        );
        assert_eq!(
            TradeCaptureReportType::try_from(6).unwrap(),
            TradeCaptureReportType::TradeReportCancel
        );

        assert!(TradeCaptureReportType::try_from(99).is_err());
    }

    #[test]
    fn test_trade_report_trans_type_conversions() {
        assert_eq!(i32::from(TradeReportTransType::New), 0);
        assert_eq!(i32::from(TradeReportTransType::Cancel), 1);
        assert_eq!(i32::from(TradeReportTransType::Replace), 2);
        assert_eq!(
            i32::from(TradeReportTransType::CancelDueToBackOutOfTrade),
            5
        );

        assert_eq!(
            TradeReportTransType::try_from(0).unwrap(),
            TradeReportTransType::New
        );
        assert_eq!(
            TradeReportTransType::try_from(1).unwrap(),
            TradeReportTransType::Cancel
        );
        assert_eq!(
            TradeReportTransType::try_from(2).unwrap(),
            TradeReportTransType::Replace
        );
        assert_eq!(
            TradeReportTransType::try_from(5).unwrap(),
            TradeReportTransType::CancelDueToBackOutOfTrade
        );

        assert!(TradeReportTransType::try_from(99).is_err());
    }

    #[test]
    fn test_trade_capture_report_leg_creation() {
        let leg =
            TradeCaptureReportLeg::new("BTC-PERPETUAL".to_string(), 5.0, 50000.0, OrderSide::Buy);

        assert_eq!(leg.leg_symbol, "BTC-PERPETUAL");
        assert_eq!(leg.leg_qty, 5.0);
        assert_eq!(leg.leg_price, 50000.0);
        assert_eq!(leg.leg_side, OrderSide::Buy);
    }

    #[test]
    fn test_trade_capture_report_side_creation() {
        let side = TradeCaptureReportSide::new(OrderSide::Buy, "ORDER123".to_string())
            .with_commission(0.001, "BTC".to_string());

        assert_eq!(side.side, OrderSide::Buy);
        assert_eq!(side.order_id, "ORDER123");
        assert_eq!(side.commission, Some(0.001));
        assert_eq!(side.comm_currency, Some("BTC".to_string()));
    }

    #[test]
    fn test_trade_capture_report_with_block_trade() {
        let leg1 = TradeCaptureReportLeg::new(
            "BTC-25DEC25-100000-C".to_string(),
            1.0,
            0.05,
            OrderSide::Buy,
        );
        let leg2 = TradeCaptureReportLeg::new(
            "BTC-25DEC25-110000-C".to_string(),
            1.0,
            0.03,
            OrderSide::Sell,
        );

        let side = TradeCaptureReportSide::new(OrderSide::Buy, "ORDER456".to_string())
            .with_commission(0.0005, "BTC".to_string());

        let report = TradeCaptureReport::new(
            "TR_BLOCK".to_string(),
            "COMBO-SPREAD".to_string(),
            OrderSide::Buy,
            1.0,
            1.0,
            0.02,
            "20250812".to_string(),
        )
        .with_trd_match_id("BLOCK_TRADE_123".to_string())
        .with_legs(vec![leg1, leg2])
        .with_side(side);

        assert_eq!(report.trd_match_id, Some("BLOCK_TRADE_123".to_string()));
        assert_eq!(report.legs.len(), 2);
        assert_eq!(report.sides.len(), 1);
    }

    #[test]
    fn test_trade_capture_report_block_trade_fix_message() {
        let leg =
            TradeCaptureReportLeg::new("ETH-PERPETUAL".to_string(), 2.0, 3500.0, OrderSide::Buy);

        let side = TradeCaptureReportSide::new(OrderSide::Buy, "ORD789".to_string())
            .with_commission(0.01, "ETH".to_string());

        let report = TradeCaptureReport::new_trade(
            "TR_FIX".to_string(),
            "TRADE_FIX".to_string(),
            "ETH-PERPETUAL".to_string(),
            OrderSide::Buy,
            2.0,
            2.0,
            3500.0,
            "20250812".to_string(),
        )
        .with_trd_match_id("BLOCK_456".to_string())
        .with_leg(leg)
        .with_side(side);

        let fix_message = report.to_fix_message("SENDER", "TARGET", 1).unwrap();

        // Check block trade fields
        assert!(fix_message.contains("880=BLOCK_456")); // TrdMatchID
        assert!(fix_message.contains("555=1")); // NoLegs
        assert!(fix_message.contains("600=ETH-PERPETUAL")); // LegSymbol
        assert!(fix_message.contains("687=2")); // LegQty
        assert!(fix_message.contains("566=3500")); // LegPrice
        assert!(fix_message.contains("624=1")); // LegSide=Buy
        assert!(fix_message.contains("552=1")); // NoSides
        assert!(fix_message.contains("37=ORD789")); // OrderId
        assert!(fix_message.contains("12=0.01")); // Commission
        assert!(fix_message.contains("479=ETH")); // CommCurrency
    }
}
