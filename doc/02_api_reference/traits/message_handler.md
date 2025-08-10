# MessageHandler Trait

## Overview

The `MessageHandler` trait defines the interface for processing and handling FIX protocol messages within the Deribit FIX client. It provides methods for parsing, validating, transforming, and routing FIX messages between the client and the exchange.

## Purpose

- **Message Processing**: Handle incoming and outgoing FIX messages
- **Protocol Compliance**: Ensure messages conform to FIX protocol standards
- **Message Routing**: Direct messages to appropriate handlers based on type
- **Validation**: Validate message structure and business logic
- **Transformation**: Convert between internal types and FIX format

## Public Interface

### Trait Definition

```rust
#[async_trait]
pub trait MessageHandler: Send + Sync {
    async fn handle_incoming_message(&mut self, message: FixMessage) -> Result<(), MessageHandlerError>;
    async fn handle_outgoing_message(&mut self, message: FixMessage) -> Result<(), MessageHandlerError>;
    async fn parse_fix_message(&mut self, fix_string: &str) -> Result<FixMessage, MessageParseError>;
    async fn validate_message(&mut self, message: &FixMessage) -> Result<(), Vec<MessageValidationError>>;
    async fn transform_message(&mut self, message: FixMessage) -> Result<FixMessage, MessageTransformError>;
    async fn route_message(&mut self, message: &FixMessage) -> Result<MessageRoute, MessageRoutingError>;
    fn get_message_stats(&self) -> MessageHandlerStats;
    fn get_supported_message_types(&self) -> Vec<FixMsgType>;
    async fn handle_heartbeat(&mut self, message: &FixMessage) -> Result<(), MessageHandlerError>;
    async fn handle_test_request(&mut self, message: &FixMessage) -> Result<(), MessageHandlerError>;
    async fn handle_reject(&mut self, message: &FixMessage) -> Result<(), MessageHandlerError>;
    async fn handle_logout(&mut self, message: &FixMessage) -> Result<(), MessageHandlerError>;
}
```

### Associated Types

```rust
pub enum MessageHandlerError {
    ParseError(MessageParseError),
    ValidationError(Vec<MessageValidationError>),
    TransformError(MessageTransformError),
    RoutingError(MessageRoutingError),
    HandlerNotFound(FixMsgType),
    InvalidMessageState(String),
    ProtocolViolation(String),
}

pub enum MessageParseError {
    InvalidFormat(String),
    MissingRequiredField(FixField),
    InvalidFieldValue(FixField, String),
    MalformedMessage(String),
    EncodingError(String),
}

pub enum MessageValidationError {
    MissingField(FixField),
    InvalidFieldValue(FixField, String),
    BusinessRuleViolation(String),
    ProtocolViolation(String),
    CrossFieldValidation(String),
}

pub enum MessageTransformError {
    UnsupportedMessageType(FixMsgType),
    InvalidTransformation(String),
    DataLoss(String),
    SerializationError(String),
}

pub enum MessageRoutingError {
    NoHandlerFound(FixMsgType),
    HandlerBusy(FixMsgType),
    InvalidRoute(String),
    CircularRoute(String),
}

pub enum MessageRoute {
    OrderHandler,
    MarketDataHandler,
    PositionHandler,
    AccountHandler,
    SessionHandler,
    Custom(String),
}

pub struct MessageHandlerStats {
    pub messages_processed: u64,
    pub messages_parsed: u64,
    pub messages_validated: u64,
    pub messages_transformed: u64,
    pub messages_routed: u64,
    pub errors_encountered: u64,
    pub average_processing_time: Duration,
    pub last_message_time: Option<DateTime<Utc>>,
}
```

## Usage Examples

### Basic Message Handling

```rust
use deribit_fix::traits::MessageHandler;
use deribit_fix::message::FixMessage;

struct MyMessageHandler;

#[async_trait]
impl MessageHandler for MyMessageHandler {
    async fn handle_incoming_message(&mut self, message: FixMessage) -> Result<(), MessageHandlerError> {
        // Parse and validate the message
        let parsed = self.parse_fix_message(&message.to_fix_string()?).await?;
        self.validate_message(&parsed).await?;
        
        // Route the message to appropriate handler
        let route = self.route_message(&parsed).await?;
        self.process_message_by_route(parsed, route).await?;
        
        Ok(())
    }
    
    // ... implement other required methods
}
```

### Message Validation

```rust
impl MyMessageHandler {
    async fn validate_order_message(&self, message: &FixMessage) -> Result<(), Vec<MessageValidationError>> {
        let mut errors = Vec::new();
        
        // Check required fields
        if !message.has_field(FixField::Symbol) {
            errors.push(MessageValidationError::MissingField(FixField::Symbol));
        }
        
        if !message.has_field(FixField::OrderQty) {
            errors.push(MessageValidationError::MissingField(FixField::OrderQty));
        }
        
        // Validate field values
        if let Some(qty) = message.get_field_value(FixField::OrderQty) {
            if qty.parse::<f64>().unwrap_or(0.0) <= 0.0 {
                errors.push(MessageValidationError::InvalidFieldValue(
                    FixField::OrderQty,
                    "Quantity must be positive".to_string()
                ));
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

### Message Transformation

```rust
impl MyMessageHandler {
    async fn transform_order_message(&self, message: FixMessage) -> Result<FixMessage, MessageTransformError> {
        let mut transformed = message.clone();
        
        // Add required fields if missing
        if !transformed.has_field(FixField::TransactTime) {
            transformed.set_field(FixField::TransactTime, &Utc::now().to_rfc3339());
        }
        
        // Normalize field values
        if let Some(side) = transformed.get_field_value(FixField::Side) {
            let normalized_side = side.to_uppercase();
            transformed.set_field(FixField::Side, &normalized_side);
        }
        
        Ok(transformed)
    }
}
```

### Message Routing

```rust
impl MyMessageHandler {
    async fn route_by_message_type(&self, message: &FixMessage) -> Result<MessageRoute, MessageRoutingError> {
        let msg_type = message.get_field_value(FixField::MsgType)
            .ok_or_else(|| MessageRoutingError::NoHandlerFound(FixMsgType::Unknown))?;
        
        match msg_type.as_str() {
            "D" => Ok(MessageRoute::OrderHandler),      // NewOrderSingle
            "8" => Ok(MessageRoute::OrderHandler),      // ExecutionReport
            "F" => Ok(MessageRoute::OrderHandler),      // OrderCancelRequest
            "G" => Ok(MessageRoute::OrderHandler),      // OrderCancelReplaceRequest
            "W" => Ok(MessageRoute::MarketDataHandler), // MarketDataRequest
            "Y" => Ok(MessageRoute::MarketDataHandler), // MarketDataRequestReject
            "V" => Ok(MessageRoute::PositionHandler),   // MarketDataSnapshotFullRefresh
            "0" => Ok(MessageRoute::SessionHandler),    // Heartbeat
            "1" => Ok(MessageRoute::SessionHandler),    // TestRequest
            "5" => Ok(MessageRoute::SessionHandler),    // Logout
            _ => Ok(MessageRoute::Custom(msg_type.to_string())),
        }
    }
}
```

### Error Handling

```rust
impl MyMessageHandler {
    async fn handle_message_with_retry(&mut self, message: FixMessage) -> Result<(), MessageHandlerError> {
        let mut attempts = 0;
        let max_attempts = 3;
        
        while attempts < max_attempts {
            match self.handle_incoming_message(message.clone()).await {
                Ok(()) => return Ok(()),
                Err(MessageHandlerError::ValidationError(errors)) => {
                    // Log validation errors and stop retrying
                    log::error!("Message validation failed: {:?}", errors);
                    return Err(MessageHandlerError::ValidationError(errors));
                }
                Err(MessageHandlerError::HandlerNotFound(msg_type)) => {
                    // Log and stop retrying
                    log::error!("No handler found for message type: {:?}", msg_type);
                    return Err(MessageHandlerError::HandlerNotFound(msg_type));
                }
                Err(e) => {
                    attempts += 1;
                    if attempts >= max_attempts {
                        return Err(e);
                    }
                    // Wait before retry
                    tokio::time::sleep(Duration::from_millis(100 * attempts as u64)).await;
                }
            }
        }
        
        Err(MessageHandlerError::InvalidMessageState("Max retry attempts exceeded".to_string()))
    }
}
```

## Module Dependencies

- **`message`**: Uses `FixMessage`, `FixField`, `FixMsgType` for message handling
- **`error`**: Uses `MessageHandlerError` and related error types
- **`types`**: May use trading types for message transformation
- **`session`**: May interact with session management for routing

## Related Types

- **`FixMessage`**: The core message structure being handled
- **`FixField`**: Individual fields within FIX messages
- **`FixMsgType`**: Types of FIX messages
- **`MessageHandlerError`**: Error types for message handling operations
- **`MessageRoute`**: Routing destinations for messages

## Testing

### Unit Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    
    #[tokio::test]
    async fn test_message_validation() {
        let handler = MyMessageHandler::new();
        let message = create_test_order_message();
        
        let result = handler.validate_message(&message).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_message_routing() {
        let handler = MyMessageHandler::new();
        let message = create_test_order_message();
        
        let route = handler.route_message(&message).await.unwrap();
        assert!(matches!(route, MessageRoute::OrderHandler));
    }
}
```

### Integration Testing

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_end_to_end_message_handling() {
        let mut handler = MyMessageHandler::new();
        let fix_string = "8=FIX.4.4|9=123|35=D|55=BTC-PERPETUAL|54=1|38=1.0|40=2|10=123|";
        
        let message = handler.parse_fix_message(fix_string).await.unwrap();
        handler.validate_message(&message).await.unwrap();
        
        let route = handler.route_message(&message).await.unwrap();
        assert!(matches!(route, MessageRoute::OrderHandler));
        
        handler.handle_incoming_message(message).await.unwrap();
    }
}
```

## Performance Considerations

- **Async Processing**: All operations are async to avoid blocking
- **Batch Processing**: Consider batching multiple messages for efficiency
- **Caching**: Cache parsed message templates for repeated message types
- **Validation Optimization**: Use early returns for validation failures
- **Memory Management**: Reuse message objects when possible

## Security Considerations

- **Input Validation**: Validate all incoming message data
- **Message Sanitization**: Sanitize message content before processing
- **Access Control**: Restrict message handling to authorized sources
- **Audit Logging**: Log all message processing activities
- **Rate Limiting**: Implement rate limiting for message processing
