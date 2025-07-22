/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 22/7/25
******************************************************************************/

//! Order Cancel Request FIX Message Implementation

use crate::error::{DeribitFixError, Result as DeribitFixResult};
use crate::{message::builder::MessageBuilder, model::types::MsgType};
use chrono::Utc;
use deribit_base::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

/// Order Cancel Request message (MsgType = 'F')
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderCancelRequest {
    /// Original order identifier assigned by the user (optional)
    pub cl_ord_id: Option<String>,
    /// Order identifier assigned by Deribit (optional)
    pub orig_cl_ord_id: Option<String>,
    /// Custom label for order (optional)
    pub deribit_label: Option<String>,
    /// Instrument symbol (required if OrigClOrdId is absent)
    pub symbol: Option<String>,
    /// Currency to speed up search
    pub currency: Option<String>,
}

impl OrderCancelRequest {
    /// Create cancel request by original client order ID
    pub fn by_orig_cl_ord_id(orig_cl_ord_id: String) -> Self {
        Self {
            cl_ord_id: None,
            orig_cl_ord_id: Some(orig_cl_ord_id),
            deribit_label: None,
            symbol: None,
            currency: None,
        }
    }

    /// Create cancel request by client order ID
    pub fn by_cl_ord_id(cl_ord_id: String, symbol: String) -> Self {
        Self {
            cl_ord_id: Some(cl_ord_id),
            orig_cl_ord_id: None,
            deribit_label: None,
            symbol: Some(symbol),
            currency: None,
        }
    }

    /// Create cancel request by Deribit label
    pub fn by_deribit_label(deribit_label: String, symbol: String) -> Self {
        Self {
            cl_ord_id: None,
            orig_cl_ord_id: None,
            deribit_label: Some(deribit_label),
            symbol: Some(symbol),
            currency: None,
        }
    }

    /// Set currency for faster search
    pub fn with_currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
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
            .msg_type(MsgType::OrderCancelRequest)
            .sender_comp_id(sender_comp_id.to_string())
            .target_comp_id(target_comp_id.to_string())
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now());

        // At least one identifier must be present
        if self.cl_ord_id.is_none() && self.orig_cl_ord_id.is_none() && self.deribit_label.is_none()
        {
            return Err(DeribitFixError::Generic(
                "Either OrigClOrdId or ClOrdId must be specified".to_string(),
            ));
        }

        if let Some(cl_ord_id) = &self.cl_ord_id {
            builder = builder.field(11, cl_ord_id.clone());
        }

        if let Some(orig_cl_ord_id) = &self.orig_cl_ord_id {
            builder = builder.field(41, orig_cl_ord_id.clone());
        }

        if let Some(deribit_label) = &self.deribit_label {
            builder = builder.field(100010, deribit_label.clone());
        }

        if let Some(symbol) = &self.symbol {
            builder = builder.field(55, symbol.clone());
        }

        if let Some(currency) = &self.currency {
            builder = builder.field(15, currency.clone());
        }

        Ok(builder.build()?.to_string())
    }
}

impl_json_display!(OrderCancelRequest);
impl_json_debug_pretty!(OrderCancelRequest);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_cancel_request_by_orig_cl_ord_id() {
        let cancel_request = OrderCancelRequest::by_orig_cl_ord_id("ORIG123".to_string());

        assert_eq!(cancel_request.orig_cl_ord_id, Some("ORIG123".to_string()));
        assert_eq!(cancel_request.cl_ord_id, None);
        assert_eq!(cancel_request.deribit_label, None);
        assert_eq!(cancel_request.symbol, None);
    }

    #[test]
    fn test_order_cancel_request_by_cl_ord_id() {
        let cancel_request =
            OrderCancelRequest::by_cl_ord_id("ORDER123".to_string(), "BTC-PERPETUAL".to_string());

        assert_eq!(cancel_request.cl_ord_id, Some("ORDER123".to_string()));
        assert_eq!(cancel_request.symbol, Some("BTC-PERPETUAL".to_string()));
        assert_eq!(cancel_request.orig_cl_ord_id, None);
        assert_eq!(cancel_request.deribit_label, None);
    }

    #[test]
    fn test_order_cancel_request_by_deribit_label() {
        let cancel_request = OrderCancelRequest::by_deribit_label(
            "my-order".to_string(),
            "BTC-PERPETUAL".to_string(),
        );

        assert_eq!(cancel_request.deribit_label, Some("my-order".to_string()));
        assert_eq!(cancel_request.symbol, Some("BTC-PERPETUAL".to_string()));
        assert_eq!(cancel_request.cl_ord_id, None);
        assert_eq!(cancel_request.orig_cl_ord_id, None);
    }

    #[test]
    fn test_order_cancel_request_with_currency() {
        let cancel_request =
            OrderCancelRequest::by_cl_ord_id("ORDER123".to_string(), "BTC-PERPETUAL".to_string())
                .with_currency("BTC".to_string());

        assert_eq!(cancel_request.currency, Some("BTC".to_string()));
    }

    #[test]
    fn test_order_cancel_request_to_fix_message() {
        let cancel_request = OrderCancelRequest::by_orig_cl_ord_id("ORIG123".to_string());

        let fix_message = cancel_request.to_fix_message("CLIENT", "DERIBITSERVER", 1);
        assert!(fix_message.is_ok());

        let message = fix_message.unwrap();
        assert!(message.contains("35=F")); // MsgType = OrderCancelRequest
        assert!(message.contains("41=ORIG123")); // OrigClOrdId
    }

    #[test]
    fn test_order_cancel_request_validation_error() {
        let cancel_request = OrderCancelRequest {
            cl_ord_id: None,
            orig_cl_ord_id: None,
            deribit_label: None,
            symbol: None,
            currency: None,
        };

        let fix_message = cancel_request.to_fix_message("CLIENT", "DERIBITSERVER", 1);
        assert!(fix_message.is_err());
    }
}
