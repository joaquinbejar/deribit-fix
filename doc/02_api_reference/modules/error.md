# error Module

## Overview

The `error` module provides comprehensive error handling for the `deribit-fix` crate, including error types, severity levels, error context, and recovery strategies.

## Purpose

- **Error Classification**: Categorize errors by type and severity
- **Error Context**: Provide detailed context for debugging and logging
- **Recovery Strategies**: Implement automatic retry and circuit breaker patterns
- **User Experience**: Provide clear, actionable error messages

## Public Interface

### Main Error Enum

```rust
#[derive(Debug, thiserror::Error)]
pub enum DeribitFixError {
    // Connection Errors
    #[error("Connection failed: {0}")]
    Connection(#[from] ConnectionError),
    
    // Authentication Errors
    #[error("Authentication failed: {0}")]
    Authentication(#[from] AuthenticationError),
    
    // FIX Protocol Errors
    #[error("FIX protocol error: {0}")]
    FixProtocol(#[from] FixProtocolError),
    
    // Business Logic Errors
    #[error("Business logic error: {0}")]
    BusinessLogic(#[from] BusinessLogicError),
    
    // Configuration Errors
    #[error("Configuration error: {0}")]
    Configuration(#[from] ConfigError),
    
    // Timeout Errors
    #[error("Operation timed out after {:?}", duration)]
    Timeout { duration: Duration },
    
    // Internal Errors
    #[error("Internal error: {message}")]
    Internal { message: String },
}
```

### Error Severity Levels

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    Low,      // Non-critical, can continue
    Medium,   // May affect functionality
    High,     // Significant impact, retry recommended
    Critical, // Fatal, cannot continue
}
```

### Error Context

```rust
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub timestamp: DateTime<Utc>,
    pub operation: String,
    pub component: String,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub request_id: Option<String>,
    pub additional_data: HashMap<String, String>,
}
```

### Specific Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    #[error("Failed to establish connection: {reason}")]
    ConnectionFailed { reason: String },
    
    #[error("Connection lost: {reason}")]
    ConnectionLost { reason: String },
    
    #[error("SSL/TLS error: {reason}")]
    SslError { reason: String },
    
    #[error("Network timeout after {:?}", duration)]
    NetworkTimeout { duration: Duration },
}

#[derive(Debug, thiserror::Error)]
pub enum FixProtocolError {
    #[error("Invalid FIX message: {reason}")]
    InvalidMessage { reason: String },
    
    #[error("Sequence number mismatch: expected {expected}, got {actual}")]
    SequenceMismatch { expected: u32, actual: u32 },
    
    #[error("Message type not supported: {msg_type}")]
    UnsupportedMessageType { msg_type: String },
    
    #[error("Required field missing: {field}")]
    MissingRequiredField { field: String },
    
    #[error("Field validation failed: {field} = {value}, reason: {reason}")]
    FieldValidationFailed { field: String, value: String, reason: String },
}

#[derive(Debug, thiserror::Error)]
pub enum BusinessLogicError {
    #[error("Order rejected: {reason}")]
    OrderRejected { reason: String },
    
    #[error("Insufficient funds: required {required}, available {available}")]
    InsufficientFunds { required: f64, available: f64 },
    
    #[error("Position limit exceeded: {limit}")]
    PositionLimitExceeded { limit: f64 },
    
    #[error("Instrument not found: {instrument}")]
    InstrumentNotFound { instrument: String },
    
    #[error("Market closed for instrument: {instrument}")]
    MarketClosed { instrument: String },
}
```

## Usage Examples

### Basic Error Handling

```rust
use deribit_fix::{DeribitFixError, ErrorSeverity};

async fn place_order(client: &mut DeribitFixClient, order: Order) -> Result<String, DeribitFixError> {
    match client.place_order(order).await {
        Ok(order_id) => Ok(order_id),
        Err(DeribitFixError::BusinessLogic(BusinessLogicError::InsufficientFunds { required, available })) => {
            log::warn!("Insufficient funds: required {}, available {}", required, available);
            Err(DeribitFixError::BusinessLogic(BusinessLogicError::InsufficientFunds { required, available }))
        }
        Err(DeribitFixError::Connection(ConnectionError::ConnectionLost { reason })) => {
            log::error!("Connection lost: {}", reason);
            // Attempt reconnection
            client.reconnect().await?;
            client.place_order(order).await
        }
        Err(e) => {
            log::error!("Unexpected error: {}", e);
            Err(e)
        }
    }
}
```

### Error Context Creation

```rust
use deribit_fix::{ErrorContext, DeribitFixError};

fn create_error_context(operation: &str, component: &str) -> ErrorContext {
    ErrorContext {
        timestamp: Utc::now(),
        operation: operation.to_string(),
        component: component.to_string(),
        user_id: None,
        session_id: None,
        request_id: Some(uuid::Uuid::new_v4().to_string()),
        additional_data: HashMap::new(),
    }
}

async fn execute_with_context<F, T>(
    operation: F,
    operation_name: &str,
    component: &str,
) -> Result<T, DeribitFixError>
where
    F: Future<Output = Result<T, DeribitFixError>>,
{
    let context = create_error_context(operation_name, component);
    
    match operation.await {
        Ok(result) => Ok(result),
        Err(mut error) => {
            // Add context to the error
            error.add_context(context);
            Err(error)
        }
    }
}
```

### Retry Logic with Error Classification

```rust
use deribit_fix::{DeribitFixError, BackoffStrategy};

impl DeribitFixClient {
    pub async fn execute_with_retry<F, T>(
        &mut self,
        operation: F,
        max_retries: usize,
        backoff_strategy: BackoffStrategy,
    ) -> Result<T, DeribitFixError>
    where
        F: FnMut() -> Future<Output = Result<T, DeribitFixError>> + Send,
    {
        let mut attempts = 0;
        let mut last_error = None;
        
        while attempts < max_retries {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(error) => {
                    last_error = Some(error.clone());
                    
                    // Check if error is retryable
                    if !error.is_retryable() {
                        return Err(error);
                    }
                    
                    // Check if we should use circuit breaker
                    if self.circuit_breaker.is_open() {
                        return Err(DeribitFixError::CircuitBreakerOpen);
                    }
                    
                    attempts += 1;
                    if attempts < max_retries {
                        let delay = backoff_strategy.delay(attempts);
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }
        
        Err(last_error.unwrap_or_else(|| {
            DeribitFixError::Internal {
                message: "Max retries exceeded".to_string(),
            }
        }))
    }
}
```

## Error Recovery Strategies

### Circuit Breaker Pattern

```rust
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{Duration, Instant};

pub struct CircuitBreaker {
    failure_threshold: u32,
    success_threshold: u32,
    timeout: Duration,
    failure_count: AtomicU32,
    last_failure_time: Mutex<Option<Instant>>,
    state: Mutex<CircuitBreakerState>,
}

#[derive(Debug, Clone)]
enum CircuitBreakerState {
    Closed,     // Normal operation
    Open,       // Failing, reject requests
    HalfOpen,   // Testing if service recovered
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u32, timeout: Duration) -> Self {
        Self {
            failure_threshold,
            success_threshold: failure_threshold / 2,
            timeout,
            failure_count: AtomicU32::new(0),
            last_failure_time: Mutex::new(None),
            state: Mutex::new(CircuitBreakerState::Closed),
        }
    }
    
    pub async fn execute<F, T>(&self, operation: F) -> Result<T, DeribitFixError>
    where
        F: Future<Output = Result<T, DeribitFixError>>,
    {
        let state = *self.state.lock().unwrap();
        
        match state {
            CircuitBreakerState::Open => {
                if self.should_attempt_reset().await {
                    self.transition_to_half_open().await;
                } else {
                    return Err(DeribitFixError::CircuitBreakerOpen);
                }
            }
            CircuitBreakerState::HalfOpen => {
                // Allow one request to test recovery
            }
            CircuitBreakerState::Closed => {
                // Normal operation
            }
        }
        
        match operation.await {
            Ok(result) => {
                self.on_success().await;
                Ok(result)
            }
            Err(error) => {
                self.on_failure().await;
                Err(error)
            }
        }
    }
}
```

### Backoff Strategies

```rust
#[derive(Debug, Clone)]
pub enum BackoffStrategy {
    Fixed(Duration),
    Exponential { base: Duration, max: Duration },
    Fibonacci { base: Duration, max: Duration },
}

impl BackoffStrategy {
    pub fn delay(&self, attempt: usize) -> Duration {
        match self {
            BackoffStrategy::Fixed(duration) => *duration,
            
            BackoffStrategy::Exponential { base, max } => {
                let delay = base * 2_u32.pow(attempt as u32);
                delay.min(*max)
            }
            
            BackoffStrategy::Fibonacci { base, max } => {
                let delay = base * fibonacci(attempt as u32);
                delay.min(*max)
            }
        }
    }
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
```

## Error Logging and Monitoring

### Structured Error Logging

```rust
use log::{error, warn, info};
use serde_json::json;

impl DeribitFixError {
    pub fn log_with_context(&self, context: &ErrorContext) {
        let error_data = json!({
            "error_type": self.error_type(),
            "severity": self.severity(),
            "message": self.to_string(),
            "context": {
                "timestamp": context.timestamp.to_rfc3339(),
                "operation": context.operation,
                "component": context.component,
                "user_id": context.user_id,
                "session_id": context.session_id,
                "request_id": context.request_id,
                "additional_data": context.additional_data,
            }
        });
        
        match self.severity() {
            ErrorSeverity::Critical | ErrorSeverity::High => {
                error!("{}", serde_json::to_string_pretty(&error_data).unwrap());
            }
            ErrorSeverity::Medium => {
                warn!("{}", serde_json::to_string_pretty(&error_data).unwrap());
            }
            ErrorSeverity::Low => {
                info!("{}", serde_json::to_string_pretty(&error_data).unwrap());
            }
        }
    }
}
```

## Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_severity_ordering() {
        assert!(ErrorSeverity::Low < ErrorSeverity::Medium);
        assert!(ErrorSeverity::Medium < ErrorSeverity::High);
        assert!(ErrorSeverity::High < ErrorSeverity::Critical);
    }

    #[test]
    fn test_error_context_creation() {
        let context = create_error_context("test_operation", "test_component");
        
        assert_eq!(context.operation, "test_operation");
        assert_eq!(context.component, "test_component");
        assert!(context.request_id.is_some());
    }

    #[tokio::test]
    async fn test_circuit_breaker() {
        let breaker = CircuitBreaker::new(3, Duration::from_secs(1));
        
        // Simulate failures
        for _ in 0..3 {
            breaker.on_failure().await;
        }
        
        // Circuit should be open
        let state = breaker.state.lock().unwrap();
        assert!(matches!(*state, CircuitBreakerState::Open));
    }
}
```

## Module Dependencies

- `thiserror`: Error derive macro
- `chrono`: Timestamp handling
- `log`: Logging integration
- `serde`: JSON serialization for logging
- `uuid`: Request ID generation
