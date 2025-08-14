//! Heartbeat message example for Deribit FIX API
//!
//! This example demonstrates how to create and send Heartbeat messages (MsgType = 0)
//! according to the FIX 4.4 specification. Heartbeat messages are used to maintain
//! session connectivity and respond to Test Request messages.

use deribit_base::prelude::*;
use deribit_fix::prelude::*;
use tokio::time::{Duration, sleep};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    // Setup logging with debug level to see all messages
    unsafe {
        std::env::set_var("LOGLEVEL", "debug");
    }
    // Setup logging
    setup_logger();

    info!("=== Deribit FIX Heartbeat Example ===");

    // Create configuration for test environment
    let config = DeribitFixConfig::default()
        .with_heartbeat_interval(10) // Short interval for demo
        .with_logging(true, "debug".to_string());

    // Validate configuration
    if let Err(e) = config.validate() {
        error!("Configuration validation failed: {}", e);
        return Err(e);
    }

    info!("Creating Deribit FIX client...");
    let mut client = DeribitFixClient::new(config).await?;

    info!("Connecting to Deribit FIX server...");
    client.connect().await?;
    info!("Connected successfully!");

    // Example 1: Create a periodic heartbeat (no TestReqID)
    info!("--- Example 1: Periodic Heartbeat ---");
    let heartbeat = Heartbeat::new();
    info!("Created periodic heartbeat: {:?}", heartbeat);

    let fix_message =
        heartbeat.to_fix_message("CLIENT".to_string(), "DERIBIT".to_string(), 1001)?;

    info!("FIX Message fields:");
    info!(
        "  MsgType (35): {}",
        fix_message.get_field(35).unwrap_or(&"N/A".to_string())
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

    // Example 2: Create a heartbeat response to a Test Request
    info!("--- Example 2: Heartbeat Response ---");
    let test_req_id = "TESTREQ_12345";
    let heartbeat_response = Heartbeat::new_response(test_req_id.to_string());
    info!("Created heartbeat response: {:?}", heartbeat_response);
    info!(
        "Is test response: {}",
        heartbeat_response.is_test_response()
    );

    let fix_response =
        heartbeat_response.to_fix_message("CLIENT".to_string(), "DERIBIT".to_string(), 1002)?;

    info!("FIX Response fields:");
    info!(
        "  MsgType (35): {}",
        fix_response.get_field(35).unwrap_or(&"N/A".to_string())
    );
    info!(
        "  TestReqID (112): {}",
        fix_response.get_field(112).unwrap_or(&"N/A".to_string())
    );

    // Example 3: Demonstrate automatic heartbeat functionality
    info!("--- Example 3: Automatic Heartbeats ---");
    info!("Keeping connection alive for 30 seconds to observe automatic heartbeats...");
    info!("Check the debug logs to see heartbeat messages being sent automatically.");

    // Keep connection alive to see automatic heartbeats
    for i in 1..=6 {
        sleep(Duration::from_secs(5)).await;
        info!("Heartbeat demo: {} seconds elapsed", i * 5);
    }

    // Example 4: Manual heartbeat sending
    info!("--- Example 4: Manual Heartbeat ---");
    info!("Sending manual heartbeat...");

    // Note: In a real implementation, you would send this through the session
    // For demo purposes, we're just showing the message creation
    let manual_heartbeat = Heartbeat::new();
    let manual_fix =
        manual_heartbeat.to_fix_message("CLIENT".to_string(), "DERIBIT".to_string(), 1003)?;

    info!("Manual heartbeat created successfully!");
    info!(
        "Message would be sent: MsgType={}, SeqNum={}",
        manual_fix.get_field(35).unwrap_or(&"N/A".to_string()),
        manual_fix.get_field(34).unwrap_or(&"N/A".to_string())
    );

    // Disconnect
    info!("Disconnecting...");
    client.disconnect().await?;

    info!("Heartbeat example completed successfully!");
    Ok(())
}
