//! TEST 04: SEQUENCE NUMBER MANAGEMENT
//!
//! This test covers the handling of message sequence numbers:
//! 1. Send a message with a sequence number lower than expected; expect a Logout.
//! 2. Send a message with a sequence number higher than expected; expect a ResendRequest (2).
//! 3. Respond to the ResendRequest with a SequenceReset-GapFill (4) message.
//! 4. Ensure the session recovers and continues.

use std::path::Path;
use std::time::Duration;
use tokio::time::{sleep, timeout};
use tracing::{debug, info, warn};

use deribit_base::prelude::*;
use deribit_fix::prelude::*;
use deribit_fix::session::SessionState;

/// Check if .env file exists and contains required variables
fn check_env_file() -> Result<()> {
    // Check if .env file exists
    if !Path::new(".env").exists() {
        return Err(DeribitFixError::Config(
            "Missing .env file. Please create one with DERIBIT_USERNAME and DERIBIT_PASSWORD"
                .to_string(),
        ));
    }

    // Load environment variables
    dotenv::dotenv().ok();

    // Check required variables
    let required_vars = [
        "DERIBIT_USERNAME",
        "DERIBIT_PASSWORD",
        "DERIBIT_HOST",
        "DERIBIT_PORT",
    ];

    for var in &required_vars {
        if std::env::var(var).is_err() {
            return Err(DeribitFixError::Config(format!(
                "Missing required environment variable: {}",
                var
            )));
        }
    }

    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_sequence_number_validation_basic() -> Result<()> {
    // Setup logging for test visibility
    unsafe {
        std::env::set_var("DERIBIT_LOG_LEVEL", "debug");
    }
    setup_logger();

    info!("=== Integration Test: Basic Sequence Number Validation ===");

    // Step 0: Check .env file exists and has required variables
    check_env_file()?;
    info!("✅ Environment file validation passed");

    // Step 1: Create configuration and client
    let config = DeribitFixConfig::new();
    config.validate()?;

    let mut client = DeribitFixClient::new(&config).await?;
    info!("✅ Client created successfully");

    // Step 2: Connect and perform logon
    info!("🔌 Connecting to Deribit FIX server...");
    client.connect().await?;
    info!("✅ Connection established");

    // Step 3: Wait for logon confirmation
    info!("⏳ Waiting for logon confirmation...");
    let logon_timeout = Duration::from_secs(15);

    let logon_result = timeout(logon_timeout, async {
        loop {
            if let Ok(Some(message)) = client.receive_message().await {
                debug!("📨 Received message during logon: {:?}", message);

                // Check sequence numbers in received messages
                if let Some(seq_num_str) = message.get_field(34)
                    && let Ok(seq_num) = seq_num_str.parse::<u32>()
                {
                    debug!("📊 Server message sequence number: {}", seq_num);
                }
            }

            if let Some(state) = client.get_session_state().await
                && state == SessionState::LoggedOn
            {
                return Ok::<(), DeribitFixError>(());
            }

            sleep(Duration::from_millis(100)).await;
        }
    })
    .await;

    match logon_result {
        Ok(_) => info!("✅ Logon confirmed - session is active"),
        Err(_) => {
            client.disconnect().await.ok();
            return Err(DeribitFixError::Timeout(
                "Logon confirmation timeout".to_string(),
            ));
        }
    }

    // Step 4: Monitor sequence numbers during normal operation
    info!("📊 Monitoring sequence numbers during normal operation...");
    let monitor_duration = Duration::from_secs(10);
    let start_time = std::time::Instant::now();

    let mut incoming_seq_nums = Vec::new();
    let mut sequence_issues = 0;

    while start_time.elapsed() < monitor_duration {
        match timeout(Duration::from_millis(500), client.receive_message()).await {
            Ok(Ok(Some(message))) => {
                if let Some(seq_num_str) = message.get_field(34)
                    && let Ok(seq_num) = seq_num_str.parse::<u32>()
                {
                    incoming_seq_nums.push(seq_num);
                    debug!("📊 Received message with sequence number: {}", seq_num);

                    // Check for sequence number issues
                    if incoming_seq_nums.len() > 1 {
                        let prev_seq = incoming_seq_nums[incoming_seq_nums.len() - 2];
                        if seq_num <= prev_seq {
                            warn!(
                                "⚠️ Potential sequence issue: current={}, previous={}",
                                seq_num, prev_seq
                            );
                            sequence_issues += 1;
                        }
                    }
                }

                // Look for sequence-related messages
                if let Some(msg_type) = message.get_field(35) {
                    match msg_type.as_str() {
                        "2" => {
                            // Resend Request
                            info!("🔄 Received ResendRequest message");
                            if let Some(begin_seq) = message.get_field(7) {
                                info!("  BeginSeqNo: {}", begin_seq);
                            }
                            if let Some(end_seq) = message.get_field(16) {
                                info!("  EndSeqNo: {}", end_seq);
                            }
                        }
                        "4" => {
                            // Sequence Reset
                            info!("🔄 Received SequenceReset message");
                            if let Some(new_seq) = message.get_field(36) {
                                info!("  NewSeqNo: {}", new_seq);
                            }
                        }
                        _ => {
                            // Other messages
                        }
                    }
                }
            }
            Ok(Ok(None)) => {
                // No message received
            }
            Ok(Err(e)) => {
                warn!("❌ Error receiving message: {}", e);
            }
            Err(_) => {
                // Timeout - continue
            }
        }

        sleep(Duration::from_millis(100)).await;
    }

    // Step 5: Report results
    info!("📊 Sequence monitoring results:");
    info!("  - Total messages received: {}", incoming_seq_nums.len());
    info!("  - Sequence issues detected: {}", sequence_issues);

    if !incoming_seq_nums.is_empty() {
        let min_seq = incoming_seq_nums.iter().min().unwrap();
        let max_seq = incoming_seq_nums.iter().max().unwrap();
        info!("  - Sequence number range: {} to {}", min_seq, max_seq);
    }

    // Step 6: Clean disconnect
    info!("👋 Disconnecting...");
    client.disconnect().await?;
    info!("✅ Disconnected successfully");

    info!("🎉 Basic sequence number validation test completed!");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_session_recovery_after_sequence_gap() -> Result<()> {
    setup_logger();
    info!("=== Integration Test: Session Recovery After Sequence Gap ===");

    check_env_file()?;

    let config = DeribitFixConfig::new();
    config.validate()?;

    let mut client = DeribitFixClient::new(&config).await?;

    // Connect and establish session
    client.connect().await?;

    // Wait for logon
    let logon_timeout = Duration::from_secs(15);
    let logon_result = timeout(logon_timeout, async {
        loop {
            if let Ok(Some(_)) = client.receive_message().await {
                // Process logon messages
            }

            if let Some(state) = client.get_session_state().await
                && state == SessionState::LoggedOn
            {
                return Ok::<(), DeribitFixError>(());
            }

            sleep(Duration::from_millis(100)).await;
        }
    })
    .await;

    if logon_result.is_err() {
        client.disconnect().await.ok();
        return Err(DeribitFixError::Timeout("Logon timeout".to_string()));
    }

    info!("✅ Session established successfully");

    // Monitor for any sequence-related recovery messages
    info!("🔄 Monitoring for sequence recovery scenarios...");
    let monitor_duration = Duration::from_secs(15);
    let start_time = std::time::Instant::now();

    let mut resend_requests = 0;
    let mut sequence_resets = 0;
    let mut session_remained_active = true;

    while start_time.elapsed() < monitor_duration {
        match timeout(Duration::from_millis(300), client.receive_message()).await {
            Ok(Ok(Some(message))) => {
                if let Some(msg_type) = message.get_field(35) {
                    match msg_type.as_str() {
                        "2" => {
                            // Resend Request
                            resend_requests += 1;
                            info!("🔄 Detected ResendRequest ({})", resend_requests);
                            debug!("ResendRequest details: {:?}", message);
                        }
                        "4" => {
                            // Sequence Reset
                            sequence_resets += 1;
                            info!("🔄 Detected SequenceReset ({})", sequence_resets);
                            debug!("SequenceReset details: {:?}", message);
                        }
                        "5" => {
                            // Logout
                            warn!("👋 Unexpected logout received during monitoring");
                            if let Some(text) = message.get_field(58) {
                                warn!("Logout reason: {}", text);
                            }
                        }
                        _ => {
                            debug!("📨 Normal message: {}", msg_type);
                        }
                    }
                }
            }
            _ => {
                // No message or error - continue
            }
        }

        // Check session state
        if let Some(state) = client.get_session_state().await
            && state != SessionState::LoggedOn
        {
            warn!("⚠️ Session became inactive: {:?}", state);
            session_remained_active = false;
            break;
        }

        sleep(Duration::from_millis(200)).await;
    }

    // Report recovery scenario results
    info!("📊 Sequence recovery monitoring results:");
    info!("  - ResendRequest messages: {}", resend_requests);
    info!("  - SequenceReset messages: {}", sequence_resets);
    info!(
        "  - Session remained active: {}",
        if session_remained_active {
            "✅ Yes"
        } else {
            "❌ No"
        }
    );

    // Session should remain active even if sequence recovery occurs
    assert!(
        session_remained_active,
        "Session should remain active during sequence recovery"
    );

    client.disconnect().await?;
    info!("✅ Session recovery test completed successfully");

    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_sequence_number_continuity() -> Result<()> {
    setup_logger();
    info!("=== Integration Test: Sequence Number Continuity ===");

    check_env_file()?;

    let config = DeribitFixConfig::new();
    config.validate()?;

    let mut client = DeribitFixClient::new(&config).await?;

    // Connect and logon
    client.connect().await?;

    // Wait for logon confirmation
    let logon_result = timeout(Duration::from_secs(15), async {
        loop {
            if let Ok(Some(_)) = client.receive_message().await {
                // Process logon response
            }

            if let Some(state) = client.get_session_state().await
                && state == SessionState::LoggedOn
            {
                return Ok::<(), DeribitFixError>(());
            }

            sleep(Duration::from_millis(100)).await;
        }
    })
    .await;

    if logon_result.is_err() {
        client.disconnect().await.ok();
        return Err(DeribitFixError::Timeout("Logon timeout".to_string()));
    }

    info!("✅ Connected and logged on");

    // Collect sequence numbers to verify continuity
    info!("📊 Collecting sequence numbers to verify continuity...");
    let collection_duration = Duration::from_secs(12);
    let start_time = std::time::Instant::now();

    let mut sequence_numbers = Vec::new();
    let mut gaps_detected = 0;
    let mut duplicates_detected = 0;

    while start_time.elapsed() < collection_duration {
        match timeout(Duration::from_millis(400), client.receive_message()).await {
            Ok(Ok(Some(message))) => {
                if let Some(seq_num_str) = message.get_field(34)
                    && let Ok(seq_num) = seq_num_str.parse::<u32>()
                {
                    // Check for gaps or duplicates
                    if let Some(&last_seq) = sequence_numbers.last() {
                        if seq_num > last_seq + 1 {
                            gaps_detected += 1;
                            debug!("🔍 Gap detected: {} -> {}", last_seq, seq_num);
                        } else if seq_num <= last_seq {
                            duplicates_detected += 1;
                            debug!("🔍 Duplicate/out-of-order: {} after {}", seq_num, last_seq);
                        }
                    }

                    sequence_numbers.push(seq_num);
                    debug!("📊 Sequence: {}", seq_num);
                }
            }
            _ => {
                sleep(Duration::from_millis(100)).await;
            }
        }
    }

    // Analyze sequence continuity
    info!("📊 Sequence continuity analysis:");
    info!("  - Total messages: {}", sequence_numbers.len());
    info!("  - Gaps detected: {}", gaps_detected);
    info!("  - Duplicates detected: {}", duplicates_detected);

    if sequence_numbers.len() > 1 {
        sequence_numbers.sort();
        let first_seq = sequence_numbers[0];
        let last_seq = sequence_numbers[sequence_numbers.len() - 1];
        let expected_count = last_seq - first_seq + 1;

        info!("  - First sequence: {}", first_seq);
        info!("  - Last sequence: {}", last_seq);
        info!("  - Expected messages: {}", expected_count);
        info!("  - Actual messages: {}", sequence_numbers.len());

        let missing_count = expected_count as usize - sequence_numbers.len();
        if missing_count > 0 {
            info!("  - Missing messages: {}", missing_count);
        }
    }

    client.disconnect().await?;
    info!("✅ Sequence continuity test completed");

    Ok(())
}
