/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 10/8/25
******************************************************************************/

//! RFQ Request FIX Message Implementation

use crate::error::Result as DeribitFixResult;
use crate::message::builder::MessageBuilder;
use crate::message::orders::{OrderSide, TimeInForce};
use crate::model::types::MsgType;
use chrono::{DateTime, Utc};
use deribit_base::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

/// RFQ request type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RfqRequestType {
    /// Automatic
    Automatic,
    /// Manual
    Manual,
}

impl From<RfqRequestType> for i32 {
    fn from(request_type: RfqRequestType) -> Self {
        match request_type {
            RfqRequestType::Automatic => 1,
            RfqRequestType::Manual => 0,
        }
    }
}

impl TryFrom<i32> for RfqRequestType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(RfqRequestType::Automatic),
            0 => Ok(RfqRequestType::Manual),
            _ => Err(format!("Invalid RfqRequestType: {}", value)),
        }
    }
}

/// RFQ request side enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RfqSide {
    /// Buy
    Buy,
    /// Sell
    Sell,
    /// Cross (buy and sell)
    Cross,
    /// Trade
    Trade,
}

impl From<RfqSide> for i32 {
    fn from(side: RfqSide) -> Self {
        match side {
            RfqSide::Buy => 1,
            RfqSide::Sell => 2,
            RfqSide::Cross => 8,
            RfqSide::Trade => 9,
        }
    }
}

impl TryFrom<i32> for RfqSide {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(RfqSide::Buy),
            2 => Ok(RfqSide::Sell),
            8 => Ok(RfqSide::Cross),
            9 => Ok(RfqSide::Trade),
            _ => Err(format!("Invalid RfqSide: {}", value)),
        }
    }
}

/// RFQ request leg for multi-leg RFQ
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct RfqRequestLeg {
    /// Leg symbol
    pub leg_symbol: String,
    /// Leg side
    pub leg_side: OrderSide,
    /// Leg quantity
    pub leg_qty: f64,
    /// Leg settlement type
    pub leg_settlement_type: Option<char>,
    /// Leg settlement date
    pub leg_settlement_date: Option<String>,
    /// Leg price
    pub leg_price: Option<f64>,
}

impl RfqRequestLeg {
    /// Create a new RFQ request leg
    pub fn new(leg_symbol: String, leg_side: OrderSide, leg_qty: f64) -> Self {
        Self {
            leg_symbol,
            leg_side,
            leg_qty,
            leg_settlement_type: None,
            leg_settlement_date: None,
            leg_price: None,
        }
    }

    /// Set leg settlement type
    pub fn with_settlement_type(mut self, settlement_type: char) -> Self {
        self.leg_settlement_type = Some(settlement_type);
        self
    }

    /// Set leg settlement date
    pub fn with_settlement_date(mut self, settlement_date: String) -> Self {
        self.leg_settlement_date = Some(settlement_date);
        self
    }

    /// Set leg price
    pub fn with_price(mut self, price: f64) -> Self {
        self.leg_price = Some(price);
        self
    }
}

/// RFQ Request message (MsgType = 'AH')
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct RfqRequest {
    /// RFQ request ID
    pub rfq_req_id: String,
    /// No related symbols
    pub no_related_sym: i32,
    /// RFQ request type
    pub rfq_request_type: Option<RfqRequestType>,
    /// Subscription request type
    pub subscription_request_type: Option<char>,
    /// Instrument symbol
    pub symbol: String,
    /// Side
    pub side: Option<RfqSide>,
    /// Order quantity
    pub order_qty: f64,
    /// Valid until time
    pub valid_until_time: Option<DateTime<Utc>>,
    /// Time in force
    pub time_in_force: Option<TimeInForce>,
    /// Clearing business date
    pub clearing_business_date: Option<String>,
    /// Settlement type
    pub settlement_type: Option<char>,
    /// Settlement date
    pub settlement_date: Option<String>,
    /// Currency
    pub currency: Option<String>,
    /// Parties
    pub parties: Option<String>,
    /// Account
    pub account: Option<String>,
    /// Clearing account
    pub clearing_account: Option<String>,
    /// Account type
    pub account_type: Option<i32>,
    /// Position effect
    pub position_effect: Option<char>,
    /// Opening flag
    pub opening_flag: Option<String>,
    /// No legs (for multi-leg RFQ)
    pub no_legs: Option<i32>,
    /// RFQ request legs
    pub rfq_request_legs: Vec<RfqRequestLeg>,
    /// Trading session ID
    pub trading_session_id: Option<String>,
    /// Trading session sub ID
    pub trading_session_sub_id: Option<String>,
    /// Text
    pub text: Option<String>,
    /// Custom label
    pub deribit_label: Option<String>,
}

impl RfqRequest {
    /// Create a new RFQ request
    pub fn new(rfq_req_id: String, symbol: String, order_qty: f64) -> Self {
        Self {
            rfq_req_id,
            no_related_sym: 1,
            rfq_request_type: None,
            subscription_request_type: None,
            symbol,
            side: None,
            order_qty,
            valid_until_time: None,
            time_in_force: None,
            clearing_business_date: None,
            settlement_type: None,
            settlement_date: None,
            currency: None,
            parties: None,
            account: None,
            clearing_account: None,
            account_type: None,
            position_effect: None,
            opening_flag: None,
            no_legs: None,
            rfq_request_legs: Vec::new(),
            trading_session_id: None,
            trading_session_sub_id: None,
            text: None,
            deribit_label: None,
        }
    }

    /// Create an RFQ request for buying
    pub fn buy(rfq_req_id: String, symbol: String, order_qty: f64) -> Self {
        let mut rfq = Self::new(rfq_req_id, symbol, order_qty);
        rfq.side = Some(RfqSide::Buy);
        rfq
    }

    /// Create an RFQ request for selling
    pub fn sell(rfq_req_id: String, symbol: String, order_qty: f64) -> Self {
        let mut rfq = Self::new(rfq_req_id, symbol, order_qty);
        rfq.side = Some(RfqSide::Sell);
        rfq
    }

    /// Create a multi-leg RFQ request
    pub fn multi_leg(rfq_req_id: String, legs: Vec<RfqRequestLeg>) -> Self {
        let no_legs = legs.len() as i32;

        Self {
            rfq_req_id,
            no_related_sym: 1,
            rfq_request_type: None,
            subscription_request_type: None,
            symbol: "MULTI_LEG".to_string(), // Placeholder for multi-leg
            side: None,
            order_qty: 0.0, // Will be calculated from legs
            valid_until_time: None,
            time_in_force: None,
            clearing_business_date: None,
            settlement_type: None,
            settlement_date: None,
            currency: None,
            parties: None,
            account: None,
            clearing_account: None,
            account_type: None,
            position_effect: None,
            opening_flag: None,
            no_legs: Some(no_legs),
            rfq_request_legs: legs,
            trading_session_id: None,
            trading_session_sub_id: None,
            text: None,
            deribit_label: None,
        }
    }

    /// Set RFQ request type
    pub fn with_rfq_request_type(mut self, rfq_request_type: RfqRequestType) -> Self {
        self.rfq_request_type = Some(rfq_request_type);
        self
    }

    /// Set side
    pub fn with_side(mut self, side: RfqSide) -> Self {
        self.side = Some(side);
        self
    }

    /// Set valid until time
    pub fn with_valid_until(mut self, valid_until: DateTime<Utc>) -> Self {
        self.valid_until_time = Some(valid_until);
        self
    }

    /// Set time in force
    pub fn with_time_in_force(mut self, tif: TimeInForce) -> Self {
        self.time_in_force = Some(tif);
        self
    }

    /// Set settlement type
    pub fn with_settlement_type(mut self, settlement_type: char) -> Self {
        self.settlement_type = Some(settlement_type);
        self
    }

    /// Set settlement date
    pub fn with_settlement_date(mut self, settlement_date: String) -> Self {
        self.settlement_date = Some(settlement_date);
        self
    }

    /// Set currency
    pub fn with_currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
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

    /// Add RFQ request leg
    pub fn add_leg(mut self, leg: RfqRequestLeg) -> Self {
        self.rfq_request_legs.push(leg);
        self.no_legs = Some(self.rfq_request_legs.len() as i32);
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
            .msg_type(MsgType::RfqRequest)
            .sender_comp_id(sender_comp_id.to_string())
            .target_comp_id(target_comp_id.to_string())
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now());

        // Required fields
        builder = builder
            .field(644, self.rfq_req_id.clone()) // RFQReqID
            .field(146, self.no_related_sym.to_string()) // NoRelatedSym
            .field(55, self.symbol.clone()) // Symbol
            .field(38, self.order_qty.to_string()); // OrderQty

        // Optional fields
        if let Some(rfq_request_type) = &self.rfq_request_type {
            builder = builder.field(303, i32::from(*rfq_request_type).to_string());
        }

        if let Some(subscription_request_type) = &self.subscription_request_type {
            builder = builder.field(263, subscription_request_type.to_string());
        }

        if let Some(side) = &self.side {
            builder = builder.field(54, i32::from(*side).to_string());
        }

        if let Some(valid_until_time) = &self.valid_until_time {
            builder = builder.field(
                62,
                valid_until_time.format("%Y%m%d-%H:%M:%S%.3f").to_string(),
            );
        }

        if let Some(time_in_force) = &self.time_in_force {
            builder = builder.field(59, char::from(*time_in_force).to_string());
        }

        if let Some(settlement_type) = &self.settlement_type {
            builder = builder.field(63, settlement_type.to_string());
        }

        if let Some(settlement_date) = &self.settlement_date {
            builder = builder.field(64, settlement_date.clone());
        }

        if let Some(currency) = &self.currency {
            builder = builder.field(15, currency.clone());
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

        if let Some(no_legs) = &self.no_legs {
            builder = builder.field(555, no_legs.to_string());
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

        // Add RFQ request legs (simplified - in real implementation would need repeating groups)
        for (i, leg) in self.rfq_request_legs.iter().enumerate() {
            let base_tag = 5000 + (i * 100); // Custom tag range for RFQ legs

            builder = builder
                .field(base_tag as u32, leg.leg_symbol.clone()) // LegSymbol
                .field((base_tag + 1) as u32, char::from(leg.leg_side).to_string()) // LegSide
                .field((base_tag + 2) as u32, leg.leg_qty.to_string()); // LegQty

            if let Some(leg_settlement_type) = &leg.leg_settlement_type {
                builder = builder.field((base_tag + 3) as u32, leg_settlement_type.to_string());
            }

            if let Some(leg_settlement_date) = &leg.leg_settlement_date {
                builder = builder.field((base_tag + 4) as u32, leg_settlement_date.clone());
            }

            if let Some(leg_price) = &leg.leg_price {
                builder = builder.field((base_tag + 5) as u32, leg_price.to_string());
            }
        }

        Ok(builder.build()?.to_string())
    }
}

impl_json_display!(RfqRequest, RfqRequestLeg);
impl_json_debug_pretty!(RfqRequest, RfqRequestLeg);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rfq_request_leg_creation() {
        let leg = RfqRequestLeg::new("BTC-PERPETUAL".to_string(), OrderSide::Buy, 10.0);

        assert_eq!(leg.leg_symbol, "BTC-PERPETUAL");
        assert_eq!(leg.leg_side, OrderSide::Buy);
        assert_eq!(leg.leg_qty, 10.0);
        assert!(leg.leg_settlement_type.is_none());
        assert!(leg.leg_price.is_none());
    }

    #[test]
    fn test_rfq_request_leg_with_options() {
        let leg = RfqRequestLeg::new("ETH-PERPETUAL".to_string(), OrderSide::Sell, 5.0)
            .with_settlement_type('0')
            .with_settlement_date("20250815".to_string())
            .with_price(3200.0);

        assert_eq!(leg.leg_settlement_type, Some('0'));
        assert_eq!(leg.leg_settlement_date, Some("20250815".to_string()));
        assert_eq!(leg.leg_price, Some(3200.0));
    }

    #[test]
    fn test_rfq_request_creation() {
        let rfq = RfqRequest::new("RFQ123".to_string(), "BTC-PERPETUAL".to_string(), 10.0);

        assert_eq!(rfq.rfq_req_id, "RFQ123");
        assert_eq!(rfq.symbol, "BTC-PERPETUAL");
        assert_eq!(rfq.order_qty, 10.0);
        assert_eq!(rfq.no_related_sym, 1);
        assert!(rfq.side.is_none());
    }

    #[test]
    fn test_rfq_request_buy() {
        let rfq = RfqRequest::buy("RFQ456".to_string(), "ETH-PERPETUAL".to_string(), 20.0);

        assert_eq!(rfq.side, Some(RfqSide::Buy));
        assert_eq!(rfq.order_qty, 20.0);
    }

    #[test]
    fn test_rfq_request_sell() {
        let rfq = RfqRequest::sell("RFQ789".to_string(), "BTC-PERPETUAL".to_string(), 15.0);

        assert_eq!(rfq.side, Some(RfqSide::Sell));
        assert_eq!(rfq.order_qty, 15.0);
    }

    #[test]
    fn test_rfq_request_multi_leg() {
        let leg1 = RfqRequestLeg::new("BTC-PERPETUAL".to_string(), OrderSide::Buy, 10.0);
        let leg2 = RfqRequestLeg::new("ETH-PERPETUAL".to_string(), OrderSide::Sell, 5.0);

        let rfq = RfqRequest::multi_leg("RFQ999".to_string(), vec![leg1, leg2]);

        assert_eq!(rfq.no_legs, Some(2));
        assert_eq!(rfq.rfq_request_legs.len(), 2);
        assert_eq!(rfq.symbol, "MULTI_LEG");
    }

    #[test]
    fn test_rfq_request_add_leg() {
        let leg = RfqRequestLeg::new("BTC-PERPETUAL".to_string(), OrderSide::Buy, 10.0);

        let rfq = RfqRequest::new("RFQ111".to_string(), "BASE".to_string(), 0.0).add_leg(leg);

        assert_eq!(rfq.no_legs, Some(1));
        assert_eq!(rfq.rfq_request_legs.len(), 1);
    }

    #[test]
    fn test_rfq_request_with_options() {
        let valid_until = Utc::now() + chrono::Duration::hours(1);
        let rfq = RfqRequest::buy("RFQ222".to_string(), "BTC-PERPETUAL".to_string(), 25.0)
            .with_rfq_request_type(RfqRequestType::Manual)
            .with_valid_until(valid_until)
            .with_time_in_force(TimeInForce::GoodTillCancelled)
            .with_settlement_type('0')
            .with_settlement_date("20250815".to_string())
            .with_currency("USD".to_string())
            .with_account("ACC123".to_string())
            .with_position_effect('O')
            .with_trading_session_id("SESSION1".to_string())
            .with_text("Large block RFQ".to_string())
            .with_label("test-rfq".to_string());

        assert_eq!(rfq.rfq_request_type, Some(RfqRequestType::Manual));
        assert_eq!(rfq.valid_until_time, Some(valid_until));
        assert_eq!(rfq.time_in_force, Some(TimeInForce::GoodTillCancelled));
        assert_eq!(rfq.settlement_type, Some('0'));
        assert_eq!(rfq.settlement_date, Some("20250815".to_string()));
        assert_eq!(rfq.currency, Some("USD".to_string()));
        assert_eq!(rfq.account, Some("ACC123".to_string()));
        assert_eq!(rfq.position_effect, Some('O'));
        assert_eq!(rfq.trading_session_id, Some("SESSION1".to_string()));
        assert_eq!(rfq.text, Some("Large block RFQ".to_string()));
        assert_eq!(rfq.deribit_label, Some("test-rfq".to_string()));
    }

    #[test]
    fn test_rfq_request_to_fix_message() {
        let rfq = RfqRequest::buy("RFQ123".to_string(), "BTC-PERPETUAL".to_string(), 10.0)
            .with_rfq_request_type(RfqRequestType::Manual)
            .with_label("test-label".to_string());

        let fix_message = rfq.to_fix_message("SENDER", "TARGET", 1).unwrap();

        // Check that the message contains required fields
        assert!(fix_message.contains("35=AH")); // MsgType
        assert!(fix_message.contains("644=RFQ123")); // RFQReqID
        assert!(fix_message.contains("146=1")); // NoRelatedSym
        assert!(fix_message.contains("55=BTC-PERPETUAL")); // Symbol
        assert!(fix_message.contains("38=10")); // OrderQty
        assert!(fix_message.contains("54=1")); // Side=Buy
        assert!(fix_message.contains("303=0")); // RFQRequestType=Manual
        assert!(fix_message.contains("100010=test-label")); // Custom label
    }

    #[test]
    fn test_rfq_request_multi_leg_to_fix_message() {
        let leg = RfqRequestLeg::new("BTC-PERPETUAL".to_string(), OrderSide::Buy, 5.0)
            .with_price(50000.0);

        let rfq = RfqRequest::multi_leg("RFQ456".to_string(), vec![leg]);

        let fix_message = rfq.to_fix_message("SENDER", "TARGET", 2).unwrap();

        // Check required fields
        assert!(fix_message.contains("35=AH")); // MsgType
        assert!(fix_message.contains("644=RFQ456")); // RFQReqID
        assert!(fix_message.contains("555=1")); // NoLegs

        // Check leg fields (simplified check)
        assert!(fix_message.contains("5000=BTC-PERPETUAL")); // LegSymbol
        assert!(fix_message.contains("5001=1")); // LegSide=Buy
        assert!(fix_message.contains("5002=5")); // LegQty
        assert!(fix_message.contains("5005=50000")); // LegPrice
    }

    #[test]
    fn test_rfq_request_type_conversions() {
        assert_eq!(i32::from(RfqRequestType::Automatic), 1);
        assert_eq!(i32::from(RfqRequestType::Manual), 0);

        assert_eq!(
            RfqRequestType::try_from(1).unwrap(),
            RfqRequestType::Automatic
        );
        assert_eq!(RfqRequestType::try_from(0).unwrap(), RfqRequestType::Manual);

        assert!(RfqRequestType::try_from(99).is_err());
    }

    #[test]
    fn test_rfq_side_conversions() {
        assert_eq!(i32::from(RfqSide::Buy), 1);
        assert_eq!(i32::from(RfqSide::Sell), 2);
        assert_eq!(i32::from(RfqSide::Cross), 8);
        assert_eq!(i32::from(RfqSide::Trade), 9);

        assert_eq!(RfqSide::try_from(1).unwrap(), RfqSide::Buy);
        assert_eq!(RfqSide::try_from(2).unwrap(), RfqSide::Sell);
        assert_eq!(RfqSide::try_from(8).unwrap(), RfqSide::Cross);
        assert_eq!(RfqSide::try_from(9).unwrap(), RfqSide::Trade);

        assert!(RfqSide::try_from(99).is_err());
    }
}
