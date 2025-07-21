/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/
use crate::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

/// Instrument kind for Deribit
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstrumentKind {
    Future,
    Option,
    Spot,
    FutureCombo,
    OptionCombo,
}

impl InstrumentKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            InstrumentKind::Future => "future",
            InstrumentKind::Option => "option",
            InstrumentKind::Spot => "spot",
            InstrumentKind::FutureCombo => "future_combo",
            InstrumentKind::OptionCombo => "option_combo",
        }
    }
}

impl_json_debug_pretty!(InstrumentKind);
impl_json_display!(InstrumentKind);
