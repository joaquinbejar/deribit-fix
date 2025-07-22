// Unit tests for MessageBuilder

use chrono::Utc;
use deribit_fix::message::MessageBuilder;
use deribit_fix::model::types::MsgType;

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to create a complete valid message builder
    fn create_complete_builder() -> MessageBuilder {
        MessageBuilder::new()
            .msg_type(MsgType::Heartbeat)
            .sender_comp_id("CLIENT".to_string())
            .target_comp_id("DERIBIT".to_string())
            .msg_seq_num(1)
    }

    #[test]
    fn test_message_builder_new() {
        let message = create_complete_builder().build().unwrap();

        // Should have BeginString set to FIX.4.4
        assert!(message.has_field(8), "Should have BeginString field");
        assert_eq!(
            message.get_field(8).unwrap(),
            "FIX.4.4",
            "BeginString should be FIX.4.4"
        );
    }

    #[test]
    fn test_message_builder_msg_type() {
        let message = MessageBuilder::new()
            .msg_type(MsgType::TestRequest)
            .sender_comp_id("CLIENT".to_string())
            .target_comp_id("DERIBIT".to_string())
            .msg_seq_num(1)
            .build()
            .unwrap();

        assert!(message.has_field(35), "Should have MsgType field");
        assert_eq!(
            message.get_field(35).unwrap(),
            "1",
            "MsgType should be 1 for TestRequest"
        );
    }

    #[test]
    fn test_message_builder_sender_comp_id() {
        let sender_id = "TESTCLIENT".to_string();
        let message = MessageBuilder::new()
            .msg_type(MsgType::Heartbeat)
            .sender_comp_id(sender_id.clone())
            .target_comp_id("DERIBIT".to_string())
            .msg_seq_num(1)
            .build()
            .unwrap();

        assert!(message.has_field(49), "Should have SenderCompID field");
        assert_eq!(
            message.get_field(49).unwrap(),
            &sender_id,
            "SenderCompID should match"
        );
    }

    #[test]
    fn test_message_builder_target_comp_id() {
        let target_id = "TESTDERIBIT".to_string();
        let message = MessageBuilder::new()
            .msg_type(MsgType::Heartbeat)
            .sender_comp_id("CLIENT".to_string())
            .target_comp_id(target_id.clone())
            .msg_seq_num(1)
            .build()
            .unwrap();

        assert!(message.has_field(56), "Should have TargetCompID field");
        assert_eq!(
            message.get_field(56).unwrap(),
            &target_id,
            "TargetCompID should match"
        );
    }

    #[test]
    fn test_message_builder_msg_seq_num() {
        let seq_num = 123u32;
        let message = MessageBuilder::new()
            .msg_type(MsgType::Heartbeat)
            .sender_comp_id("CLIENT".to_string())
            .target_comp_id("DERIBIT".to_string())
            .msg_seq_num(seq_num)
            .build()
            .unwrap();

        assert!(message.has_field(34), "Should have MsgSeqNum field");
        assert_eq!(
            message.get_field(34).unwrap(),
            &seq_num.to_string(),
            "MsgSeqNum should match"
        );
    }

    #[test]
    fn test_message_builder_sending_time() {
        let now = Utc::now();
        let message = MessageBuilder::new()
            .msg_type(MsgType::Heartbeat)
            .sender_comp_id("CLIENT".to_string())
            .target_comp_id("DERIBIT".to_string())
            .msg_seq_num(1)
            .sending_time(now)
            .build()
            .unwrap();

        assert!(message.has_field(52), "Should have SendingTime field");
        let sending_time = message.get_field(52).unwrap();
        assert!(!sending_time.is_empty(), "SendingTime should not be empty");
        // Should be in YYYYMMDD-HH:MM:SS.sss format
        assert!(
            sending_time.contains('-'),
            "SendingTime should contain date-time separator"
        );
    }

    #[test]
    fn test_message_builder_custom_field() {
        let message = MessageBuilder::new()
            .msg_type(MsgType::TestRequest)
            .sender_comp_id("CLIENT".to_string())
            .target_comp_id("DERIBIT".to_string())
            .msg_seq_num(1)
            .field(112, "TEST123".to_string()) // TestReqID
            .build()
            .unwrap();

        assert!(message.has_field(112), "Should have custom field");
        assert_eq!(
            message.get_field(112).unwrap(),
            "TEST123",
            "Custom field value should match"
        );
    }

    #[test]
    fn test_message_builder_chaining() {
        let message = MessageBuilder::new()
            .msg_type(MsgType::TestRequest)
            .sender_comp_id("CLIENT".to_string())
            .target_comp_id("DERIBIT".to_string())
            .msg_seq_num(456)
            .field(112, "TEST456".to_string())
            .build()
            .unwrap();

        // Verify all fields are set
        assert_eq!(
            message.get_field(8).unwrap(),
            "FIX.4.4",
            "BeginString should be set"
        );
        assert_eq!(
            message.get_field(35).unwrap(),
            "1",
            "MsgType should be 1 for TestRequest"
        );
        assert_eq!(
            message.get_field(49).unwrap(),
            "CLIENT",
            "SenderCompID should be set"
        );
        assert_eq!(
            message.get_field(56).unwrap(),
            "DERIBIT",
            "TargetCompID should be set"
        );
        assert_eq!(
            message.get_field(34).unwrap(),
            "456",
            "MsgSeqNum should be set"
        );
        assert_eq!(
            message.get_field(112).unwrap(),
            "TEST456",
            "TestReqID should be set"
        );
    }

    #[test]
    fn test_message_builder_complete_message() {
        let now = Utc::now();
        let message = MessageBuilder::new()
            .msg_type(MsgType::Logon)
            .sender_comp_id("CLIENT".to_string())
            .target_comp_id("DERIBITSERVER".to_string())
            .msg_seq_num(1)
            .sending_time(now)
            .field(98, "0".to_string()) // EncryptMethod
            .field(108, "30".to_string()) // HeartBtInt
            .build()
            .unwrap();

        // Verify it's a valid logon message
        assert_eq!(
            message.get_field(35).unwrap(),
            "A",
            "Should be Logon message"
        );
        assert!(message.has_field(98), "Should have EncryptMethod");
        assert!(message.has_field(108), "Should have HeartBtInt");
        assert!(message.has_field(52), "Should have SendingTime");

        // Should have checksum calculated
        assert!(message.has_field(10), "Should have checksum field");
    }

    #[test]
    fn test_message_builder_different_msg_types() {
        let msg_types = vec![
            (MsgType::Heartbeat, "0"),
            (MsgType::TestRequest, "1"),
            (MsgType::ResendRequest, "2"),
            (MsgType::Reject, "3"),
            (MsgType::Logout, "5"),
            (MsgType::Logon, "A"),
            (MsgType::NewOrderSingle, "D"),
        ];

        for (msg_type, expected_str) in msg_types {
            let message = MessageBuilder::new()
                .msg_type(msg_type)
                .sender_comp_id("CLIENT".to_string())
                .target_comp_id("DERIBIT".to_string())
                .msg_seq_num(1)
                .build()
                .unwrap();

            assert_eq!(
                message.get_field(35).unwrap(),
                expected_str,
                "MsgType should match for {msg_type:?}"
            );
        }
    }

    #[test]
    fn test_message_builder_field_overwrite() {
        let message = MessageBuilder::new()
            .msg_type(MsgType::Heartbeat)
            .sender_comp_id("FIRST".to_string())
            .sender_comp_id("SECOND".to_string()) // Should overwrite
            .target_comp_id("DERIBIT".to_string())
            .msg_seq_num(1)
            .build()
            .unwrap();

        assert_eq!(
            message.get_field(49).unwrap(),
            "SECOND",
            "Should use last set value"
        );
    }

    #[test]
    fn test_message_builder_empty_values() {
        let message = MessageBuilder::new()
            .msg_type(MsgType::TestRequest)
            .sender_comp_id("".to_string())
            .target_comp_id("".to_string())
            .msg_seq_num(1)
            .field(112, "".to_string())
            .build()
            .unwrap();

        // Empty values should still be set
        assert!(
            message.has_field(49),
            "Should have SenderCompID field even if empty"
        );
        assert!(
            message.has_field(56),
            "Should have TargetCompID field even if empty"
        );
        assert!(
            message.has_field(112),
            "Should have custom field even if empty"
        );

        assert_eq!(
            message.get_field(49).unwrap(),
            "",
            "Empty SenderCompID should be preserved"
        );
        assert_eq!(
            message.get_field(56).unwrap(),
            "",
            "Empty TargetCompID should be preserved"
        );
        assert_eq!(
            message.get_field(112).unwrap(),
            "",
            "Empty custom field should be preserved"
        );
    }

    #[test]
    fn test_message_builder_raw_message_generation() {
        let message = MessageBuilder::new()
            .msg_type(MsgType::Heartbeat)
            .sender_comp_id("CLIENT".to_string())
            .target_comp_id("DERIBIT".to_string())
            .msg_seq_num(1)
            .build()
            .unwrap();

        let raw_message = message.to_string();

        // Should contain FIX protocol elements
        assert!(
            raw_message.contains("8=FIX.4.4"),
            "Should contain BeginString"
        );
        assert!(raw_message.contains("35=0"), "Should contain MsgType");
        assert!(
            raw_message.contains("49=CLIENT"),
            "Should contain SenderCompID"
        );
        assert!(
            raw_message.contains("56=DERIBIT"),
            "Should contain TargetCompID"
        );
        assert!(raw_message.contains("34=1"), "Should contain MsgSeqNum");
        assert!(raw_message.contains("10="), "Should contain checksum");

        // Should use SOH delimiter
        assert!(raw_message.contains('\x01'), "Should contain SOH delimiter");
    }

    #[test]
    fn test_message_builder_validation_errors() {
        // Test missing MsgType
        let result = MessageBuilder::new()
            .sender_comp_id("CLIENT".to_string())
            .target_comp_id("DERIBIT".to_string())
            .msg_seq_num(1)
            .build();

        assert!(result.is_err(), "Should fail without MsgType");

        // Test missing SenderCompID
        let result = MessageBuilder::new()
            .msg_type(MsgType::Heartbeat)
            .target_comp_id("DERIBIT".to_string())
            .msg_seq_num(1)
            .build();

        assert!(result.is_err(), "Should fail without SenderCompID");

        // Test missing TargetCompID
        let result = MessageBuilder::new()
            .msg_type(MsgType::Heartbeat)
            .sender_comp_id("CLIENT".to_string())
            .msg_seq_num(1)
            .build();

        assert!(result.is_err(), "Should fail without TargetCompID");

        // Test missing MsgSeqNum
        let result = MessageBuilder::new()
            .msg_type(MsgType::Heartbeat)
            .sender_comp_id("CLIENT".to_string())
            .target_comp_id("DERIBIT".to_string())
            .build();

        assert!(result.is_err(), "Should fail without MsgSeqNum");
    }
}
