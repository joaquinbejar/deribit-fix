// Unit tests for FIX types

use deribit_fix::model::types::MsgType;

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
            assert_eq!(msg_type, msg_type.clone());
        }
    }

    #[test]
    fn test_msg_type_session() {
        // Test session message types exist
        let session_types = vec![MsgType::Logon, MsgType::Logout];

        for msg_type in session_types {
            // Just test that they can be created and compared
            assert_eq!(msg_type, msg_type.clone());
        }
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
