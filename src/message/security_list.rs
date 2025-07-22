/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 22/7/25
******************************************************************************/

//! Security List Request and Security List messages for FIX protocol
//!
//! This module implements the FIX messages for requesting and receiving
//! security/instrument information from Deribit according to the official
//! FIX API specification.

use crate::error::Result as DeribitFixResult;
use crate::message::MessageBuilder;
use crate::model::types::MsgType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Security List Request Type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityListRequestType {
    /// Snapshot - return current list of instruments
    Snapshot = 0,
    /// Snapshot + Updates - return current list and subscribe to updates
    SnapshotAndUpdates = 4,
}

impl From<SecurityListRequestType> for i32 {
    fn from(request_type: SecurityListRequestType) -> Self {
        request_type as i32
    }
}

impl TryFrom<i32> for SecurityListRequestType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SecurityListRequestType::Snapshot),
            4 => Ok(SecurityListRequestType::SnapshotAndUpdates),
            _ => Err(format!("Invalid SecurityListRequestType: {value}")),
        }
    }
}

/// Subscription Request Type for security list updates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubscriptionRequestType {
    /// Snapshot only
    Snapshot = 0,
    /// Snapshot + Updates (Subscribe)
    SnapshotPlusUpdates = 1,
    /// Disable previous Snapshot + Update Request (Unsubscribe)
    Unsubscribe = 2,
}

impl From<SubscriptionRequestType> for i32 {
    fn from(request_type: SubscriptionRequestType) -> Self {
        request_type as i32
    }
}

impl TryFrom<i32> for SubscriptionRequestType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SubscriptionRequestType::Snapshot),
            1 => Ok(SubscriptionRequestType::SnapshotPlusUpdates),
            2 => Ok(SubscriptionRequestType::Unsubscribe),
            _ => Err(format!("Invalid SubscriptionRequestType: {value}")),
        }
    }
}

/// Security Type enumeration for FIX protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityType {
    /// Currency exchange spot
    FxSpot,
    /// Futures
    Future,
    /// Options
    Option,
    /// Future combo
    FutureCombo,
    /// Option combo
    OptionCombo,
    /// Indexes
    Index,
}

impl SecurityType {
    /// Convert to FIX string representation
    pub fn as_fix_str(&self) -> &'static str {
        match self {
            SecurityType::FxSpot => "FXSPOT",
            SecurityType::Future => "FUT",
            SecurityType::Option => "OPT",
            SecurityType::FutureCombo => "FUTCO",
            SecurityType::OptionCombo => "OPTCO",
            SecurityType::Index => "INDEX",
        }
    }

    /// Parse from FIX string representation
    pub fn from_fix_str(s: &str) -> Result<Self, String> {
        match s {
            "FXSPOT" => Ok(SecurityType::FxSpot),
            "FUT" => Ok(SecurityType::Future),
            "OPT" => Ok(SecurityType::Option),
            "FUTCO" => Ok(SecurityType::FutureCombo),
            "OPTCO" => Ok(SecurityType::OptionCombo),
            "INDEX" => Ok(SecurityType::Index),
            _ => Err(format!("Invalid SecurityType: {s}")),
        }
    }
}

/// Security List Request message (MsgType = x)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityListRequest {
    /// User-generated ID for this request (Tag 320)
    pub security_req_id: String,
    /// Security List Request Type (Tag 559)
    pub security_list_request_type: SecurityListRequestType,
    /// Subscription Request Type (Tag 263) - Optional
    pub subscription_request_type: Option<SubscriptionRequestType>,
    /// Display Multicast Instrument ID (Tag 9013) - Custom tag
    pub display_multicast_instrument_id: Option<bool>,
    /// Display Increment Steps (Tag 9018) - Custom tag
    pub display_increment_steps: Option<bool>,
    /// Currency filter (Tag 15) - Optional
    pub currency: Option<String>,
    /// Secondary Currency filter (Tag 5544) - Optional
    pub secondary_currency: Option<String>,
    /// Security Type filter (Tag 167) - Optional
    pub security_type: Option<SecurityType>,
}

impl SecurityListRequest {
    /// Create a new Security List Request
    pub fn new(security_req_id: String, request_type: SecurityListRequestType) -> Self {
        Self {
            security_req_id,
            security_list_request_type: request_type,
            subscription_request_type: None,
            display_multicast_instrument_id: None,
            display_increment_steps: None,
            currency: None,
            secondary_currency: None,
            security_type: None,
        }
    }

    /// Create a snapshot request for all instruments
    pub fn snapshot(security_req_id: String) -> Self {
        Self::new(security_req_id, SecurityListRequestType::Snapshot)
    }

    /// Create a subscription request for instrument updates
    pub fn subscription(security_req_id: String) -> Self {
        let mut request = Self::new(security_req_id, SecurityListRequestType::SnapshotAndUpdates);
        request.subscription_request_type = Some(SubscriptionRequestType::SnapshotPlusUpdates);
        request
    }

    /// Set currency filter
    pub fn with_currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }

    /// Set secondary currency filter
    pub fn with_secondary_currency(mut self, secondary_currency: String) -> Self {
        self.secondary_currency = Some(secondary_currency);
        self
    }

    /// Set security type filter
    pub fn with_security_type(mut self, security_type: SecurityType) -> Self {
        self.security_type = Some(security_type);
        self
    }

    /// Enable multicast instrument ID display
    pub fn with_multicast_instrument_id(mut self, enable: bool) -> Self {
        self.display_multicast_instrument_id = Some(enable);
        self
    }

    /// Enable increment steps display
    pub fn with_increment_steps(mut self, enable: bool) -> Self {
        self.display_increment_steps = Some(enable);
        self
    }

    /// Convert to FIX message
    pub fn to_fix_message(
        &self,
        sender_comp_id: String,
        target_comp_id: String,
        msg_seq_num: u32,
    ) -> DeribitFixResult<crate::model::message::FixMessage> {
        let mut builder = MessageBuilder::new()
            .msg_type(MsgType::SecurityListRequest)
            .sender_comp_id(sender_comp_id)
            .target_comp_id(target_comp_id)
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now())
            .field(320, self.security_req_id.clone()) // SecurityReqId
            .field(559, i32::from(self.security_list_request_type).to_string()); // SecurityListRequestType

        // Add optional fields
        if let Some(subscription_type) = self.subscription_request_type {
            builder = builder.field(263, i32::from(subscription_type).to_string()); // SubscriptionRequestType
        }

        if let Some(display_multicast) = self.display_multicast_instrument_id {
            builder = builder.field(
                9013,
                if display_multicast {
                    "Y".to_string()
                } else {
                    "N".to_string()
                },
            ); // DisplayMulticastInstrumentID
        }

        if let Some(display_steps) = self.display_increment_steps {
            builder = builder.field(
                9018,
                if display_steps {
                    "Y".to_string()
                } else {
                    "N".to_string()
                },
            ); // DisplayIncrementSteps
        }

        if let Some(ref currency) = self.currency {
            builder = builder.field(15, currency.clone()); // Currency
        }

        if let Some(ref secondary_currency) = self.secondary_currency {
            builder = builder.field(5544, secondary_currency.clone()); // SecondaryCurrency
        }

        if let Some(ref security_type) = self.security_type {
            builder = builder.field(167, security_type.as_fix_str().to_string()); // SecurityType
        }

        builder.build()
    }
}

/// Put or Call indicator for options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PutOrCall {
    /// Put option
    Put = 0,
    /// Call option
    Call = 1,
}

impl From<PutOrCall> for i32 {
    fn from(put_or_call: PutOrCall) -> Self {
        put_or_call as i32
    }
}

impl TryFrom<i32> for PutOrCall {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PutOrCall::Put),
            1 => Ok(PutOrCall::Call),
            _ => Err(format!("Invalid PutOrCall: {value}")),
        }
    }
}

/// Security Status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityStatus {
    /// Active/Started
    Active = 1,
    /// Terminated/Inactive
    Terminated = 2,
    /// Closed
    Closed = 4,
    /// Published/Created
    Published = 10,
    /// Settled
    Settled = 12,
}

impl From<SecurityStatus> for i32 {
    fn from(status: SecurityStatus) -> Self {
        status as i32
    }
}

impl TryFrom<i32> for SecurityStatus {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(SecurityStatus::Active),
            2 => Ok(SecurityStatus::Terminated),
            4 => Ok(SecurityStatus::Closed),
            10 => Ok(SecurityStatus::Published),
            12 => Ok(SecurityStatus::Settled),
            _ => Err(format!("Invalid SecurityStatus: {value}")),
        }
    }
}

/// Security Alternative ID information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAltId {
    /// Security identifier (Tag 455)
    pub security_alt_id: String,
    /// Source of the identifier (Tag 456)
    pub security_alt_id_source: String,
}

impl SecurityAltId {
    /// Create multicast identifier
    pub fn multicast(id: String) -> Self {
        Self {
            security_alt_id: id,
            security_alt_id_source: "101".to_string(), // Multicast identifier
        }
    }

    /// Create combo instrument identifier
    pub fn combo(id: String) -> Self {
        Self {
            security_alt_id: id,
            security_alt_id_source: "102".to_string(), // Combo instrument identifier
        }
    }
}

/// Price increment rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickRule {
    /// Above this price, the tick increment applies (Tag 1206)
    pub start_tick_price_range: f64,
    /// Valid price increment for prices above the start range (Tag 1208)
    pub tick_increment: f64,
}

/// Security information in Security List response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityInfo {
    /// Symbol name (Tag 55)
    pub symbol: String,
    /// Security description (Tag 107)
    pub security_desc: Option<String>,
    /// Security type (Tag 167)
    pub security_type: Option<SecurityType>,
    /// Put or Call indicator (Tag 201) - Options only
    pub put_or_call: Option<PutOrCall>,
    /// Strike price (Tag 202) - Options only
    pub strike_price: Option<f64>,
    /// Strike currency (Tag 947)
    pub strike_currency: Option<String>,
    /// Currency (Tag 15)
    pub currency: Option<String>,
    /// Price quote currency (Tag 1524)
    pub price_quote_currency: Option<String>,
    /// Instrument price precision (Tag 2576)
    pub instrument_price_precision: Option<i32>,
    /// Minimum price increment (Tag 969)
    pub min_price_increment: Option<f64>,
    /// Underlying symbol (Tag 311) - Options only
    pub underlying_symbol: Option<String>,
    /// Issue date (Tag 225)
    pub issue_date: Option<DateTime<Utc>>,
    /// Maturity date (Tag 541)
    pub maturity_date: Option<DateTime<Utc>>,
    /// Maturity time (Tag 1079)
    pub maturity_time: Option<DateTime<Utc>>,
    /// Minimum trade volume (Tag 562)
    pub min_trade_vol: Option<f64>,
    /// Settlement type (Tag 63)
    pub settl_type: Option<String>,
    /// Settlement currency (Tag 120)
    pub settl_currency: Option<String>,
    /// Commission currency (Tag 479)
    pub comm_currency: Option<String>,
    /// Contract multiplier (Tag 231)
    pub contract_multiplier: Option<f64>,
    /// Alternative security identifiers (Tag 454)
    pub security_alt_ids: Vec<SecurityAltId>,
    /// Price increment rules (Tag 1205)
    pub tick_rules: Vec<TickRule>,
    /// Security status (Tag 965) - Present in notifications
    pub security_status: Option<SecurityStatus>,
}

impl SecurityInfo {
    /// Create a new security info with minimal required fields
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            security_desc: None,
            security_type: None,
            put_or_call: None,
            strike_price: None,
            strike_currency: None,
            currency: None,
            price_quote_currency: None,
            instrument_price_precision: None,
            min_price_increment: None,
            underlying_symbol: None,
            issue_date: None,
            maturity_date: None,
            maturity_time: None,
            min_trade_vol: None,
            settl_type: None,
            settl_currency: None,
            comm_currency: None,
            contract_multiplier: None,
            security_alt_ids: Vec::new(),
            tick_rules: Vec::new(),
            security_status: None,
        }
    }

    /// Check if this is an option
    pub fn is_option(&self) -> bool {
        matches!(
            self.security_type,
            Some(SecurityType::Option | SecurityType::OptionCombo)
        )
    }

    /// Check if this is a future
    pub fn is_future(&self) -> bool {
        matches!(
            self.security_type,
            Some(SecurityType::Future | SecurityType::FutureCombo)
        )
    }

    /// Check if this is a spot instrument
    pub fn is_spot(&self) -> bool {
        matches!(self.security_type, Some(SecurityType::FxSpot))
    }
}

/// Security List response message (MsgType = y)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityList {
    /// Security Request ID from the original request (Tag 320)
    pub security_req_id: String,
    /// Security Response ID (Tag 322)
    pub security_response_id: String,
    /// Security Request Result (Tag 560) - Always 0 for successful response
    pub security_request_result: i32,
    /// List of securities (Tag 146 - NoRelatedSym)
    pub securities: Vec<SecurityInfo>,
}

impl SecurityList {
    /// Create a new Security List response
    pub fn new(
        security_req_id: String,
        security_response_id: String,
        securities: Vec<SecurityInfo>,
    ) -> Self {
        Self {
            security_req_id,
            security_response_id,
            security_request_result: 0, // Always 0 for successful response
            securities,
        }
    }

    /// Create a successful response
    pub fn success(
        security_req_id: String,
        security_response_id: String,
        securities: Vec<SecurityInfo>,
    ) -> Self {
        Self::new(security_req_id, security_response_id, securities)
    }

    /// Get number of securities in the list
    pub fn count(&self) -> usize {
        self.securities.len()
    }

    /// Check if the response is successful
    pub fn is_successful(&self) -> bool {
        self.security_request_result == 0
    }

    /// Filter securities by type
    pub fn filter_by_type(&self, security_type: SecurityType) -> Vec<&SecurityInfo> {
        self.securities
            .iter()
            .filter(|s| s.security_type == Some(security_type))
            .collect()
    }

    /// Filter securities by currency
    pub fn filter_by_currency(&self, currency: &str) -> Vec<&SecurityInfo> {
        self.securities
            .iter()
            .filter(|s| s.currency.as_deref() == Some(currency))
            .collect()
    }

    /// Convert to FIX message
    pub fn to_fix_message(
        &self,
        sender_comp_id: String,
        target_comp_id: String,
        msg_seq_num: u32,
    ) -> DeribitFixResult<crate::model::message::FixMessage> {
        let mut builder = MessageBuilder::new()
            .msg_type(MsgType::SecurityList)
            .sender_comp_id(sender_comp_id)
            .target_comp_id(target_comp_id)
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now())
            .field(320, self.security_req_id.clone()) // SecurityReqId
            .field(322, self.security_response_id.clone()) // SecurityResponseID
            .field(560, self.security_request_result.to_string()) // SecurityRequestResult
            .field(146, self.securities.len().to_string()); // NoRelatedSym

        // Add security information
        // Note: In a real implementation, you would need to add all the repeating group fields
        // for each security. This is a simplified version showing the structure.
        for (i, security) in self.securities.iter().enumerate() {
            let prefix = format!("{}.", i + 1);
            builder = builder.field(55, format!("{}{}", prefix, security.symbol)); // Symbol

            if let Some(ref desc) = security.security_desc {
                builder = builder.field(107, format!("{prefix}{desc}")); // SecurityDesc
            }

            if let Some(ref sec_type) = security.security_type {
                builder = builder.field(167, format!("{}{}", prefix, sec_type.as_fix_str())); // SecurityType
            }

            // Add other fields as needed...
        }

        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_list_request_creation() {
        let request = SecurityListRequest::snapshot("REQ123".to_string());
        assert_eq!(request.security_req_id, "REQ123");
        assert_eq!(
            request.security_list_request_type,
            SecurityListRequestType::Snapshot
        );
        assert!(request.subscription_request_type.is_none());
    }

    #[test]
    fn test_security_list_request_subscription() {
        let request = SecurityListRequest::subscription("SUB456".to_string());
        assert_eq!(request.security_req_id, "SUB456");
        assert_eq!(
            request.security_list_request_type,
            SecurityListRequestType::SnapshotAndUpdates
        );
        assert_eq!(
            request.subscription_request_type,
            Some(SubscriptionRequestType::SnapshotPlusUpdates)
        );
    }

    #[test]
    fn test_security_list_request_with_filters() {
        let request = SecurityListRequest::snapshot("FILTER789".to_string())
            .with_currency("BTC".to_string())
            .with_security_type(SecurityType::Future)
            .with_multicast_instrument_id(true);

        assert_eq!(request.currency, Some("BTC".to_string()));
        assert_eq!(request.security_type, Some(SecurityType::Future));
        assert_eq!(request.display_multicast_instrument_id, Some(true));
    }

    #[test]
    fn test_security_type_conversion() {
        assert_eq!(SecurityType::Future.as_fix_str(), "FUT");
        assert_eq!(SecurityType::Option.as_fix_str(), "OPT");
        assert_eq!(SecurityType::FxSpot.as_fix_str(), "FXSPOT");

        assert_eq!(
            SecurityType::from_fix_str("FUT").unwrap(),
            SecurityType::Future
        );
        assert_eq!(
            SecurityType::from_fix_str("OPT").unwrap(),
            SecurityType::Option
        );
        assert!(SecurityType::from_fix_str("INVALID").is_err());
    }

    #[test]
    fn test_security_info_creation() {
        let mut security = SecurityInfo::new("BTC-PERPETUAL".to_string());
        security.security_type = Some(SecurityType::Future);
        security.currency = Some("BTC".to_string());

        assert_eq!(security.symbol, "BTC-PERPETUAL");
        assert!(security.is_future());
        assert!(!security.is_option());
        assert!(!security.is_spot());
    }

    #[test]
    fn test_security_list_response() {
        let securities = vec![
            SecurityInfo::new("BTC-PERPETUAL".to_string()),
            SecurityInfo::new("ETH-PERPETUAL".to_string()),
        ];

        let response =
            SecurityList::success("REQ123".to_string(), "RESP456".to_string(), securities);

        assert_eq!(response.security_req_id, "REQ123");
        assert_eq!(response.security_response_id, "RESP456");
        assert_eq!(response.count(), 2);
        assert!(response.is_successful());
    }

    #[test]
    fn test_put_or_call_conversion() {
        assert_eq!(i32::from(PutOrCall::Put), 0);
        assert_eq!(i32::from(PutOrCall::Call), 1);
        assert_eq!(PutOrCall::try_from(0).unwrap(), PutOrCall::Put);
        assert_eq!(PutOrCall::try_from(1).unwrap(), PutOrCall::Call);
        assert!(PutOrCall::try_from(2).is_err());
    }

    #[test]
    fn test_security_status_conversion() {
        assert_eq!(i32::from(SecurityStatus::Active), 1);
        assert_eq!(i32::from(SecurityStatus::Terminated), 2);
        assert_eq!(SecurityStatus::try_from(1).unwrap(), SecurityStatus::Active);
        assert_eq!(
            SecurityStatus::try_from(2).unwrap(),
            SecurityStatus::Terminated
        );
        assert!(SecurityStatus::try_from(99).is_err());
    }

    #[test]
    fn test_security_alt_id_creation() {
        let multicast_id = SecurityAltId::multicast("MC123".to_string());
        assert_eq!(multicast_id.security_alt_id, "MC123");
        assert_eq!(multicast_id.security_alt_id_source, "101");

        let combo_id = SecurityAltId::combo("COMBO456".to_string());
        assert_eq!(combo_id.security_alt_id, "COMBO456");
        assert_eq!(combo_id.security_alt_id_source, "102");
    }

    #[test]
    fn test_tick_rule_creation() {
        let tick_rule = TickRule {
            start_tick_price_range: 100.0,
            tick_increment: 0.5,
        };

        assert_eq!(tick_rule.start_tick_price_range, 100.0);
        assert_eq!(tick_rule.tick_increment, 0.5);
    }
}
