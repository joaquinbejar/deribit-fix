/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/
use crate::DeribitFixError;
use crate::model::types::MsgType;
use std::str::FromStr;

/// FIX message representation
#[derive(Clone)]
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
        self.get_field(34)?.parse().ok()
    }

    /// Check if a field exists
    pub fn has_field(&self, tag: u32) -> bool {
        self.fields.iter().any(|(t, _)| *t == tag)
    }

    /// Calculate checksum for the message
    pub fn calculate_checksum(&self) -> u8 {
        // Build message string without checksum field (tag 10), sorted by tag number
        let mut field_pairs: Vec<_> = self.fields.iter().collect();
        field_pairs.sort_by_key(|(tag, _)| *tag);

        let mut message_parts = Vec::new();

        // Add all fields except checksum (tag 10)
        for (tag, value) in field_pairs {
            if *tag != 10 {
                // Exclude checksum field
                message_parts.push(format!("{tag}={value}"));
            }
        }

        let message_str = message_parts.join("\x01") + "\x01";
        let bytes = message_str.as_bytes();
        let mut checksum: u32 = 0;

        // Sum all bytes in the message
        for &byte in bytes {
            checksum += byte as u32;
        }

        (checksum % 256) as u8
    }
}

impl FromStr for FixMessage {
    type Err = DeribitFixError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
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

impl std::fmt::Debug for FixMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Create a readable version by replacing SOH (\x01) with " | "
        let mut readable_message = self.raw_message.replace('\x01', " | ");
        // Remove trailing separator if present
        if readable_message.ends_with(" | ") {
            readable_message.truncate(readable_message.len() - 3);
        }

        // Get common FIX field names for better readability
        let mut field_descriptions = Vec::new();
        for (tag, value) in &self.fields {
            let field_name = match *tag {
                8 => "BeginString",
                9 => "BodyLength",
                35 => "MsgType",
                49 => "SenderCompID",
                56 => "TargetCompID",
                34 => "MsgSeqNum",
                52 => "SendingTime",
                10 => "CheckSum",
                11 => "ClOrdID",
                37 => "OrderID",
                38 => "OrderQty",
                39 => "OrdStatus",
                40 => "OrdType",
                44 => "Price",
                54 => "Side",
                55 => "Symbol",
                59 => "TimeInForce",
                95 => "SecureDataLen",
                96 => "SecureData",
                98 => "EncryptMethod",
                108 => "HeartBtInt",
                553 => "Username",
                554 => "Password",
                584 => "MassStatusReqID",
                585 => "MassStatusReqType",
                710 => "PosReqID",
                721 => "PosMaintRptID",
                _ => "Unknown",
            };
            field_descriptions.push(format!("{field_name}({tag})={value}"));
        }

        f.debug_struct("FixMessage")
            .field("fields", &field_descriptions)
            .field("readable_message", &readable_message)
            .finish()
    }
}
