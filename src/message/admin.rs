//! Administrative FIX Messages
//!
//! This module implements administrative messages used for session management
//! and connectivity testing in the FIX protocol. These messages are essential
//! for maintaining reliable communication between trading counterparties.
//!
//! ## Message Types Implemented
//!
//! - **Heartbeat (0)**: Periodic keep-alive messages to maintain session connectivity
//! - **Test Request (1)**: Request for heartbeat response to test connectivity  
//! - **Resend Request (2)**: Request to resend specific messages by sequence number range
//! - **Reject (3)**: Rejection of received messages due to validation errors
//! - **Business Message Reject (j)**: Business-level rejection of application messages

use crate::error::Result;
use crate::message::MessageBuilder;
use crate::model::message::FixMessage;
use crate::model::types::MsgType;
use chrono::Utc;
use deribit_base::impl_json_display;
use serde::{Deserialize, Serialize};

/// Heartbeat message (MsgType = 0)
///
/// The Heartbeat message is used to monitor the status of the communication link.
/// It is sent periodically to ensure the counterparty is still active and responsive.
/// If no messages are received within the heartbeat interval, a Test Request should be sent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Heartbeat {
    /// TestReqID (112) - Optional field echoed from Test Request
    /// Present only when responding to a Test Request message
    pub test_req_id: Option<String>,
}

/// Business-level reject reason codes (FIX 4.4, tag 380)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BusinessRejectReason {
    /// Other
    Other = 0,
    /// Unknown ID
    UnknownId = 1,
    /// Unknown Security
    UnknownSecurity = 2,
    /// Unsupported Message Type
    UnsupportedMessageType = 3,
    /// Application not available
    ApplicationNotAvailable = 4,
    /// Conditionally required field missing
    ConditionallyRequiredFieldMissing = 5,
    /// Not authorized
    NotAuthorized = 6,
    /// DeliverTo firm not available at this time
    DeliverToFirmNotAvailableAtThisTime = 7,
}

/// Business Message Reject (MsgType = j)
///
/// This message is used to reject application-level (business) messages
/// when they cannot be processed for business reasons.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BusinessMessageReject {
    /// RefMsgType (372) - Message type of the rejected application message (required)
    pub ref_msg_type: String,

    /// BusinessRejectReason (380) - Reason for rejection (required)
    pub business_reject_reason: BusinessRejectReason,

    /// BusinessRejectRefID (379) - ID from the rejected message for correlation (optional)
    pub business_reject_ref_id: Option<String>,

    /// Text (58) - Optional free-form text with details
    pub text: Option<String>,
}

/// Test Request message (MsgType = 1)
///
/// The Test Request message is sent to force a Heartbeat response from the counterparty.
/// It is typically used when no messages have been received within the expected heartbeat interval.
/// The receiving party must respond with a Heartbeat message containing the same TestReqID.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestRequest {
    /// TestReqID (112) - Unique identifier for this test request
    /// Must be echoed back in the responding Heartbeat message
    pub test_req_id: String,
}

/// Resend Request message (MsgType = 2)
///
/// The Resend Request message is sent to request retransmission of messages
/// within a specified sequence number range. This is used for gap recovery
/// when messages are detected as missing from the sequence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResendRequest {
    /// BeginSeqNo (7) - Starting sequence number for resend range
    pub begin_seq_no: u32,

    /// EndSeqNo (16) - Ending sequence number for resend range
    /// Set to 0 to request all messages from BeginSeqNo to current
    pub end_seq_no: u32,
}

/// Sequence Reset message (MsgType = 4)
///
/// The Sequence Reset message is used to recover from an out-of-sequence condition,
/// to reestablish a FIX session after a sequence loss. The MsgSeqNum(34) in the 
/// header is ignored.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SequenceReset {
    /// NewSeqNo (36) - New sequence number to reset to
    /// This can only increase the sequence number, never decrease it
    pub new_seq_no: u32,

    /// GapFillFlag (123) - Indicates if this is a gap fill
    /// Y = Gap Fill message, N = Sequence Reset message
    pub gap_fill_flag: Option<bool>,
}

/// Reject message (MsgType = 3)
///
/// The Reject message is sent when a received message cannot be processed
/// due to validation errors, formatting issues, or other problems.
/// It provides detailed information about why the message was rejected.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reject {
    /// RefSeqNum (45) - Sequence number of the rejected message
    pub ref_seq_num: u32,

    /// RefTagID (371) - Tag number of the field that caused rejection
    /// Optional field present when rejection is due to a specific field
    pub ref_tag_id: Option<u32>,

    /// RefMsgType (372) - Message type of the rejected message
    /// Optional field to identify which message type was rejected
    pub ref_msg_type: Option<String>,

    /// SessionRejectReason (373) - Reason code for rejection
    /// Standardized codes defined in FIX specification
    pub session_reject_reason: Option<u32>,

    /// Text (58) - Human-readable description of rejection reason
    /// Optional free-form text providing additional details
    pub text: Option<String>,
}

/// Session reject reason codes as defined in FIX specification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SessionRejectReason {
    /// Invalid tag number
    InvalidTagNumber = 0,
    /// Required tag missing
    RequiredTagMissing = 1,
    /// Tag not defined for this message type
    TagNotDefinedForMessageType = 2,
    /// Undefined tag
    UndefinedTag = 3,
    /// Tag specified without a value
    TagSpecifiedWithoutValue = 4,
    /// Value is incorrect (out of range) for this tag
    ValueIncorrectForTag = 5,
    /// Incorrect data format for value
    IncorrectDataFormat = 6,
    /// Decryption problem
    DecryptionProblem = 7,
    /// Signature problem
    SignatureProblem = 8,
    /// CompID problem
    CompIdProblem = 9,
    /// SendingTime accuracy problem
    SendingTimeAccuracyProblem = 10,
    /// Invalid MsgType
    InvalidMsgType = 11,
    /// XML validation error
    XmlValidationError = 12,
    /// Tag appears more than once
    TagAppearsMoreThanOnce = 13,
    /// Tag specified out of required order
    TagSpecifiedOutOfOrder = 14,
    /// Repeating group fields out of order
    RepeatingGroupFieldsOutOfOrder = 15,
    /// Incorrect NumInGroup count for repeating group
    IncorrectNumInGroupCount = 16,
    /// Non "data" value includes field delimiter
    NonDataValueIncludesFieldDelimiter = 17,
    /// Other
    Other = 99,
}

// Implement JSON display for all message types
impl_json_display!(Heartbeat);
impl_json_display!(TestRequest);
impl_json_display!(ResendRequest);
impl_json_display!(SequenceReset);
impl_json_display!(Reject);
impl_json_display!(BusinessMessageReject);

impl Heartbeat {
    /// Create a new Heartbeat message without TestReqID (periodic heartbeat)
    pub fn new() -> Self {
        Self { test_req_id: None }
    }

    /// Create a new Heartbeat message responding to a Test Request
    pub fn new_response(test_req_id: String) -> Self {
        Self {
            test_req_id: Some(test_req_id),
        }
    }

    /// Check if this heartbeat is a response to a test request
    pub fn is_test_response(&self) -> bool {
        self.test_req_id.is_some()
    }

    /// Build a FIX message for this Heartbeat
    pub fn to_fix_message(
        &self,
        sender_comp_id: String,
        target_comp_id: String,
        msg_seq_num: u32,
    ) -> Result<FixMessage> {
        let mut builder = MessageBuilder::new()
            .msg_type(MsgType::Heartbeat)
            .sender_comp_id(sender_comp_id)
            .target_comp_id(target_comp_id)
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now());

        // Add TestReqID if present
        if let Some(ref test_req_id) = self.test_req_id {
            builder = builder.field(112, test_req_id.clone());
        }

        builder.build()
    }
}

impl TestRequest {
    /// Create a new Test Request message with the specified ID
    pub fn new(test_req_id: String) -> Self {
        Self { test_req_id }
    }

    /// Generate a Test Request with a timestamp-based ID
    pub fn new_with_timestamp() -> Self {
        let test_req_id = format!("TESTREQ_{}", Utc::now().timestamp_millis());
        Self::new(test_req_id)
    }

    /// Build a FIX message for this Test Request
    pub fn to_fix_message(
        &self,
        sender_comp_id: String,
        target_comp_id: String,
        msg_seq_num: u32,
    ) -> Result<FixMessage> {
        MessageBuilder::new()
            .msg_type(MsgType::TestRequest)
            .sender_comp_id(sender_comp_id)
            .target_comp_id(target_comp_id)
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now())
            .field(112, self.test_req_id.clone()) // TestReqID
            .build()
    }
}

impl ResendRequest {
    /// Create a new Resend Request for a specific sequence range
    pub fn new(begin_seq_no: u32, end_seq_no: u32) -> Self {
        Self {
            begin_seq_no,
            end_seq_no,
        }
    }

    /// Create a Resend Request for all messages from the specified sequence number
    pub fn new_from_sequence(begin_seq_no: u32) -> Self {
        Self {
            begin_seq_no,
            end_seq_no: 0, // 0 means "all messages from begin_seq_no"
        }
    }

    /// Check if this request is for all messages from the begin sequence
    pub fn is_infinite_range(&self) -> bool {
        self.end_seq_no == 0
    }

    /// Get the number of messages requested (if not infinite range)
    pub fn message_count(&self) -> Option<u32> {
        if self.is_infinite_range() {
            None
        } else {
            Some(self.end_seq_no.saturating_sub(self.begin_seq_no) + 1)
        }
    }

    /// Build a FIX message for this Resend Request
    pub fn to_fix_message(
        &self,
        sender_comp_id: String,
        target_comp_id: String,
        msg_seq_num: u32,
    ) -> Result<FixMessage> {
        MessageBuilder::new()
            .msg_type(MsgType::ResendRequest)
            .sender_comp_id(sender_comp_id)
            .target_comp_id(target_comp_id)
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now())
            .field(7, self.begin_seq_no.to_string()) // BeginSeqNo
            .field(16, self.end_seq_no.to_string()) // EndSeqNo
            .build()
    }
}

impl SequenceReset {
    /// Create a new Sequence Reset message
    pub fn new(new_seq_no: u32) -> Self {
        Self {
            new_seq_no,
            gap_fill_flag: None,
        }
    }

    /// Create a gap fill Sequence Reset message
    pub fn new_gap_fill(new_seq_no: u32) -> Self {
        Self {
            new_seq_no,
            gap_fill_flag: Some(true),
        }
    }

    /// Create a sequence reset (not gap fill) message
    pub fn new_reset(new_seq_no: u32) -> Self {
        Self {
            new_seq_no,
            gap_fill_flag: Some(false),
        }
    }

    /// Check if this is a gap fill message
    pub fn is_gap_fill(&self) -> bool {
        self.gap_fill_flag.unwrap_or(false)
    }

    /// Build a FIX message for this Sequence Reset
    pub fn to_fix_message(
        &self,
        sender_comp_id: String,
        target_comp_id: String,
        msg_seq_num: u32,
    ) -> Result<FixMessage> {
        let mut builder = MessageBuilder::new()
            .msg_type(MsgType::SequenceReset)
            .sender_comp_id(sender_comp_id)
            .target_comp_id(target_comp_id)
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now())
            .field(36, self.new_seq_no.to_string()); // NewSeqNo

        // Add GapFillFlag if specified
        if let Some(gap_fill) = self.gap_fill_flag {
            builder = builder.field(123, if gap_fill { "Y" } else { "N" }.to_string());
        }

        builder.build()
    }
}

impl Reject {
    /// Create a new Reject message with minimal required fields
    pub fn new(ref_seq_num: u32) -> Self {
        Self {
            ref_seq_num,
            ref_tag_id: None,
            ref_msg_type: None,
            session_reject_reason: None,
            text: None,
        }
    }

    /// Create a Reject message with detailed rejection information
    pub fn new_detailed(
        ref_seq_num: u32,
        ref_tag_id: Option<u32>,
        ref_msg_type: Option<String>,
        session_reject_reason: Option<SessionRejectReason>,
        text: Option<String>,
    ) -> Self {
        Self {
            ref_seq_num,
            ref_tag_id,
            ref_msg_type,
            session_reject_reason: session_reject_reason.map(|r| r as u32),
            text,
        }
    }

    /// Create a Reject for invalid tag number
    pub fn new_invalid_tag(ref_seq_num: u32, tag_id: u32) -> Self {
        Self::new_detailed(
            ref_seq_num,
            Some(tag_id),
            None,
            Some(SessionRejectReason::InvalidTagNumber),
            Some(format!("Invalid tag number: {tag_id}")),
        )
    }

    /// Create a Reject for missing required tag
    pub fn new_missing_tag(ref_seq_num: u32, tag_id: u32, msg_type: String) -> Self {
        Self::new_detailed(
            ref_seq_num,
            Some(tag_id),
            Some(msg_type),
            Some(SessionRejectReason::RequiredTagMissing),
            Some(format!("Required tag {tag_id} missing")),
        )
    }

    /// Create a Reject for incorrect data format
    pub fn new_incorrect_format(ref_seq_num: u32, tag_id: u32, text: String) -> Self {
        Self::new_detailed(
            ref_seq_num,
            Some(tag_id),
            None,
            Some(SessionRejectReason::IncorrectDataFormat),
            Some(text),
        )
    }

    /// Build a FIX message for this Reject
    pub fn to_fix_message(
        &self,
        sender_comp_id: String,
        target_comp_id: String,
        msg_seq_num: u32,
    ) -> Result<FixMessage> {
        let mut builder = MessageBuilder::new()
            .msg_type(MsgType::Reject)
            .sender_comp_id(sender_comp_id)
            .target_comp_id(target_comp_id)
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now())
            .field(45, self.ref_seq_num.to_string()); // RefSeqNum

        // Add optional fields
        if let Some(ref_tag_id) = self.ref_tag_id {
            builder = builder.field(371, ref_tag_id.to_string()); // RefTagID
        }

        if let Some(ref ref_msg_type) = self.ref_msg_type {
            builder = builder.field(372, ref_msg_type.clone()); // RefMsgType
        }

        if let Some(session_reject_reason) = self.session_reject_reason {
            builder = builder.field(373, session_reject_reason.to_string()); // SessionRejectReason
        }

        if let Some(ref text) = self.text {
            builder = builder.field(58, text.clone()); // Text
        }

        builder.build()
    }
}

impl BusinessMessageReject {
    /// Create a new Business Message Reject
    pub fn new(ref_msg_type: String, business_reject_reason: BusinessRejectReason) -> Self {
        Self {
            ref_msg_type,
            business_reject_reason,
            business_reject_ref_id: None,
            text: None,
        }
    }

    /// Set BusinessRejectRefID (379)
    pub fn with_ref_id(mut self, business_reject_ref_id: String) -> Self {
        self.business_reject_ref_id = Some(business_reject_ref_id);
        self
    }

    /// Set Text (58)
    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    /// Build a FIX message for this Business Message Reject
    pub fn to_fix_message(
        &self,
        sender_comp_id: String,
        target_comp_id: String,
        msg_seq_num: u32,
    ) -> Result<FixMessage> {
        let mut builder = MessageBuilder::new()
            .msg_type(MsgType::BusinessMessageReject)
            .sender_comp_id(sender_comp_id)
            .target_comp_id(target_comp_id)
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now())
            .field(372, self.ref_msg_type.clone()) // RefMsgType
            .field(380, (self.business_reject_reason as u32).to_string()); // BusinessRejectReason

        if let Some(ref ref_id) = self.business_reject_ref_id {
            builder = builder.field(379, ref_id.clone()); // BusinessRejectRefID
        }

        if let Some(ref text) = self.text {
            builder = builder.field(58, text.clone()); // Text
        }

        builder.build()
    }
}

impl Default for Heartbeat {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heartbeat_creation() {
        let heartbeat = Heartbeat::new();
        assert_eq!(heartbeat.test_req_id, None);
        assert!(!heartbeat.is_test_response());

        let response = Heartbeat::new_response("TEST123".to_string());
        assert_eq!(response.test_req_id, Some("TEST123".to_string()));
        assert!(response.is_test_response());
    }

    #[test]
    fn test_test_request_creation() {
        let test_req = TestRequest::new("REQ123".to_string());
        assert_eq!(test_req.test_req_id, "REQ123");

        let timestamp_req = TestRequest::new_with_timestamp();
        assert!(timestamp_req.test_req_id.starts_with("TESTREQ_"));
    }

    #[test]
    fn test_resend_request_creation() {
        let resend = ResendRequest::new(10, 20);
        assert_eq!(resend.begin_seq_no, 10);
        assert_eq!(resend.end_seq_no, 20);
        assert!(!resend.is_infinite_range());
        assert_eq!(resend.message_count(), Some(11));

        let infinite = ResendRequest::new_from_sequence(15);
        assert_eq!(infinite.begin_seq_no, 15);
        assert_eq!(infinite.end_seq_no, 0);
        assert!(infinite.is_infinite_range());
        assert_eq!(infinite.message_count(), None);
    }

    #[test]
    fn test_reject_creation() {
        let basic_reject = Reject::new(123);
        assert_eq!(basic_reject.ref_seq_num, 123);
        assert_eq!(basic_reject.ref_tag_id, None);

        let invalid_tag = Reject::new_invalid_tag(456, 999);
        assert_eq!(invalid_tag.ref_seq_num, 456);
        assert_eq!(invalid_tag.ref_tag_id, Some(999));
        assert_eq!(
            invalid_tag.session_reject_reason,
            Some(SessionRejectReason::InvalidTagNumber as u32)
        );

        let missing_tag = Reject::new_missing_tag(789, 35, "D".to_string());
        assert_eq!(missing_tag.ref_seq_num, 789);
        assert_eq!(missing_tag.ref_tag_id, Some(35));
        assert_eq!(missing_tag.ref_msg_type, Some("D".to_string()));
        assert_eq!(
            missing_tag.session_reject_reason,
            Some(SessionRejectReason::RequiredTagMissing as u32)
        );
    }

    #[test]
    fn test_session_reject_reason_values() {
        assert_eq!(SessionRejectReason::InvalidTagNumber as u32, 0);
        assert_eq!(SessionRejectReason::RequiredTagMissing as u32, 1);
        assert_eq!(SessionRejectReason::Other as u32, 99);
    }

    #[test]
    fn test_heartbeat_to_fix_message() {
        let heartbeat = Heartbeat::new_response("TEST123".to_string());
        let fix_msg = heartbeat.to_fix_message("SENDER".to_string(), "TARGET".to_string(), 100);

        assert!(fix_msg.is_ok());
        let msg = fix_msg.unwrap();
        assert_eq!(msg.get_field(35), Some(&"0".to_string())); // MsgType = Heartbeat
        assert_eq!(msg.get_field(112), Some(&"TEST123".to_string())); // TestReqID
    }

    #[test]
    fn test_test_request_to_fix_message() {
        let test_req = TestRequest::new("REQ456".to_string());
        let fix_msg = test_req.to_fix_message("CLIENT".to_string(), "SERVER".to_string(), 200);

        assert!(fix_msg.is_ok());
        let msg = fix_msg.unwrap();
        assert_eq!(msg.get_field(35), Some(&"1".to_string())); // MsgType = TestRequest
        assert_eq!(msg.get_field(112), Some(&"REQ456".to_string())); // TestReqID
    }

    #[test]
    fn test_business_message_reject_to_fix_message() {
        let bmr = BusinessMessageReject::new(
            "D".to_string(),
            BusinessRejectReason::UnsupportedMessageType,
        )
        .with_ref_id("ABC123".to_string())
        .with_text("Unsupported type".to_string());

        let fix_msg = bmr.to_fix_message("SENDER".to_string(), "TARGET".to_string(), 77);
        assert!(fix_msg.is_ok());
        let msg = fix_msg.unwrap();
        assert_eq!(msg.get_field(35), Some(&"j".to_string())); // MsgType = BusinessMessageReject
        assert_eq!(msg.get_field(372), Some(&"D".to_string())); // RefMsgType
        assert_eq!(msg.get_field(379), Some(&"ABC123".to_string())); // BusinessRejectRefID
        assert_eq!(msg.get_field(380), Some(&(BusinessRejectReason::UnsupportedMessageType as u32).to_string())); // Reason
        assert_eq!(msg.get_field(58), Some(&"Unsupported type".to_string())); // Text
    }
}
