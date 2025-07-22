//! Tests for custom Debug implementation of FixMessage

use deribit_fix::message::MessageBuilder;
use deribit_fix::model::types::MsgType;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_message_debug_format() {
        // Create a sample FIX message for testing debug output
        let fix_message = MessageBuilder::new()
            .msg_type(MsgType::NewOrderSingle)
            .sender_comp_id("CLIENT".to_string())
            .target_comp_id("DERIBITSERVER".to_string())
            .msg_seq_num(1)
            .field(11, "CLIENT_ORDER_123".to_string()) // ClOrdID
            .field(55, "BTC-PERPETUAL".to_string()) // Symbol
            .field(54, "1".to_string()) // Side (Buy)
            .field(38, "1.0".to_string()) // OrderQty
            .field(40, "2".to_string()) // OrdType (Limit)
            .field(44, "50000.0".to_string()) // Price
            .field(59, "1".to_string()) // TimeInForce (GTC)
            .build()
            .expect("Should build valid FIX message");

        // Test debug formatting
        let debug_output = format!("{fix_message:?}");

        // Verify that debug output contains expected field names
        assert!(debug_output.contains("BeginString(8)=FIX.4.4"));
        assert!(debug_output.contains("MsgType(35)=D"));
        assert!(debug_output.contains("SenderCompID(49)=CLIENT"));
        assert!(debug_output.contains("TargetCompID(56)=DERIBITSERVER"));
        assert!(debug_output.contains("MsgSeqNum(34)=1"));
        assert!(debug_output.contains("ClOrdID(11)=CLIENT_ORDER_123"));
        assert!(debug_output.contains("Symbol(55)=BTC-PERPETUAL"));
        assert!(debug_output.contains("Side(54)=1"));
        assert!(debug_output.contains("OrderQty(38)=1.0"));
        assert!(debug_output.contains("OrdType(40)=2"));
        assert!(debug_output.contains("Price(44)=50000.0"));
        assert!(debug_output.contains("TimeInForce(59)=1"));

        // Verify readable_message field exists and uses pipe separators
        assert!(debug_output.contains("readable_message"));
        assert!(debug_output.contains(" | "));

        // Verify no trailing separator in readable_message
        // Extract the readable_message part
        if let Some(start) = debug_output.find("readable_message: \"") {
            let start_pos = start + "readable_message: \"".len();
            if let Some(end) = debug_output[start_pos..].find("\",") {
                let readable_part = &debug_output[start_pos..start_pos + end];
                assert!(
                    !readable_part.ends_with(" | "),
                    "readable_message should not end with separator"
                );
                assert!(
                    readable_part.ends_with("="),
                    "readable_message should end with field value"
                );
            }
        }
    }

    #[test]
    fn test_fix_message_debug_vs_display() {
        let fix_message = MessageBuilder::new()
            .msg_type(MsgType::Heartbeat)
            .sender_comp_id("CLIENT".to_string())
            .target_comp_id("DERIBITSERVER".to_string())
            .msg_seq_num(2)
            .field(112, "TEST123".to_string()) // TestReqID
            .build()
            .expect("Should build valid FIX message");

        let display_output = format!("{fix_message}");
        let debug_output = format!("{fix_message:?}");

        // Display should show raw message with SOH characters (not visible in string)
        assert!(display_output.contains("8=FIX.4.4"));
        assert!(display_output.contains("35=0")); // Heartbeat MsgType
        assert!(display_output.contains("112=TEST123"));

        // Debug should be more readable
        assert!(debug_output.contains("MsgType(35)=0"));
        assert!(debug_output.contains("Unknown(112)=TEST123")); // TestReqID not in common fields
        assert!(debug_output.contains("readable_message"));

        // Debug should contain pipe separators while display should not
        assert!(debug_output.contains(" | "));
        assert!(!display_output.contains(" | "));
    }

    #[test]
    fn test_fix_message_debug_field_names() {
        let fix_message = MessageBuilder::new()
            .msg_type(MsgType::Logon)
            .sender_comp_id("CLIENT".to_string())
            .target_comp_id("DERIBITSERVER".to_string())
            .msg_seq_num(1)
            .field(98, "0".to_string()) // EncryptMethod
            .field(108, "30".to_string()) // HeartBtInt
            .field(553, "username".to_string()) // Username
            .field(554, "password".to_string()) // Password
            .build()
            .expect("Should build valid FIX message");

        let debug_output = format!("{fix_message:?}");

        // Test that common FIX field names are recognized
        assert!(debug_output.contains("EncryptMethod(98)=0"));
        assert!(debug_output.contains("HeartBtInt(108)=30"));
        assert!(debug_output.contains("Username(553)=username"));
        assert!(debug_output.contains("Password(554)=password"));
    }

    #[test]
    fn test_fix_message_debug_unknown_fields() {
        let fix_message = MessageBuilder::new()
            .msg_type(MsgType::TestRequest)
            .sender_comp_id("CLIENT".to_string())
            .target_comp_id("DERIBITSERVER".to_string())
            .msg_seq_num(1)
            .field(9999, "custom_value".to_string()) // Unknown field
            .build()
            .expect("Should build valid FIX message");

        let debug_output = format!("{fix_message:?}");

        // Unknown fields should be labeled as "Unknown"
        assert!(debug_output.contains("Unknown(9999)=custom_value"));
    }

    #[test]
    fn test_fix_message_debug_pretty_format() {
        let fix_message = MessageBuilder::new()
            .msg_type(MsgType::NewOrderSingle)
            .sender_comp_id("CLIENT".to_string())
            .target_comp_id("DERIBITSERVER".to_string())
            .msg_seq_num(1)
            .field(11, "ORDER_001".to_string())
            .build()
            .expect("Should build valid FIX message");

        let pretty_debug = format!("{fix_message:#?}");
        let compact_debug = format!("{fix_message:?}");

        // Pretty format should have newlines and indentation
        assert!(pretty_debug.contains("FixMessage {\n"));
        assert!(pretty_debug.contains("    fields: ["));
        assert!(pretty_debug.contains("    ],"));
        assert!(pretty_debug.contains("    readable_message:"));

        // Compact format should be on fewer lines
        assert!(!compact_debug.contains("\n    fields: ["));

        // Both should contain the same field information
        assert!(pretty_debug.contains("ClOrdID(11)=ORDER_001"));
        assert!(compact_debug.contains("ClOrdID(11)=ORDER_001"));
    }

    #[test]
    fn test_fix_message_debug_empty_message() {
        let fix_message = MessageBuilder::new()
            .msg_type(MsgType::Heartbeat)
            .sender_comp_id("CLIENT".to_string())
            .target_comp_id("DERIBITSERVER".to_string())
            .msg_seq_num(1)
            .build()
            .expect("Should build valid FIX message");

        let debug_output = format!("{fix_message:?}");

        // Should still contain basic FIX fields
        assert!(debug_output.contains("BeginString(8)=FIX.4.4"));
        assert!(debug_output.contains("MsgType(35)=0"));
        assert!(debug_output.contains("readable_message"));

        // Should not end with trailing separator
        assert!(debug_output.contains("readable_message: \""));
        if let Some(start) = debug_output.find("readable_message: \"") {
            let start_pos = start + "readable_message: \"".len();
            if let Some(end) = debug_output[start_pos..].find("\"") {
                let readable_part = &debug_output[start_pos..start_pos + end];
                assert!(!readable_part.ends_with(" | "));
            }
        }
    }
}
