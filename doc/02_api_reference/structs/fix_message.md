# FixMessage

## Overview

`FixMessage` represents a Financial Information eXchange (FIX) protocol message. It is the core data structure for all FIX communication in the Deribit system, encapsulating the header, body, and trailer components of a FIX message.

## Purpose

- **FIX protocol compliance**: Implements the standard FIX message structure
- **Message parsing**: Converts raw FIX strings to structured data
- **Message serialization**: Converts structured data back to FIX format
- **Message validation**: Ensures FIX message integrity and compliance
- **Protocol abstraction**: Provides a clean interface for FIX operations

## Public Interface

### Struct Definition

```rust
pub struct FixMessage {
    pub header: MessageHeader,
    pub body: MessageBody,
    pub trailer: MessageTrailer,
}
```

### Nested Components

#### MessageHeader

```rust
pub struct MessageHeader {
    pub begin_string: String,
    pub body_length: Option<u32>,
    pub msg_type: FixMsgType,
    pub sender_comp_id: String,
    pub target_comp_id: String,
    pub msg_seq_num: Option<u32>,
    pub sending_time: Option<DateTime<Utc>>,
    pub orig_sending_time: Option<DateTime<Utc>>,
    pub poss_dup_flag: Option<bool>,
    pub poss_resend: Option<bool>,
    pub encrypt_method: Option<u8>,
    pub heart_bt_int: Option<u32>,
    pub test_req_id: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}
```

#### MessageBody

```rust
pub struct MessageBody {
    pub fields: HashMap<FixField, String>,
    pub groups: HashMap<FixField, Vec<FixGroup>>,
}
```

#### MessageTrailer

```rust
pub struct MessageTrailer {
    pub check_sum: Option<u32>,
    pub signature: Option<String>,
    pub signature_length: Option<u32>,
}
```

#### FixGroup

```rust
pub struct FixGroup {
    pub fields: HashMap<FixField, String>,
    pub sub_groups: HashMap<FixField, Vec<FixGroup>>,
}
```

### Methods

```rust
impl FixMessage {
    /// Creates a new FIX message
    pub fn new(msg_type: FixMsgType, sender: String, target: String) -> Self
    
    /// Parses a FIX message from string
    pub fn from_fix_string(fix_string: &str) -> Result<Self, FixParseError>
    
    /// Converts the message to FIX string format
    pub fn to_fix_string(&self) -> Result<String, FixSerializationError>
    
    /// Validates the FIX message
    pub fn validate(&self) -> Result<(), Vec<FixValidationError>>
    
    /// Gets a field value
    pub fn get_field(&self, field: FixField) -> Option<&String>
    
    /// Sets a field value
    pub fn set_field(&mut self, field: FixField, value: String)
    
    /// Removes a field
    pub fn remove_field(&mut self, field: FixField) -> Option<String>
    
    /// Adds a repeating group
    pub fn add_group(&mut self, field: FixField, group: FixGroup)
    
    /// Gets repeating groups
    pub fn get_groups(&self, field: FixField) -> Option<&Vec<FixGroup>>
    
    /// Calculates and sets the body length
    pub fn calculate_body_length(&mut self)
    
    /// Calculates and sets the checksum
    pub fn calculate_checksum(&mut self)
}
```

## Usage Examples

### Creating FIX Messages

```rust
use deribit_fix::{FixMessage, FixMsgType, FixField};
use chrono::Utc;

// Create a new order single message
let mut message = FixMessage::new(
    FixMsgType::NewOrderSingle,
    "CLIENT".to_string(),
    "DERIBIT".to_string()
);

// Set header fields
message.header.msg_seq_num = Some(1);
message.header.sending_time = Some(Utc::now());

// Set body fields
message.set_field(FixField::ClOrdID, "ORDER_001".to_string());
message.set_field(FixField::Symbol, "BTC-PERPETUAL".to_string());
message.set_field(FixField::Side, "1".to_string()); // Buy
message.set_field(FixField::OrdType, "2".to_string()); // Limit
message.set_field(FixField::OrderQty, "1.0".to_string());
message.set_field(FixField::Price, "50000.0".to_string());
message.set_field(FixField::TimeInForce, "1".to_string()); // GTC

// Calculate body length and checksum
message.calculate_body_length();
message.calculate_checksum();

// Convert to FIX string
let fix_string = message.to_fix_string()?;
println!("FIX Message: {}", fix_string);
```

### Parsing FIX Messages

```rust
use deribit_fix::{FixMessage, FixField};

// Parse a FIX message from string
let fix_string = "8=FIX.4.4|9=123|35=D|49=CLIENT|56=DERIBIT|34=1|52=20231201-10:30:00|11=ORDER_001|21=1|55=BTC-PERPETUAL|54=1|60=2|38=1.0|40=2|44=50000.0|59=1|10=123|";

let message = FixMessage::from_fix_string(fix_string)?;

// Access message components
println!("Message Type: {:?}", message.header.msg_type);
println!("Sender: {}", message.header.sender_comp_id);
println!("Target: {}", message.header.target_comp_id);

// Access body fields
if let Some(cl_ord_id) = message.get_field(FixField::ClOrdID) {
    println!("Client Order ID: {}", cl_ord_id);
}

if let Some(symbol) = message.get_field(FixField::Symbol) {
    println!("Symbol: {}", symbol);
}

if let Some(side) = message.get_field(FixField::Side) {
    match side.as_str() {
        "1" => println!("Side: Buy"),
        "2" => println!("Side: Sell"),
        _ => println!("Side: Unknown"),
    }
}
```

### Working with Repeating Groups

```rust
use deribit_fix::{FixMessage, FixField, FixGroup};

// Create a message with repeating groups
let mut message = FixMessage::new(
    FixMsgType::SecurityList,
    "CLIENT".to_string(),
    "DERIBIT".to_string()
);

// Create a security group
let mut security_group = FixGroup::new();
security_group.set_field(FixField::Symbol, "BTC-PERPETUAL".to_string());
security_group.set_field(FixField::SecurityType, "FUT".to_string());
security_group.set_field(FixField::Currency, "USD".to_string());

// Add the group to the message
message.add_group(FixField::NoRelatedSym, security_group);

// Create another security group
let mut security_group2 = FixGroup::new();
security_group2.set_field(FixField::Symbol, "ETH-PERPETUAL".to_string());
security_group2.set_field(FixField::SecurityType, "FUT".to_string());
security_group2.set_field(FixField::Currency, "USD".to_string());

message.add_group(FixField::NoRelatedSym, security_group2);

// Access groups
if let Some(groups) = message.get_groups(FixField::NoRelatedSym) {
    for (i, group) in groups.iter().enumerate() {
        if let Some(symbol) = group.get_field(FixField::Symbol) {
            println!("Security {}: {}", i + 1, symbol);
        }
    }
}
```

### Message Validation

```rust
use deribit_fix::FixMessage;

fn validate_order_message(message: &FixMessage) -> Result<(), Vec<FixValidationError>> {
    // Validate the entire message
    message.validate()?;
    
    // Check required fields for order messages
    let required_fields = [
        FixField::ClOrdID,
        FixField::Symbol,
        FixField::Side,
        FixField::OrdType,
        FixField::OrderQty,
        FixField::TimeInForce,
    ];
    
    for field in &required_fields {
        if message.get_field(*field).is_none() {
            return Err(vec![FixValidationError::RequiredFieldMissing(*field)]);
        }
    }
    
    // Check order type specific requirements
    if let Some(ord_type) = message.get_field(FixField::OrdType) {
        match ord_type.as_str() {
            "2" => { // Limit order
                if message.get_field(FixField::Price).is_none() {
                    return Err(vec![FixValidationError::PriceRequiredForLimitOrder]);
                }
            },
            "1" => { // Market order
                if message.get_field(FixField::Price).is_some() {
                    return Err(vec![FixValidationError::PriceNotAllowedForMarketOrder]);
                }
            },
            _ => {}
        }
    }
    
    Ok(())
}

// Usage
let message = FixMessage::from_fix_string(fix_string)?;
validate_order_message(&message)?;
```

### Message Transformation

```rust
use deribit_fix::{FixMessage, Order, ExecutionReport};

// Convert Order to FIX message
impl From<Order> for FixMessage {
    fn from(order: Order) -> Self {
        let mut message = FixMessage::new(
            FixMsgType::NewOrderSingle,
            "CLIENT".to_string(),
            "DERIBIT".to_string()
        );
        
        // Set order fields
        if let Some(cl_ord_id) = order.client_order_id {
            message.set_field(FixField::ClOrdID, cl_ord_id);
        }
        
        message.set_field(FixField::Symbol, order.instrument);
        message.set_field(FixField::Side, order.side.to_fix_string());
        message.set_field(FixField::OrdType, order.order_type.to_fix_string());
        message.set_field(FixField::OrderQty, order.quantity.to_string());
        
        if let Some(price) = order.price {
            message.set_field(FixField::Price, price.to_string());
        }
        
        message.set_field(FixField::TimeInForce, order.time_in_force.to_fix_string());
        
        message
    }
}

// Convert FIX message to ExecutionReport
impl TryFrom<FixMessage> for ExecutionReport {
    type Error = FixParseError;
    
    fn try_from(message: FixMessage) -> Result<Self, Self::Error> {
        let order_id = message.get_field(FixField::OrderID)
            .ok_or(FixParseError::RequiredFieldMissing(FixField::OrderID))?
            .clone();
        
        let cl_ord_id = message.get_field(FixField::ClOrdID)
            .cloned();
        
        let symbol = message.get_field(FixField::Symbol)
            .ok_or(FixParseError::RequiredFieldMissing(FixField::Symbol))?
            .clone();
        
        let side = message.get_field(FixField::Side)
            .ok_or(FixParseError::RequiredFieldMissing(FixField::Side))?
            .parse()?;
        
        let ord_type = message.get_field(FixField::OrdType)
            .ok_or(FixParseError::RequiredFieldMissing(FixField::OrdType))?
            .parse()?;
        
        let order_qty = message.get_field(FixField::OrderQty)
            .ok_or(FixParseError::RequiredFieldMissing(FixField::OrderQty))?
            .parse()?;
        
        let price = message.get_field(FixField::Price)
            .and_then(|p| p.parse().ok());
        
        let exec_type = message.get_field(FixField::ExecType)
            .ok_or(FixParseError::RequiredFieldMissing(FixField::ExecType))?
            .parse()?;
        
        let ord_status = message.get_field(FixField::OrdStatus)
            .ok_or(FixParseError::RequiredFieldMissing(FixField::OrdStatus))?
            .parse()?;
        
        let cum_qty = message.get_field(FixField::CumQty)
            .unwrap_or("0.0")
            .parse()
            .unwrap_or(0.0);
        
        let avg_price = message.get_field(FixField::AvgPx)
            .and_then(|p| p.parse().ok());
        
        Ok(ExecutionReport {
            order_id,
            cl_ord_id,
            symbol,
            side,
            ord_type,
            order_qty,
            price,
            exec_type,
            ord_status,
            cum_qty,
            avg_price,
            ..Default::default()
        })
    }
}
```

## FIX Field Management

### Field Access and Modification

```rust
impl FixMessage {
    /// Gets a field value with type conversion
    pub fn get_field_as<T>(&self, field: FixField) -> Option<Result<T, FixParseError>>
    where
        T: FromStr,
        T::Err: std::error::Error + Send + Sync + 'static,
    {
        self.get_field(field)
            .map(|value| value.parse::<T>().map_err(|e| FixParseError::FieldParseError(field, e.to_string())))
    }
    
    /// Sets a field with automatic type conversion
    pub fn set_field_as<T>(&mut self, field: FixField, value: T)
    where
        T: ToString,
    {
        self.set_field(field, value.to_string());
    }
    
    /// Checks if a field exists
    pub fn has_field(&self, field: FixField) -> bool {
        self.get_field(field).is_some()
    }
    
    /// Gets all field names
    pub fn get_field_names(&self) -> Vec<FixField> {
        self.body.fields.keys().cloned().collect()
    }
    
    /// Gets all field values
    pub fn get_field_values(&self) -> Vec<&String> {
        self.body.fields.values().collect()
    }
}
```

### Field Validation

```rust
impl FixMessage {
    /// Validates required fields
    pub fn validate_required_fields(&self, required: &[FixField]) -> Result<(), Vec<FixField>> {
        let missing: Vec<FixField> = required.iter()
            .filter(|&&field| !self.has_field(field))
            .cloned()
            .collect();
        
        if missing.is_empty() {
            Ok(())
        } else {
            Err(missing)
        }
    }
    
    /// Validates field values
    pub fn validate_field_values(&self) -> Result<(), Vec<FixValidationError>> {
        let mut errors = Vec::new();
        
        for (field, value) in &self.body.fields {
            match field {
                FixField::OrderQty | FixField::Price | FixField::StopPx => {
                    if let Err(_) = value.parse::<f64>() {
                        errors.push(FixValidationError::InvalidFieldValue(*field, value.clone()));
                    }
                },
                FixField::Side => {
                    if !["1", "2"].contains(&value.as_str()) {
                        errors.push(FixValidationError::InvalidFieldValue(*field, value.clone()));
                    }
                },
                FixField::OrdType => {
                    if !["1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"].contains(&value.as_str()) {
                        errors.push(FixValidationError::InvalidFieldValue(*field, value.clone()));
                    }
                },
                _ => {}
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fix_message_creation() {
        let message = FixMessage::new(
            FixMsgType::Heartbeat,
            "CLIENT".to_string(),
            "DERIBIT".to_string()
        );
        
        assert_eq!(message.header.msg_type, FixMsgType::Heartbeat);
        assert_eq!(message.header.sender_comp_id, "CLIENT");
        assert_eq!(message.header.target_comp_id, "DERIBIT");
    }
    
    #[test]
    fn test_field_operations() {
        let mut message = FixMessage::new(
            FixMsgType::NewOrderSingle,
            "CLIENT".to_string(),
            "DERIBIT".to_string()
        );
        
        // Set field
        message.set_field(FixField::ClOrdID, "ORDER_001".to_string());
        
        // Check field exists
        assert!(message.has_field(FixField::ClOrdID));
        
        // Get field value
        assert_eq!(message.get_field(FixField::ClOrdID), Some(&"ORDER_001".to_string()));
        
        // Remove field
        let removed = message.remove_field(FixField::ClOrdID);
        assert_eq!(removed, Some("ORDER_001".to_string()));
        assert!(!message.has_field(FixField::ClOrdID));
    }
    
    #[test]
    fn test_message_validation() {
        let mut message = FixMessage::new(
            FixMsgType::NewOrderSingle,
            "CLIENT".to_string(),
            "DERIBIT".to_string()
        );
        
        // Should fail validation without required fields
        assert!(message.validate().is_err());
        
        // Add required fields
        message.set_field(FixField::ClOrdID, "ORDER_001".to_string());
        message.set_field(FixField::Symbol, "BTC-PERPETUAL".to_string());
        message.set_field(FixField::Side, "1".to_string());
        message.set_field(FixField::OrdType, "2".to_string());
        message.set_field(FixField::OrderQty, "1.0".to_string());
        message.set_field(FixField::TimeInForce, "1".to_string());
        message.set_field(FixField::Price, "50000.0".to_string());
        
        // Should pass validation
        assert!(message.validate().is_ok());
    }
}
```

### Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_fix_string_roundtrip() {
        let mut original = FixMessage::new(
            FixMsgType::NewOrderSingle,
            "CLIENT".to_string(),
            "DERIBIT".to_string()
        );
        
        original.set_field(FixField::ClOrdID, "ORDER_001".to_string());
        original.set_field(FixField::Symbol, "BTC-PERPETUAL".to_string());
        original.set_field(FixField::Side, "1".to_string());
        original.set_field(FixField::OrdType, "2".to_string());
        original.set_field(FixField::OrderQty, "1.0".to_string());
        original.set_field(FixField::TimeInForce, "1".to_string());
        original.set_field(FixField::Price, "50000.0".to_string());
        
        // Convert to string
        let fix_string = original.to_fix_string().unwrap();
        
        // Parse back
        let parsed = FixMessage::from_fix_string(&fix_string).unwrap();
        
        // Compare
        assert_eq!(original.header.msg_type, parsed.header.msg_type);
        assert_eq!(original.header.sender_comp_id, parsed.header.sender_comp_id);
        assert_eq!(original.header.target_comp_id, parsed.header.target_comp_id);
        assert_eq!(original.get_field(FixField::ClOrdID), parsed.get_field(FixField::ClOrdID));
        assert_eq!(original.get_field(FixField::Symbol), parsed.get_field(FixField::Symbol));
    }
    
    #[test]
    fn test_repeating_groups() {
        let mut message = FixMessage::new(
            FixMsgType::SecurityList,
            "CLIENT".to_string(),
            "DERIBIT".to_string()
        );
        
        // Create groups
        let mut group1 = FixGroup::new();
        group1.set_field(FixField::Symbol, "BTC-PERPETUAL".to_string());
        
        let mut group2 = FixGroup::new();
        group2.set_field(FixField::Symbol, "ETH-PERPETUAL".to_string());
        
        message.add_group(FixField::NoRelatedSym, group1);
        message.add_group(FixField::NoRelatedSym, group2);
        
        // Convert to string and back
        let fix_string = message.to_fix_string().unwrap();
        let parsed = FixMessage::from_fix_string(&fix_string).unwrap();
        
        // Check groups
        let groups = parsed.get_groups(FixField::NoRelatedSym).unwrap();
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].get_field(FixField::Symbol), Some(&"BTC-PERPETUAL".to_string()));
        assert_eq!(groups[1].get_field(FixField::Symbol), Some(&"ETH-PERPETUAL".to_string()));
    }
}
```

## Module Dependencies

- **types**: For `FixMsgType`, `FixField` enums
- **chrono**: For `DateTime<Utc>` timestamp handling
- **error**: For `FixParseError`, `FixSerializationError`, `FixValidationError`

## Related Types

- **MessageHeader**: FIX message header component
- **MessageBody**: FIX message body component
- **MessageTrailer**: FIX message trailer component
- **FixGroup**: Repeating group structure
- **FixField**: FIX field identifiers
- **FixMsgType**: FIX message types
- **FixParseError**: FIX parsing errors
- **FixSerializationError**: FIX serialization errors
- **FixValidationError**: FIX validation errors
