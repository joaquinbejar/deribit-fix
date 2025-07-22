// Unit tests for FIX message models

use deribit_fix::model::types::MsgType;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg_type_clone() {
        let msg_type = MsgType::Heartbeat;
        let cloned = msg_type;
        assert_eq!(msg_type, cloned);
    }

    #[test]
    fn test_msg_type_debug() {
        let msg_type = MsgType::TestRequest;
        let debug_str = format!("{msg_type:?}");
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn test_msg_type_equality() {
        let msg1 = MsgType::Heartbeat;
        let msg2 = MsgType::Heartbeat;
        let msg3 = MsgType::TestRequest;

        assert_eq!(msg1, msg2);
        assert_ne!(msg1, msg3);
    }

    #[test]
    fn test_msg_type_variants_exist() {
        // Test that all expected variants exist
        let _heartbeat = MsgType::Heartbeat;
        let _test_request = MsgType::TestRequest;
        let _resend_request = MsgType::ResendRequest;
        let _reject = MsgType::Reject;
        let _logon = MsgType::Logon;
        let _logout = MsgType::Logout;
        let _new_order_single = MsgType::NewOrderSingle;
        let _execution_report = MsgType::ExecutionReport;
    }

    #[test]
    fn test_msg_type_copy() {
        let msg_type = MsgType::Heartbeat;
        let copied = msg_type;
        assert_eq!(msg_type, copied);
    }
}
