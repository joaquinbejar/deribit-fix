/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/
use serde::{Deserialize, Serialize};

/// Currency enumeration for Deribit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Currency {
    BTC,
    ETH,
    USD,
    USDC,
}

impl Currency {
    pub fn as_str(&self) -> &'static str {
        match self {
            Currency::BTC => "BTC",
            Currency::ETH => "ETH",
            Currency::USD => "USD",
            Currency::USDC => "USDC",
        }
    }
}
