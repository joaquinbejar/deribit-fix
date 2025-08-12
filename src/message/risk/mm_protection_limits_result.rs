/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 12/8/25
******************************************************************************/

//! MM Protection Limits Result/Reject FIX Message Implementation

use crate::error::Result as DeribitFixResult;
use crate::message::builder::MessageBuilder;
use crate::model::types::MsgType;
use chrono::{DateTime, Utc};
use deribit_base::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

// Re-export from mm_protection_limits module
pub use super::mm_protection_limits::{MMProtectionAction, MMProtectionScope};

/// MM Protection result status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MMProtectionResultStatus {
    /// Request accepted
    Accepted,
    /// Request rejected
    Rejected,
    /// Request completed successfully
    Completed,
    /// Request partially completed
    PartiallyCompleted,
    /// Request pending
    Pending,
}

impl From<MMProtectionResultStatus> for i32 {
    fn from(status: MMProtectionResultStatus) -> Self {
        match status {
            MMProtectionResultStatus::Accepted => 0,
            MMProtectionResultStatus::Rejected => 1,
            MMProtectionResultStatus::Completed => 2,
            MMProtectionResultStatus::PartiallyCompleted => 3,
            MMProtectionResultStatus::Pending => 4,
        }
    }
}

impl TryFrom<i32> for MMProtectionResultStatus {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MMProtectionResultStatus::Accepted),
            1 => Ok(MMProtectionResultStatus::Rejected),
            2 => Ok(MMProtectionResultStatus::Completed),
            3 => Ok(MMProtectionResultStatus::PartiallyCompleted),
            4 => Ok(MMProtectionResultStatus::Pending),
            _ => Err(format!("Invalid MMProtectionResultStatus: {}", value)),
        }
    }
}

/// MM Protection reject reason enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MMProtectionRejectReason {
    /// Unknown request
    UnknownRequest,
    /// Invalid action
    InvalidAction,
    /// Invalid scope
    InvalidScope,
    /// Unknown symbol
    UnknownSymbol,
    /// Invalid limits
    InvalidLimits,
    /// Insufficient permissions
    InsufficientPermissions,
    /// System error
    SystemError,
    /// Request timeout
    RequestTimeout,
    /// Limits already exist
    LimitsAlreadyExist,
    /// Limits not found
    LimitsNotFound,
    /// Other
    Other,
}

impl From<MMProtectionRejectReason> for i32 {
    fn from(reason: MMProtectionRejectReason) -> Self {
        match reason {
            MMProtectionRejectReason::UnknownRequest => 1,
            MMProtectionRejectReason::InvalidAction => 2,
            MMProtectionRejectReason::InvalidScope => 3,
            MMProtectionRejectReason::UnknownSymbol => 4,
            MMProtectionRejectReason::InvalidLimits => 5,
            MMProtectionRejectReason::InsufficientPermissions => 6,
            MMProtectionRejectReason::SystemError => 7,
            MMProtectionRejectReason::RequestTimeout => 8,
            MMProtectionRejectReason::LimitsAlreadyExist => 9,
            MMProtectionRejectReason::LimitsNotFound => 10,
            MMProtectionRejectReason::Other => 99,
        }
    }
}

impl TryFrom<i32> for MMProtectionRejectReason {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(MMProtectionRejectReason::UnknownRequest),
            2 => Ok(MMProtectionRejectReason::InvalidAction),
            3 => Ok(MMProtectionRejectReason::InvalidScope),
            4 => Ok(MMProtectionRejectReason::UnknownSymbol),
            5 => Ok(MMProtectionRejectReason::InvalidLimits),
            6 => Ok(MMProtectionRejectReason::InsufficientPermissions),
            7 => Ok(MMProtectionRejectReason::SystemError),
            8 => Ok(MMProtectionRejectReason::RequestTimeout),
            9 => Ok(MMProtectionRejectReason::LimitsAlreadyExist),
            10 => Ok(MMProtectionRejectReason::LimitsNotFound),
            99 => Ok(MMProtectionRejectReason::Other),
            _ => Err(format!("Invalid MMProtectionRejectReason: {}", value)),
        }
    }
}

/// MM Protection Limits Result/Reject message (MsgType = 'MR')
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct MMProtectionLimitsResult {
    /// MM Protection request ID (from original request)
    pub mm_protection_req_id: String,
    /// MM Protection action (from original request)
    pub mm_protection_action: MMProtectionAction,
    /// MM Protection scope (from original request)
    pub mm_protection_scope: MMProtectionScope,
    /// Result status
    pub mm_protection_result_status: MMProtectionResultStatus,
    /// Reject reason (if rejected)
    pub mm_protection_reject_reason: Option<MMProtectionRejectReason>,
    /// Instrument symbol (if applicable)
    pub symbol: Option<String>,
    /// Underlying symbol (if applicable)
    pub underlying_symbol: Option<String>,
    /// Instrument group (if applicable)
    pub instrument_group: Option<String>,
    /// Current maximum position limit
    pub current_max_position_limit: Option<f64>,
    /// Current maximum order quantity limit
    pub current_max_order_qty_limit: Option<f64>,
    /// Current maximum number of orders limit
    pub current_max_orders_limit: Option<i32>,
    /// Current time window (in seconds)
    pub current_time_window_seconds: Option<i32>,
    /// Current delta limit
    pub current_delta_limit: Option<f64>,
    /// Current vega limit
    pub current_vega_limit: Option<f64>,
    /// Current gamma limit
    pub current_gamma_limit: Option<f64>,
    /// Current theta limit
    pub current_theta_limit: Option<f64>,
    /// Current total risk limit
    pub current_total_risk_limit: Option<f64>,
    /// Current valid from time
    pub current_valid_from: Option<DateTime<Utc>>,
    /// Current valid until time
    pub current_valid_until: Option<DateTime<Utc>>,
    /// Processing time
    pub processing_time: DateTime<Utc>,
    /// Number of affected instruments
    pub affected_instruments_count: Option<i32>,
    /// Account
    pub account: Option<String>,
    /// Parties
    pub parties: Option<String>,
    /// Text
    pub text: Option<String>,
    /// Custom label
    pub deribit_label: Option<String>,
}

impl MMProtectionLimitsResult {
    /// Create a new MM Protection Limits Result
    pub fn new(
        mm_protection_req_id: String,
        mm_protection_action: MMProtectionAction,
        mm_protection_scope: MMProtectionScope,
        mm_protection_result_status: MMProtectionResultStatus,
    ) -> Self {
        Self {
            mm_protection_req_id,
            mm_protection_action,
            mm_protection_scope,
            mm_protection_result_status,
            mm_protection_reject_reason: None,
            symbol: None,
            underlying_symbol: None,
            instrument_group: None,
            current_max_position_limit: None,
            current_max_order_qty_limit: None,
            current_max_orders_limit: None,
            current_time_window_seconds: None,
            current_delta_limit: None,
            current_vega_limit: None,
            current_gamma_limit: None,
            current_theta_limit: None,
            current_total_risk_limit: None,
            current_valid_from: None,
            current_valid_until: None,
            processing_time: Utc::now(),
            affected_instruments_count: None,
            account: None,
            parties: None,
            text: None,
            deribit_label: None,
        }
    }

    /// Create an accepted result
    pub fn accepted(
        mm_protection_req_id: String,
        mm_protection_action: MMProtectionAction,
        mm_protection_scope: MMProtectionScope,
    ) -> Self {
        Self::new(
            mm_protection_req_id,
            mm_protection_action,
            mm_protection_scope,
            MMProtectionResultStatus::Accepted,
        )
    }

    /// Create a rejected result
    pub fn rejected(
        mm_protection_req_id: String,
        mm_protection_action: MMProtectionAction,
        mm_protection_scope: MMProtectionScope,
        reject_reason: MMProtectionRejectReason,
        text: Option<String>,
    ) -> Self {
        let mut result = Self::new(
            mm_protection_req_id,
            mm_protection_action,
            mm_protection_scope,
            MMProtectionResultStatus::Rejected,
        );
        result.mm_protection_reject_reason = Some(reject_reason);
        result.text = text;
        result
    }

    /// Create a completed result with current limits
    pub fn completed(
        mm_protection_req_id: String,
        mm_protection_action: MMProtectionAction,
        mm_protection_scope: MMProtectionScope,
    ) -> Self {
        Self::new(
            mm_protection_req_id,
            mm_protection_action,
            mm_protection_scope,
            MMProtectionResultStatus::Completed,
        )
    }

    /// Set symbol
    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    /// Set underlying symbol
    pub fn with_underlying_symbol(mut self, underlying_symbol: String) -> Self {
        self.underlying_symbol = Some(underlying_symbol);
        self
    }

    /// Set instrument group
    pub fn with_instrument_group(mut self, instrument_group: String) -> Self {
        self.instrument_group = Some(instrument_group);
        self
    }

    /// Set current limits
    #[allow(clippy::too_many_arguments)]
    pub fn with_current_limits(
        mut self,
        position_limit: Option<f64>,
        order_qty_limit: Option<f64>,
        max_orders_limit: Option<i32>,
        time_window_seconds: Option<i32>,
        total_risk_limit: Option<f64>,
    ) -> Self {
        self.current_max_position_limit = position_limit;
        self.current_max_order_qty_limit = order_qty_limit;
        self.current_max_orders_limit = max_orders_limit;
        self.current_time_window_seconds = time_window_seconds;
        self.current_total_risk_limit = total_risk_limit;
        self
    }

    /// Set current Greeks limits
    pub fn with_current_greeks_limits(
        mut self,
        delta: Option<f64>,
        vega: Option<f64>,
        gamma: Option<f64>,
        theta: Option<f64>,
    ) -> Self {
        self.current_delta_limit = delta;
        self.current_vega_limit = vega;
        self.current_gamma_limit = gamma;
        self.current_theta_limit = theta;
        self
    }

    /// Set current validity period
    pub fn with_current_validity_period(mut self, from: DateTime<Utc>, until: DateTime<Utc>) -> Self {
        self.current_valid_from = Some(from);
        self.current_valid_until = Some(until);
        self
    }

    /// Set affected instruments count
    pub fn with_affected_instruments_count(mut self, count: i32) -> Self {
        self.affected_instruments_count = Some(count);
        self
    }

    /// Set account
    pub fn with_account(mut self, account: String) -> Self {
        self.account = Some(account);
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

    /// Convert to FIX message
    pub fn to_fix_message(
        &self,
        sender_comp_id: &str,
        target_comp_id: &str,
        msg_seq_num: u32,
    ) -> DeribitFixResult<String> {
        let mut builder = MessageBuilder::new()
            .msg_type(MsgType::MmProtectionLimitsResult)
            .sender_comp_id(sender_comp_id.to_string())
            .target_comp_id(target_comp_id.to_string())
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now());

        // Required fields
        builder = builder
            .field(9001, self.mm_protection_req_id.clone()) // MMProtectionReqID
            .field(9002, i32::from(self.mm_protection_action).to_string()) // MMProtectionAction
            .field(9003, i32::from(self.mm_protection_scope).to_string()) // MMProtectionScope
            .field(9017, i32::from(self.mm_protection_result_status).to_string()) // MMProtectionResultStatus
            .field(9018, self.processing_time.format("%Y%m%d-%H:%M:%S%.3f").to_string()); // ProcessingTime

        // Optional fields
        if let Some(reject_reason) = &self.mm_protection_reject_reason {
            builder = builder.field(9019, i32::from(*reject_reason).to_string());
        }

        if let Some(symbol) = &self.symbol {
            builder = builder.field(55, symbol.clone());
        }

        if let Some(underlying_symbol) = &self.underlying_symbol {
            builder = builder.field(311, underlying_symbol.clone());
        }

        if let Some(instrument_group) = &self.instrument_group {
            builder = builder.field(9004, instrument_group.clone());
        }

        // Current limits
        if let Some(current_max_position_limit) = &self.current_max_position_limit {
            builder = builder.field(9020, current_max_position_limit.to_string());
        }

        if let Some(current_max_order_qty_limit) = &self.current_max_order_qty_limit {
            builder = builder.field(9021, current_max_order_qty_limit.to_string());
        }

        if let Some(current_max_orders_limit) = &self.current_max_orders_limit {
            builder = builder.field(9022, current_max_orders_limit.to_string());
        }

        if let Some(current_time_window_seconds) = &self.current_time_window_seconds {
            builder = builder.field(9023, current_time_window_seconds.to_string());
        }

        // Current Greeks limits
        if let Some(current_delta_limit) = &self.current_delta_limit {
            builder = builder.field(9024, current_delta_limit.to_string());
        }

        if let Some(current_vega_limit) = &self.current_vega_limit {
            builder = builder.field(9025, current_vega_limit.to_string());
        }

        if let Some(current_gamma_limit) = &self.current_gamma_limit {
            builder = builder.field(9026, current_gamma_limit.to_string());
        }

        if let Some(current_theta_limit) = &self.current_theta_limit {
            builder = builder.field(9027, current_theta_limit.to_string());
        }

        if let Some(current_total_risk_limit) = &self.current_total_risk_limit {
            builder = builder.field(9028, current_total_risk_limit.to_string());
        }

        // Current validity period
        if let Some(current_valid_from) = &self.current_valid_from {
            builder = builder.field(
                9029,
                current_valid_from.format("%Y%m%d-%H:%M:%S%.3f").to_string(),
            );
        }

        if let Some(current_valid_until) = &self.current_valid_until {
            builder = builder.field(
                9030,
                current_valid_until.format("%Y%m%d-%H:%M:%S%.3f").to_string(),
            );
        }

        if let Some(affected_instruments_count) = &self.affected_instruments_count {
            builder = builder.field(9031, affected_instruments_count.to_string());
        }

        // Standard optional fields
        if let Some(account) = &self.account {
            builder = builder.field(1, account.clone());
        }

        if let Some(parties) = &self.parties {
            builder = builder.field(453, parties.clone());
        }

        if let Some(text) = &self.text {
            builder = builder.field(58, text.clone());
        }

        if let Some(deribit_label) = &self.deribit_label {
            builder = builder.field(100010, deribit_label.clone());
        }

        Ok(builder.build()?.to_string())
    }
}

impl_json_display!(MMProtectionLimitsResult);
impl_json_debug_pretty!(MMProtectionLimitsResult);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mm_protection_limits_result_creation() {
        let result = MMProtectionLimitsResult::new(
            "MMP123".to_string(),
            MMProtectionAction::SetLimits,
            MMProtectionScope::AllInstruments,
            MMProtectionResultStatus::Accepted,
        );

        assert_eq!(result.mm_protection_req_id, "MMP123");
        assert_eq!(result.mm_protection_action, MMProtectionAction::SetLimits);
        assert_eq!(result.mm_protection_scope, MMProtectionScope::AllInstruments);
        assert_eq!(result.mm_protection_result_status, MMProtectionResultStatus::Accepted);
        assert!(result.mm_protection_reject_reason.is_none());
    }

    #[test]
    fn test_mm_protection_limits_result_accepted() {
        let result = MMProtectionLimitsResult::accepted(
            "MMP456".to_string(),
            MMProtectionAction::UpdateLimits,
            MMProtectionScope::SpecificInstrument,
        );

        assert_eq!(result.mm_protection_result_status, MMProtectionResultStatus::Accepted);
        assert_eq!(result.mm_protection_action, MMProtectionAction::UpdateLimits);
        assert_eq!(result.mm_protection_scope, MMProtectionScope::SpecificInstrument);
    }

    #[test]
    fn test_mm_protection_limits_result_rejected() {
        let result = MMProtectionLimitsResult::rejected(
            "MMP789".to_string(),
            MMProtectionAction::SetLimits,
            MMProtectionScope::SpecificInstrument,
            MMProtectionRejectReason::UnknownSymbol,
            Some("Symbol not found".to_string()),
        );

        assert_eq!(result.mm_protection_result_status, MMProtectionResultStatus::Rejected);
        assert_eq!(result.mm_protection_reject_reason, Some(MMProtectionRejectReason::UnknownSymbol));
        assert_eq!(result.text, Some("Symbol not found".to_string()));
    }

    #[test]
    fn test_mm_protection_limits_result_completed() {
        let result = MMProtectionLimitsResult::completed(
            "MMP999".to_string(),
            MMProtectionAction::QueryLimits,
            MMProtectionScope::AllInstruments,
        );

        assert_eq!(result.mm_protection_result_status, MMProtectionResultStatus::Completed);
        assert_eq!(result.mm_protection_action, MMProtectionAction::QueryLimits);
    }

    #[test]
    fn test_mm_protection_limits_result_with_options() {
        let valid_from = Utc::now();
        let valid_until = valid_from + chrono::Duration::hours(24);

        let result = MMProtectionLimitsResult::completed(
            "MMP111".to_string(),
            MMProtectionAction::SetLimits,
            MMProtectionScope::SpecificInstrument,
        )
        .with_symbol("BTC-PERPETUAL".to_string())
        .with_current_limits(Some(1000.0), Some(100.0), Some(50), Some(300), Some(10000.0))
        .with_current_greeks_limits(Some(10.0), Some(5.0), Some(2.0), Some(-1.0))
        .with_current_validity_period(valid_from, valid_until)
        .with_affected_instruments_count(1)
        .with_account("ACC123".to_string())
        .with_text("Limits updated successfully".to_string())
        .with_label("test-result".to_string());

        assert_eq!(result.symbol, Some("BTC-PERPETUAL".to_string()));
        assert_eq!(result.current_max_position_limit, Some(1000.0));
        assert_eq!(result.current_max_order_qty_limit, Some(100.0));
        assert_eq!(result.current_max_orders_limit, Some(50));
        assert_eq!(result.current_time_window_seconds, Some(300));
        assert_eq!(result.current_delta_limit, Some(10.0));
        assert_eq!(result.current_vega_limit, Some(5.0));
        assert_eq!(result.current_gamma_limit, Some(2.0));
        assert_eq!(result.current_theta_limit, Some(-1.0));
        assert_eq!(result.current_total_risk_limit, Some(10000.0));
        assert_eq!(result.current_valid_from, Some(valid_from));
        assert_eq!(result.current_valid_until, Some(valid_until));
        assert_eq!(result.affected_instruments_count, Some(1));
        assert_eq!(result.account, Some("ACC123".to_string()));
        assert_eq!(result.text, Some("Limits updated successfully".to_string()));
        assert_eq!(result.deribit_label, Some("test-result".to_string()));
    }

    #[test]
    fn test_mm_protection_limits_result_to_fix_message() {
        let result = MMProtectionLimitsResult::accepted(
            "MMP123".to_string(),
            MMProtectionAction::SetLimits,
            MMProtectionScope::SpecificInstrument,
        )
        .with_symbol("BTC-PERPETUAL".to_string())
        .with_current_limits(Some(500.0), Some(50.0), None, None, None)
        .with_label("test-label".to_string());

        let fix_message = result.to_fix_message("SENDER", "TARGET", 1).unwrap();

        // Check that the message contains required fields
        assert!(fix_message.contains("35=MR")); // MsgType
        assert!(fix_message.contains("9001=MMP123")); // MMProtectionReqID
        assert!(fix_message.contains("9002=1")); // MMProtectionAction=SetLimits
        assert!(fix_message.contains("9003=2")); // MMProtectionScope=SpecificInstrument
        assert!(fix_message.contains("9017=0")); // MMProtectionResultStatus=Accepted
        assert!(fix_message.contains("55=BTC-PERPETUAL")); // Symbol
        assert!(fix_message.contains("9020=500")); // CurrentMaxPositionLimit
        assert!(fix_message.contains("9021=50")); // CurrentMaxOrderQtyLimit
        assert!(fix_message.contains("100010=test-label")); // Custom label
    }

    #[test]
    fn test_mm_protection_result_status_conversions() {
        assert_eq!(i32::from(MMProtectionResultStatus::Accepted), 0);
        assert_eq!(i32::from(MMProtectionResultStatus::Rejected), 1);
        assert_eq!(i32::from(MMProtectionResultStatus::Completed), 2);
        assert_eq!(i32::from(MMProtectionResultStatus::PartiallyCompleted), 3);
        assert_eq!(i32::from(MMProtectionResultStatus::Pending), 4);

        assert_eq!(MMProtectionResultStatus::try_from(0).unwrap(), MMProtectionResultStatus::Accepted);
        assert_eq!(MMProtectionResultStatus::try_from(1).unwrap(), MMProtectionResultStatus::Rejected);
        assert_eq!(MMProtectionResultStatus::try_from(2).unwrap(), MMProtectionResultStatus::Completed);
        assert_eq!(MMProtectionResultStatus::try_from(3).unwrap(), MMProtectionResultStatus::PartiallyCompleted);
        assert_eq!(MMProtectionResultStatus::try_from(4).unwrap(), MMProtectionResultStatus::Pending);

        assert!(MMProtectionResultStatus::try_from(99).is_err());
    }

    #[test]
    fn test_mm_protection_reject_reason_conversions() {
        assert_eq!(i32::from(MMProtectionRejectReason::UnknownRequest), 1);
        assert_eq!(i32::from(MMProtectionRejectReason::InvalidAction), 2);
        assert_eq!(i32::from(MMProtectionRejectReason::UnknownSymbol), 4);
        assert_eq!(i32::from(MMProtectionRejectReason::Other), 99);

        assert_eq!(MMProtectionRejectReason::try_from(1).unwrap(), MMProtectionRejectReason::UnknownRequest);
        assert_eq!(MMProtectionRejectReason::try_from(2).unwrap(), MMProtectionRejectReason::InvalidAction);
        assert_eq!(MMProtectionRejectReason::try_from(4).unwrap(), MMProtectionRejectReason::UnknownSymbol);
        assert_eq!(MMProtectionRejectReason::try_from(99).unwrap(), MMProtectionRejectReason::Other);

        assert!(MMProtectionRejectReason::try_from(50).is_err());
    }
}