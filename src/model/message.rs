/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/
use crate::DeribitFixError;
use crate::model::types::MsgType;

/// FIX message representation
#[derive(Debug, Clone)]
pub struct FixMessage {
    /// FIX message fields as (tag, value) pairs
    pub fields: Vec<(u32, String)>,
    /// Raw message string
    pub raw_message: String,
}

impl FixMessage {
    /// Create a new empty FIX message
    pub fn new() -> Self {
        Self {
            fields: Vec::new(),
            raw_message: String::new(),
        }
    }

    /// Parse a FIX message from a string
    pub fn parse(raw_message: &str) -> crate::Result<Self> {
        let mut fields = Vec::new();
        for part in raw_message.split('\x01').filter(|s| !s.is_empty()) {
            let mut pair = part.splitn(2, '=');
            if let (Some(tag_str), Some(value)) = (pair.next(), pair.next()) {
                if let Ok(tag) = tag_str.parse::<u32>() {
                    fields.push((tag, value.to_string()));
                } else {
                    return Err(DeribitFixError::MessageParsing(format!(
                        "Invalid tag: {tag_str}"
                    )));
                }
            } else {
                return Err(DeribitFixError::MessageParsing(format!(
                    "Invalid field: {part}"
                )));
            }
        }

        Ok(Self {
            fields,
            raw_message: raw_message.to_string(),
        })
    }

    /// Get a field value by tag
    pub fn get_field(&self, tag: u32) -> Option<&String> {
        self.fields.iter().find(|(t, _)| *t == tag).map(|(_, v)| v)
    }

    /// Set a field value
    pub fn set_field(&mut self, tag: u32, value: String) {
        if let Some(field) = self.fields.iter_mut().find(|(t, _)| *t == tag) {
            field.1 = value;
        } else {
            self.fields.push((tag, value));
        }
    }

    /// Get message type
    pub fn msg_type(&self) -> Option<MsgType> {
        self.get_field(35).and_then(|s| s.parse().ok())
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
        self.get_field(34).and_then(|s| s.parse().ok())
    }
}

impl Default for FixMessage {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for FixMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw_message)
    }
}
