/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 22/7/25
******************************************************************************/

//! Order Cancel Reject FIX Message Implementation

use super::*;
use crate::error::Result as DeribitFixResult;
use crate::message::builder::MessageBuilder;
use crate::model::types::MsgType;
use chrono::{DateTime, Utc};
use deribit_base::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

/// Order Cancel Reject message (MsgType = '9')
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderCancelReject {
    /// Time of message transmission
    pub sending_time: DateTime<Utc>,
    /// Order status
    pub ord_status: Option<OrderStatus>,
    /// Cancel reject reason
    pub cxl_rej_reason: Option<i32>,
    /// Cancel reject response to
    pub cxl_rej_response_to: Option<char>,
    /// Free format text string
    pub text: Option<String>,
    /// Original client order ID
    pub cl_ord_id: Option<String>,
    /// Deribit order ID
    pub orig_cl_ord_id: Option<String>,
    /// Deribit label
    pub deribit_label: Option<String>,
}

impl OrderCancelReject {
    /// Create new order cancel reject
    pub fn new(
        ord_status: Option<OrderStatus>,
        cxl_rej_reason: Option<i32>,
        text: Option<String>,
    ) -> Self {
        Self {
            sending_time: Utc::now(),
            ord_status,
            cxl_rej_reason,
            cxl_rej_response_to: None,
            text,
            cl_ord_id: None,
            orig_cl_ord_id: None,
            deribit_label: None,
        }
    }

    /// Set client order ID
    pub fn with_cl_ord_id(mut self, cl_ord_id: String) -> Self {
        self.cl_ord_id = Some(cl_ord_id);
        self
    }

    /// Set original client order ID
    pub fn with_orig_cl_ord_id(mut self, orig_cl_ord_id: String) -> Self {
        self.orig_cl_ord_id = Some(orig_cl_ord_id);
        self
    }

    /// Set Deribit label
    pub fn with_deribit_label(mut self, deribit_label: String) -> Self {
        self.deribit_label = Some(deribit_label);
        self
    }

    /// Set cancel reject response to
    pub fn with_cxl_rej_response_to(mut self, response_to: char) -> Self {
        self.cxl_rej_response_to = Some(response_to);
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
            .msg_type(MsgType::OrderCancelReject)
            .sender_comp_id(sender_comp_id.to_string())
            .target_comp_id(target_comp_id.to_string())
            .msg_seq_num(msg_seq_num)
            .sending_time(self.sending_time);

        // Required field
        builder = builder.field(
            52,
            self.sending_time.format("%Y%m%d-%H:%M:%S%.3f").to_string(),
        );

        // Optional fields
        if let Some(ord_status) = &self.ord_status {
            builder = builder.field(39, char::from(*ord_status).to_string());
        }

        if let Some(cxl_rej_reason) = &self.cxl_rej_reason {
            builder = builder.field(102, cxl_rej_reason.to_string());
        }

        if let Some(cxl_rej_response_to) = &self.cxl_rej_response_to {
            builder = builder.field(434, cxl_rej_response_to.to_string());
        }

        if let Some(text) = &self.text {
            builder = builder.field(58, text.clone());
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

        Ok(builder.build()?.to_string())
    }
}

impl_json_display!(OrderCancelReject);
impl_json_debug_pretty!(OrderCancelReject);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_cancel_reject_creation() {
        let reject = OrderCancelReject::new(
            Some(OrderStatus::New),
            Some(5), // Unknown order
            Some("Order not found".to_string()),
        );

        assert_eq!(reject.ord_status, Some(OrderStatus::New));
        assert_eq!(reject.cxl_rej_reason, Some(5));
        assert_eq!(reject.text, Some("Order not found".to_string()));
        assert!(reject.sending_time <= Utc::now());
    }

    #[test]
    fn test_order_cancel_reject_with_cl_ord_id() {
        let reject = OrderCancelReject::new(
            Some(OrderStatus::Cancelled),
            Some(0), // No reject
            None,
        )
        .with_cl_ord_id("ORDER123".to_string());

        assert_eq!(reject.cl_ord_id, Some("ORDER123".to_string()));
    }

    #[test]
    fn test_order_cancel_reject_with_orig_cl_ord_id() {
        let reject = OrderCancelReject::new(
            Some(OrderStatus::New),
            Some(1), // Unknown symbol
            Some("Invalid symbol".to_string()),
        )
        .with_orig_cl_ord_id("ORIG456".to_string());

        assert_eq!(reject.orig_cl_ord_id, Some("ORIG456".to_string()));
    }

    #[test]
    fn test_order_cancel_reject_with_deribit_label() {
        let reject = OrderCancelReject::new(
            Some(OrderStatus::PartiallyFilled),
            Some(6), // Duplicate order
            None,
        )
        .with_deribit_label("my-order".to_string());

        assert_eq!(reject.deribit_label, Some("my-order".to_string()));
    }

    #[test]
    fn test_order_cancel_reject_to_fix_message() {
        let reject = OrderCancelReject::new(
            Some(OrderStatus::New),
            Some(5), // Unknown order
            Some("Order not found".to_string()),
        )
        .with_cl_ord_id("ORDER123".to_string());

        let fix_message = reject.to_fix_message("DERIBITSERVER", "CLIENT", 1);
        assert!(fix_message.is_ok());

        let message = fix_message.unwrap();
        assert!(message.contains("35=9")); // MsgType = OrderCancelReject
        assert!(message.contains("39=0")); // OrdStatus = New
        assert!(message.contains("102=5")); // CxlRejReason = Unknown order
        assert!(message.contains("58=Order not found")); // Text
        assert!(message.contains("11=ORDER123")); // ClOrdID
    }

    #[test]
    fn test_order_cancel_reject_minimal() {
        let reject = OrderCancelReject::new(None, None, None);

        let fix_message = reject.to_fix_message("DERIBITSERVER", "CLIENT", 1);
        assert!(fix_message.is_ok());

        let message = fix_message.unwrap();
        assert!(message.contains("35=9")); // MsgType = OrderCancelReject
        assert!(message.contains("52=")); // SendingTime should be present
    }
}
