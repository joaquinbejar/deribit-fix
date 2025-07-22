/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 22/7/25
******************************************************************************/

//! Order Mass Status Request FIX Message Implementation

use super::*;
use crate::error::{DeribitFixError, Result as DeribitFixResult};
use crate::message::builder::MessageBuilder;
use crate::model::types::MsgType;
use chrono::Utc;
use deribit_base::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

/// Order Mass Status Request message (MsgType = 'AF')
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderMassStatusRequest {
    /// Client-assigned unique ID of this request
    pub mass_status_req_id: String,
    /// Specifies the scope of the mass status request
    pub mass_status_req_type: MassStatusRequestType,
    /// Defines which ID or label represents the MassStatusReqID
    pub mass_status_req_id_type: Option<MassStatusRequestIdType>,
    /// Currency to search the order by DeribitLabel or ClOrdID
    pub currency: Option<String>,
    /// Symbol to search the order by DeribitLabel or ClOrdID
    pub symbol: Option<String>,
}

impl OrderMassStatusRequest {
    /// Create mass status request for all orders
    pub fn all_orders(mass_status_req_id: String) -> Self {
        Self {
            mass_status_req_id,
            mass_status_req_type: MassStatusRequestType::AllOrders,
            mass_status_req_id_type: None,
            currency: None,
            symbol: None,
        }
    }

    /// Create mass status request for a specific order by OrigClOrdID
    pub fn specific_order_by_orig_cl_ord_id(orig_cl_ord_id: String) -> Self {
        Self {
            mass_status_req_id: orig_cl_ord_id,
            mass_status_req_type: MassStatusRequestType::SpecificOrder,
            mass_status_req_id_type: Some(MassStatusRequestIdType::OrigClOrdId),
            currency: None,
            symbol: None,
        }
    }

    /// Create mass status request for a specific order by ClOrdID
    pub fn specific_order_by_cl_ord_id(
        cl_ord_id: String,
        currency: Option<String>,
        symbol: Option<String>,
    ) -> Self {
        Self {
            mass_status_req_id: cl_ord_id,
            mass_status_req_type: MassStatusRequestType::AllOrders, // Use 7 for search by ClOrdID
            mass_status_req_id_type: Some(MassStatusRequestIdType::ClOrdId),
            currency,
            symbol,
        }
    }

    /// Create mass status request for a specific order by DeribitLabel
    pub fn specific_order_by_deribit_label(
        deribit_label: String,
        currency: Option<String>,
        symbol: Option<String>,
    ) -> Self {
        Self {
            mass_status_req_id: deribit_label,
            mass_status_req_type: MassStatusRequestType::AllOrders, // Use 7 for search by DeribitLabel
            mass_status_req_id_type: Some(MassStatusRequestIdType::DeribitLabel),
            currency,
            symbol,
        }
    }

    /// Set currency for search
    pub fn with_currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }

    /// Set symbol for search
    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
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
            .msg_type(MsgType::OrderMassStatusRequest)
            .sender_comp_id(sender_comp_id.to_string())
            .target_comp_id(target_comp_id.to_string())
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now());

        // Required fields
        builder = builder
            .field(584, self.mass_status_req_id.clone()) // MassStatusReqID
            .field(585, i32::from(self.mass_status_req_type).to_string()); // MassStatusReqType

        // Optional fields
        if let Some(mass_status_req_id_type) = &self.mass_status_req_id_type {
            builder = builder.field(9014, i32::from(*mass_status_req_id_type).to_string());
        }

        // Validation: Currency or Symbol required if MassStatusReqIDType is ClOrdId or DeribitLabel
        if let Some(id_type) = &self.mass_status_req_id_type {
            match id_type {
                MassStatusRequestIdType::ClOrdId | MassStatusRequestIdType::DeribitLabel => {
                    if self.currency.is_none() && self.symbol.is_none() {
                        return Err(DeribitFixError::Generic("Currency or Symbol is required when searching by ClOrdId or DeribitLabel".to_string()));
                    }
                }
                MassStatusRequestIdType::OrigClOrdId => {
                    // No additional validation required
                }
            }
        }

        if let Some(currency) = &self.currency {
            builder = builder.field(15, currency.clone()); // Note: Using tag 15 for Currency, but doc shows tag 11
        }

        if let Some(symbol) = &self.symbol {
            builder = builder.field(55, symbol.clone());
        }

        Ok(builder.build()?.to_string())
    }
}

impl_json_display!(OrderMassStatusRequest);
impl_json_debug_pretty!(OrderMassStatusRequest);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_mass_status_request_all_orders() {
        let request = OrderMassStatusRequest::all_orders("STATUS123".to_string());

        assert_eq!(request.mass_status_req_id, "STATUS123");
        assert_eq!(
            request.mass_status_req_type,
            MassStatusRequestType::AllOrders
        );
        assert_eq!(request.mass_status_req_id_type, None);
        assert_eq!(request.currency, None);
        assert_eq!(request.symbol, None);
    }

    #[test]
    fn test_order_mass_status_request_specific_order_by_orig_cl_ord_id() {
        let request =
            OrderMassStatusRequest::specific_order_by_orig_cl_ord_id("ORIG123".to_string());

        assert_eq!(request.mass_status_req_id, "ORIG123");
        assert_eq!(
            request.mass_status_req_type,
            MassStatusRequestType::SpecificOrder
        );
        assert_eq!(
            request.mass_status_req_id_type,
            Some(MassStatusRequestIdType::OrigClOrdId)
        );
    }

    #[test]
    fn test_order_mass_status_request_specific_order_by_cl_ord_id() {
        let request = OrderMassStatusRequest::specific_order_by_cl_ord_id(
            "ORDER123".to_string(),
            Some("BTC".to_string()),
            None,
        );

        assert_eq!(request.mass_status_req_id, "ORDER123");
        assert_eq!(
            request.mass_status_req_type,
            MassStatusRequestType::AllOrders
        );
        assert_eq!(
            request.mass_status_req_id_type,
            Some(MassStatusRequestIdType::ClOrdId)
        );
        assert_eq!(request.currency, Some("BTC".to_string()));
    }

    #[test]
    fn test_order_mass_status_request_specific_order_by_deribit_label() {
        let request = OrderMassStatusRequest::specific_order_by_deribit_label(
            "my-order".to_string(),
            None,
            Some("BTC-PERPETUAL".to_string()),
        );

        assert_eq!(request.mass_status_req_id, "my-order");
        assert_eq!(
            request.mass_status_req_type,
            MassStatusRequestType::AllOrders
        );
        assert_eq!(
            request.mass_status_req_id_type,
            Some(MassStatusRequestIdType::DeribitLabel)
        );
        assert_eq!(request.symbol, Some("BTC-PERPETUAL".to_string()));
    }

    #[test]
    fn test_order_mass_status_request_with_currency_and_symbol() {
        let request = OrderMassStatusRequest::all_orders("STATUS456".to_string())
            .with_currency("BTC".to_string())
            .with_symbol("BTC-PERPETUAL".to_string());

        assert_eq!(request.currency, Some("BTC".to_string()));
        assert_eq!(request.symbol, Some("BTC-PERPETUAL".to_string()));
    }

    #[test]
    fn test_order_mass_status_request_to_fix_message() {
        let request = OrderMassStatusRequest::all_orders("STATUS123".to_string());

        let fix_message = request.to_fix_message("CLIENT", "DERIBITSERVER", 1);
        assert!(fix_message.is_ok());

        let message = fix_message.unwrap();
        assert!(message.contains("35=AF")); // MsgType = OrderMassStatusRequest
        assert!(message.contains("584=STATUS123")); // MassStatusReqID
        assert!(message.contains("585=7")); // MassStatusReqType = AllOrders
    }

    #[test]
    fn test_order_mass_status_request_specific_order_to_fix_message() {
        let request =
            OrderMassStatusRequest::specific_order_by_orig_cl_ord_id("ORIG123".to_string());

        let fix_message = request.to_fix_message("CLIENT", "DERIBITSERVER", 1);
        assert!(fix_message.is_ok());

        let message = fix_message.unwrap();
        assert!(message.contains("35=AF")); // MsgType = OrderMassStatusRequest
        assert!(message.contains("584=ORIG123")); // MassStatusReqID
        assert!(message.contains("585=1")); // MassStatusReqType = SpecificOrder
        assert!(message.contains("9014=0")); // MassStatusReqIDType = OrigClOrdId
    }

    #[test]
    fn test_order_mass_status_request_validation_error() {
        let request = OrderMassStatusRequest::specific_order_by_cl_ord_id(
            "ORDER123".to_string(),
            None, // No currency
            None, // No symbol
        );

        let fix_message = request.to_fix_message("CLIENT", "DERIBITSERVER", 1);
        assert!(fix_message.is_err());
    }

    #[test]
    fn test_order_mass_status_request_with_deribit_label_and_symbol() {
        let request = OrderMassStatusRequest::specific_order_by_deribit_label(
            "my-order".to_string(),
            None,
            Some("BTC-PERPETUAL".to_string()),
        );

        let fix_message = request.to_fix_message("CLIENT", "DERIBITSERVER", 1);
        assert!(fix_message.is_ok());

        let message = fix_message.unwrap();
        assert!(message.contains("35=AF")); // MsgType = OrderMassStatusRequest
        assert!(message.contains("584=my-order")); // MassStatusReqID
        assert!(message.contains("585=7")); // MassStatusReqType = AllOrders
        assert!(message.contains("9014=2")); // MassStatusReqIDType = DeribitLabel
        assert!(message.contains("55=BTC-PERPETUAL")); // Symbol
    }
}
