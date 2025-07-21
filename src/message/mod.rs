//! FIX message parsing and construction

use crate::{
    error::{DeribitFixError, Result},
    types::MsgType,
};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use tracing::{warn};

/// FIX message representation
#[derive(Debug, Clone)]
pub struct FixMessage {
    pub fields: HashMap<u32, String>,
    pub raw_message: String,
}

impl FixMessage {
    /// Create a new empty FIX message
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            raw_message: String::new(),
        }
    }

    /// Parse a FIX message from a string
    pub fn parse(raw_message: &str) -> Result<Self> {
        let mut fields = HashMap::new();
        
        // Split by SOH character (ASCII 1)
        let parts: Vec<&str> = raw_message.split('\x01').collect();
        
        for part in parts {
            if part.is_empty() {
                continue;
            }
            
            if let Some(eq_pos) = part.find('=') {
                let tag_str = &part[..eq_pos];
                let value = &part[eq_pos + 1..];
                
                if let Ok(tag) = tag_str.parse::<u32>() {
                    fields.insert(tag, value.to_string());
                } else {
                    warn!("Invalid FIX tag: {}", tag_str);
                }
            }
        }

        // Validate required fields
        if !fields.contains_key(&8) {  // BeginString
            return Err(DeribitFixError::MessageParsing("Missing BeginString (8)".to_string()));
        }
        
        if !fields.contains_key(&35) { // MsgType
            return Err(DeribitFixError::MessageParsing("Missing MsgType (35)".to_string()));
        }

        Ok(Self {
            fields,
            raw_message: raw_message.to_string(),
        })
    }

    /// Get a field value by tag
    pub fn get_field(&self, tag: u32) -> Option<&String> {
        self.fields.get(&tag)
    }

    /// Set a field value
    pub fn set_field(&mut self, tag: u32, value: String) {
        self.fields.insert(tag, value);
    }

    /// Get message type
    pub fn msg_type(&self) -> Option<MsgType> {
        self.get_field(35)
            .and_then(|s| MsgType::from_str(s))
    }

    /// Get sender company ID
    pub fn sender_comp_id(&self) -> Option<&String> {
        self.get_field(49)
    }

    /// Get target company ID
    pub fn target_comp_id(&self) -> Option<&String> {
        self.get_field(56)
    }

    /// Get message sequence number
    pub fn msg_seq_num(&self) -> Option<u32> {
        self.get_field(34)
            .and_then(|s| s.parse().ok())
    }

    /// Convert to FIX string format
    pub fn to_string(&self) -> String {
        let mut message = String::new();
        
        // Standard FIX message order: BeginString, BodyLength, MsgType, then other fields
        let ordered_tags = [8, 9, 35]; // BeginString, BodyLength, MsgType
        
        // Add ordered fields first
        for &tag in &ordered_tags {
            if let Some(value) = self.fields.get(&tag) {
                message.push_str(&format!("{}={}\x01", tag, value));
            }
        }
        
        // Add remaining fields (except checksum which goes last)
        for (&tag, value) in &self.fields {
            if !ordered_tags.contains(&tag) && tag != 10 { // Skip checksum
                message.push_str(&format!("{}={}\x01", tag, value));
            }
        }
        
        // Calculate and add checksum
        let checksum = self.calculate_checksum(&message);
        message.push_str(&format!("10={:03}\x01", checksum));
        
        message
    }

    /// Calculate FIX checksum
    fn calculate_checksum(&self, message: &str) -> u8 {
        let sum: u32 = message.bytes().map(|b| b as u32).sum();
        (sum % 256) as u8
    }

    /// Validate message integrity
    pub fn validate(&self) -> Result<()> {
        // Check required fields
        if !self.fields.contains_key(&8) {
            return Err(DeribitFixError::MessageParsing("Missing BeginString".to_string()));
        }
        
        if !self.fields.contains_key(&35) {
            return Err(DeribitFixError::MessageParsing("Missing MsgType".to_string()));
        }
        
        if !self.fields.contains_key(&49) {
            return Err(DeribitFixError::MessageParsing("Missing SenderCompID".to_string()));
        }
        
        if !self.fields.contains_key(&56) {
            return Err(DeribitFixError::MessageParsing("Missing TargetCompID".to_string()));
        }
        
        if !self.fields.contains_key(&34) {
            return Err(DeribitFixError::MessageParsing("Missing MsgSeqNum".to_string()));
        }

        Ok(())
    }
}

impl Default for FixMessage {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for FixMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

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
        // Calculate body length (all fields except BeginString, BodyLength, and Checksum)
        let temp_message = self.message.to_string();
        let body_start = temp_message.find("35=").unwrap_or(0);
        let body_end = temp_message.rfind("10=").unwrap_or(temp_message.len());
        let body_length = body_end - body_start;
        
        self.message.set_field(9, body_length.to_string());
        
        // Validate the message
        self.message.validate()?;
        
        Ok(self.message)
    }
}

impl Default for MessageBuilder {
    fn default() -> Self {
        Self::new()
    }
}
