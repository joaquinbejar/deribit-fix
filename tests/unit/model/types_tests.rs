// Unit tests for FIX types

use deribit_fix::model::types::{MsgType, ParseMsgTypeError};
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg_type_basic_functionality() {
        // Test basic functionality without relying on specific string representations
        let heartbeat = MsgType::Heartbeat;
        let test_request = MsgType::TestRequest;

        assert_eq!(heartbeat, MsgType::Heartbeat);
        assert_ne!(heartbeat, test_request);
    }

    #[test]
    fn test_msg_type_clone() {
        let original = MsgType::Heartbeat;
        let cloned = original;
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_msg_type_debug() {
        let msg_type = MsgType::TestRequest;
        let debug_str = format!("{msg_type:?}");
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn test_msg_type_partial_eq() {
        assert_eq!(MsgType::Heartbeat, MsgType::Heartbeat);
        assert_ne!(MsgType::Heartbeat, MsgType::TestRequest);
    }

    #[test]
    fn test_msg_type_administrative() {
        // Test administrative message types exist
        let admin_types = vec![
            MsgType::Heartbeat,
            MsgType::TestRequest,
            MsgType::ResendRequest,
            MsgType::Reject,
        ];

        for msg_type in admin_types {
            // Just test that they can be created and compared
            assert_eq!(msg_type, msg_type);
        }
    }

    /// Test session-level message types
    #[test]
    fn test_msg_type_session() {
        let session_types = vec![
            MsgType::Heartbeat,
            MsgType::TestRequest,
            MsgType::ResendRequest,
            MsgType::Reject,
            MsgType::SequenceReset,
            MsgType::Logout,
            MsgType::Logon,
        ];

        for msg_type in session_types {
            assert_eq!(msg_type, msg_type);
            let debug_str = format!("{msg_type:?}");
            assert!(!debug_str.is_empty());
        }
    }

    /// Test application-level message types
    #[test]
    fn test_msg_type_application() {
        let app_types = vec![
            MsgType::ExecutionReport,
            MsgType::OrderCancelReject,
            MsgType::NewOrderSingle,
            MsgType::OrderCancelRequest,
            MsgType::OrderCancelReplaceRequest,
        ];

        for msg_type in app_types {
            assert_eq!(msg_type, msg_type);
            let debug_str = format!("{msg_type:?}");
            assert!(!debug_str.is_empty());
        }
    }

    /// Test market data message types
    #[test]
    fn test_msg_type_market_data() {
        let market_data_types = vec![
            MsgType::MarketDataRequest,
            MsgType::MarketDataSnapshotFullRefresh,
            MsgType::MarketDataIncrementalRefresh,
            MsgType::MarketDataRequestReject,
        ];

        for msg_type in market_data_types {
            assert_eq!(msg_type, msg_type);
            let debug_str = format!("{msg_type:?}");
            assert!(!debug_str.is_empty());
        }
    }

    /// Test quote-related message types
    #[test]
    fn test_msg_type_quotes() {
        let quote_types = vec![
            MsgType::QuoteRequest,
            MsgType::QuoteCancel,
            MsgType::MassQuoteAcknowledgement,
            MsgType::MassQuote,
            MsgType::QuoteStatusReport,
            MsgType::RfqRequest,
            MsgType::QuoteRequestReject,
        ];

        for msg_type in quote_types {
            assert_eq!(msg_type, msg_type);
            let debug_str = format!("{msg_type:?}");
            assert!(!debug_str.is_empty());
        }
    }

    /// Test security-related message types
    #[test]
    fn test_msg_type_security() {
        let security_types = vec![
            MsgType::SecurityDefinitionRequest,
            MsgType::SecurityDefinition,
            MsgType::SecurityStatusRequest,
            MsgType::SecurityStatus,
            MsgType::SecurityListRequest,
            MsgType::SecurityList,
        ];

        for msg_type in security_types {
            assert_eq!(msg_type, msg_type);
            let debug_str = format!("{msg_type:?}");
            assert!(!debug_str.is_empty());
        }
    }

    /// Test mass operations message types
    #[test]
    fn test_msg_type_mass_operations() {
        let mass_op_types = vec![
            MsgType::OrderMassCancelRequest,
            MsgType::OrderMassCancelReport,
            MsgType::OrderMassStatusRequest,
        ];

        for msg_type in mass_op_types {
            assert_eq!(msg_type, msg_type);
            let debug_str = format!("{msg_type:?}");
            assert!(!debug_str.is_empty());
        }
    }

    /// Test trade capture message types
    #[test]
    fn test_msg_type_trade_capture() {
        let trade_types = vec![
            MsgType::TradeCaptureReportRequest,
            MsgType::TradeCaptureReport,
            MsgType::TradeCaptureReportRequestAck,
        ];

        for msg_type in trade_types {
            assert_eq!(msg_type, msg_type);
            let debug_str = format!("{msg_type:?}");
            assert!(!debug_str.is_empty());
        }
    }

    /// Test position-related message types
    #[test]
    fn test_msg_type_positions() {
        let position_types = vec![
            MsgType::RequestForPositions,
            MsgType::PositionReport,
        ];

        for msg_type in position_types {
            assert_eq!(msg_type, msg_type);
            let debug_str = format!("{msg_type:?}");
            assert!(!debug_str.is_empty());
        }
    }

    /// Test user-related message types
    #[test]
    fn test_msg_type_user() {
        let user_types = vec![
            MsgType::UserRequest,
            MsgType::UserResponse,
        ];

        for msg_type in user_types {
            assert_eq!(msg_type, msg_type);
            let debug_str = format!("{msg_type:?}");
            assert!(!debug_str.is_empty());
        }
    }

    /// Test MM protection message types
    #[test]
    fn test_msg_type_mm_protection() {
        let mm_types = vec![
            MsgType::MmProtectionLimits,
            MsgType::MmProtectionLimitsResult,
            MsgType::MmProtectionReset,
        ];

        for msg_type in mm_types {
            assert_eq!(msg_type, msg_type);
            let debug_str = format!("{msg_type:?}");
            assert!(!debug_str.is_empty());
        }
    }

    /// Test MsgType as_str() method
    #[test]
    fn test_msg_type_as_str() {
        let test_cases = vec![
            (MsgType::Heartbeat, "0"),
            (MsgType::TestRequest, "1"),
            (MsgType::ResendRequest, "2"),
            (MsgType::Reject, "3"),
            (MsgType::SequenceReset, "4"),
            (MsgType::Logout, "5"),
            (MsgType::ExecutionReport, "8"),
            (MsgType::OrderCancelReject, "9"),
            (MsgType::Logon, "A"),
            (MsgType::NewOrderSingle, "D"),
            (MsgType::OrderCancelRequest, "F"),
            (MsgType::OrderCancelReplaceRequest, "G"),
            (MsgType::QuoteRequest, "R"),
            (MsgType::MarketDataRequest, "V"),
            (MsgType::MarketDataSnapshotFullRefresh, "W"),
            (MsgType::MarketDataIncrementalRefresh, "X"),
            (MsgType::MarketDataRequestReject, "Y"),
            (MsgType::QuoteCancel, "Z"),
            (MsgType::MassQuoteAcknowledgement, "b"),
            (MsgType::SecurityDefinitionRequest, "c"),
            (MsgType::SecurityDefinition, "d"),
            (MsgType::SecurityStatusRequest, "e"),
            (MsgType::SecurityStatus, "f"),
            (MsgType::MassQuote, "i"),
            (MsgType::OrderMassCancelRequest, "q"),
            (MsgType::OrderMassCancelReport, "r"),
            (MsgType::SecurityListRequest, "x"),
            (MsgType::SecurityList, "y"),
            (MsgType::QuoteStatusReport, "AI"),
            (MsgType::RfqRequest, "AH"),
            (MsgType::QuoteRequestReject, "AG"),
            (MsgType::TradeCaptureReportRequest, "AD"),
            (MsgType::TradeCaptureReport, "AE"),
            (MsgType::TradeCaptureReportRequestAck, "AQ"),
            (MsgType::OrderMassStatusRequest, "AF"),
            (MsgType::RequestForPositions, "AN"),
            (MsgType::PositionReport, "AP"),
            (MsgType::UserRequest, "BE"),
            (MsgType::UserResponse, "BF"),
            (MsgType::MmProtectionLimits, "MM"),
            (MsgType::MmProtectionLimitsResult, "MR"),
            (MsgType::MmProtectionReset, "MZ"),
        ];

        for (msg_type, expected_str) in test_cases {
            assert_eq!(msg_type.as_str(), expected_str);
        }
    }

    /// Test MsgType FromStr implementation
    #[test]
    fn test_msg_type_from_str() {
        let test_cases = vec![
            ("0", MsgType::Heartbeat),
            ("1", MsgType::TestRequest),
            ("2", MsgType::ResendRequest),
            ("3", MsgType::Reject),
            ("4", MsgType::SequenceReset),
            ("5", MsgType::Logout),
            ("8", MsgType::ExecutionReport),
            ("9", MsgType::OrderCancelReject),
            ("A", MsgType::Logon),
            ("D", MsgType::NewOrderSingle),
            ("F", MsgType::OrderCancelRequest),
            ("G", MsgType::OrderCancelReplaceRequest),
            ("R", MsgType::QuoteRequest),
            ("V", MsgType::MarketDataRequest),
            ("W", MsgType::MarketDataSnapshotFullRefresh),
            ("X", MsgType::MarketDataIncrementalRefresh),
            ("Y", MsgType::MarketDataRequestReject),
            ("Z", MsgType::QuoteCancel),
            ("b", MsgType::MassQuoteAcknowledgement),
            ("c", MsgType::SecurityDefinitionRequest),
            ("d", MsgType::SecurityDefinition),
            ("e", MsgType::SecurityStatusRequest),
            ("f", MsgType::SecurityStatus),
            ("i", MsgType::MassQuote),
            ("q", MsgType::OrderMassCancelRequest),
            ("r", MsgType::OrderMassCancelReport),
            ("x", MsgType::SecurityListRequest),
            ("y", MsgType::SecurityList),
            ("MM", MsgType::MmProtectionLimits),
            ("MR", MsgType::MmProtectionLimitsResult),
            ("MZ", MsgType::MmProtectionReset),
        ];

        for (input_str, expected_type) in test_cases {
            let result = MsgType::from_str(input_str);
            assert!(result.is_ok(), "Failed to parse '{input_str}'");
            assert_eq!(result.unwrap(), expected_type);
        }
    }

    /// Test MsgType FromStr with invalid inputs
    #[test]
    fn test_msg_type_from_str_invalid() {
        let invalid_inputs = vec![
            "INVALID",
            "99",
            "ZZ",
            "",
            "a",
            "B",
            "10",
            "XX",
        ];

        for invalid_input in invalid_inputs {
            let result = MsgType::from_str(invalid_input);
            assert!(result.is_err(), "Should fail to parse '{invalid_input}'");
            
            match result {
                Err(ParseMsgTypeError(msg)) => {
                    assert_eq!(msg, invalid_input);
                },
                _ => panic!("Expected ParseMsgTypeError"),
            }
        }
    }

    /// Test ParseMsgTypeError display
    #[test]
    fn test_parse_msg_type_error_display() {
        let error = ParseMsgTypeError("INVALID".to_string());
        let display_str = format!("{error}");
        assert!(display_str.contains("Unknown message type"));
        assert!(display_str.contains("INVALID"));
        
        let debug_str = format!("{error:?}");
        assert!(debug_str.contains("ParseMsgTypeError"));
        assert!(debug_str.contains("INVALID"));
    }

    /// Test ParseMsgTypeError as std::error::Error
    #[test]
    fn test_parse_msg_type_error_as_error() {
        let error = ParseMsgTypeError("TEST".to_string());
        let error_trait: &dyn std::error::Error = &error;
        assert!(!error_trait.to_string().is_empty());
    }

    /// Test round-trip conversion (as_str -> from_str)
    #[test]
    fn test_msg_type_round_trip() {
        let all_types = vec![
            MsgType::Heartbeat,
            MsgType::TestRequest,
            MsgType::ResendRequest,
            MsgType::Reject,
            MsgType::SequenceReset,
            MsgType::Logout,
            MsgType::ExecutionReport,
            MsgType::OrderCancelReject,
            MsgType::Logon,
            MsgType::NewOrderSingle,
            MsgType::OrderCancelRequest,
            MsgType::OrderCancelReplaceRequest,
            MsgType::QuoteRequest,
            MsgType::MarketDataRequest,
            MsgType::MarketDataSnapshotFullRefresh,
            MsgType::MarketDataIncrementalRefresh,
            MsgType::MarketDataRequestReject,
            MsgType::QuoteCancel,
            MsgType::MassQuoteAcknowledgement,
            MsgType::SecurityDefinitionRequest,
            MsgType::SecurityDefinition,
            MsgType::SecurityStatusRequest,
            MsgType::SecurityStatus,
            MsgType::MassQuote,
            MsgType::OrderMassCancelRequest,
            MsgType::OrderMassCancelReport,
            MsgType::SecurityListRequest,
            MsgType::SecurityList,
            MsgType::QuoteStatusReport,
            MsgType::RfqRequest,
            MsgType::QuoteRequestReject,
            MsgType::TradeCaptureReportRequest,
            MsgType::TradeCaptureReport,
            MsgType::TradeCaptureReportRequestAck,
            MsgType::OrderMassStatusRequest,
            MsgType::RequestForPositions,
            MsgType::PositionReport,
            MsgType::UserRequest,
            MsgType::UserResponse,
            MsgType::MmProtectionLimits,
            MsgType::MmProtectionLimitsResult,
            MsgType::MmProtectionReset,
        ];

        for original_type in all_types {
            let str_repr = original_type.as_str();
            let parsed_type = MsgType::from_str(str_repr).unwrap();
            assert_eq!(original_type, parsed_type, 
                "Round-trip failed for {original_type:?} -> '{str_repr}' -> {parsed_type:?}");
        }
    }

    /// Test MsgType equality and inequality
    #[test]
    fn test_msg_type_equality() {
        // Test equality
        assert_eq!(MsgType::Heartbeat, MsgType::Heartbeat);
        assert_eq!(MsgType::Logon, MsgType::Logon);
        assert_eq!(MsgType::NewOrderSingle, MsgType::NewOrderSingle);
        
        // Test inequality
        assert_ne!(MsgType::Heartbeat, MsgType::TestRequest);
        assert_ne!(MsgType::Logon, MsgType::Logout);
        assert_ne!(MsgType::NewOrderSingle, MsgType::OrderCancelRequest);
    }

    /// Test MsgType clone and copy
    #[test]
    fn test_msg_type_clone_copy() {
        let original = MsgType::ExecutionReport;
        let cloned = original;
        let copied = original;
        
        assert_eq!(original, cloned);
        assert_eq!(original, copied);
        assert_eq!(cloned, copied);
    }



    #[test]
    fn test_msg_type_trading() {
        // Test trading message types exist
        let trading_types = vec![MsgType::NewOrderSingle, MsgType::ExecutionReport];

        for msg_type in trading_types {
            // Just test that they can be created and compared
            assert_eq!(msg_type, msg_type.clone());
        }
    }
}
