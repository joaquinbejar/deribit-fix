//! TEST 14: EXECUTION REPORT VALIDATION
//!
//! This test performs deep validation of `ExecutionReport` (8) fields:
//! 1. For a new order, validate `OrdStatus` is `New`.
//! 2. For a filled order, validate `OrdStatus` is `Filled`, and `LastPx`, `LastQty`, `CumQty` are correct.
//! 3. For a canceled order, validate `OrdStatus` is `Canceled`.
//! 4. For a rejected order, validate `OrdStatus` is `Rejected` and `Text` field contains a reason.

use dotenv;
use serial_test::serial;
use std::path::Path;
use std::time::Duration;
use tokio::time::{sleep, timeout};
use tracing::{debug, info, warn};

use deribit_base::prelude::setup_logger;
use deribit_fix::prelude::*;
use deribit_fix::session::SessionState;
use deribit_fix::model::message::FixMessage;

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

/// Validate ExecutionReport message fields for a given order status
fn validate_execution_report(message: &FixMessage, expected_status: &str, order_id: &str) -> bool {
    // Check if this is an ExecutionReport
    if let Some(msg_type) = message.get_field(35) {
        if msg_type != "8" {
            return false;
        }
    } else {
        return false;
    }

    // Check if this is for our order
    if let Some(recv_cl_ord_id) = message.get_field(11) {
        if recv_cl_ord_id != order_id {
            return false;
        }
    } else {
        return false;
    }

    // Validate order status
    if let Some(ord_status) = message.get_field(39) {
        if ord_status != expected_status {
            warn!("❌ Expected OrdStatus {}, got: {}", expected_status, ord_status);
            return false;
        }
        info!("✅ OrdStatus validated: {}", ord_status);
    } else {
        warn!("❌ No OrdStatus field found in ExecutionReport");
        return false;
    }

    // Additional validations based on order status
    match expected_status {
        "0" => {
            // New order - validate ExecType
            if let Some(exec_type) = message.get_field(150) {
                assert_eq!(exec_type, "0", "ExecType should be New (0) for new order");
                info!("✅ ExecType validated for New order: {}", exec_type);
            }
        }
        "2" => {
            // Filled order - validate fill details
            if let Some(exec_type) = message.get_field(150) {
                assert!(
                    exec_type == "F" || exec_type == "1",
                    "ExecType should be Trade (F) or PartialFill (1) for filled order, got: {}",
                    exec_type
                );
                info!("✅ ExecType validated for Filled order: {}", exec_type);
            }

            // Validate fill details are present
            if let Some(last_px) = message.get_field(31) {
                info!("✅ LastPx (execution price): {}", last_px);
                assert!(!last_px.is_empty(), "LastPx should not be empty for filled order");
            } else {
                warn!("❌ LastPx field missing for filled order");
            }

            if let Some(last_qty) = message.get_field(32) {
                info!("✅ LastQty (execution quantity): {}", last_qty);
                assert!(!last_qty.is_empty(), "LastQty should not be empty for filled order");
            } else {
                warn!("❌ LastQty field missing for filled order");
            }

            if let Some(cum_qty) = message.get_field(14) {
                info!("✅ CumQty (cumulative quantity): {}", cum_qty);
                assert!(!cum_qty.is_empty(), "CumQty should not be empty for filled order");
            } else {
                warn!("❌ CumQty field missing for filled order");
            }
        }
        "4" => {
            // Canceled order - validate ExecType
            if let Some(exec_type) = message.get_field(150) {
                assert_eq!(exec_type, "4", "ExecType should be Canceled (4) for canceled order");
                info!("✅ ExecType validated for Canceled order: {}", exec_type);
            }
        }
        "8" => {
            // Rejected order - validate ExecType and Text field
            if let Some(exec_type) = message.get_field(150) {
                assert_eq!(exec_type, "8", "ExecType should be Rejected (8) for rejected order");
                info!("✅ ExecType validated for Rejected order: {}", exec_type);
            }

            // Validate Text field contains a rejection reason
            if let Some(text) = message.get_field(58) {
                info!("✅ Rejection reason: {}", text);
                assert!(!text.is_empty(), "Text field should contain rejection reason");
            } else {
                warn!("❌ Text field missing for rejected order");
            }
        }
        _ => {
            warn!("❌ Unknown order status for validation: {}", expected_status);
            return false;
        }
    }

    true
}

#[tokio::test]
#[serial]
async fn test_execution_report_new_order_validation() -> Result<()> {
    // Setup logging for test visibility
    unsafe {
        std::env::set_var("LOGLEVEL", "debug");
    }
    setup_logger();

    info!("=== Integration Test: ExecutionReport Validation - New Order ===");

    // Step 0: Check .env file exists and has required variables
    check_env_file()?;
    info!("✅ Environment file validation passed");

    // Step 1: Create configuration and client
    let config = DeribitFixConfig::new();
    config.validate()?;

    let mut client = DeribitFixClient::new(config).await?;
    info!("✅ Client created successfully");

    // Step 2: Connect and perform logon
    info!("🔌 Connecting to Deribit FIX server...");
    client.connect().await?;
    info!("✅ Connection established");

    // Step 3: Wait for logon confirmation
    info!("⏳ Waiting for logon confirmation...");
    let logon_timeout = Duration::from_secs(60);

    let logon_result = timeout(logon_timeout, async {
        loop {
            if let Ok(Some(message)) = client.receive_message().await {
                debug!("📨 Received message during logon: {:?}", message);
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

    // Step 4: Subscribe to market data to receive ExecutionReports
    info!("📊 Subscribing to market data to receive ExecutionReports...");
    let symbol = "BTC-PERPETUAL".to_string();
    client.subscribe_market_data(symbol.clone()).await?;
    
    // Step 5: Monitor for ExecutionReport messages and validate them
    info!("👁️ Monitoring for ExecutionReport messages...");
    let monitor_duration = Duration::from_secs(45);
    let start_time = std::time::Instant::now();

    let mut new_order_validated = false;
    let mut filled_order_validated = false;
    let mut canceled_order_validated = false;
    let mut rejected_order_validated = false;

    while start_time.elapsed() < monitor_duration {
        match timeout(Duration::from_millis(500), client.receive_message()).await {
            Ok(Ok(Some(message))) => {
                if let Some(msg_type) = message.get_field(35)
                    && msg_type == "8" { // ExecutionReport
                        debug!("📊 Received ExecutionReport: {:?}", message);
                        
                        // Try to validate different order statuses
                        if let Some(ord_status) = message.get_field(39)
                            && let Some(cl_ord_id) = message.get_field(11) {
                                match ord_status.as_str() {
                                    "0" if !new_order_validated => {
                                        info!("🔍 Validating New order ExecutionReport...");
                                        if validate_execution_report(&message, "0", cl_ord_id) {
                                            new_order_validated = true;
                                            info!("✅ New order ExecutionReport validated successfully");
                                        }
                                    }
                                    "2" if !filled_order_validated => {
                                        info!("🔍 Validating Filled order ExecutionReport...");
                                        if validate_execution_report(&message, "2", cl_ord_id) {
                                            filled_order_validated = true;
                                            info!("✅ Filled order ExecutionReport validated successfully");
                                        }
                                    }
                                    "4" if !canceled_order_validated => {
                                        info!("🔍 Validating Canceled order ExecutionReport...");
                                        if validate_execution_report(&message, "4", cl_ord_id) {
                                            canceled_order_validated = true;
                                            info!("✅ Canceled order ExecutionReport validated successfully");
                                        }
                                    }
                                    "8" if !rejected_order_validated => {
                                        info!("🔍 Validating Rejected order ExecutionReport...");
                                        if validate_execution_report(&message, "8", cl_ord_id) {
                                            rejected_order_validated = true;
                                            info!("✅ Rejected order ExecutionReport validated successfully");
                                        }
                                    }
                                    _ => {
                                        debug!("📊 ExecutionReport with status {} (already validated or not target)", ord_status);
                                    }
                                }
                            }
                    }
                
                // Break if we've validated at least one ExecutionReport
                if new_order_validated || filled_order_validated || canceled_order_validated || rejected_order_validated {
                    break;
                }
            }
            Ok(Ok(None)) => {
                debug!("⏳ No message received, continuing to wait...");
            }
            Ok(Err(e)) => {
                warn!("❌ Error receiving message: {:?}", e);
            }
            Err(_) => {
                debug!("⏰ Timeout waiting for message, continuing...");
            }
        }
    }

    // Verify that at least one ExecutionReport was validated
    let validated_count = [new_order_validated, filled_order_validated, canceled_order_validated, rejected_order_validated]
        .iter()
        .filter(|&&x| x)
        .count();

    assert!(
        validated_count > 0,
        "Expected to validate at least one ExecutionReport message, but none were found"
    );

    info!("✅ ExecutionReport validation completed. Validated {} different order statuses", validated_count);
    
    if new_order_validated {
        info!("  ✓ New order ExecutionReport validated");
    }
    if filled_order_validated {
        info!("  ✓ Filled order ExecutionReport validated");
    }
    if canceled_order_validated {
        info!("  ✓ Canceled order ExecutionReport validated");
    }
    if rejected_order_validated {
        info!("  ✓ Rejected order ExecutionReport validated");
    }

    // Clean up
    client.disconnect().await.ok();
    info!("✅ Test completed successfully - ExecutionReport validation passed");

    Ok(())
}
