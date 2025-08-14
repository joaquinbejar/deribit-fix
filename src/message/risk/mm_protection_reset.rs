/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 12/8/25
******************************************************************************/

//! MM Protection Reset FIX Message Implementation

use crate::error::Result as DeribitFixResult;
use crate::message::builder::MessageBuilder;
use crate::model::types::MsgType;
use chrono::{DateTime, Utc};
use deribit_base::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

// Re-export from mm_protection_limits module
pub use super::mm_protection_limits::MMProtectionScope;

/// MM Protection reset type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MMProtectionResetType {
    /// Reset all limits
    ResetAllLimits,
    /// Reset position limits only
    ResetPositionLimits,
    /// Reset order limits only
    ResetOrderLimits,
    /// Reset Greeks limits only
    ResetGreeksLimits,
    /// Reset risk limits only
    ResetRiskLimits,
    /// Reset time-based limits only
    ResetTimeBasedLimits,
    /// Soft reset (keep limits but reset counters)
    SoftReset,
    /// Hard reset (remove all limits and counters)
    HardReset,
}

impl From<MMProtectionResetType> for i32 {
    fn from(reset_type: MMProtectionResetType) -> Self {
        match reset_type {
            MMProtectionResetType::ResetAllLimits => 1,
            MMProtectionResetType::ResetPositionLimits => 2,
            MMProtectionResetType::ResetOrderLimits => 3,
            MMProtectionResetType::ResetGreeksLimits => 4,
            MMProtectionResetType::ResetRiskLimits => 5,
            MMProtectionResetType::ResetTimeBasedLimits => 6,
            MMProtectionResetType::SoftReset => 7,
            MMProtectionResetType::HardReset => 8,
        }
    }
}

impl TryFrom<i32> for MMProtectionResetType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(MMProtectionResetType::ResetAllLimits),
            2 => Ok(MMProtectionResetType::ResetPositionLimits),
            3 => Ok(MMProtectionResetType::ResetOrderLimits),
            4 => Ok(MMProtectionResetType::ResetGreeksLimits),
            5 => Ok(MMProtectionResetType::ResetRiskLimits),
            6 => Ok(MMProtectionResetType::ResetTimeBasedLimits),
            7 => Ok(MMProtectionResetType::SoftReset),
            8 => Ok(MMProtectionResetType::HardReset),
            _ => Err(format!("Invalid MMProtectionResetType: {}", value)),
        }
    }
}

/// MM Protection reset reason enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MMProtectionResetReason {
    /// Manual reset requested
    ManualResetRequested,
    /// Automatic reset by system
    AutomaticResetBySystem,
    /// End of trading session
    EndOfTradingSession,
    /// Start of new trading session
    StartOfNewTradingSession,
    /// Risk limit breach resolved
    RiskLimitBreachResolved,
    /// System maintenance
    SystemMaintenance,
    /// Circuit breaker deactivated
    CircuitBreakerDeactivated,
    /// Emergency reset
    EmergencyReset,
    /// Scheduled reset
    ScheduledReset,
    /// Configuration change
    ConfigurationChange,
    /// Other
    Other,
}

impl From<MMProtectionResetReason> for i32 {
    fn from(reason: MMProtectionResetReason) -> Self {
        match reason {
            MMProtectionResetReason::ManualResetRequested => 1,
            MMProtectionResetReason::AutomaticResetBySystem => 2,
            MMProtectionResetReason::EndOfTradingSession => 3,
            MMProtectionResetReason::StartOfNewTradingSession => 4,
            MMProtectionResetReason::RiskLimitBreachResolved => 5,
            MMProtectionResetReason::SystemMaintenance => 6,
            MMProtectionResetReason::CircuitBreakerDeactivated => 7,
            MMProtectionResetReason::EmergencyReset => 8,
            MMProtectionResetReason::ScheduledReset => 9,
            MMProtectionResetReason::ConfigurationChange => 10,
            MMProtectionResetReason::Other => 99,
        }
    }
}

impl TryFrom<i32> for MMProtectionResetReason {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(MMProtectionResetReason::ManualResetRequested),
            2 => Ok(MMProtectionResetReason::AutomaticResetBySystem),
            3 => Ok(MMProtectionResetReason::EndOfTradingSession),
            4 => Ok(MMProtectionResetReason::StartOfNewTradingSession),
            5 => Ok(MMProtectionResetReason::RiskLimitBreachResolved),
            6 => Ok(MMProtectionResetReason::SystemMaintenance),
            7 => Ok(MMProtectionResetReason::CircuitBreakerDeactivated),
            8 => Ok(MMProtectionResetReason::EmergencyReset),
            9 => Ok(MMProtectionResetReason::ScheduledReset),
            10 => Ok(MMProtectionResetReason::ConfigurationChange),
            99 => Ok(MMProtectionResetReason::Other),
            _ => Err(format!("Invalid MMProtectionResetReason: {}", value)),
        }
    }
}

/// MM Protection Reset message (MsgType = 'MZ')
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct MMProtectionReset {
    /// MM Protection reset request ID
    pub mm_protection_reset_req_id: String,
    /// MM Protection reset type
    pub mm_protection_reset_type: MMProtectionResetType,
    /// MM Protection reset reason
    pub mm_protection_reset_reason: MMProtectionResetReason,
    /// MM Protection scope
    pub mm_protection_scope: MMProtectionScope,
    /// Instrument symbol (if applicable)
    pub symbol: Option<String>,
    /// Underlying symbol (if applicable)
    pub underlying_symbol: Option<String>,
    /// Instrument group (if applicable)
    pub instrument_group: Option<String>,
    /// Reset effective time
    pub reset_effective_time: Option<DateTime<Utc>>,
    /// Reset expiry time
    pub reset_expiry_time: Option<DateTime<Utc>>,
    /// Force reset flag
    pub force_reset: Option<bool>,
    /// Notify all participants flag
    pub notify_all_participants: Option<bool>,
    /// Reset position counters
    pub reset_position_counters: Option<bool>,
    /// Reset order counters
    pub reset_order_counters: Option<bool>,
    /// Reset volume counters
    pub reset_volume_counters: Option<bool>,
    /// Reset time window counters
    pub reset_time_window_counters: Option<bool>,
    /// Reset Greeks counters
    pub reset_greeks_counters: Option<bool>,
    /// Reset risk counters
    pub reset_risk_counters: Option<bool>,
    /// Account
    pub account: Option<String>,
    /// Parties
    pub parties: Option<String>,
    /// Trading session ID
    pub trading_session_id: Option<String>,
    /// Trading session sub ID
    pub trading_session_sub_id: Option<String>,
    /// Text
    pub text: Option<String>,
    /// Custom label
    pub deribit_label: Option<String>,
}

impl MMProtectionReset {
    /// Create a new MM Protection Reset
    pub fn new(
        mm_protection_reset_req_id: String,
        mm_protection_reset_type: MMProtectionResetType,
        mm_protection_reset_reason: MMProtectionResetReason,
        mm_protection_scope: MMProtectionScope,
    ) -> Self {
        Self {
            mm_protection_reset_req_id,
            mm_protection_reset_type,
            mm_protection_reset_reason,
            mm_protection_scope,
            symbol: None,
            underlying_symbol: None,
            instrument_group: None,
            reset_effective_time: None,
            reset_expiry_time: None,
            force_reset: None,
            notify_all_participants: None,
            reset_position_counters: None,
            reset_order_counters: None,
            reset_volume_counters: None,
            reset_time_window_counters: None,
            reset_greeks_counters: None,
            reset_risk_counters: None,
            account: None,
            parties: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            text: None,
            deribit_label: None,
        }
    }

    /// Create a manual reset for all limits
    pub fn manual_reset_all(mm_protection_reset_req_id: String) -> Self {
        Self::new(
            mm_protection_reset_req_id,
            MMProtectionResetType::ResetAllLimits,
            MMProtectionResetReason::ManualResetRequested,
            MMProtectionScope::AllInstruments,
        )
    }

    /// Create a soft reset (reset counters but keep limits)
    pub fn soft_reset(
        mm_protection_reset_req_id: String,
        mm_protection_scope: MMProtectionScope,
    ) -> Self {
        Self::new(
            mm_protection_reset_req_id,
            MMProtectionResetType::SoftReset,
            MMProtectionResetReason::ManualResetRequested,
            mm_protection_scope,
        )
    }

    /// Create a hard reset (remove all limits and counters)
    pub fn hard_reset(
        mm_protection_reset_req_id: String,
        mm_protection_scope: MMProtectionScope,
    ) -> Self {
        Self::new(
            mm_protection_reset_req_id,
            MMProtectionResetType::HardReset,
            MMProtectionResetReason::ManualResetRequested,
            mm_protection_scope,
        )
    }

    /// Create an emergency reset
    pub fn emergency_reset(mm_protection_reset_req_id: String, reason_text: String) -> Self {
        let mut reset = Self::new(
            mm_protection_reset_req_id,
            MMProtectionResetType::HardReset,
            MMProtectionResetReason::EmergencyReset,
            MMProtectionScope::AllInstruments,
        );
        reset.force_reset = Some(true);
        reset.notify_all_participants = Some(true);
        reset.text = Some(reason_text);
        reset
    }

    /// Create a scheduled reset
    pub fn scheduled_reset(
        mm_protection_reset_req_id: String,
        reset_type: MMProtectionResetType,
        effective_time: DateTime<Utc>,
        expiry_time: Option<DateTime<Utc>>,
    ) -> Self {
        let mut reset = Self::new(
            mm_protection_reset_req_id,
            reset_type,
            MMProtectionResetReason::ScheduledReset,
            MMProtectionScope::AllInstruments,
        );
        reset.reset_effective_time = Some(effective_time);
        reset.reset_expiry_time = expiry_time;
        reset
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

    /// Set reset effective time
    pub fn with_reset_effective_time(mut self, effective_time: DateTime<Utc>) -> Self {
        self.reset_effective_time = Some(effective_time);
        self
    }

    /// Set reset expiry time
    pub fn with_reset_expiry_time(mut self, expiry_time: DateTime<Utc>) -> Self {
        self.reset_expiry_time = Some(expiry_time);
        self
    }

    /// Set force reset flag
    pub fn with_force_reset(mut self, force: bool) -> Self {
        self.force_reset = Some(force);
        self
    }

    /// Set notify all participants flag
    pub fn with_notify_all_participants(mut self, notify: bool) -> Self {
        self.notify_all_participants = Some(notify);
        self
    }

    /// Set counter reset flags
    pub fn with_counter_resets(
        mut self,
        position: bool,
        order: bool,
        volume: bool,
        time_window: bool,
        greeks: bool,
        risk: bool,
    ) -> Self {
        self.reset_position_counters = Some(position);
        self.reset_order_counters = Some(order);
        self.reset_volume_counters = Some(volume);
        self.reset_time_window_counters = Some(time_window);
        self.reset_greeks_counters = Some(greeks);
        self.reset_risk_counters = Some(risk);
        self
    }

    /// Set account
    pub fn with_account(mut self, account: String) -> Self {
        self.account = Some(account);
        self
    }

    /// Set trading session
    pub fn with_trading_session(
        mut self,
        session_id: String,
        session_sub_id: Option<String>,
    ) -> Self {
        self.trading_session_id = Some(session_id);
        self.trading_session_sub_id = session_sub_id;
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
            .msg_type(MsgType::MmProtectionReset)
            .sender_comp_id(sender_comp_id.to_string())
            .target_comp_id(target_comp_id.to_string())
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now());

        // Required fields
        builder = builder
            .field(9032, self.mm_protection_reset_req_id.clone()) // MMProtectionResetReqID
            .field(9033, i32::from(self.mm_protection_reset_type).to_string()) // MMProtectionResetType
            .field(9034, i32::from(self.mm_protection_reset_reason).to_string()) // MMProtectionResetReason
            .field(9003, i32::from(self.mm_protection_scope).to_string()); // MMProtectionScope

        // Optional fields
        if let Some(symbol) = &self.symbol {
            builder = builder.field(55, symbol.clone());
        }

        if let Some(underlying_symbol) = &self.underlying_symbol {
            builder = builder.field(311, underlying_symbol.clone());
        }

        if let Some(instrument_group) = &self.instrument_group {
            builder = builder.field(9004, instrument_group.clone());
        }

        if let Some(reset_effective_time) = &self.reset_effective_time {
            builder = builder.field(
                9035,
                reset_effective_time
                    .format("%Y%m%d-%H:%M:%S%.3f")
                    .to_string(),
            );
        }

        if let Some(reset_expiry_time) = &self.reset_expiry_time {
            builder = builder.field(
                9036,
                reset_expiry_time.format("%Y%m%d-%H:%M:%S%.3f").to_string(),
            );
        }

        // Boolean flags
        if let Some(force_reset) = &self.force_reset {
            builder = builder.field(9037, if *force_reset { "Y" } else { "N" }.to_string());
        }

        if let Some(notify_all_participants) = &self.notify_all_participants {
            builder = builder.field(
                9038,
                if *notify_all_participants { "Y" } else { "N" }.to_string(),
            );
        }

        // Counter reset flags
        if let Some(reset_position_counters) = &self.reset_position_counters {
            builder = builder.field(
                9039,
                if *reset_position_counters { "Y" } else { "N" }.to_string(),
            );
        }

        if let Some(reset_order_counters) = &self.reset_order_counters {
            builder = builder.field(
                9040,
                if *reset_order_counters { "Y" } else { "N" }.to_string(),
            );
        }

        if let Some(reset_volume_counters) = &self.reset_volume_counters {
            builder = builder.field(
                9041,
                if *reset_volume_counters { "Y" } else { "N" }.to_string(),
            );
        }

        if let Some(reset_time_window_counters) = &self.reset_time_window_counters {
            builder = builder.field(
                9042,
                if *reset_time_window_counters {
                    "Y"
                } else {
                    "N"
                }
                .to_string(),
            );
        }

        if let Some(reset_greeks_counters) = &self.reset_greeks_counters {
            builder = builder.field(
                9043,
                if *reset_greeks_counters { "Y" } else { "N" }.to_string(),
            );
        }

        if let Some(reset_risk_counters) = &self.reset_risk_counters {
            builder = builder.field(
                9044,
                if *reset_risk_counters { "Y" } else { "N" }.to_string(),
            );
        }

        // Standard optional fields
        if let Some(account) = &self.account {
            builder = builder.field(1, account.clone());
        }

        if let Some(parties) = &self.parties {
            builder = builder.field(453, parties.clone());
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

        Ok(builder.build()?.to_string())
    }
}

impl_json_display!(MMProtectionReset);
impl_json_debug_pretty!(MMProtectionReset);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mm_protection_reset_creation() {
        let reset = MMProtectionReset::new(
            "MMPR123".to_string(),
            MMProtectionResetType::ResetAllLimits,
            MMProtectionResetReason::ManualResetRequested,
            MMProtectionScope::AllInstruments,
        );

        assert_eq!(reset.mm_protection_reset_req_id, "MMPR123");
        assert_eq!(
            reset.mm_protection_reset_type,
            MMProtectionResetType::ResetAllLimits
        );
        assert_eq!(
            reset.mm_protection_reset_reason,
            MMProtectionResetReason::ManualResetRequested
        );
        assert_eq!(reset.mm_protection_scope, MMProtectionScope::AllInstruments);
        assert!(reset.force_reset.is_none());
    }

    #[test]
    fn test_mm_protection_reset_manual_all() {
        let reset = MMProtectionReset::manual_reset_all("MMPR456".to_string());

        assert_eq!(
            reset.mm_protection_reset_type,
            MMProtectionResetType::ResetAllLimits
        );
        assert_eq!(
            reset.mm_protection_reset_reason,
            MMProtectionResetReason::ManualResetRequested
        );
        assert_eq!(reset.mm_protection_scope, MMProtectionScope::AllInstruments);
    }

    #[test]
    fn test_mm_protection_reset_soft() {
        let reset = MMProtectionReset::soft_reset(
            "MMPR789".to_string(),
            MMProtectionScope::SpecificInstrument,
        );

        assert_eq!(
            reset.mm_protection_reset_type,
            MMProtectionResetType::SoftReset
        );
        assert_eq!(
            reset.mm_protection_scope,
            MMProtectionScope::SpecificInstrument
        );
    }

    #[test]
    fn test_mm_protection_reset_hard() {
        let reset = MMProtectionReset::hard_reset(
            "MMPR999".to_string(),
            MMProtectionScope::InstrumentGroup,
        );

        assert_eq!(
            reset.mm_protection_reset_type,
            MMProtectionResetType::HardReset
        );
        assert_eq!(
            reset.mm_protection_scope,
            MMProtectionScope::InstrumentGroup
        );
    }

    #[test]
    fn test_mm_protection_reset_emergency() {
        let reset = MMProtectionReset::emergency_reset(
            "MMPR111".to_string(),
            "System malfunction detected".to_string(),
        );

        assert_eq!(
            reset.mm_protection_reset_type,
            MMProtectionResetType::HardReset
        );
        assert_eq!(
            reset.mm_protection_reset_reason,
            MMProtectionResetReason::EmergencyReset
        );
        assert_eq!(reset.force_reset, Some(true));
        assert_eq!(reset.notify_all_participants, Some(true));
        assert_eq!(reset.text, Some("System malfunction detected".to_string()));
    }

    #[test]
    fn test_mm_protection_reset_scheduled() {
        let effective_time = Utc::now() + chrono::Duration::hours(1);
        let expiry_time = effective_time + chrono::Duration::hours(24);

        let reset = MMProtectionReset::scheduled_reset(
            "MMPR222".to_string(),
            MMProtectionResetType::ResetPositionLimits,
            effective_time,
            Some(expiry_time),
        );

        assert_eq!(
            reset.mm_protection_reset_type,
            MMProtectionResetType::ResetPositionLimits
        );
        assert_eq!(
            reset.mm_protection_reset_reason,
            MMProtectionResetReason::ScheduledReset
        );
        assert_eq!(reset.reset_effective_time, Some(effective_time));
        assert_eq!(reset.reset_expiry_time, Some(expiry_time));
    }

    #[test]
    fn test_mm_protection_reset_with_options() {
        let effective_time = Utc::now();
        let expiry_time = effective_time + chrono::Duration::hours(8);

        let reset = MMProtectionReset::new(
            "MMPR333".to_string(),
            MMProtectionResetType::SoftReset,
            MMProtectionResetReason::ConfigurationChange,
            MMProtectionScope::SpecificInstrument,
        )
        .with_symbol("BTC-PERPETUAL".to_string())
        .with_reset_effective_time(effective_time)
        .with_reset_expiry_time(expiry_time)
        .with_force_reset(false)
        .with_notify_all_participants(true)
        .with_counter_resets(true, true, false, true, false, true)
        .with_account("ACC123".to_string())
        .with_trading_session("SESSION1".to_string(), Some("SUB1".to_string()))
        .with_text("Configuration update reset".to_string())
        .with_label("test-reset".to_string());

        assert_eq!(reset.symbol, Some("BTC-PERPETUAL".to_string()));
        assert_eq!(reset.reset_effective_time, Some(effective_time));
        assert_eq!(reset.reset_expiry_time, Some(expiry_time));
        assert_eq!(reset.force_reset, Some(false));
        assert_eq!(reset.notify_all_participants, Some(true));
        assert_eq!(reset.reset_position_counters, Some(true));
        assert_eq!(reset.reset_order_counters, Some(true));
        assert_eq!(reset.reset_volume_counters, Some(false));
        assert_eq!(reset.reset_time_window_counters, Some(true));
        assert_eq!(reset.reset_greeks_counters, Some(false));
        assert_eq!(reset.reset_risk_counters, Some(true));
        assert_eq!(reset.account, Some("ACC123".to_string()));
        assert_eq!(reset.trading_session_id, Some("SESSION1".to_string()));
        assert_eq!(reset.trading_session_sub_id, Some("SUB1".to_string()));
        assert_eq!(reset.text, Some("Configuration update reset".to_string()));
        assert_eq!(reset.deribit_label, Some("test-reset".to_string()));
    }

    #[test]
    fn test_mm_protection_reset_to_fix_message() {
        let reset = MMProtectionReset::manual_reset_all("MMPR123".to_string())
            .with_symbol("BTC-PERPETUAL".to_string())
            .with_force_reset(true)
            .with_counter_resets(true, true, true, true, true, true)
            .with_label("test-label".to_string());

        let fix_message = reset.to_fix_message("SENDER", "TARGET", 1).unwrap();

        // Check that the message contains required fields
        assert!(fix_message.contains("35=MZ")); // MsgType
        assert!(fix_message.contains("9032=MMPR123")); // MMProtectionResetReqID
        assert!(fix_message.contains("9033=1")); // MMProtectionResetType=ResetAllLimits
        assert!(fix_message.contains("9034=1")); // MMProtectionResetReason=ManualResetRequested
        assert!(fix_message.contains("9003=1")); // MMProtectionScope=AllInstruments
        assert!(fix_message.contains("55=BTC-PERPETUAL")); // Symbol
        assert!(fix_message.contains("9037=Y")); // ForceReset=Y
        assert!(fix_message.contains("9039=Y")); // ResetPositionCounters=Y
        assert!(fix_message.contains("9040=Y")); // ResetOrderCounters=Y
        assert!(fix_message.contains("100010=test-label")); // Custom label
    }

    #[test]
    fn test_mm_protection_reset_type_conversions() {
        assert_eq!(i32::from(MMProtectionResetType::ResetAllLimits), 1);
        assert_eq!(i32::from(MMProtectionResetType::ResetPositionLimits), 2);
        assert_eq!(i32::from(MMProtectionResetType::SoftReset), 7);
        assert_eq!(i32::from(MMProtectionResetType::HardReset), 8);

        assert_eq!(
            MMProtectionResetType::try_from(1).unwrap(),
            MMProtectionResetType::ResetAllLimits
        );
        assert_eq!(
            MMProtectionResetType::try_from(2).unwrap(),
            MMProtectionResetType::ResetPositionLimits
        );
        assert_eq!(
            MMProtectionResetType::try_from(7).unwrap(),
            MMProtectionResetType::SoftReset
        );
        assert_eq!(
            MMProtectionResetType::try_from(8).unwrap(),
            MMProtectionResetType::HardReset
        );

        assert!(MMProtectionResetType::try_from(99).is_err());
    }

    #[test]
    fn test_mm_protection_reset_reason_conversions() {
        assert_eq!(i32::from(MMProtectionResetReason::ManualResetRequested), 1);
        assert_eq!(
            i32::from(MMProtectionResetReason::AutomaticResetBySystem),
            2
        );
        assert_eq!(i32::from(MMProtectionResetReason::EmergencyReset), 8);
        assert_eq!(i32::from(MMProtectionResetReason::Other), 99);

        assert_eq!(
            MMProtectionResetReason::try_from(1).unwrap(),
            MMProtectionResetReason::ManualResetRequested
        );
        assert_eq!(
            MMProtectionResetReason::try_from(2).unwrap(),
            MMProtectionResetReason::AutomaticResetBySystem
        );
        assert_eq!(
            MMProtectionResetReason::try_from(8).unwrap(),
            MMProtectionResetReason::EmergencyReset
        );
        assert_eq!(
            MMProtectionResetReason::try_from(99).unwrap(),
            MMProtectionResetReason::Other
        );

        assert!(MMProtectionResetReason::try_from(50).is_err());
    }
}
