/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 12/8/25
******************************************************************************/

//! MM Protection Limits FIX Message Implementation

use crate::error::Result as DeribitFixResult;
use crate::message::builder::MessageBuilder;
use crate::model::types::MsgType;
use chrono::{DateTime, Utc};
use deribit_base::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

/// MM Protection action enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MMProtectionAction {
    /// Set new limits
    SetLimits,
    /// Update existing limits
    UpdateLimits,
    /// Query current limits
    QueryLimits,
    /// Remove limits
    RemoveLimits,
}

impl From<MMProtectionAction> for i32 {
    fn from(action: MMProtectionAction) -> Self {
        match action {
            MMProtectionAction::SetLimits => 1,
            MMProtectionAction::UpdateLimits => 2,
            MMProtectionAction::QueryLimits => 3,
            MMProtectionAction::RemoveLimits => 4,
        }
    }
}

impl TryFrom<i32> for MMProtectionAction {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(MMProtectionAction::SetLimits),
            2 => Ok(MMProtectionAction::UpdateLimits),
            3 => Ok(MMProtectionAction::QueryLimits),
            4 => Ok(MMProtectionAction::RemoveLimits),
            _ => Err(format!("Invalid MMProtectionAction: {}", value)),
        }
    }
}

/// MM Protection scope enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MMProtectionScope {
    /// Apply to all instruments
    AllInstruments,
    /// Apply to specific instrument
    SpecificInstrument,
    /// Apply to instrument group
    InstrumentGroup,
    /// Apply to underlying
    Underlying,
}

impl From<MMProtectionScope> for i32 {
    fn from(scope: MMProtectionScope) -> Self {
        match scope {
            MMProtectionScope::AllInstruments => 1,
            MMProtectionScope::SpecificInstrument => 2,
            MMProtectionScope::InstrumentGroup => 3,
            MMProtectionScope::Underlying => 4,
        }
    }
}

impl TryFrom<i32> for MMProtectionScope {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(MMProtectionScope::AllInstruments),
            2 => Ok(MMProtectionScope::SpecificInstrument),
            3 => Ok(MMProtectionScope::InstrumentGroup),
            4 => Ok(MMProtectionScope::Underlying),
            _ => Err(format!("Invalid MMProtectionScope: {}", value)),
        }
    }
}

/// MM Protection Limits message (MsgType = 'MM')
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct MMProtectionLimits {
    /// MM Protection request ID
    pub mm_protection_req_id: String,
    /// MM Protection action
    pub mm_protection_action: MMProtectionAction,
    /// MM Protection scope
    pub mm_protection_scope: MMProtectionScope,
    /// Instrument symbol (required for specific instrument scope)
    pub symbol: Option<String>,
    /// Underlying symbol (for underlying scope)
    pub underlying_symbol: Option<String>,
    /// Instrument group (for group scope)
    pub instrument_group: Option<String>,
    /// Maximum position limit
    pub max_position_limit: Option<f64>,
    /// Maximum order quantity limit
    pub max_order_qty_limit: Option<f64>,
    /// Maximum number of orders limit
    pub max_orders_limit: Option<i32>,
    /// Time window for limits (in seconds)
    pub time_window_seconds: Option<i32>,
    /// Delta limit
    pub delta_limit: Option<f64>,
    /// Vega limit
    pub vega_limit: Option<f64>,
    /// Gamma limit
    pub gamma_limit: Option<f64>,
    /// Theta limit
    pub theta_limit: Option<f64>,
    /// Total risk limit
    pub total_risk_limit: Option<f64>,
    /// Valid from time
    pub valid_from: Option<DateTime<Utc>>,
    /// Valid until time
    pub valid_until: Option<DateTime<Utc>>,
    /// Trading session ID
    pub trading_session_id: Option<String>,
    /// Account
    pub account: Option<String>,
    /// Parties
    pub parties: Option<String>,
    /// Text
    pub text: Option<String>,
    /// Custom label
    pub deribit_label: Option<String>,
}

impl MMProtectionLimits {
    /// Create a new MM Protection Limits request
    pub fn new(
        mm_protection_req_id: String,
        mm_protection_action: MMProtectionAction,
        mm_protection_scope: MMProtectionScope,
    ) -> Self {
        Self {
            mm_protection_req_id,
            mm_protection_action,
            mm_protection_scope,
            symbol: None,
            underlying_symbol: None,
            instrument_group: None,
            max_position_limit: None,
            max_order_qty_limit: None,
            max_orders_limit: None,
            time_window_seconds: None,
            delta_limit: None,
            vega_limit: None,
            gamma_limit: None,
            theta_limit: None,
            total_risk_limit: None,
            valid_from: None,
            valid_until: None,
            trading_session_id: None,
            account: None,
            parties: None,
            text: None,
            deribit_label: None,
        }
    }

    /// Create limits for all instruments
    pub fn all_instruments(mm_protection_req_id: String) -> Self {
        Self::new(
            mm_protection_req_id,
            MMProtectionAction::SetLimits,
            MMProtectionScope::AllInstruments,
        )
    }

    /// Create limits for specific instrument
    pub fn for_instrument(mm_protection_req_id: String, symbol: String) -> Self {
        let mut limits = Self::new(
            mm_protection_req_id,
            MMProtectionAction::SetLimits,
            MMProtectionScope::SpecificInstrument,
        );
        limits.symbol = Some(symbol);
        limits
    }

    /// Create limits for instrument group
    pub fn for_group(mm_protection_req_id: String, instrument_group: String) -> Self {
        let mut limits = Self::new(
            mm_protection_req_id,
            MMProtectionAction::SetLimits,
            MMProtectionScope::InstrumentGroup,
        );
        limits.instrument_group = Some(instrument_group);
        limits
    }

    /// Create limits for underlying
    pub fn for_underlying(mm_protection_req_id: String, underlying_symbol: String) -> Self {
        let mut limits = Self::new(
            mm_protection_req_id,
            MMProtectionAction::SetLimits,
            MMProtectionScope::Underlying,
        );
        limits.underlying_symbol = Some(underlying_symbol);
        limits
    }

    /// Create a query request
    pub fn query(mm_protection_req_id: String, scope: MMProtectionScope) -> Self {
        Self::new(
            mm_protection_req_id,
            MMProtectionAction::QueryLimits,
            scope,
        )
    }

    /// Set position limit
    pub fn with_position_limit(mut self, limit: f64) -> Self {
        self.max_position_limit = Some(limit);
        self
    }

    /// Set order quantity limit
    pub fn with_order_qty_limit(mut self, limit: f64) -> Self {
        self.max_order_qty_limit = Some(limit);
        self
    }

    /// Set maximum orders limit
    pub fn with_max_orders_limit(mut self, limit: i32) -> Self {
        self.max_orders_limit = Some(limit);
        self
    }

    /// Set time window
    pub fn with_time_window(mut self, seconds: i32) -> Self {
        self.time_window_seconds = Some(seconds);
        self
    }

    /// Set Greeks limits
    pub fn with_greeks_limits(
        mut self,
        delta: Option<f64>,
        vega: Option<f64>,
        gamma: Option<f64>,
        theta: Option<f64>,
    ) -> Self {
        self.delta_limit = delta;
        self.vega_limit = vega;
        self.gamma_limit = gamma;
        self.theta_limit = theta;
        self
    }

    /// Set total risk limit
    pub fn with_total_risk_limit(mut self, limit: f64) -> Self {
        self.total_risk_limit = Some(limit);
        self
    }

    /// Set validity period
    pub fn with_validity_period(mut self, from: DateTime<Utc>, until: DateTime<Utc>) -> Self {
        self.valid_from = Some(from);
        self.valid_until = Some(until);
        self
    }

    /// Set account
    pub fn with_account(mut self, account: String) -> Self {
        self.account = Some(account);
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

    /// Convert to FIX message
    pub fn to_fix_message(
        &self,
        sender_comp_id: &str,
        target_comp_id: &str,
        msg_seq_num: u32,
    ) -> DeribitFixResult<String> {
        let mut builder = MessageBuilder::new()
            .msg_type(MsgType::MmProtectionLimits)
            .sender_comp_id(sender_comp_id.to_string())
            .target_comp_id(target_comp_id.to_string())
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now());

        // Required fields
        builder = builder
            .field(9001, self.mm_protection_req_id.clone()) // MMProtectionReqID (custom tag)
            .field(9002, i32::from(self.mm_protection_action).to_string()) // MMProtectionAction (custom tag)
            .field(9003, i32::from(self.mm_protection_scope).to_string()); // MMProtectionScope (custom tag)

        // Optional fields based on scope
        if let Some(symbol) = &self.symbol {
            builder = builder.field(55, symbol.clone());
        }

        if let Some(underlying_symbol) = &self.underlying_symbol {
            builder = builder.field(311, underlying_symbol.clone());
        }

        if let Some(instrument_group) = &self.instrument_group {
            builder = builder.field(9004, instrument_group.clone()); // Custom tag for instrument group
        }

        // Risk limits
        if let Some(max_position_limit) = &self.max_position_limit {
            builder = builder.field(9005, max_position_limit.to_string());
        }

        if let Some(max_order_qty_limit) = &self.max_order_qty_limit {
            builder = builder.field(9006, max_order_qty_limit.to_string());
        }

        if let Some(max_orders_limit) = &self.max_orders_limit {
            builder = builder.field(9007, max_orders_limit.to_string());
        }

        if let Some(time_window_seconds) = &self.time_window_seconds {
            builder = builder.field(9009, time_window_seconds.to_string());
        }

        // Greeks limits
        if let Some(delta_limit) = &self.delta_limit {
            builder = builder.field(9010, delta_limit.to_string());
        }

        if let Some(vega_limit) = &self.vega_limit {
            builder = builder.field(9011, vega_limit.to_string());
        }

        if let Some(gamma_limit) = &self.gamma_limit {
            builder = builder.field(9012, gamma_limit.to_string());
        }

        if let Some(theta_limit) = &self.theta_limit {
            builder = builder.field(9013, theta_limit.to_string());
        }

        if let Some(total_risk_limit) = &self.total_risk_limit {
            builder = builder.field(9014, total_risk_limit.to_string());
        }

        // Validity period
        if let Some(valid_from) = &self.valid_from {
            builder = builder.field(
                9015,
                valid_from.format("%Y%m%d-%H:%M:%S%.3f").to_string(),
            );
        }

        if let Some(valid_until) = &self.valid_until {
            builder = builder.field(
                9016,
                valid_until.format("%Y%m%d-%H:%M:%S%.3f").to_string(),
            );
        }

        // Standard optional fields
        if let Some(trading_session_id) = &self.trading_session_id {
            builder = builder.field(336, trading_session_id.clone());
        }

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

impl_json_display!(MMProtectionLimits);
impl_json_debug_pretty!(MMProtectionLimits);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mm_protection_limits_creation() {
        let limits = MMProtectionLimits::new(
            "MMP123".to_string(),
            MMProtectionAction::SetLimits,
            MMProtectionScope::AllInstruments,
        );

        assert_eq!(limits.mm_protection_req_id, "MMP123");
        assert_eq!(limits.mm_protection_action, MMProtectionAction::SetLimits);
        assert_eq!(limits.mm_protection_scope, MMProtectionScope::AllInstruments);
        assert!(limits.symbol.is_none());
        assert!(limits.max_position_limit.is_none());
    }

    #[test]
    fn test_mm_protection_limits_all_instruments() {
        let limits = MMProtectionLimits::all_instruments("MMP456".to_string());

        assert_eq!(limits.mm_protection_scope, MMProtectionScope::AllInstruments);
        assert_eq!(limits.mm_protection_action, MMProtectionAction::SetLimits);
    }

    #[test]
    fn test_mm_protection_limits_for_instrument() {
        let limits = MMProtectionLimits::for_instrument(
            "MMP789".to_string(),
            "BTC-PERPETUAL".to_string(),
        );

        assert_eq!(limits.mm_protection_scope, MMProtectionScope::SpecificInstrument);
        assert_eq!(limits.symbol, Some("BTC-PERPETUAL".to_string()));
    }

    #[test]
    fn test_mm_protection_limits_for_group() {
        let limits = MMProtectionLimits::for_group(
            "MMP999".to_string(),
            "CRYPTO_PERPS".to_string(),
        );

        assert_eq!(limits.mm_protection_scope, MMProtectionScope::InstrumentGroup);
        assert_eq!(limits.instrument_group, Some("CRYPTO_PERPS".to_string()));
    }

    #[test]
    fn test_mm_protection_limits_for_underlying() {
        let limits = MMProtectionLimits::for_underlying(
            "MMP111".to_string(),
            "BTC".to_string(),
        );

        assert_eq!(limits.mm_protection_scope, MMProtectionScope::Underlying);
        assert_eq!(limits.underlying_symbol, Some("BTC".to_string()));
    }

    #[test]
    fn test_mm_protection_limits_query() {
        let limits = MMProtectionLimits::query(
            "MMP222".to_string(),
            MMProtectionScope::AllInstruments,
        );

        assert_eq!(limits.mm_protection_action, MMProtectionAction::QueryLimits);
        assert_eq!(limits.mm_protection_scope, MMProtectionScope::AllInstruments);
    }

    #[test]
    fn test_mm_protection_limits_with_options() {
        let valid_from = Utc::now();
        let valid_until = valid_from + chrono::Duration::hours(24);

        let limits = MMProtectionLimits::for_instrument(
            "MMP333".to_string(),
            "ETH-PERPETUAL".to_string(),
        )
        .with_position_limit(1000.0)
        .with_order_qty_limit(100.0)
        .with_max_orders_limit(50)
        .with_time_window(300)
        .with_greeks_limits(Some(10.0), Some(5.0), Some(2.0), Some(-1.0))
        .with_total_risk_limit(10000.0)
        .with_validity_period(valid_from, valid_until)
        .with_account("ACC123".to_string())
        .with_trading_session_id("SESSION1".to_string())
        .with_text("MM protection limits for ETH".to_string())
        .with_label("test-mm-protection".to_string());

        assert_eq!(limits.max_position_limit, Some(1000.0));
        assert_eq!(limits.max_order_qty_limit, Some(100.0));
        assert_eq!(limits.max_orders_limit, Some(50));
        assert_eq!(limits.time_window_seconds, Some(300));
        assert_eq!(limits.delta_limit, Some(10.0));
        assert_eq!(limits.vega_limit, Some(5.0));
        assert_eq!(limits.gamma_limit, Some(2.0));
        assert_eq!(limits.theta_limit, Some(-1.0));
        assert_eq!(limits.total_risk_limit, Some(10000.0));
        assert_eq!(limits.valid_from, Some(valid_from));
        assert_eq!(limits.valid_until, Some(valid_until));
        assert_eq!(limits.account, Some("ACC123".to_string()));
        assert_eq!(limits.trading_session_id, Some("SESSION1".to_string()));
        assert_eq!(limits.text, Some("MM protection limits for ETH".to_string()));
        assert_eq!(limits.deribit_label, Some("test-mm-protection".to_string()));
    }

    #[test]
    fn test_mm_protection_limits_to_fix_message() {
        let limits = MMProtectionLimits::for_instrument(
            "MMP123".to_string(),
            "BTC-PERPETUAL".to_string(),
        )
        .with_position_limit(500.0)
        .with_order_qty_limit(50.0)
        .with_label("test-label".to_string());

        let fix_message = limits.to_fix_message("SENDER", "TARGET", 1).unwrap();

        // Check that the message contains required fields
        assert!(fix_message.contains("35=MM")); // MsgType
        assert!(fix_message.contains("9001=MMP123")); // MMProtectionReqID
        assert!(fix_message.contains("9002=1")); // MMProtectionAction=SetLimits
        assert!(fix_message.contains("9003=2")); // MMProtectionScope=SpecificInstrument
        assert!(fix_message.contains("55=BTC-PERPETUAL")); // Symbol
        assert!(fix_message.contains("9005=500")); // MaxPositionLimit
        assert!(fix_message.contains("9006=50")); // MaxOrderQtyLimit
        assert!(fix_message.contains("100010=test-label")); // Custom label
    }

    #[test]
    fn test_mm_protection_action_conversions() {
        assert_eq!(i32::from(MMProtectionAction::SetLimits), 1);
        assert_eq!(i32::from(MMProtectionAction::UpdateLimits), 2);
        assert_eq!(i32::from(MMProtectionAction::QueryLimits), 3);
        assert_eq!(i32::from(MMProtectionAction::RemoveLimits), 4);

        assert_eq!(MMProtectionAction::try_from(1).unwrap(), MMProtectionAction::SetLimits);
        assert_eq!(MMProtectionAction::try_from(2).unwrap(), MMProtectionAction::UpdateLimits);
        assert_eq!(MMProtectionAction::try_from(3).unwrap(), MMProtectionAction::QueryLimits);
        assert_eq!(MMProtectionAction::try_from(4).unwrap(), MMProtectionAction::RemoveLimits);

        assert!(MMProtectionAction::try_from(99).is_err());
    }

    #[test]
    fn test_mm_protection_scope_conversions() {
        assert_eq!(i32::from(MMProtectionScope::AllInstruments), 1);
        assert_eq!(i32::from(MMProtectionScope::SpecificInstrument), 2);
        assert_eq!(i32::from(MMProtectionScope::InstrumentGroup), 3);
        assert_eq!(i32::from(MMProtectionScope::Underlying), 4);

        assert_eq!(MMProtectionScope::try_from(1).unwrap(), MMProtectionScope::AllInstruments);
        assert_eq!(MMProtectionScope::try_from(2).unwrap(), MMProtectionScope::SpecificInstrument);
        assert_eq!(MMProtectionScope::try_from(3).unwrap(), MMProtectionScope::InstrumentGroup);
        assert_eq!(MMProtectionScope::try_from(4).unwrap(), MMProtectionScope::Underlying);

        assert!(MMProtectionScope::try_from(99).is_err());
    }
}