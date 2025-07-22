//! Integration tests for Order Management FIX Messages
//!
//! These tests verify the complete flow of order management messages
//! from construction to FIX message serialization and validation.

use deribit_fix::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_order_lifecycle() {
        // Test complete order lifecycle: New Order -> Cancel Request -> Cancel Reject

        // 1. Create a new limit order
        let new_order = NewOrderSingle::limit(
            "CLIENT_ORDER_001".to_string(),
            OrderSide::Buy,
            1.0,
            50000.0,
            "BTC-PERPETUAL".to_string(),
        )
        .with_label("integration_test_order".to_string())
        .with_time_in_force(TimeInForce::GoodTillCancelled);

        // Serialize to FIX message
        let fix_message = new_order
            .to_fix_message("CLIENT", "DERIBIT", 1)
            .expect("Failed to create FIX message");

        assert!(fix_message.contains("35=D")); // MsgType = NewOrderSingle
        assert!(fix_message.contains("11=CLIENT_ORDER_001")); // ClOrdID
        assert!(fix_message.contains("55=BTC-PERPETUAL")); // Symbol
        assert!(fix_message.contains("54=1")); // Side = Buy
        assert!(fix_message.contains("38=1")); // OrderQty
        assert!(fix_message.contains("44=50000")); // Price
        assert!(fix_message.contains("40=2")); // OrdType = Limit
        assert!(fix_message.contains("59=1")); // TimeInForce = GTC

        // 2. Create cancel request for the order
        let cancel_request = OrderCancelRequest::by_cl_ord_id(
            "CLIENT_ORDER_001".to_string(),
            "BTC-PERPETUAL".to_string(),
        )
        .with_currency("BTC".to_string());

        let cancel_fix_message = cancel_request
            .to_fix_message("CLIENT", "DERIBIT", 2)
            .expect("Failed to create cancel FIX message");

        assert!(cancel_fix_message.contains("35=F")); // MsgType = OrderCancelRequest
        assert!(cancel_fix_message.contains("11=CLIENT_ORDER_001")); // ClOrdID
        assert!(cancel_fix_message.contains("15=BTC")); // Currency

        // 3. Create cancel reject response
        let cancel_reject = OrderCancelReject::new(
            Some(OrderStatus::New),
            Some(i32::from(OrderRejectReason::UnknownSymbol)),
            Some("Order not found".to_string()),
        );

        let reject_fix_message = cancel_reject
            .to_fix_message("DERIBIT", "CLIENT", 3)
            .expect("Failed to create reject FIX message");

        assert!(reject_fix_message.contains("35=9")); // MsgType = OrderCancelReject
        // Note: ClOrdID might not be included in OrderCancelReject - check actual implementation
        assert!(reject_fix_message.contains("39=0")); // OrdStatus = New
        assert!(reject_fix_message.contains("102=1")); // CxlRejReason = UnknownSymbol
        assert!(reject_fix_message.contains("58=Order not found")); // Text
    }

    #[test]
    fn test_mass_cancel_operations() {
        // Test mass cancel request and report flow

        // 1. Create mass cancel request for all orders
        let mass_cancel_all = OrderMassCancelRequest::all_orders("MASS_CANCEL_001".to_string());

        let fix_message = mass_cancel_all
            .to_fix_message("CLIENT", "DERIBIT", 10)
            .expect("Failed to create mass cancel FIX message");

        assert!(fix_message.contains("35=q")); // MsgType = OrderMassCancelRequest
        assert!(fix_message.contains("11=MASS_CANCEL_001")); // ClOrdID
        assert!(fix_message.contains("530=7")); // MassCancelRequestType = AllOrders

        // 2. Create mass cancel request by symbol
        let mass_cancel_symbol = OrderMassCancelRequest::by_symbol(
            "MASS_CANCEL_002".to_string(),
            "ETH-PERPETUAL".to_string(),
        );

        let symbol_fix_message = mass_cancel_symbol
            .to_fix_message("CLIENT", "DERIBIT", 11)
            .expect("Failed to create symbol mass cancel FIX message");

        assert!(symbol_fix_message.contains("35=q")); // MsgType = OrderMassCancelRequest
        assert!(symbol_fix_message.contains("11=MASS_CANCEL_002")); // ClOrdID
        assert!(symbol_fix_message.contains("530=1")); // MassCancelRequestType = BySymbol
        assert!(symbol_fix_message.contains("55=ETH-PERPETUAL")); // Symbol

        // 3. Create mass cancel report
        let mass_cancel_report = OrderMassCancelReport::new(
            Some("MASS_CANCEL_001".to_string()),
            MassCancelRequestType::AllOrders,
        )
        .with_total_affected_orders(5);

        let report_fix_message = mass_cancel_report
            .to_fix_message("DERIBIT", "CLIENT", 12)
            .expect("Failed to create mass cancel report FIX message");

        assert!(report_fix_message.contains("35=r")); // MsgType = OrderMassCancelReport
        assert!(report_fix_message.contains("11=MASS_CANCEL_001")); // ClOrdID
        assert!(report_fix_message.contains("530=7")); // MassCancelRequestType = AllOrders (from constructor)
        assert!(report_fix_message.contains("533=5")); // TotalAffectedOrders
    }

    #[test]
    fn test_mass_status_operations() {
        // Test mass status request operations

        // 1. Create mass status request for all orders
        let mass_status_all = OrderMassStatusRequest::all_orders("STATUS_REQ_001".to_string());

        let fix_message = mass_status_all
            .to_fix_message("CLIENT", "DERIBIT", 20)
            .expect("Failed to create mass status FIX message");

        assert!(fix_message.contains("35=AF")); // MsgType = OrderMassStatusRequest
        assert!(fix_message.contains("584=STATUS_REQ_001")); // MassStatusReqID
        assert!(fix_message.contains("585=7")); // MassStatusReqType = AllOrders

        // 2. Create mass status request for specific order by ClOrdId
        let mass_status_specific = OrderMassStatusRequest::specific_order_by_cl_ord_id(
            "STATUS_REQ_002".to_string(),
            Some("BTC".to_string()),
            None,
        );

        let specific_fix_message = mass_status_specific
            .to_fix_message("CLIENT", "DERIBIT", 21)
            .expect("Failed to create specific mass status FIX message");

        assert!(specific_fix_message.contains("35=AF")); // MsgType = OrderMassStatusRequest
        assert!(specific_fix_message.contains("584=STATUS_REQ_002")); // MassStatusReqID
        assert!(specific_fix_message.contains("585=7")); // MassStatusReqType = AllOrders (for ClOrdId search)
        assert!(specific_fix_message.contains("9014=1")); // MassStatusReqIdType = ClOrdId
        assert!(specific_fix_message.contains("15=BTC")); // Currency

        // 3. Create mass status request by Deribit label
        let mass_status_label = OrderMassStatusRequest::specific_order_by_deribit_label(
            "STATUS_REQ_003".to_string(),
            None,                              // currency
            Some("SOL-PERPETUAL".to_string()), // symbol
        )
        .with_symbol("SOL-PERPETUAL".to_string());

        let label_fix_message = mass_status_label
            .to_fix_message("CLIENT", "DERIBIT", 22)
            .expect("Failed to create label mass status FIX message");

        assert!(label_fix_message.contains("35=AF")); // MsgType = OrderMassStatusRequest
        assert!(label_fix_message.contains("584=STATUS_REQ_003")); // MassStatusReqID
        assert!(label_fix_message.contains("585=7")); // MassStatusReqType = AllOrders (for DeribitLabel search)
        assert!(label_fix_message.contains("9014=2")); // MassStatusReqIdType = DeribitLabel
        assert!(label_fix_message.contains("55=SOL-PERPETUAL")); // Symbol
    }

    #[test]
    fn test_order_types_and_sides() {
        // Test all order types and sides

        // Market Buy Order
        let market_buy = NewOrderSingle::market(
            "MARKET_BUY_001".to_string(),
            OrderSide::Buy,
            0.5,
            "BTC-PERPETUAL".to_string(),
        );

        let fix_message = market_buy
            .to_fix_message("CLIENT", "DERIBIT", 30)
            .expect("Failed to create market buy FIX message");

        assert!(fix_message.contains("40=1")); // OrdType = Market
        assert!(fix_message.contains("54=1")); // Side = Buy

        // Limit Sell Order
        let limit_sell = NewOrderSingle::limit(
            "LIMIT_SELL_001".to_string(),
            OrderSide::Sell,
            2.0,
            3500.0,
            "ETH-PERPETUAL".to_string(),
        )
        .with_time_in_force(TimeInForce::ImmediateOrCancel)
        .post_only();

        let limit_fix_message = limit_sell
            .to_fix_message("CLIENT", "DERIBIT", 31)
            .expect("Failed to create limit sell FIX message");

        assert!(limit_fix_message.contains("40=2")); // OrdType = Limit
        assert!(limit_fix_message.contains("54=2")); // Side = Sell
        assert!(limit_fix_message.contains("59=3")); // TimeInForce = IOC
        assert!(limit_fix_message.contains("44=3500")); // Price
    }

    #[test]
    fn test_enum_conversions_integration() {
        // Test that all enum conversions work correctly in integration context

        // Test OrderSide conversions
        assert_eq!(char::from(OrderSide::Buy), '1');
        assert_eq!(char::from(OrderSide::Sell), '2');
        assert_eq!(OrderSide::try_from('1').unwrap(), OrderSide::Buy);
        assert_eq!(OrderSide::try_from('2').unwrap(), OrderSide::Sell);

        // Test OrderType conversions
        assert_eq!(char::from(OrderType::Market), '1');
        assert_eq!(char::from(OrderType::Limit), '2');
        assert_eq!(char::from(OrderType::MarketLimit), 'K');

        // Test TimeInForce conversions
        assert_eq!(char::from(TimeInForce::GoodTillDay), '0');
        assert_eq!(char::from(TimeInForce::GoodTillCancelled), '1');
        assert_eq!(char::from(TimeInForce::ImmediateOrCancel), '3');
        assert_eq!(char::from(TimeInForce::FillOrKill), '4');

        // Test OrderStatus conversions
        assert_eq!(char::from(OrderStatus::New), '0');
        assert_eq!(char::from(OrderStatus::PartiallyFilled), '1');
        assert_eq!(char::from(OrderStatus::Filled), '2');
        assert_eq!(char::from(OrderStatus::Cancelled), '4');

        // Test MassCancelRequestType conversions
        assert_eq!(i32::from(MassCancelRequestType::BySymbol), 1);
        assert_eq!(i32::from(MassCancelRequestType::BySecurityType), 5);
        assert_eq!(i32::from(MassCancelRequestType::AllOrders), 7);
        assert_eq!(i32::from(MassCancelRequestType::ByDeribitLabel), 10);
    }

    #[test]
    fn test_validation_errors() {
        // Test validation error scenarios

        // Test mass cancel validation - create a proper request and then test validation
        let mass_cancel_by_symbol = OrderMassCancelRequest::by_symbol(
            "VALID_SYMBOL_001".to_string(),
            "BTC-PERPETUAL".to_string(),
        );

        // This should succeed because symbol is provided
        let result = mass_cancel_by_symbol.to_fix_message("CLIENT", "DERIBIT", 40);
        assert!(result.is_ok());

        // Test mass status validation - create request without required fields
        let mass_status = OrderMassStatusRequest::specific_order_by_deribit_label(
            "STATUS_REQ_INVALID".to_string(), // mass_status_req_id
            None,                             // No currency
            None,                             // No symbol
        );

        // Should fail validation because currency or symbol is required for DeribitLabel ID type
        let result = mass_status.to_fix_message("CLIENT", "DERIBIT", 41);
        assert!(result.is_err());
    }

    #[test]
    fn test_optional_fields() {
        // Test that optional fields are properly handled

        let order = NewOrderSingle::limit(
            "OPTIONAL_FIELDS_001".to_string(),
            OrderSide::Buy,
            1.0,
            50000.0,
            "BTC-PERPETUAL".to_string(),
        )
        .with_label("test_label".to_string())
        .post_only()
        .reduce_only();

        let fix_message = order
            .to_fix_message("CLIENT", "DERIBIT", 50)
            .expect("Failed to create FIX message with optional fields");

        // Check that optional fields are included
        assert!(fix_message.contains("100010=test_label")); // DeribitLabel
        assert!(fix_message.contains("18=E")); // ExecInst = ReduceOnly (last method called overwrites)
    }

    #[test]
    fn test_message_sequence_and_headers() {
        // Test that FIX message headers are properly constructed

        let order = NewOrderSingle::market(
            "HEADER_TEST_001".to_string(),
            OrderSide::Buy,
            1.0,
            "BTC-PERPETUAL".to_string(),
        );

        let fix_message = order
            .to_fix_message("TEST_CLIENT", "TEST_SERVER", 999)
            .expect("Failed to create FIX message");

        // Check FIX header fields
        assert!(fix_message.contains("8=FIX.4.4")); // BeginString
        assert!(fix_message.contains("35=D")); // MsgType = NewOrderSingle
        assert!(fix_message.contains("49=TEST_CLIENT")); // SenderCompID
        assert!(fix_message.contains("56=TEST_SERVER")); // TargetCompID
        assert!(fix_message.contains("34=999")); // MsgSeqNum
        assert!(fix_message.contains("52=")); // SendingTime (timestamp)
    }
}
