# SessionManager

## Overview

The `SessionManager` trait defines the interface for managing FIX sessions, including session establishment, message sequencing, heartbeats, and session recovery.

## Purpose

- **Session Management**: Establishes and maintains FIX sessions
- **Message Sequencing**: Manages sequence numbers and message ordering
- **Heartbeat Management**: Handles session keep-alive and monitoring
- **Session Recovery**: Implements session recovery and gap filling
- **Authentication**: Manages session authentication and authorization

## Public Interface

### Trait Definition

```rust
#[async_trait]
pub trait SessionManager: Send + Sync {
    /// Initialize a new FIX session
    async fn initialize_session(&mut self, config: &SessionConfig) -> Result<(), SessionError>

    /// Start the FIX session
    async fn start_session(&mut self) -> Result<(), SessionError>

    /// Stop the FIX session
    async fn stop_session(&mut self) -> Result<(), SessionError>

    /// Check if session is active
    fn is_session_active(&self) -> bool

    /// Get current session status
    fn session_status(&self) -> SessionStatus

    /// Send a message with proper sequencing
    async fn send_message(&mut self, message: FixMessage) -> Result<(), SessionError>

    /// Receive and process incoming messages
    async fn receive_message(&mut self) -> Result<Option<FixMessage>, SessionError>

    /// Send heartbeat message
    async fn send_heartbeat(&mut self) -> Result<(), SessionError>

    /// Process test request
    async fn process_test_request(&mut self, test_req_id: &str) -> Result<(), SessionError>

    /// Process sequence reset
    async fn process_sequence_reset(&mut self, new_seq_num: u64) -> Result<(), SessionError>

    /// Get session statistics
    fn get_session_stats(&self) -> SessionStats

    /// Get current sequence numbers
    fn get_sequence_numbers(&self) -> (u64, u64) // (incoming, outgoing)

    /// Resend messages from sequence number
    async fn resend_from_sequence(&mut self, from_seq_num: u64) -> Result<(), SessionError>

    /// Logout from session
    async fn logout(&mut self, reason: &str) -> Result<(), SessionError>
}
```

### Associated Types

```rust
/// Session status enumeration
pub enum SessionStatus {
    Disconnected,
    Connecting,
    Connected,
    LoggingIn,
    LoggedIn,
    Active,
    LoggingOut,
    LoggedOut,
    Error(SessionError),
}

/// Session statistics
pub struct SessionStats {
    pub session_start_time: DateTime<Utc>,
    pub messages_sent: u64,
    pub messages_received: u64,
    pub heartbeats_sent: u64,
    pub heartbeats_received: u64,
    pub test_requests_processed: u64,
    pub sequence_resets: u64,
    pub resend_requests: u64,
    pub logout_count: u32,
    pub errors_count: u32,
}

/// Session configuration
pub struct SessionConfig {
    pub sender_comp_id: String,
    pub target_comp_id: String,
    pub begin_string: String,
    pub heartbeat_interval: Duration,
    pub test_request_delay: Duration,
    pub max_message_size: usize,
    pub enable_sequence_gap_filling: bool,
    pub max_sequence_gap: u64,
    pub session_timeout: Duration,
}
```

## Usage Examples

### Basic Session Management

```rust
use deribit_fix::session::{SessionManager, SessionConfig, SessionStatus};
use deribit_fix::message::FixMessage;

// Create session configuration
let session_config = SessionConfig {
    sender_comp_id: "MYCLIENT".to_string(),
    target_comp_id: "DERIBIT".to_string(),
    begin_string: "FIX.4.4".to_string(),
    heartbeat_interval: Duration::from_secs(30),
    test_request_delay: Duration::from_secs(10),
    max_message_size: 1024 * 1024, // 1MB
    enable_sequence_gap_filling: true,
    max_sequence_gap: 1000,
    session_timeout: Duration::from_secs(300), // 5 minutes
};

// Initialize and start session
let mut session_manager = FixSessionManager::new();
session_manager.initialize_session(&session_config).await?;
session_manager.start_session().await?;

// Check session status
if session_manager.is_session_active() {
    println!("Session is active");
    println!("Status: {:?}", session_manager.session_status());
}

// Send a message
let message = FixMessage::new_order_single(order);
session_manager.send_message(message).await?;

// Receive messages
loop {
    match session_manager.receive_message().await {
        Ok(Some(message)) => {
            println!("Received message: {:?}", message);
            // Process the message
        }
        Ok(None) => break,
        Err(error) => {
            eprintln!("Error receiving message: {:?}", error);
            break;
        }
    }
}

// Stop session
session_manager.stop_session().await?;
```

### Session Monitoring and Heartbeats

```rust
// Monitor session statistics
let stats = session_manager.get_session_stats();
println!("Session started: {}", stats.session_start_time);
println!("Messages sent: {}", stats.messages_sent);
println!("Messages received: {}", stats.messages_received);
println!("Heartbeats sent: {}", stats.heartbeats_sent);
println!("Sequence resets: {}", stats.sequence_resets);

// Get current sequence numbers
let (incoming_seq, outgoing_seq) = session_manager.get_sequence_numbers();
println!("Incoming sequence: {}", incoming_seq);
println!("Outgoing sequence: {}", outgoing_seq);

// Send heartbeat manually if needed
session_manager.send_heartbeat().await?;
```

### Session Recovery and Gap Filling

```rust
// Process sequence reset
let new_sequence = 1000;
session_manager.process_sequence_reset(new_sequence).await?;

// Resend messages from specific sequence number
let from_sequence = 950;
session_manager.resend_from_sequence(from_sequence).await?;

// Handle test request
let test_req_id = "TEST123";
session_manager.process_test_request(test_req_id).await?;
```

### Error Handling and Recovery

```rust
// Check session status and handle errors
match session_manager.session_status() {
    SessionStatus::Active => {
        println!("Session is running normally");
    }
    SessionStatus::Error(error) => {
        eprintln!("Session error: {:?}", error);
        
        // Attempt to recover session
        match session_manager.start_session().await {
            Ok(()) => println!("Session recovered successfully"),
            Err(recovery_error) => {
                eprintln!("Session recovery failed: {:?}", recovery_error);
                
                // Logout and reinitialize if recovery fails
                session_manager.logout("Recovery failed").await?;
                session_manager.initialize_session(&session_config).await?;
                session_manager.start_session().await?;
            }
        }
    }
    _ => {
        println!("Session status: {:?}", session_manager.session_status());
    }
}
```

## Module Dependencies

### Direct Dependencies

- **`async_trait`**: For async trait methods
- **`message`**: `FixMessage`
- **`config`**: `SessionConfig`
- **`error`**: `SessionError`
- **`chrono`**: `DateTime<Utc>`
- **`std::time`**: `Duration`

### Related Types

- **`SessionStatus`**: Enum representing session states
- **`SessionStats`**: Statistics about session performance
- **`SessionConfig`**: Configuration parameters for sessions
- **`SessionError`**: Specific error types for session operations
- **`FixMessage`**: FIX protocol messages

## Testing

### Mock Testing

```rust
use mockall::predicate::*;
use mockall::*;

mock! {
    SessionManagerMock {}
    
    #[async_trait]
    impl SessionManager for SessionManagerMock {
        async fn initialize_session(&mut self, config: &SessionConfig) -> Result<(), SessionError>;
        async fn start_session(&mut self) -> Result<(), SessionError>;
        async fn stop_session(&mut self) -> Result<(), SessionError>;
        fn is_session_active(&self) -> bool;
        fn session_status(&self) -> SessionStatus;
        async fn send_message(&mut self, message: FixMessage) -> Result<(), SessionError>;
        async fn receive_message(&mut self) -> Result<Option<FixMessage>, SessionError>;
        async fn send_heartbeat(&mut self) -> Result<(), SessionError>;
        async fn process_test_request(&mut self, test_req_id: &str) -> Result<(), SessionError>;
        async fn process_sequence_reset(&mut self, new_seq_num: u64) -> Result<(), SessionError>;
        fn get_session_stats(&self) -> SessionStats;
        fn get_sequence_numbers(&self) -> (u64, u64);
        async fn resend_from_sequence(&mut self, from_seq_num: u64) -> Result<(), SessionError>;
        async fn logout(&mut self, reason: &str) -> Result<(), SessionError>;
    }
}

#[tokio::test]
async fn test_session_manager_mock() {
    let mut mock = MockSessionManagerMock::new();
    
    // Set expectations
    mock.expect_initialize_session()
        .times(1)
        .returning(|_| Ok(()));
    
    mock.expect_start_session()
        .times(1)
        .returning(|| Ok(()));
    
    mock.expect_is_session_active()
        .times(1)
        .returning(|| true);
    
    // Test session initialization and start
    let config = SessionConfig::default();
    assert!(mock.initialize_session(&config).await.is_ok());
    assert!(mock.start_session().await.is_ok());
    assert!(mock.is_session_active());
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_session_lifecycle() {
    let config = SessionConfig::test_config();
    let mut session_manager = FixSessionManager::new();

    // Test session initialization
    assert!(session_manager.initialize_session(&config).await.is_ok());
    assert_eq!(session_manager.session_status(), SessionStatus::Disconnected);

    // Test session start
    assert!(session_manager.start_session().await.is_ok());
    assert!(session_manager.is_session_active());
    assert_eq!(session_manager.session_status(), SessionStatus::Active);

    // Test session stop
    assert!(session_manager.stop_session().await.is_ok());
    assert!(!session_manager.is_session_active());
    assert_eq!(session_manager.session_status(), SessionStatus::LoggedOut);
}

#[tokio::test]
async fn test_message_sequencing() {
    let mut session_manager = create_test_session_manager();
    
    // Send multiple messages and verify sequence numbers
    let message1 = FixMessage::heartbeat();
    let message2 = FixMessage::heartbeat();
    
    session_manager.send_message(message1).await?;
    session_manager.send_message(message2).await?;
    
    let (_, outgoing_seq) = session_manager.get_sequence_numbers();
    assert_eq!(outgoing_seq, 3); // Initial + 2 messages
}
```

## Performance Considerations

- **Message Batching**: Batch multiple messages when possible
- **Efficient Sequencing**: Use optimized sequence number management
- **Heartbeat Optimization**: Minimize heartbeat overhead
- **Gap Filling**: Implement efficient gap detection and filling
- **Session Recovery**: Fast session recovery mechanisms

## Security Considerations

- **Authentication**: Secure session authentication
- **Message Validation**: Validate all incoming messages
- **Sequence Security**: Prevent sequence number manipulation
- **Session Isolation**: Isolate sessions between different clients
- **Audit Logging**: Log all session events and errors
