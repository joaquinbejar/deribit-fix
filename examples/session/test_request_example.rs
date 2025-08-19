//! Test Request message example for Deribit FIX API
//!
//! This example demonstrates how to create and send Test Request messages (MsgType = 1)
//! according to the FIX 4.4 specification. Test Request messages are used to test
//! connectivity when no messages have been received for a period of time.

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

    info!("=== Deribit FIX Test Request Example ===");

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

    // Example 1: Create a Test Request with custom ID
    info!("--- Example 1: Custom Test Request ---");
    let custom_id = "CUSTOM_TEST_123";
    let test_request = TestRequest::new(custom_id.to_string());
    info!("Created test request: {:?}", test_request);
    info!("Test Request ID: {}", test_request.test_req_id);

    let fix_message =
        test_request.to_fix_message("CLIENT".to_string(), "DERIBIT".to_string(), 2001)?;

    info!("FIX Message fields:");
    info!(
        "  MsgType (35): {}",
        fix_message.get_field(35).unwrap_or(&"N/A".to_string())
    );
    info!(
        "  TestReqID (112): {}",
        fix_message.get_field(112).unwrap_or(&"N/A".to_string())
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

    // Example 2: Create a Test Request with timestamp-based ID
    info!("--- Example 2: Timestamp-based Test Request ---");
    let timestamp_test = TestRequest::new_with_timestamp();
    info!("Created timestamp test request: {:?}", timestamp_test);
    info!(
        "Auto-generated Test Request ID: {}",
        timestamp_test.test_req_id
    );

    let fix_timestamp =
        timestamp_test.to_fix_message("CLIENT".to_string(), "DERIBIT".to_string(), 2002)?;

    info!("Timestamp-based FIX Message fields:");
    info!(
        "  MsgType (35): {}",
        fix_timestamp.get_field(35).unwrap_or(&"N/A".to_string())
    );
    info!(
        "  TestReqID (112): {}",
        fix_timestamp.get_field(112).unwrap_or(&"N/A".to_string())
    );

    // Example 3: Multiple Test Requests with different IDs
    info!("--- Example 3: Multiple Test Requests ---");
    for i in 1..=3 {
        let test_id = format!("BATCH_TEST_{}", i);
        let batch_test = TestRequest::new(test_id.clone());

        let fix_batch =
            batch_test.to_fix_message("CLIENT".to_string(), "DERIBIT".to_string(), 2000 + i + 2)?;

        info!(
            "Batch Test Request {}: ID={}, SeqNum={}",
            i,
            test_id,
            fix_batch.get_field(34).unwrap_or(&"N/A".to_string())
        );

        // Small delay between requests
        sleep(Duration::from_millis(500)).await;
    }

    // Example 4: Demonstrate Test Request -> Heartbeat Response cycle
    info!("--- Example 4: Test Request Response Cycle ---");
    let cycle_test_id = "CYCLE_TEST_456";
    let cycle_test = TestRequest::new(cycle_test_id.to_string());

    info!("Sending Test Request with ID: {}", cycle_test_id);
    let _test_fix = cycle_test.to_fix_message("CLIENT".to_string(), "DERIBIT".to_string(), 2006)?;

    // Simulate the expected heartbeat response
    info!("Expected Heartbeat response would contain:");
    let expected_heartbeat = Heartbeat::new_response(cycle_test_id.to_string());
    let heartbeat_fix =
        expected_heartbeat.to_fix_message("DERIBIT".to_string(), "CLIENT".to_string(), 3001)?;

    info!(
        "  Response MsgType (35): {}",
        heartbeat_fix.get_field(35).unwrap_or(&"N/A".to_string())
    );
    info!(
        "  Response TestReqID (112): {}",
        heartbeat_fix.get_field(112).unwrap_or(&"N/A".to_string())
    );
    info!("  Correlation: Test Request ID matches Heartbeat TestReqID");

    // Example 5: Connectivity testing scenario
    info!("--- Example 5: Connectivity Testing Scenario ---");
    info!("Simulating connectivity test after period of inactivity...");

    // Wait to simulate inactivity
    sleep(Duration::from_secs(3)).await;

    let connectivity_test = TestRequest::new_with_timestamp();
    info!("No messages received for 3 seconds, sending connectivity test:");
    info!("  Test Request ID: {}", connectivity_test.test_req_id);

    let _connectivity_fix =
        connectivity_test.to_fix_message("CLIENT".to_string(), "DERIBIT".to_string(), 2007)?;

    info!("Connectivity test message created successfully!");
    info!("In real scenario, we would wait for Heartbeat response within timeout period");

    // Keep connection alive briefly
    info!("Keeping connection alive for 10 seconds...");
    sleep(Duration::from_secs(10)).await;

    // Disconnect
    info!("Disconnecting...");
    client.disconnect().await?;

    info!("Test Request example completed successfully!");
    Ok(())
}
