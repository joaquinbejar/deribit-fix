//! FIX message parsing and construction

use crate::error::{DeribitFixError, Result};
use crate::model::message::FixMessage;
use crate::model::types::MsgType;
use chrono::{DateTime, Utc};

/// Builder for constructing FIX messages
pub struct MessageBuilder {
    message: FixMessage,
}

impl MessageBuilder {
    /// Create a new message builder
    pub fn new() -> Self {
        let mut message = FixMessage::new();

        // Set standard fields
        message.set_field(8, "FIX.4.4".to_string()); // BeginString

        Self { message }
    }

    /// Set message type
    pub fn msg_type(mut self, msg_type: MsgType) -> Self {
        self.message.set_field(35, msg_type.as_str().to_string());
        self
    }

    /// Set sender company ID
    pub fn sender_comp_id(mut self, sender_comp_id: String) -> Self {
        self.message.set_field(49, sender_comp_id);
        self
    }

    /// Set target company ID
    pub fn target_comp_id(mut self, target_comp_id: String) -> Self {
        self.message.set_field(56, target_comp_id);
        self
    }

    /// Set message sequence number
    pub fn msg_seq_num(mut self, seq_num: u32) -> Self {
        self.message.set_field(34, seq_num.to_string());
        self
    }

    /// Set sending time
    pub fn sending_time(mut self, time: DateTime<Utc>) -> Self {
        let time_str = time.format("%Y%m%d-%H:%M:%S%.3f").to_string();
        self.message.set_field(52, time_str);
        self
    }

    /// Add a custom field
    pub fn field(mut self, tag: u32, value: String) -> Self {
        self.message.set_field(tag, value);
        self
    }

    /// Build the message
    pub fn build(mut self) -> Result<FixMessage> {
        // Isolate body fields and calculate body content string
        let body_content: String = self
            .message
            .fields
            .iter()
            .filter(|(t, _)| *t != 8 && *t != 9 && *t != 10)
            .map(|(tag, value)| format!("{tag}={value}\x01"))
            .collect();

        // Calculate body length
        let body_length = body_content.len();

        // Get BeginString
        let begin_string = self.message.get_field(8).ok_or_else(|| {
            DeribitFixError::MessageConstruction("Missing BeginString (8)".to_string())
        })?;

        // Construct header
        let header = format!("8={begin_string}\x019={body_length}\x01");

        // Combine for checksum calculation
        let message_for_checksum = format!("{header}{body_content}");

        // Calculate and append checksum
        let checksum = message_for_checksum
            .as_bytes()
            .iter()
            .map(|&b| b as u32)
            .sum::<u32>()
            % 256;
        self.message.raw_message = format!("{message_for_checksum}10={checksum:03}\x01");

        Ok(self.message)
    }
}

impl Default for MessageBuilder {
    fn default() -> Self {
        Self::new()
    }
}
