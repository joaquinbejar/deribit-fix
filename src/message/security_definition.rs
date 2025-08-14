/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 14/8/25
******************************************************************************/

//! FIX Security Definition messages implementation
//!
//! This module provides functionality for creating and parsing FIX security
//! definition messages used in communication with Deribit, including:
//! - SecurityDefinitionRequest (MsgType = "c")
//! - SecurityDefinition (MsgType = "d")

use crate::error::{DeribitFixError, Result as DeribitFixResult};
use crate::message::MessageBuilder;
use crate::model::message::FixMessage;
use crate::model::types::MsgType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Security Definition Request Type enumeration (tag 856)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityDefinitionRequestType {
    /// Request Security identity and specifications (0)
    RequestSecurityIdentityAndSpecs,
    /// Request Security identity for the specifications provided (1)
    RequestSecurityIdentityForSpecs,
    /// Request List Security Types (2)
    RequestListSecurityTypes,
    /// Request List Securities (3)
    RequestListSecurities,
}

impl From<SecurityDefinitionRequestType> for i32 {
    fn from(request_type: SecurityDefinitionRequestType) -> Self {
        match request_type {
            SecurityDefinitionRequestType::RequestSecurityIdentityAndSpecs => 0,
            SecurityDefinitionRequestType::RequestSecurityIdentityForSpecs => 1,
            SecurityDefinitionRequestType::RequestListSecurityTypes => 2,
            SecurityDefinitionRequestType::RequestListSecurities => 3,
        }
    }
}

impl TryFrom<i32> for SecurityDefinitionRequestType {
    type Error = DeribitFixError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SecurityDefinitionRequestType::RequestSecurityIdentityAndSpecs),
            1 => Ok(SecurityDefinitionRequestType::RequestSecurityIdentityForSpecs),
            2 => Ok(SecurityDefinitionRequestType::RequestListSecurityTypes),
            3 => Ok(SecurityDefinitionRequestType::RequestListSecurities),
            _ => Err(DeribitFixError::MessageParsing(format!(
                "Invalid SecurityDefinitionRequestType: {}",
                value
            ))),
        }
    }
}

/// Security Definition Request message (MsgType = "c")
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecurityDefinitionRequest {
    /// Security Request ID (tag 320)
    pub security_req_id: String,
    /// Security Definition Request Type (tag 856)
    pub request_type: SecurityDefinitionRequestType,
    /// Symbol (tag 55) - optional
    pub symbol: Option<String>,
    /// Security Type (tag 167) - optional
    pub security_type: Option<String>,
    /// Currency (tag 15) - optional
    pub currency: Option<String>,
    /// Text (tag 58) - optional
    pub text: Option<String>,
    /// Subscription Request Type (tag 263) - optional
    pub subscription_request_type: Option<i32>,
}

impl SecurityDefinitionRequest {
    /// Create a new Security Definition Request
    pub fn new(security_req_id: String, request_type: SecurityDefinitionRequestType) -> Self {
        Self {
            security_req_id,
            request_type,
            symbol: None,
            security_type: None,
            currency: None,
            text: None,
            subscription_request_type: None,
        }
    }

    /// Create a request for security identity and specifications
    pub fn request_identity_and_specs(security_req_id: String) -> Self {
        Self::new(
            security_req_id,
            SecurityDefinitionRequestType::RequestSecurityIdentityAndSpecs,
        )
    }

    /// Create a request for listing securities
    pub fn request_list_securities(security_req_id: String) -> Self {
        Self::new(
            security_req_id,
            SecurityDefinitionRequestType::RequestListSecurities,
        )
    }

    /// Add symbol filter
    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    /// Add security type filter
    pub fn with_security_type(mut self, security_type: String) -> Self {
        self.security_type = Some(security_type);
        self
    }

    /// Add currency filter
    pub fn with_currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }

    /// Add text description
    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    /// Add subscription request type
    pub fn with_subscription_request_type(mut self, subscription_type: i32) -> Self {
        self.subscription_request_type = Some(subscription_type);
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
            .msg_type(MsgType::SecurityDefinitionRequest)
            .sender_comp_id(sender_comp_id)
            .target_comp_id(target_comp_id)
            .msg_seq_num(msg_seq_num)
            .field(320, self.security_req_id.clone()) // SecurityReqID
            .field(856, i32::from(self.request_type).to_string()); // SecurityDefinitionRequestType

        // Add optional fields
        if let Some(ref symbol) = self.symbol {
            builder = builder.field(55, symbol.clone()); // Symbol
        }

        if let Some(ref security_type) = self.security_type {
            builder = builder.field(167, security_type.clone()); // SecurityType
        }

        if let Some(ref currency) = self.currency {
            builder = builder.field(15, currency.clone()); // Currency
        }

        if let Some(ref text) = self.text {
            builder = builder.field(58, text.clone()); // Text
        }

        if let Some(subscription_type) = self.subscription_request_type {
            builder = builder.field(263, subscription_type.to_string()); // SubscriptionRequestType
        }

        builder.build()
    }
}

/// Security Definition message (MsgType = "d")
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityDefinition {
    /// Security Request ID (tag 320)
    pub security_req_id: String,
    /// Security Response ID (tag 322)
    pub security_response_id: String,
    /// Symbol (tag 55)
    pub symbol: String,
    /// Security Type (tag 167) - optional
    pub security_type: Option<String>,
    /// Currency (tag 15) - optional
    pub currency: Option<String>,
    /// Security Description (tag 107) - optional
    pub security_desc: Option<String>,
    /// Strike Price (tag 202) - optional for options
    pub strike_price: Option<f64>,
    /// Strike Currency (tag 947) - optional for options
    pub strike_currency: Option<String>,
    /// Put or Call (tag 201) - optional for options (0=Put, 1=Call)
    pub put_or_call: Option<i32>,
    /// Contract Multiplier (tag 231) - optional
    pub contract_multiplier: Option<f64>,
    /// Maturity Date (tag 541) - optional
    pub maturity_date: Option<String>,
    /// Issue Date (tag 225) - optional
    pub issue_date: Option<String>,
    /// Minimum Trade Volume (tag 562) - optional
    pub min_trade_vol: Option<f64>,
    /// Security Definition Response Type (tag 1570) - optional
    pub security_def_response_type: Option<i32>,
    /// Last update time
    pub last_update_time: Option<DateTime<Utc>>,
}

impl SecurityDefinition {
    /// Create a new Security Definition
    pub fn new(security_req_id: String, security_response_id: String, symbol: String) -> Self {
        Self {
            security_req_id,
            security_response_id,
            symbol,
            security_type: None,
            currency: None,
            security_desc: None,
            strike_price: None,
            strike_currency: None,
            put_or_call: None,
            contract_multiplier: None,
            maturity_date: None,
            issue_date: None,
            min_trade_vol: None,
            security_def_response_type: None,
            last_update_time: Some(Utc::now()),
        }
    }

    /// Parse from FIX message
    pub fn from_fix_message(message: &FixMessage) -> DeribitFixResult<Self> {
        let security_req_id = message
            .get_field(320)
            .ok_or_else(|| {
                DeribitFixError::MessageParsing("Missing SecurityReqID (320)".to_string())
            })?
            .clone();

        let security_response_id = message
            .get_field(322)
            .ok_or_else(|| {
                DeribitFixError::MessageParsing("Missing SecurityResponseID (322)".to_string())
            })?
            .clone();

        let symbol = message
            .get_field(55)
            .ok_or_else(|| DeribitFixError::MessageParsing("Missing Symbol (55)".to_string()))?
            .clone();

        let security_type = message.get_field(167).cloned();
        let currency = message.get_field(15).cloned();
        let security_desc = message.get_field(107).cloned();

        let strike_price = message.get_field(202).and_then(|s| s.parse::<f64>().ok());

        let strike_currency = message.get_field(947).cloned();

        let put_or_call = message.get_field(201).and_then(|s| s.parse::<i32>().ok());

        let contract_multiplier = message.get_field(231).and_then(|s| s.parse::<f64>().ok());

        let maturity_date = message.get_field(541).cloned();
        let issue_date = message.get_field(225).cloned();

        let min_trade_vol = message.get_field(562).and_then(|s| s.parse::<f64>().ok());

        let security_def_response_type =
            message.get_field(1570).and_then(|s| s.parse::<i32>().ok());

        Ok(Self {
            security_req_id,
            security_response_id,
            symbol,
            security_type,
            currency,
            security_desc,
            strike_price,
            strike_currency,
            put_or_call,
            contract_multiplier,
            maturity_date,
            issue_date,
            min_trade_vol,
            security_def_response_type,
            last_update_time: Some(Utc::now()),
        })
    }

    /// Convert to FIX message
    pub fn to_fix_message(
        &self,
        sender_comp_id: String,
        target_comp_id: String,
        msg_seq_num: u32,
    ) -> DeribitFixResult<FixMessage> {
        let mut builder = MessageBuilder::new()
            .msg_type(MsgType::SecurityDefinition)
            .sender_comp_id(sender_comp_id)
            .target_comp_id(target_comp_id)
            .msg_seq_num(msg_seq_num)
            .field(320, self.security_req_id.clone()) // SecurityReqID
            .field(322, self.security_response_id.clone()) // SecurityResponseID
            .field(55, self.symbol.clone()); // Symbol

        // Add optional fields
        if let Some(ref security_type) = self.security_type {
            builder = builder.field(167, security_type.clone()); // SecurityType
        }

        if let Some(ref currency) = self.currency {
            builder = builder.field(15, currency.clone()); // Currency
        }

        if let Some(ref security_desc) = self.security_desc {
            builder = builder.field(107, security_desc.clone()); // SecurityDesc
        }

        if let Some(strike_price) = self.strike_price {
            builder = builder.field(202, strike_price.to_string()); // StrikePrice
        }

        if let Some(ref strike_currency) = self.strike_currency {
            builder = builder.field(947, strike_currency.clone()); // StrikeCurrency
        }

        if let Some(put_or_call) = self.put_or_call {
            builder = builder.field(201, put_or_call.to_string()); // PutOrCall
        }

        if let Some(contract_multiplier) = self.contract_multiplier {
            builder = builder.field(231, contract_multiplier.to_string()); // ContractMultiplier
        }

        if let Some(ref maturity_date) = self.maturity_date {
            builder = builder.field(541, maturity_date.clone()); // MaturityDate
        }

        if let Some(ref issue_date) = self.issue_date {
            builder = builder.field(225, issue_date.clone()); // IssueDate
        }

        if let Some(min_trade_vol) = self.min_trade_vol {
            builder = builder.field(562, min_trade_vol.to_string()); // MinTradeVol
        }

        if let Some(response_type) = self.security_def_response_type {
            builder = builder.field(1570, response_type.to_string()); // SecurityDefinitionResponseType
        }

        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_definition_request_type_conversion() {
        assert_eq!(
            i32::from(SecurityDefinitionRequestType::RequestSecurityIdentityAndSpecs),
            0
        );
        assert_eq!(
            i32::from(SecurityDefinitionRequestType::RequestSecurityIdentityForSpecs),
            1
        );

        assert_eq!(
            SecurityDefinitionRequestType::try_from(0).unwrap(),
            SecurityDefinitionRequestType::RequestSecurityIdentityAndSpecs
        );
        assert_eq!(
            SecurityDefinitionRequestType::try_from(1).unwrap(),
            SecurityDefinitionRequestType::RequestSecurityIdentityForSpecs
        );

        assert!(SecurityDefinitionRequestType::try_from(99).is_err());
    }

    #[test]
    fn test_security_definition_request_creation() {
        let request =
            SecurityDefinitionRequest::request_identity_and_specs("SECDEF_123".to_string());
        assert_eq!(request.security_req_id, "SECDEF_123");
        assert_eq!(
            request.request_type,
            SecurityDefinitionRequestType::RequestSecurityIdentityAndSpecs
        );
    }

    #[test]
    fn test_security_definition_request_with_filters() {
        let request = SecurityDefinitionRequest::request_list_securities("SECDEF_456".to_string())
            .with_symbol("BTC-PERPETUAL".to_string())
            .with_currency("USD".to_string())
            .with_security_type("FUT".to_string());

        assert_eq!(request.security_req_id, "SECDEF_456");
        assert_eq!(request.symbol, Some("BTC-PERPETUAL".to_string()));
        assert_eq!(request.currency, Some("USD".to_string()));
        assert_eq!(request.security_type, Some("FUT".to_string()));
    }

    #[test]
    fn test_security_definition_request_to_fix_message() {
        let request =
            SecurityDefinitionRequest::request_identity_and_specs("SECDEF_789".to_string())
                .with_symbol("ETH-PERPETUAL".to_string());

        let fix_message = request
            .to_fix_message("SENDER".to_string(), "TARGET".to_string(), 1)
            .unwrap();

        assert_eq!(fix_message.get_field(35), Some(&"c".to_string())); // MsgType
        assert_eq!(fix_message.get_field(320), Some(&"SECDEF_789".to_string())); // SecurityReqID
        assert_eq!(fix_message.get_field(856), Some(&"0".to_string())); // SecurityDefinitionRequestType
        assert_eq!(
            fix_message.get_field(55),
            Some(&"ETH-PERPETUAL".to_string())
        ); // Symbol
    }

    #[test]
    fn test_security_definition_creation() {
        let definition = SecurityDefinition::new(
            "SECDEF_123".to_string(),
            "RESP_456".to_string(),
            "BTC-PERPETUAL".to_string(),
        );

        assert_eq!(definition.security_req_id, "SECDEF_123");
        assert_eq!(definition.security_response_id, "RESP_456");
        assert_eq!(definition.symbol, "BTC-PERPETUAL");
        assert!(definition.last_update_time.is_some());
    }

    #[test]
    fn test_security_definition_from_fix_message() {
        let mut fix_message = FixMessage::new();
        fix_message.set_field(320, "SECDEF_123".to_string());
        fix_message.set_field(322, "RESP_456".to_string());
        fix_message.set_field(55, "BTC-PERPETUAL".to_string());
        fix_message.set_field(167, "FUT".to_string());
        fix_message.set_field(15, "USD".to_string());
        fix_message.set_field(107, "Bitcoin Perpetual Future".to_string());

        let definition = SecurityDefinition::from_fix_message(&fix_message).unwrap();

        assert_eq!(definition.security_req_id, "SECDEF_123");
        assert_eq!(definition.security_response_id, "RESP_456");
        assert_eq!(definition.symbol, "BTC-PERPETUAL");
        assert_eq!(definition.security_type, Some("FUT".to_string()));
        assert_eq!(definition.currency, Some("USD".to_string()));
        assert_eq!(
            definition.security_desc,
            Some("Bitcoin Perpetual Future".to_string())
        );
    }

    #[test]
    fn test_security_definition_to_fix_message() {
        let definition = SecurityDefinition::new(
            "SECDEF_123".to_string(),
            "RESP_456".to_string(),
            "BTC-PERPETUAL".to_string(),
        );

        let fix_message = definition
            .to_fix_message("SENDER".to_string(), "TARGET".to_string(), 1)
            .unwrap();

        assert_eq!(fix_message.get_field(35), Some(&"d".to_string())); // MsgType
        assert_eq!(fix_message.get_field(320), Some(&"SECDEF_123".to_string())); // SecurityReqID
        assert_eq!(fix_message.get_field(322), Some(&"RESP_456".to_string())); // SecurityResponseID
        assert_eq!(
            fix_message.get_field(55),
            Some(&"BTC-PERPETUAL".to_string())
        ); // Symbol
    }
}
