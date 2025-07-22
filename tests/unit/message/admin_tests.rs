// Unit tests for FIX administrative messages

use deribit_fix::message::admin::{
    Heartbeat, Reject, ResendRequest, SessionRejectReason, TestRequest,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heartbeat_new() {
        let heartbeat = Heartbeat::new();
        assert!(heartbeat.test_req_id.is_none());
    }

    #[test]
    fn test_heartbeat_response() {
        let test_req_id = "TEST123".to_string();
        let heartbeat = Heartbeat::new_response(test_req_id.clone());
        assert_eq!(heartbeat.test_req_id, Some(test_req_id));
    }

    #[test]
    fn test_heartbeat_is_test_response() {
        let heartbeat = Heartbeat::new();
        assert!(!heartbeat.is_test_response());

        let heartbeat_response = Heartbeat::new_response("TEST123".to_string());
        assert!(heartbeat_response.is_test_response());
    }

    #[test]
    fn test_test_request_new() {
        let test_req_id = "REQ456".to_string();
        let test_request = TestRequest::new(test_req_id.clone());
        assert_eq!(test_request.test_req_id, test_req_id);
    }

    #[test]
    fn test_test_request_with_timestamp() {
        let test_request = TestRequest::new_with_timestamp();
        assert!(!test_request.test_req_id.is_empty());
        // Should contain timestamp-based ID
        assert!(test_request.test_req_id.len() > 10);
    }

    #[test]
    fn test_resend_request_new() {
        let begin_seq = 10;
        let end_seq = 20;
        let resend_request = ResendRequest::new(begin_seq, end_seq);
        assert_eq!(resend_request.begin_seq_no, begin_seq);
        assert_eq!(resend_request.end_seq_no, end_seq);
    }

    #[test]
    fn test_resend_request_from_sequence() {
        let begin_seq = 15;
        let resend_request = ResendRequest::new_from_sequence(begin_seq);
        assert_eq!(resend_request.begin_seq_no, begin_seq);
        assert_eq!(resend_request.end_seq_no, 0); // 0 means all messages from begin_seq
    }

    #[test]
    fn test_resend_request_is_infinite_range() {
        let finite_request = ResendRequest::new(10, 20);
        assert!(!finite_request.is_infinite_range());

        let infinite_request = ResendRequest::new_from_sequence(10);
        assert!(infinite_request.is_infinite_range());
    }

    #[test]
    fn test_resend_request_message_count() {
        let request = ResendRequest::new(10, 15);
        assert_eq!(request.message_count(), Some(6)); // 10,11,12,13,14,15 = 6 messages

        let infinite_request = ResendRequest::new_from_sequence(10);
        assert_eq!(infinite_request.message_count(), None);
    }

    #[test]
    fn test_reject_new() {
        let ref_seq_num = 123;
        let reject = Reject::new(ref_seq_num);
        assert_eq!(reject.ref_seq_num, ref_seq_num);
        assert!(reject.ref_tag_id.is_none());
        assert!(reject.ref_msg_type.is_none());
        assert!(reject.session_reject_reason.is_none());
        assert!(reject.text.is_none());
    }

    #[test]
    fn test_reject_detailed() {
        let ref_seq_num = 456;
        let ref_tag_id = 35;
        let ref_msg_type = "D".to_string();
        let reason = SessionRejectReason::InvalidTagNumber;
        let text = "Invalid tag".to_string();

        let reject = Reject::new_detailed(
            ref_seq_num,
            Some(ref_tag_id),
            Some(ref_msg_type.clone()),
            Some(reason),
            Some(text.clone()),
        );

        assert_eq!(reject.ref_seq_num, ref_seq_num);
        assert_eq!(reject.ref_tag_id, Some(ref_tag_id));
        assert_eq!(reject.ref_msg_type, Some(ref_msg_type));
        assert_eq!(reject.session_reject_reason, Some(reason as u32));
        assert_eq!(reject.text, Some(text));
    }

    #[test]
    fn test_reject_invalid_tag() {
        let ref_seq_num = 789;
        let invalid_tag = 999;
        let reject = Reject::new_invalid_tag(ref_seq_num, invalid_tag);

        assert_eq!(reject.ref_seq_num, ref_seq_num);
        assert_eq!(reject.ref_tag_id, Some(invalid_tag));
        assert_eq!(
            reject.session_reject_reason,
            Some(SessionRejectReason::InvalidTagNumber as u32)
        );
    }

    #[test]
    fn test_session_reject_reason_values() {
        assert_eq!(SessionRejectReason::InvalidTagNumber as i32, 0);
        assert_eq!(SessionRejectReason::RequiredTagMissing as i32, 1);
        assert_eq!(SessionRejectReason::TagNotDefinedForMessageType as i32, 2);
        assert_eq!(SessionRejectReason::UndefinedTag as i32, 3);
        assert_eq!(SessionRejectReason::TagSpecifiedWithoutValue as i32, 4);
    }

    #[test]
    fn test_session_reject_reason_clone() {
        let reason = SessionRejectReason::InvalidTagNumber;
        let cloned = reason;
        assert_eq!(reason, cloned);
    }

    #[test]
    fn test_session_reject_reason_debug() {
        let reason = SessionRejectReason::RequiredTagMissing;
        let debug_str = format!("{reason:?}");
        assert!(debug_str.contains("RequiredTagMissing"));
    }
}
