//! Resend Request message example for Deribit FIX API
//!
//! This example demonstrates how to create and send Resend Request messages (MsgType = 2)
//! according to the FIX 4.4 specification. Resend Request messages are used to request
//! retransmission of messages when sequence gaps are detected.

use deribit_base::prelude::*;
use deribit_fix::prelude::*;
use tokio::time::{Duration, sleep};
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    // Setup logging with debug level to see all messages
    unsafe {
        std::env::set_var("LOGLEVEL", "debug");
    }
    // Setup logging
    setup_logger();

    info!("=== Deribit FIX Resend Request Example ===");

    // Create configuration for test environment
    let config = DeribitFixConfig::default()
        .with_heartbeat_interval(30)
        .with_logging(true, "debug".to_string());

    // Validate configuration
    if let Err(e) = config.validate() {
        error!("Configuration validation failed: {}", e);
        return Err(e);
    }

    info!("Creating Deribit FIX client...");
    let mut client = DeribitFixClient::new(&config).await?;

    info!("Connecting to Deribit FIX server...");
    client.connect().await?;
    info!("Connected successfully!");

    // Example 1: Request specific range of messages
    info!("--- Example 1: Specific Range Resend Request ---");
    let range_resend = ResendRequest::new(100, 110);
    info!("Created resend request: {:?}", range_resend);
    info!("Begin Sequence: {}", range_resend.begin_seq_no);
    info!("End Sequence: {}", range_resend.end_seq_no);
    info!("Is infinite range: {}", range_resend.is_infinite_range());
    info!("Message count: {:?}", range_resend.message_count());

    let fix_message =
        range_resend.to_fix_message("CLIENT".to_string(), "DERIBIT".to_string(), 3001)?;

    info!("FIX Message fields:");
    info!(
        "  MsgType (35): {}",
        fix_message.get_field(35).unwrap_or(&"N/A".to_string())
    );
    info!(
        "  BeginSeqNo (7): {}",
        fix_message.get_field(7).unwrap_or(&"N/A".to_string())
    );
    info!(
        "  EndSeqNo (16): {}",
        fix_message.get_field(16).unwrap_or(&"N/A".to_string())
    );
    info!(
        "  SenderCompID (49): {}",
        fix_message.get_field(49).unwrap_or(&"N/A".to_string())
    );
    info!(
        "  TargetCompID (56): {}",
        fix_message.get_field(56).unwrap_or(&"N/A".to_string())
    );
    info!(
        "  MsgSeqNum (34): {}",
        fix_message.get_field(34).unwrap_or(&"N/A".to_string())
    );

    // Example 2: Request all messages from a sequence number (infinite range)
    info!("--- Example 2: Infinite Range Resend Request ---");
    let infinite_resend = ResendRequest::new_from_sequence(75);
    info!("Created infinite resend request: {:?}", infinite_resend);
    info!("Begin Sequence: {}", infinite_resend.begin_seq_no);
    info!(
        "End Sequence: {} (0 = all messages from begin)",
        infinite_resend.end_seq_no
    );
    info!("Is infinite range: {}", infinite_resend.is_infinite_range());
    info!(
        "Message count: {:?} (None = infinite)",
        infinite_resend.message_count()
    );

    let fix_infinite =
        infinite_resend.to_fix_message("CLIENT".to_string(), "DERIBIT".to_string(), 3002)?;

    info!("Infinite Range FIX Message fields:");
    info!(
        "  MsgType (35): {}",
        fix_infinite.get_field(35).unwrap_or(&"N/A".to_string())
    );
    info!(
        "  BeginSeqNo (7): {}",
        fix_infinite.get_field(7).unwrap_or(&"N/A".to_string())
    );
    info!(
        "  EndSeqNo (16): {}",
        fix_infinite.get_field(16).unwrap_or(&"N/A".to_string())
    );

    // Example 3: Gap detection and recovery scenario
    info!("--- Example 3: Gap Detection Scenario ---");
    info!("Simulating sequence gap detection...");

    // Simulate receiving messages with sequence numbers: 50, 51, 52, 55, 56
    // Gap detected: missing 53, 54
    let expected_seq = 53;
    let received_seq = 55;
    let gap_start = expected_seq;
    let gap_end = received_seq - 1;

    warn!("Sequence gap detected!");
    warn!("  Expected sequence: {}", expected_seq);
    warn!("  Received sequence: {}", received_seq);
    warn!("  Gap range: {} to {}", gap_start, gap_end);

    let gap_resend = ResendRequest::new(gap_start, gap_end);
    info!("Created gap recovery resend request: {:?}", gap_resend);
    info!(
        "Requesting {} missing messages",
        gap_resend.message_count().unwrap_or(0)
    );

    let _fix_gap = gap_resend.to_fix_message("CLIENT".to_string(), "DERIBIT".to_string(), 3003)?;

    info!("Gap recovery message created successfully!");

    // Example 4: Multiple gap scenarios
    info!("--- Example 4: Multiple Gap Recovery ---");
    let gap_scenarios = vec![
        (10, 15, "Small gap"),
        (100, 105, "Medium gap"),
        (500, 520, "Large gap"),
    ];

    for (i, (start, end, description)) in gap_scenarios.iter().enumerate() {
        let multi_resend = ResendRequest::new(*start, *end);
        info!(
            "Gap scenario {}: {} (seq {} to {})",
            i + 1,
            description,
            start,
            end
        );
        info!(
            "  Messages to resend: {}",
            multi_resend.message_count().unwrap_or(0)
        );

        let fix_multi = multi_resend.to_fix_message(
            "CLIENT".to_string(),
            "DERIBIT".to_string(),
            3004 + i as u32,
        )?;

        info!(
            "  FIX SeqNum: {}",
            fix_multi.get_field(34).unwrap_or(&"N/A".to_string())
        );

        // Small delay between requests
        sleep(Duration::from_millis(200)).await;
    }

    // Example 5: Recovery after connection loss
    info!("--- Example 5: Connection Recovery Scenario ---");
    info!("Simulating recovery after connection loss...");

    // Assume last received sequence was 200, now reconnecting
    let last_received = 200;
    let recovery_resend = ResendRequest::new_from_sequence(last_received + 1);

    info!(
        "Last received sequence before disconnect: {}",
        last_received
    );
    info!(
        "Requesting all messages from sequence: {}",
        recovery_resend.begin_seq_no
    );
    info!("Recovery resend request: {:?}", recovery_resend);

    let _fix_recovery =
        recovery_resend.to_fix_message("CLIENT".to_string(), "DERIBIT".to_string(), 3007)?;

    info!("Connection recovery message created!");
    info!(
        "  This would request all messages from {} onwards",
        recovery_resend.begin_seq_no
    );

    // Example 6: Resend request validation
    info!("--- Example 6: Resend Request Validation ---");

    // Valid scenarios
    let valid_cases = vec![
        ResendRequest::new(1, 10),
        ResendRequest::new(50, 50),            // Single message
        ResendRequest::new_from_sequence(100), // Infinite
    ];

    for (i, resend) in valid_cases.iter().enumerate() {
        info!(
            "Valid case {}: BeginSeq={}, EndSeq={}, Count={:?}",
            i + 1,
            resend.begin_seq_no,
            resend.end_seq_no,
            resend.message_count()
        );
    }

    // Edge cases
    info!("Edge case - Single message resend:");
    let single_msg = ResendRequest::new(42, 42);
    info!(
        "  Single message at sequence {}: Count={:?}",
        single_msg.begin_seq_no,
        single_msg.message_count()
    );

    // Keep connection alive briefly
    info!("Keeping connection alive for 10 seconds...");
    sleep(Duration::from_secs(10)).await;

    // Disconnect
    info!("Disconnecting...");
    client.disconnect().await?;

    info!("Resend Request example completed successfully!");
    Ok(())
}
