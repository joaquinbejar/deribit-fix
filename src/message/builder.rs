//! FIX message parsing and construction
//!
//! This module provides functionality for creating, parsing, and manipulating
//! FIX protocol messages used in communication with Deribit.

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
        // Validate required fields
        if !self.message.has_field(8) {
            return Err(DeribitFixError::MessageConstruction(
                "BeginString (8) is required".to_string(),
            ));
        }

        if !self.message.has_field(35) {
            return Err(DeribitFixError::MessageConstruction(
                "MsgType (35) is required".to_string(),
            ));
        }

        if !self.message.has_field(49) {
            return Err(DeribitFixError::MessageConstruction(
                "SenderCompID (49) is required".to_string(),
            ));
        }

        if !self.message.has_field(56) {
            return Err(DeribitFixError::MessageConstruction(
                "TargetCompID (56) is required".to_string(),
            ));
        }

        if !self.message.has_field(34) {
            return Err(DeribitFixError::MessageConstruction(
                "MsgSeqNum (34) is required".to_string(),
            ));
        }

        if !self.message.has_field(52) {
            // Set current time if not provided
            let now = Utc::now();
            let time_str = now.format("%Y%m%d-%H:%M:%S%.3f").to_string();
            self.message.set_field(52, time_str);
        }

        // Calculate BodyLength (all fields except BeginString and BodyLength itself)
        let body_length = self.calculate_body_length();
        self.message.set_field(9, body_length.to_string());

        // Calculate and set checksum
        let checksum = self.message.calculate_checksum();
        self.message.set_field(10, format!("{checksum:03}"));

        // Generate raw message string with proper FIX field ordering:
        // 1. BeginString (8) - first
        // 2. BodyLength (9) - second
        // 3. All other fields sorted by tag number
        // 4. CheckSum (10) - last
        let mut field_pairs: Vec<_> = self.message.fields.iter().collect();
        field_pairs.sort_by_key(|(tag, _)| *tag);

        let mut raw_parts = Vec::new();
        let mut checksum_part = None;

        // Add BeginString first if present
        if let Some((_, value)) = field_pairs.iter().find(|(tag, _)| *tag == 8) {
            raw_parts.push(format!("8={value}"));
        }

        // Add BodyLength second if present
        if let Some((_, value)) = field_pairs.iter().find(|(tag, _)| *tag == 9) {
            raw_parts.push(format!("9={value}"));
        }

        // Add all other fields except BeginString, BodyLength, and CheckSum
        for (tag, value) in field_pairs {
            if *tag == 10 {
                // Save checksum for last
                checksum_part = Some(format!("{tag}={value}"));
            } else if *tag != 8 && *tag != 9 {
                raw_parts.push(format!("{tag}={value}"));
            }
        }

        // Add checksum at the end
        if let Some(checksum) = checksum_part {
            raw_parts.push(checksum);
        }

        self.message.raw_message = raw_parts.join("\x01") + "\x01";

        Ok(self.message)
    }

    /// Calculate the body length for the FIX message
    /// Body length includes all fields except BeginString (8), BodyLength (9), and CheckSum (10)
    fn calculate_body_length(&self) -> usize {
        let mut body_parts = Vec::new();

        // Add all fields except BeginString (8), BodyLength (9), and CheckSum (10)
        for (tag, value) in &self.message.fields {
            if *tag != 8 && *tag != 9 && *tag != 10 {
                body_parts.push(format!("{tag}={value}"));
            }
        }

        // Join with SOH and add final SOH
        let body_str = body_parts.join("\x01") + "\x01";
        body_str.len()
    }
}

impl Default for MessageBuilder {
    fn default() -> Self {
        Self::new()
    }
}
