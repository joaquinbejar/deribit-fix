/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 22/7/25
******************************************************************************/

//! Order Mass Cancel FIX Messages Implementation

use super::*;
use crate::error::{DeribitFixError, Result as DeribitFixResult};
use crate::message::builder::MessageBuilder;
use crate::model::types::MsgType;
use chrono::Utc;
use deribit_base::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

/// Order Mass Cancel Request message (MsgType = 'q')
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderMassCancelRequest {
    /// Unique ID of Order Mass Cancel Request as assigned by the client
    pub cl_ord_id: String,
    /// Specifies the type of cancellation requested
    pub mass_cancel_request_type: MassCancelRequestType,
    /// Custom label for orders (required if MassCancelRequestType = ByDeribitLabel)
    pub deribit_label: Option<String>,
    /// Security type (required if MassCancelRequestType = BySecurityType)
    pub security_type: Option<String>,
    /// Symbol (required if MassCancelRequestType = BySymbol)
    pub symbol: Option<String>,
    /// Currency to cancel only certain currency
    pub currency: Option<String>,
    /// Whether to reject incoming quotes for 1 second after cancelling
    pub freeze_quotes: Option<bool>,
}

impl OrderMassCancelRequest {
    /// Create mass cancel request for all orders
    pub fn all_orders(cl_ord_id: String) -> Self {
        Self {
            cl_ord_id,
            mass_cancel_request_type: MassCancelRequestType::AllOrders,
            deribit_label: None,
            security_type: None,
            symbol: None,
            currency: None,
            freeze_quotes: None,
        }
    }

    /// Create mass cancel request by symbol
    pub fn by_symbol(cl_ord_id: String, symbol: String) -> Self {
        Self {
            cl_ord_id,
            mass_cancel_request_type: MassCancelRequestType::BySymbol,
            deribit_label: None,
            security_type: None,
            symbol: Some(symbol),
            currency: None,
            freeze_quotes: None,
        }
    }

    /// Create mass cancel request by security type
    pub fn by_security_type(cl_ord_id: String, security_type: String) -> Self {
        Self {
            cl_ord_id,
            mass_cancel_request_type: MassCancelRequestType::BySecurityType,
            deribit_label: None,
            security_type: Some(security_type),
            symbol: None,
            currency: None,
            freeze_quotes: None,
        }
    }

    /// Create mass cancel request by Deribit label
    pub fn by_deribit_label(cl_ord_id: String, deribit_label: String) -> Self {
        Self {
            cl_ord_id,
            mass_cancel_request_type: MassCancelRequestType::ByDeribitLabel,
            deribit_label: Some(deribit_label),
            security_type: None,
            symbol: None,
            currency: None,
            freeze_quotes: None,
        }
    }

    /// Set currency filter
    pub fn with_currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }

    /// Set freeze quotes flag
    pub fn with_freeze_quotes(mut self, freeze_quotes: bool) -> Self {
        self.freeze_quotes = Some(freeze_quotes);
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
            .msg_type(MsgType::OrderMassCancelRequest)
            .sender_comp_id(sender_comp_id.to_string())
            .target_comp_id(target_comp_id.to_string())
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now());

        // Required fields
        builder = builder
            .field(11, self.cl_ord_id.clone()) // ClOrdID
            .field(530, i32::from(self.mass_cancel_request_type).to_string()); // MassCancelRequestType

        // Conditional required fields
        match self.mass_cancel_request_type {
            MassCancelRequestType::ByDeribitLabel => {
                if let Some(deribit_label) = &self.deribit_label {
                    builder = builder.field(100010, deribit_label.clone());
                } else {
                    return Err(DeribitFixError::Generic(
                        "DeribitLabel is required for ByDeribitLabel mass cancel type".to_string(),
                    ));
                }
            }
            MassCancelRequestType::BySecurityType => {
                if let Some(security_type) = &self.security_type {
                    builder = builder.field(167, security_type.clone());
                } else {
                    return Err(DeribitFixError::Generic(
                        "SecurityType is required for BySecurityType mass cancel type".to_string(),
                    ));
                }
            }
            MassCancelRequestType::BySymbol => {
                if let Some(symbol) = &self.symbol {
                    builder = builder.field(55, symbol.clone());
                } else {
                    return Err(DeribitFixError::Generic(
                        "Symbol is required for BySymbol mass cancel type".to_string(),
                    ));
                }
            }
            MassCancelRequestType::AllOrders => {
                // No additional fields required
            }
        }

        // Optional fields
        if let Some(currency) = &self.currency {
            builder = builder.field(15, currency.clone());
        }

        if let Some(freeze_quotes) = &self.freeze_quotes {
            builder = builder.field(9031, if *freeze_quotes { "Y" } else { "N" }.to_string());
        }

        Ok(builder.build()?.to_string())
    }
}

impl_json_display!(OrderMassCancelRequest);
impl_json_debug_pretty!(OrderMassCancelRequest);

/// Order Mass Cancel Report message (MsgType = 'r')
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderMassCancelReport {
    /// Unique Identifier assigned by the client in the Order Mass Cancel Request
    pub cl_ord_id: Option<String>,
    /// Unique ID assigned by Deribit for this order
    pub order_id: Option<String>,
    /// Specifies the type of cancellation request
    pub mass_cancel_request_type: MassCancelRequestType,
    /// If successful, echoes the MassCancelRequestType
    pub mass_cancel_response: Option<i32>,
    /// Reason for mass cancel reject
    pub mass_cancel_reject_reason: Option<i32>,
    /// Total number of orders affected by Order Mass Cancel Request
    pub total_affected_orders: Option<i32>,
    /// Number of order identifiers for orders affected by the Order Mass Cancel Request
    pub no_affected_orders: Option<i32>,
    /// List of affected order IDs
    pub affected_orig_cl_ord_ids: Vec<String>,
    /// Free format text string
    pub text: Option<String>,
}

impl OrderMassCancelReport {
    /// Create new mass cancel report
    pub fn new(cl_ord_id: Option<String>, mass_cancel_request_type: MassCancelRequestType) -> Self {
        Self {
            cl_ord_id,
            order_id: None,
            mass_cancel_request_type,
            mass_cancel_response: None,
            mass_cancel_reject_reason: None,
            total_affected_orders: None,
            no_affected_orders: None,
            affected_orig_cl_ord_ids: Vec::new(),
            text: None,
        }
    }

    /// Set mass cancel response
    pub fn with_response(mut self, response: i32) -> Self {
        self.mass_cancel_response = Some(response);
        self
    }

    /// Set mass cancel reject reason
    pub fn with_reject_reason(mut self, reason: i32) -> Self {
        self.mass_cancel_reject_reason = Some(reason);
        self
    }

    /// Set total affected orders
    pub fn with_total_affected_orders(mut self, total: i32) -> Self {
        self.total_affected_orders = Some(total);
        self
    }

    /// Set affected order IDs
    pub fn with_affected_orders(mut self, order_ids: Vec<String>) -> Self {
        self.no_affected_orders = Some(order_ids.len() as i32);
        self.affected_orig_cl_ord_ids = order_ids;
        self
    }

    /// Set text message
    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
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
            .msg_type(MsgType::OrderMassCancelReport)
            .sender_comp_id(sender_comp_id.to_string())
            .target_comp_id(target_comp_id.to_string())
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now());

        // Required field
        builder = builder.field(530, i32::from(self.mass_cancel_request_type).to_string());

        // Optional fields
        if let Some(cl_ord_id) = &self.cl_ord_id {
            builder = builder.field(11, cl_ord_id.clone());
        }

        if let Some(order_id) = &self.order_id {
            builder = builder.field(37, order_id.clone());
        }

        if let Some(mass_cancel_response) = &self.mass_cancel_response {
            builder = builder.field(531, mass_cancel_response.to_string());
        }

        if let Some(mass_cancel_reject_reason) = &self.mass_cancel_reject_reason {
            builder = builder.field(532, mass_cancel_reject_reason.to_string());
        }

        if let Some(total_affected_orders) = &self.total_affected_orders {
            builder = builder.field(533, total_affected_orders.to_string());
        }

        if let Some(no_affected_orders) = &self.no_affected_orders {
            builder = builder.field(534, no_affected_orders.to_string());
        }

        // Add affected order IDs as repeating group
        for (i, order_id) in self.affected_orig_cl_ord_ids.iter().enumerate() {
            builder = builder.field(535 + i as u32, order_id.clone());
        }

        if let Some(text) = &self.text {
            builder = builder.field(58, text.clone());
        }

        Ok(builder.build()?.to_string())
    }
}

impl_json_display!(OrderMassCancelReport);
impl_json_debug_pretty!(OrderMassCancelReport);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_mass_cancel_request_all_orders() {
        let request = OrderMassCancelRequest::all_orders("MASS123".to_string());

        assert_eq!(request.cl_ord_id, "MASS123");
        assert_eq!(
            request.mass_cancel_request_type,
            MassCancelRequestType::AllOrders
        );
        assert_eq!(request.deribit_label, None);
        assert_eq!(request.security_type, None);
        assert_eq!(request.symbol, None);
    }

    #[test]
    fn test_order_mass_cancel_request_by_symbol() {
        let request =
            OrderMassCancelRequest::by_symbol("MASS456".to_string(), "BTC-PERPETUAL".to_string());

        assert_eq!(request.cl_ord_id, "MASS456");
        assert_eq!(
            request.mass_cancel_request_type,
            MassCancelRequestType::BySymbol
        );
        assert_eq!(request.symbol, Some("BTC-PERPETUAL".to_string()));
    }

    #[test]
    fn test_order_mass_cancel_request_by_security_type() {
        let request =
            OrderMassCancelRequest::by_security_type("MASS789".to_string(), "FUT".to_string());

        assert_eq!(request.cl_ord_id, "MASS789");
        assert_eq!(
            request.mass_cancel_request_type,
            MassCancelRequestType::BySecurityType
        );
        assert_eq!(request.security_type, Some("FUT".to_string()));
    }

    #[test]
    fn test_order_mass_cancel_request_by_deribit_label() {
        let request = OrderMassCancelRequest::by_deribit_label(
            "MASS101".to_string(),
            "my-orders".to_string(),
        );

        assert_eq!(request.cl_ord_id, "MASS101");
        assert_eq!(
            request.mass_cancel_request_type,
            MassCancelRequestType::ByDeribitLabel
        );
        assert_eq!(request.deribit_label, Some("my-orders".to_string()));
    }

    #[test]
    fn test_order_mass_cancel_request_to_fix_message() {
        let request = OrderMassCancelRequest::all_orders("MASS123".to_string())
            .with_currency("BTC".to_string())
            .with_freeze_quotes(true);

        let fix_message = request.to_fix_message("CLIENT", "DERIBITSERVER", 1);
        assert!(fix_message.is_ok());

        let message = fix_message.unwrap();
        assert!(message.contains("35=q")); // MsgType = OrderMassCancelRequest
        assert!(message.contains("11=MASS123")); // ClOrdID
        assert!(message.contains("530=7")); // MassCancelRequestType = AllOrders
        assert!(message.contains("15=BTC")); // Currency
        assert!(message.contains("9031=Y")); // FreezeQuotes
    }

    #[test]
    fn test_order_mass_cancel_report_creation() {
        let report = OrderMassCancelReport::new(
            Some("MASS123".to_string()),
            MassCancelRequestType::AllOrders,
        )
        .with_response(7)
        .with_total_affected_orders(5)
        .with_affected_orders(vec!["ORDER1".to_string(), "ORDER2".to_string()]);

        assert_eq!(report.cl_ord_id, Some("MASS123".to_string()));
        assert_eq!(
            report.mass_cancel_request_type,
            MassCancelRequestType::AllOrders
        );
        assert_eq!(report.mass_cancel_response, Some(7));
        assert_eq!(report.total_affected_orders, Some(5));
        assert_eq!(report.no_affected_orders, Some(2));
        assert_eq!(report.affected_orig_cl_ord_ids.len(), 2);
    }

    #[test]
    fn test_order_mass_cancel_report_to_fix_message() {
        let report = OrderMassCancelReport::new(
            Some("MASS123".to_string()),
            MassCancelRequestType::AllOrders,
        )
        .with_response(7)
        .with_total_affected_orders(3);

        let fix_message = report.to_fix_message("DERIBITSERVER", "CLIENT", 1);
        assert!(fix_message.is_ok());

        let message = fix_message.unwrap();
        assert!(message.contains("35=r")); // MsgType = OrderMassCancelReport
        assert!(message.contains("11=MASS123")); // ClOrdID
        assert!(message.contains("530=7")); // MassCancelRequestType = AllOrders
        assert!(message.contains("531=7")); // MassCancelResponse
        assert!(message.contains("533=3")); // TotalAffectedOrders
    }
}
