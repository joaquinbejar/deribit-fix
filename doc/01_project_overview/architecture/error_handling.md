# Error Handling Patterns

This document explains the error handling patterns and types used throughout the `deribit-fix` crate, including error classification, recovery strategies, and best practices.

## Overview

The error handling system is designed to be comprehensive, user-friendly, and recoverable. It provides detailed error information while maintaining clean error propagation and recovery mechanisms.

## Error Classification

### 1. Error Categories

```rust
#[derive(Debug, thiserror::Error)]
pub enum DeribitFixError {
    // Connection and Network Errors
    #[error("Connection error: {0}")]
    Connection(#[from] std::io::Error),
    
    #[error("Connection timeout: {0}")]
    ConnectionTimeout(String),
    
    #[error("Connection lost: {0}")]
    ConnectionLost(String),
    
    #[error("SSL/TLS error: {0}")]
    SslError(#[from] native_tls::Error),
    
    // Authentication and Authorization Errors
    #[error("Authentication failed: {0}")]
    Authentication(String),
    
    #[error("Invalid API credentials: {0}")]
    InvalidCredentials(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Account suspended: {0}")]
    AccountSuspended(String),
    
    // FIX Protocol Errors
    #[error("FIX protocol error: {0}")]
    FixProtocol(String),
    
    #[error("Invalid FIX message: {0}")]
    InvalidFixMessage(String),
    
    #[error("Sequence number error: {0}")]
    SequenceError(String),
    
    #[error("Message parsing error: {0}")]
    MessageParsingError(String),
    
    #[error("Required field missing: {0}")]
    RequiredFieldMissing(String),
    
    // Business Logic Errors
    #[error("Business logic error: {0}")]
    BusinessLogic(String),
    
    #[error("Invalid order: {0}")]
    InvalidOrder(String),
    
    #[error("Insufficient funds: {0}")]
    InsufficientFunds(String),
    
    #[error("Order not found: {0}")]
    OrderNotFound(String),
    
    #[error("Symbol not found: {0}")]
    SymbolNotFound(String),
    
    #[error("Position limit exceeded: {0}")]
    PositionLimitExceeded(String),
    
    // Configuration Errors
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
    
    #[error("Missing required configuration: {0}")]
    MissingConfiguration(String),
    
    // Timeout and Rate Limiting Errors
    #[error("Timeout: {0}")]
    Timeout(String),
    
    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),
    
    #[error("Request throttled: {0}")]
    RequestThrottled(String),
    
    // Internal Errors
    #[error("Internal error: {0}")]
    Internal(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Deserialization error: {0}")]
    DeserializationError(#[from] serde_json::Error),
}
```

### 2. Error Severity Levels

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    /// Low severity - operation can continue
    Low,
    
    /// Medium severity - operation may be affected
    Medium,
    
    /// High severity - operation should be retried
    High,
    
    /// Critical severity - operation cannot continue
    Critical,
}

impl DeribitFixError {
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            // Low severity errors
            DeribitFixError::MessageParsingError(_) => ErrorSeverity::Low,
            DeribitFixError::RequiredFieldMissing(_) => ErrorSeverity::Low,
            
            // Medium severity errors
            DeribitFixError::Timeout(_) => ErrorSeverity::Medium,
            DeribitFixError::RateLimitExceeded(_) => ErrorSeverity::Medium,
            DeribitFixError::RequestThrottled(_) => ErrorSeverity::Medium,
            
            // High severity errors
            DeribitFixError::ConnectionTimeout(_) => ErrorSeverity::High,
            DeribitFixError::ConnectionLost(_) => ErrorSeverity::High,
            DeribitFixError::SequenceError(_) => ErrorSeverity::High,
            
            // Critical severity errors
            DeribitFixError::Authentication(_) => ErrorSeverity::Critical,
            DeribitFixError::InvalidCredentials(_) => ErrorSeverity::Critical,
            DeribitFixError::AccountSuspended(_) => ErrorSeverity::Critical,
            DeribitFixError::InsufficientFunds(_) => ErrorSeverity::Critical,
            
            // Default to medium for unknown errors
            _ => ErrorSeverity::Medium,
        }
    }
    
    pub fn is_recoverable(&self) -> bool {
        matches!(self.severity(), ErrorSeverity::Low | ErrorSeverity::Medium)
    }
    
    pub fn is_critical(&self) -> bool {
        matches!(self.severity(), ErrorSeverity::Critical)
    }
}
```

## Error Context and Information

### 1. Error Context

```rust
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// Timestamp when the error occurred
    pub timestamp: DateTime<Utc>,
    
    /// Operation that was being performed
    pub operation: String,
    
    /// Additional context information
    pub context: HashMap<String, String>,
    
    /// Stack trace (if available)
    pub stack_trace: Option<String>,
    
    /// Related error chain
    pub cause: Option<Box<DeribitFixError>>,
}

impl DeribitFixError {
    pub fn with_context(self, operation: &str, context: HashMap<String, String>) -> Self {
        // Implementation to add context to errors
        self
    }
    
    pub fn context(&self) -> Option<&ErrorContext> {
        // Return error context if available
        None
    }
}
```

### 2. Error Details

```rust
#[derive(Debug, Clone)]
pub struct FixProtocolError {
    /// FIX message that caused the error
    pub fix_message: Option<String>,
    
    /// FIX tag that caused the error
    pub problematic_tag: Option<String>,
    
    /// Expected value
    pub expected_value: Option<String>,
    
    /// Actual value received
    pub actual_value: Option<String>,
    
    /// FIX specification reference
    pub fix_spec_reference: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BusinessLogicError {
    /// Business rule that was violated
    pub business_rule: String,
    
    /// Current state when error occurred
    pub current_state: String,
    
    /// Required state for operation
    pub required_state: String,
    
    /// Suggested action to resolve
    pub suggested_action: Option<String>,
}
```

## Error Recovery Strategies

### 1. Automatic Recovery

```rust
impl DeribitFixClient {
    pub async fn execute_with_retry<T, F, Fut>(
        &mut self,
        operation: F,
        max_retries: usize,
        backoff_strategy: BackoffStrategy,
    ) -> Result<T, DeribitFixError>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T, DeribitFixError>>,
    {
        let mut attempts = 0;
        let mut delay = backoff_strategy.initial_delay();
        
        loop {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(error) => {
                    attempts += 1;
                    
                    if attempts >= max_retries || !error.is_recoverable() {
                        return Err(error);
                    }
                    
                    // Log retry attempt
                    log::warn!(
                        "Operation failed, retrying in {:?} (attempt {}/{}): {}",
                        delay,
                        attempts,
                        max_retries,
                        error
                    );
                    
                    // Wait before retry
                    tokio::time::sleep(delay).await;
                    
                    // Calculate next delay
                    delay = backoff_strategy.next_delay(delay, attempts);
                }
            }
        }
    }
}
```

### 2. Backoff Strategies

```rust
#[derive(Debug, Clone)]
pub enum BackoffStrategy {
    /// Fixed delay between retries
    Fixed(Duration),
    
    /// Exponential backoff with jitter
    Exponential {
        initial_delay: Duration,
        max_delay: Duration,
        multiplier: f64,
        jitter: bool,
    },
    
    /// Fibonacci backoff
    Fibonacci {
        initial_delay: Duration,
        max_delay: Duration,
    },
}

impl BackoffStrategy {
    pub fn initial_delay(&self) -> Duration {
        match self {
            BackoffStrategy::Fixed(delay) => *delay,
            BackoffStrategy::Exponential { initial_delay, .. } => *initial_delay,
            BackoffStrategy::Fibonacci { initial_delay, .. } => *initial_delay,
        }
    }
    
    pub fn next_delay(&self, current_delay: Duration, attempt: usize) -> Duration {
        match self {
            BackoffStrategy::Fixed(delay) => *delay,
            BackoffStrategy::Exponential { max_delay, multiplier, jitter, .. } => {
                let next_delay = current_delay.mul_f64(*multiplier);
                let next_delay = next_delay.min(*max_delay);
                
                if *jitter {
                    self.add_jitter(next_delay)
                } else {
                    next_delay
                }
            }
            BackoffStrategy::Fibonacci { max_delay, .. } => {
                let next_delay = self.fibonacci_delay(attempt);
                next_delay.min(*max_delay)
            }
        }
    }
    
    fn add_jitter(&self, delay: Duration) -> Duration {
        let jitter_factor = 0.1; // 10% jitter
        let jitter = delay.mul_f64(jitter_factor * fastrand::f64());
        delay + jitter
    }
    
    fn fibonacci_delay(&self, attempt: usize) -> Duration {
        let fib = self.fibonacci(attempt);
        Duration::from_millis(fib as u64)
    }
    
    fn fibonacci(&self, n: usize) -> u64 {
        if n <= 1 {
            1
        } else {
            self.fibonacci(n - 1) + self.fibonacci(n - 2)
        }
    }
}
```

### 3. Circuit Breaker Pattern

```rust
#[derive(Debug, Clone)]
pub struct CircuitBreaker {
    /// Current state of the circuit breaker
    state: CircuitState,
    
    /// Failure threshold to open circuit
    failure_threshold: usize,
    
    /// Timeout to attempt half-open state
    timeout: Duration,
    
    /// Current failure count
    failure_count: usize,
    
    /// Last failure time
    last_failure_time: Option<Instant>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CircuitState {
    /// Circuit is closed - operations are allowed
    Closed,
    
    /// Circuit is open - operations are blocked
    Open,
    
    /// Circuit is half-open - testing if operations work
    HalfOpen,
}

impl CircuitBreaker {
    pub fn can_execute(&self) -> bool {
        match self.state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                if let Some(last_failure) = self.last_failure_time {
                    Instant::now().duration_since(last_failure) >= self.timeout
                } else {
                    false
                }
            }
            CircuitState::HalfOpen => true,
        }
    }
    
    pub fn on_success(&mut self) {
        match self.state {
            CircuitState::Closed => {
                self.failure_count = 0;
            }
            CircuitState::HalfOpen => {
                self.state = CircuitState::Closed;
                self.failure_count = 0;
            }
            CircuitState::Open => {}
        }
    }
    
    pub fn on_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure_time = Some(Instant::now());
        
        if self.failure_count >= self.failure_threshold {
            self.state = CircuitState::Open;
        }
    }
}
```

## Error Handling in Operations

### 1. Connection Operations

```rust
impl ConnectionManager {
    pub async fn connect(&mut self) -> Result<(), DeribitFixError> {
        let result = self.attempt_connection().await;
        
        match result {
            Ok(()) => {
                log::info!("Successfully connected to Deribit FIX gateway");
                Ok(())
            }
            Err(error) => {
                log::error!("Failed to connect: {}", error);
                
                // Determine if we should retry
                if error.is_recoverable() {
                    log::info!("Connection error is recoverable, will retry");
                    Err(error)
                } else {
                    log::error!("Connection error is not recoverable");
                    Err(error)
                }
            }
        }
    }
    
    async fn attempt_connection(&mut self) -> Result<(), DeribitFixError> {
        // Implementation of connection attempt
        todo!()
    }
}
```

### 2. Order Operations

```rust
impl DeribitFixClient {
    pub async fn place_order(&mut self, order: Order) -> Result<OrderResponse, DeribitFixError> {
        // Validate order before sending
        self.validate_order(&order)?;
        
        // Attempt to place order with retry logic
        let result = self.execute_with_retry(
            || self.send_order_request(&order),
            3,
            BackoffStrategy::Exponential {
                initial_delay: Duration::from_millis(100),
                max_delay: Duration::from_secs(5),
                multiplier: 2.0,
                jitter: true,
            },
        ).await;
        
        match result {
            Ok(response) => {
                log::info!("Order placed successfully: {}", response.order_id);
                Ok(response)
            }
            Err(error) => {
                log::error!("Failed to place order: {}", error);
                
                // Handle specific error types
                match error {
                    DeribitFixError::InsufficientFunds(msg) => {
                        log::warn!("Insufficient funds: {}", msg);
                        Err(error)
                    }
                    DeribitFixError::InvalidOrder(msg) => {
                        log::warn!("Invalid order: {}", msg);
                        Err(error)
                    }
                    _ => Err(error),
                }
            }
        }
    }
}
```

### 3. Market Data Operations

```rust
impl DeribitFixClient {
    pub async fn subscribe_market_data(
        &mut self,
        symbol: &str,
        depth: MarketDepth,
    ) -> Result<MarketDataSubscription, DeribitFixError> {
        let subscription = MarketDataSubscription {
            symbol: symbol.to_string(),
            depth,
            subscription_id: Uuid::new_v4().to_string(),
        };
        
        let result = self.send_market_data_request(&subscription).await;
        
        match result {
            Ok(response) => {
                log::info!("Market data subscription successful for {}", symbol);
                Ok(response)
            }
            Err(error) => {
                log::error!("Market data subscription failed: {}", error);
                
                // Check if symbol exists
                if let DeribitFixError::SymbolNotFound(_) = error {
                    log::warn!("Symbol {} not found, checking available symbols", symbol);
                    // Could implement symbol discovery here
                }
                
                Err(error)
            }
        }
    }
}
```

## Error Logging and Monitoring

### 1. Structured Logging

```rust
impl DeribitFixError {
    pub fn log_with_context(&self, logger: &Logger) {
        let severity = self.severity();
        let context = self.context();
        
        let log_entry = serde_json::json!({
            "error_type": std::any::type_name::<Self>(),
            "severity": format!("{:?}", severity),
            "message": self.to_string(),
            "recoverable": self.is_recoverable(),
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "context": context.map(|c| serde_json::to_value(c).unwrap_or_default()),
        });
        
        match severity {
            ErrorSeverity::Low => logger.info(&log_entry),
            ErrorSeverity::Medium => logger.warn(&log_entry),
            ErrorSeverity::High => logger.error(&log_entry),
            ErrorSeverity::Critical => logger.critical(&log_entry),
        }
    }
}
```

### 2. Error Metrics

```rust
#[derive(Debug, Default)]
pub struct ErrorMetrics {
    /// Total error count by type
    pub error_counts: HashMap<String, u64>,
    
    /// Error counts by severity
    pub severity_counts: HashMap<ErrorSeverity, u64>,
    
    /// Recovery success rates
    pub recovery_rates: HashMap<String, f64>,
    
    /// Error timestamps for trend analysis
    pub error_timestamps: Vec<DateTime<Utc>>,
}

impl ErrorMetrics {
    pub fn record_error(&mut self, error: &DeribitFixError) {
        let error_type = std::any::type_name::<DeribitFixError>();
        *self.error_counts.entry(error_type.to_string()).or_insert(0) += 1;
        
        let severity = error.severity();
        *self.severity_counts.entry(severity).or_insert(0) += 1;
        
        self.error_timestamps.push(Utc::now());
        
        // Keep only last 1000 timestamps
        if self.error_timestamps.len() > 1000 {
            self.error_timestamps.remove(0);
        }
    }
    
    pub fn get_error_rate(&self, window: Duration) -> f64 {
        let cutoff = Utc::now() - window;
        let recent_errors = self.error_timestamps
            .iter()
            .filter(|&ts| *ts > cutoff)
            .count();
        
        recent_errors as f64 / window.num_seconds() as f64
    }
}
```

## Best Practices

### 1. Error Propagation
- Use `?` operator for early returns on errors
- Provide context when wrapping errors
- Don't lose original error information
- Use appropriate error types for different failure modes

### 2. Error Recovery
- Implement retry logic for transient errors
- Use exponential backoff with jitter
- Implement circuit breaker pattern for persistent failures
- Provide clear recovery instructions

### 3. Error Reporting
- Log errors with sufficient context
- Include stack traces for debugging
- Categorize errors by severity
- Track error metrics for monitoring

### 4. User Experience
- Provide user-friendly error messages
- Suggest actions to resolve errors
- Don't expose internal implementation details
- Maintain consistent error handling patterns
