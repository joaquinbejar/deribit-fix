# message Module

## Overview

The `message` module provides FIX message handling for the `deribit-fix` crate, including message parsing, serialization, validation, and transformation between Rust types and FIX protocol format.

## Purpose

- **FIX Message Handling**: Parse and serialize FIX protocol messages
- **Message Validation**: Ensure FIX messages conform to protocol specifications
- **Type Conversion**: Convert between Rust types and FIX message format
- **Protocol Compliance**: Maintain FIX 4.4 compliance with Deribit extensions

## Public Interface

### Main Message Struct

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixMessage {
    pub header: MessageHeader,
    pub body: MessageBody,
    pub trailer: MessageTrailer,
}
```

### Message Components

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageHeader {
    pub begin_string: String,      // 8: FIX version
    pub body_length: u32,          // 9: Message length
    pub msg_type: FixMsgType,      // 35: Message type
    pub sender_comp_id: String,    // 49: Sender company ID
    pub target_comp_id: String,    // 56: Target company ID
    pub msg_seq_num: u32,          // 34: Sequence number
    pub sending_time: DateTime<Utc>, // 52: Sending timestamp
    pub orig_sending_time: Option<DateTime<Utc>>, // 122: Original sending time
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageBody {
    pub fields: HashMap<FixField, String>,
    pub groups: HashMap<FixField, Vec<HashMap<FixField, String>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageTrailer {
    pub check_sum: u32,            // 10: Message checksum
    pub signature: Option<String>,  // 89: Digital signature
    pub signature_length: Option<u32>, // 93: Signature length
}
```

### FIX Field Types

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FixField {
    // Standard FIX 4.4 fields
    BeginString = 8,
    BodyLength = 9,
    MsgType = 35,
    SenderCompID = 49,
    TargetCompID = 56,
    MsgSeqNum = 34,
    SendingTime = 52,
    OrigSendingTime = 122,
    CheckSum = 10,
    
    // Order-related fields
    ClOrdID = 11,
    OrderQty = 38,
    OrdType = 40,
    Side = 54,
    TimeInForce = 59,
    Price = 44,
    StopPx = 99,
    Symbol = 55,
    SecurityType = 167,
    MaturityMonthYear = 200,
    MaturityDay = 205,
    StrikePrice = 202,
    PutOrCall = 201,
    
    // Execution fields
    ExecID = 17,
    ExecType = 20,
    OrdStatus = 39,
    LeavesQty = 151,
    CumQty = 14,
    AvgPx = 6,
    
    // Market data fields
    MDReqID = 262,
    SubscriptionRequestType = 263,
    MarketDepth = 264,
    MDUpdateType = 265,
    NoMDEntries = 268,
    MDEntryType = 269,
    MDEntryPx = 270,
    MDEntrySize = 271,
    MDEntryTime = 273,
    
    // Deribit-specific fields
    DeribitInstrumentID = 10000,
    DeribitOrderType = 10001,
    DeribitTimeInForce = 10002,
    DeribitPostOnly = 10003,
    DeribitReduceOnly = 10004,
}
```

### Message Types

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FixMsgType {
    // Session messages
    Heartbeat = 0,
    TestRequest = 1,
    ResendRequest = 2,
    Reject = 3,
    SequenceReset = 4,
    Logout = 5,
    Logon = 35,
    
    // Order management
    NewOrderSingle = 39,
    OrderCancelRequest = 40,
    OrderCancelReplaceRequest = 41,
    OrderStatusRequest = 42,
    OrderMassCancelRequest = 43,
    OrderMassStatusRequest = 44,
    
    // Execution reports
    ExecutionReport = 8,
    OrderCancelReject = 9,
    
    // Market data
    MarketDataRequest = 35,
    MarketDataSnapshotFullRefresh = 36,
    MarketDataIncrementalRefresh = 37,
    MarketDataRequestReject = 38,
    
    // Security information
    SecurityListRequest = 35,
    SecurityList = 35,
    SecurityDefinitionRequest = 35,
    SecurityDefinition = 35,
    
    // Trade reporting
    TradeCaptureReportRequest = 35,
    TradeCaptureReportRequestAck = 35,
    TradeCaptureReport = 35,
    
    // Quote management
    QuoteRequest = 35,
    Quote = 35,
    QuoteCancel = 35,
    QuoteStatusReport = 35,
}
```

## Usage Examples

### Creating FIX Messages

```rust
use deribit_fix::message::{FixMessage, MessageHeader, MessageBody, MessageTrailer, FixField, FixMsgType};
use chrono::Utc;

// Create a New Order Single message
fn create_new_order_single(
    cl_ord_id: &str,
    symbol: &str,
    side: &str,
    order_type: &str,
    quantity: f64,
    price: Option<f64>,
) -> FixMessage {
    let mut body_fields = HashMap::new();
    body_fields.insert(FixField::ClOrdID, cl_ord_id.to_string());
    body_fields.insert(FixField::Symbol, symbol.to_string());
    body_fields.insert(FixField::Side, side.to_string());
    body_fields.insert(FixField::OrdType, order_type.to_string());
    body_fields.insert(FixField::OrderQty, quantity.to_string());
    
    if let Some(price) = price {
        body_fields.insert(FixField::Price, price.to_string());
    }
    
    let header = MessageHeader {
        begin_string: "FIX.4.4".to_string(),
        body_length: 0, // Will be calculated
        msg_type: FixMsgType::NewOrderSingle,
        sender_comp_id: "CLIENT".to_string(),
        target_comp_id: "DERIBIT".to_string(),
        msg_seq_num: 1,
        sending_time: Utc::now(),
        orig_sending_time: None,
    };
    
    let body = MessageBody {
        fields: body_fields,
        groups: HashMap::new(),
    };
    
    let trailer = MessageTrailer {
        check_sum: 0, // Will be calculated
        signature: None,
        signature_length: None,
    };
    
    let mut message = FixMessage { header, body, trailer };
    message.calculate_checksum();
    message.calculate_body_length();
    
    message
}
```

### Parsing FIX Messages

```rust
use deribit_fix::message::FixMessage;

impl FixMessage {
    pub fn from_fix_string(fix_string: &str) -> Result<Self, FixParseError> {
        let mut fields = HashMap::new();
        let mut groups = HashMap::new();
        
        // Split by SOH (Start of Header) character
        let parts: Vec<&str> = fix_string.split('\u{0001}').collect();
        
        for part in parts {
            if part.is_empty() {
                continue;
            }
            
            let field_parts: Vec<&str> = part.split('=').collect();
            if field_parts.len() != 2 {
                return Err(FixParseError::InvalidFieldFormat);
            }
            
            let tag = field_parts[0].parse::<u32>()?;
            let value = field_parts[1];
            
            if let Ok(fix_field) = FixField::try_from(tag) {
                fields.insert(fix_field, value.to_string());
            }
        }
        
        // Parse header, body, and trailer
        let header = Self::parse_header(&fields)?;
        let body = MessageBody { fields, groups };
        let trailer = Self::parse_trailer(&fields)?;
        
        Ok(FixMessage { header, body, trailer })
    }
    
    fn parse_header(fields: &HashMap<FixField, String>) -> Result<MessageHeader, FixParseError> {
        let begin_string = fields.get(&FixField::BeginString)
            .ok_or(FixParseError::MissingRequiredField("BeginString"))?
            .clone();
            
        let msg_type = fields.get(&FixField::MsgType)
            .ok_or(FixParseError::MissingRequiredField("MsgType"))?
            .parse()?;
            
        let sender_comp_id = fields.get(&FixField::SenderCompID)
            .ok_or(FixParseError::MissingRequiredField("SenderCompID"))?
            .clone();
            
        let target_comp_id = fields.get(&FixField::TargetCompID)
            .ok_or(FixParseError::MissingRequiredField("TargetCompID"))?
            .clone();
            
        let msg_seq_num = fields.get(&FixField::MsgSeqNum)
            .ok_or(FixParseError::MissingRequiredField("MsgSeqNum"))?
            .parse()?;
            
        let sending_time = fields.get(&FixField::SendingTime)
            .ok_or(FixParseError::MissingRequiredField("SendingTime"))?
            .parse()?;
            
        let orig_sending_time = fields.get(&FixField::OrigSendingTime)
            .map(|s| s.parse())
            .transpose()?;
            
        Ok(MessageHeader {
            begin_string,
            body_length: 0, // Will be set later
            msg_type,
            sender_comp_id,
            target_comp_id,
            msg_seq_num,
            sending_time,
            orig_sending_time,
        })
    }
}
```

### Message Validation

```rust
impl FixMessage {
    pub fn validate(&self) -> Result<(), Vec<FixValidationError>> {
        let mut errors = Vec::new();
        
        // Validate required header fields
        if self.header.begin_string.is_empty() {
            errors.push(FixValidationError::MissingRequiredField("BeginString"));
        }
        
        if self.header.sender_comp_id.is_empty() {
            errors.push(FixValidationError::MissingRequiredField("SenderCompID"));
        }
        
        if self.header.target_comp_id.is_empty() {
            errors.push(FixValidationError::MissingRequiredField("TargetCompID"));
        }
        
        // Validate message type specific requirements
        match self.header.msg_type {
            FixMsgType::NewOrderSingle => {
                self.validate_new_order_single(&mut errors);
            }
            FixMsgType::OrderCancelRequest => {
                self.validate_order_cancel_request(&mut errors);
            }
            FixMsgType::MarketDataRequest => {
                self.validate_market_data_request(&mut errors);
            }
            _ => {}
        }
        
        // Validate checksum
        let calculated_checksum = self.calculate_checksum();
        if calculated_checksum != self.trailer.check_sum {
            errors.push(FixValidationError::InvalidChecksum {
                expected: calculated_checksum,
                actual: self.trailer.check_sum,
            });
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
    
    fn validate_new_order_single(&self, errors: &mut Vec<FixValidationError>) {
        let required_fields = [
            FixField::ClOrdID,
            FixField::Symbol,
            FixField::Side,
            FixField::OrdType,
            FixField::OrderQty,
        ];
        
        for field in &required_fields {
            if !self.body.fields.contains_key(field) {
                errors.push(FixValidationError::MissingRequiredField(
                    format!("{:?}", field)
                ));
            }
        }
        
        // Validate side field
        if let Some(side) = self.body.fields.get(&FixField::Side) {
            if !["1", "2"].contains(&side.as_str()) {
                errors.push(FixValidationError::InvalidFieldValue {
                    field: "Side".to_string(),
                    value: side.clone(),
                    reason: "Must be 1 (Buy) or 2 (Sell)".to_string(),
                });
            }
        }
        
        // Validate order type
        if let Some(ord_type) = self.body.fields.get(&FixField::OrdType) {
            if !["1", "2", "3", "4", "5"].contains(&ord_type.as_str()) {
                errors.push(FixValidationError::InvalidFieldValue {
                    field: "OrdType".to_string(),
                    value: ord_type.clone(),
                    reason: "Invalid order type".to_string(),
                });
            }
        }
    }
}
```

### Message Serialization

```rust
impl FixMessage {
    pub fn to_fix_string(&self) -> Result<String, FixSerializationError> {
        let mut parts = Vec::new();
        
        // Add header fields
        parts.push(format!("{}={}", FixField::BeginString as u32, self.header.begin_string));
        parts.push(format!("{}={}", FixField::MsgType as u32, self.header.msg_type as u8));
        parts.push(format!("{}={}", FixField::SenderCompID as u32, self.header.sender_comp_id));
        parts.push(format!("{}={}", FixField::TargetCompID as u32, self.header.target_comp_id));
        parts.push(format!("{}={}", FixField::MsgSeqNum as u32, self.header.msg_seq_num));
        parts.push(format!("{}={}", FixField::SendingTime as u32, self.header.sending_time.format("%Y%m%d-%H:%M:%S.%3f")));
        
        // Add body fields
        for (field, value) in &self.body.fields {
            parts.push(format!("{}={}", *field as u32, value));
        }
        
        // Add trailer fields
        parts.push(format!("{}={:03}", FixField::CheckSum as u32, self.trailer.check_sum));
        
        // Join with SOH character
        let fix_string = parts.join("\u{0001}");
        
        Ok(fix_string)
    }
    
    pub fn calculate_checksum(&mut self) -> u32 {
        let fix_string = self.to_fix_string().unwrap();
        let bytes = fix_string.as_bytes();
        
        let mut checksum: u32 = 0;
        for &byte in bytes {
            checksum = (checksum + byte as u32) % 256;
        }
        
        self.trailer.check_sum = checksum;
        checksum
    }
    
    pub fn calculate_body_length(&mut self) -> u32 {
        let body_string = self.body.to_fix_string().unwrap();
        let length = body_string.len() as u32;
        
        self.header.body_length = length;
        length
    }
}
```

## Message Transformation

### From Rust Types to FIX

```rust
use deribit_fix::types::Order;

impl From<Order> for FixMessage {
    fn from(order: Order) -> Self {
        let mut body_fields = HashMap::new();
        
        body_fields.insert(FixField::ClOrdID, order.id);
        body_fields.insert(FixField::Symbol, order.instrument);
        body_fields.insert(FixField::Side, order.side.to_string());
        body_fields.insert(FixField::OrdType, order.order_type.to_string());
        body_fields.insert(FixField::OrderQty, order.quantity.to_string());
        
        if let Some(price) = order.price {
            body_fields.insert(FixField::Price, price.to_string());
        }
        
        if let Some(stop_price) = order.stop_price {
            body_fields.insert(FixField::StopPx, stop_price.to_string());
        }
        
        body_fields.insert(FixField::TimeInForce, order.time_in_force.to_string());
        
        let header = MessageHeader {
            begin_string: "FIX.4.4".to_string(),
            body_length: 0,
            msg_type: FixMsgType::NewOrderSingle,
            sender_comp_id: "CLIENT".to_string(),
            target_comp_id: "DERIBIT".to_string(),
            msg_seq_num: 1,
            sending_time: Utc::now(),
            orig_sending_time: None,
        };
        
        let body = MessageBody {
            fields: body_fields,
            groups: HashMap::new(),
        };
        
        let trailer = MessageTrailer {
            check_sum: 0,
            signature: None,
            signature_length: None,
        };
        
        let mut message = FixMessage { header, body, trailer };
        message.calculate_checksum();
        message.calculate_body_length();
        
        message
    }
}
```

### From FIX to Rust Types

```rust
use deribit_fix::types::ExecutionReport;

impl TryFrom<FixMessage> for ExecutionReport {
    type Error = FixParseError;
    
    fn try_from(message: FixMessage) -> Result<Self, Self::Error> {
        if message.header.msg_type != FixMsgType::ExecutionReport {
            return Err(FixParseError::InvalidMessageType);
        }
        
        let fields = &message.body.fields;
        
        let order_id = fields.get(&FixField::ClOrdID)
            .ok_or(FixParseError::MissingRequiredField("ClOrdID"))?
            .clone();
            
        let exec_id = fields.get(&FixField::ExecID)
            .ok_or(FixParseError::MissingRequiredField("ExecID"))?
            .clone();
            
        let exec_type = fields.get(&FixField::ExecType)
            .ok_or(FixParseError::MissingRequiredField("ExecType"))?
            .parse()?;
            
        let ord_status = fields.get(&FixField::OrdStatus)
            .ok_or(FixParseError::MissingRequiredField("OrdStatus"))?
            .parse()?;
            
        let symbol = fields.get(&FixField::Symbol)
            .ok_or(FixParseError::MissingRequiredField("Symbol"))?
            .clone();
            
        let side = fields.get(&FixField::Side)
            .ok_or(FixParseError::MissingRequiredField("Side"))?
            .parse()?;
            
        let leaves_qty = fields.get(&FixField::LeavesQty)
            .unwrap_or(&"0".to_string())
            .parse()
            .unwrap_or(0.0);
            
        let cum_qty = fields.get(&FixField::CumQty)
            .unwrap_or(&"0".to_string())
            .parse()
            .unwrap_or(0.0);
            
        let avg_px = fields.get(&FixField::AvgPx)
            .map(|s| s.parse())
            .transpose()?;
            
        Ok(ExecutionReport {
            order_id,
            exec_id,
            exec_type,
            ord_status,
            symbol,
            side,
            leaves_qty,
            cum_qty,
            avg_px,
            timestamp: message.header.sending_time,
        })
    }
}
```

## Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let message = create_new_order_single(
            "ORDER001",
            "BTC-PERPETUAL",
            "1",
            "2",
            1.0,
            Some(50000.0),
        );
        
        assert_eq!(message.header.msg_type, FixMsgType::NewOrderSingle);
        assert_eq!(message.body.fields.get(&FixField::ClOrdID).unwrap(), "ORDER001");
        assert_eq!(message.body.fields.get(&FixField::Symbol).unwrap(), "BTC-PERPETUAL");
    }

    #[test]
    fn test_message_validation() {
        let mut message = create_new_order_single(
            "ORDER001",
            "BTC-PERPETUAL",
            "1",
            "2",
            1.0,
            Some(50000.0),
        );
        
        // Should be valid
        assert!(message.validate().is_ok());
        
        // Remove required field
        message.body.fields.remove(&FixField::ClOrdID);
        
        // Should be invalid
        let result = message.validate();
        assert!(result.is_err());
        
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| matches!(e, FixValidationError::MissingRequiredField(_))));
    }

    #[test]
    fn test_message_serialization() {
        let message = create_new_order_single(
            "ORDER001",
            "BTC-PERPETUAL",
            "1",
            "2",
            1.0,
            Some(50000.0),
        );
        
        let fix_string = message.to_fix_string().unwrap();
        
        // Should contain key fields
        assert!(fix_string.contains("35=D")); // MsgType = NewOrderSingle
        assert!(fix_string.contains("11=ORDER001")); // ClOrdID
        assert!(fix_string.contains("55=BTC-PERPETUAL")); // Symbol
        assert!(fix_string.contains("54=1")); // Side = Buy
        assert!(fix_string.contains("40=2")); // OrdType = Limit
    }
}
```

## Module Dependencies

- `serde`: Serialization/deserialization
- `chrono`: Timestamp handling
- `thiserror`: Error types
- `std::collections::HashMap`: Field storage
