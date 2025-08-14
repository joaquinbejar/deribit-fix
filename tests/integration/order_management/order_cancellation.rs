//! TEST 11: ORDER CANCELLATION
//!
//! This test covers the cancellation of existing orders:
//! 1. Submit a new limit order that will not be filled immediately.
//! 2. Send an `OrderCancelRequest` (F) using the order's `ClOrdID`.
//! 3. Receive and validate the `ExecutionReport` confirming the order state is `Canceled`.
//! 4. Verify the `OrderCancelReject` (9) is received if trying to cancel a filled or unknown order.

use dotenv;
use serial_test::serial;
use std::path::Path;
use std::time::Duration;
use tokio::time::{sleep, timeout};
use tracing::{debug, info, warn};

use deribit_base::prelude::{NewOrderRequest, OrderSide, OrderType, TimeInForce, setup_logger};
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
#[serial]
async fn test_order_cancellation_success() -> Result<()> {
    // Setup logging for test visibility
    unsafe {
        std::env::set_var("LOGLEVEL", "debug");
    }
    setup_logger();

    info!("=== Integration Test: Order Cancellation - Success ===");

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

    // Step 4: Submit a limit order that will not fill immediately (far from market price)
    info!("📤 Creating and sending limit order for cancellation test...");
    let symbol = "BTC-PERPETUAL".to_string();
    let price = 20000.0; // Far below market price to avoid immediate fill
    let quantity = 0.001;

    let order_request = NewOrderRequest {
        instrument_name: symbol.clone(),
        side: OrderSide::Buy,
        order_type: OrderType::Limit,
        amount: quantity,
        price: Some(price),
        time_in_force: TimeInForce::GoodTillCancel,
        post_only: Some(true), // Ensure it won't fill immediately
        reduce_only: Some(false),
        client_order_id: Some(format!(
            "TEST_CANCEL_{}",
            chrono::Utc::now().timestamp_millis()
        )),
        label: Some(format!(
            "TEST_CANCEL_{}",
            chrono::Utc::now().timestamp_millis()
        )),
        stop_price: None,
        trigger: None,
        advanced: None,
        max_show: None,
        reject_post_only: None,
        valid_until: None,
    };

    // Send the order
    let order_id = client.send_order(order_request).await?;
    info!(
        "📤 Limit order sent for cancellation test: OrderID={}, Symbol={}, Price={}, Qty={}",
        order_id, symbol, price, quantity
    );

    // Step 5: Wait for ExecutionReport confirming order is New
    info!("👁️ Waiting for ExecutionReport confirming order is New...");
    let mut order_new_confirmed = false;
    let monitor_duration = Duration::from_secs(30);
    let start_time = std::time::Instant::now();

    while start_time.elapsed() < monitor_duration && !order_new_confirmed {
        match timeout(Duration::from_millis(500), client.receive_message()).await {
            Ok(Ok(Some(message))) => {
                if let Some(msg_type) = message.get_field(35)
                    && msg_type == "8"
                {
                    // ExecutionReport
                    debug!("📊 Received ExecutionReport: {:?}", message);

                    if let Some(recv_cl_ord_id) = message.get_field(11)
                        && recv_cl_ord_id == &order_id
                        && let Some(ord_status) = message.get_field(39)
                        && ord_status == "0"
                    {
                        // New
                        info!("✅ Order confirmed as New, ready for cancellation");
                        order_new_confirmed = true;
                    }
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

    assert!(
        order_new_confirmed,
        "Expected ExecutionReport with New order status was not received"
    );

    // Step 6: Cancel the order
    info!("🚫 Sending order cancellation request...");
    client.cancel_order(order_id.clone()).await?;
    info!(
        "📤 Order cancellation request sent for OrderID: {}",
        order_id
    );

    // Step 7: Wait for ExecutionReport confirming order is Canceled
    info!("👁️ Waiting for ExecutionReport confirming order is Canceled...");
    let mut order_canceled_confirmed = false;
    let start_time = std::time::Instant::now();

    while start_time.elapsed() < monitor_duration && !order_canceled_confirmed {
        match timeout(Duration::from_millis(500), client.receive_message()).await {
            Ok(Ok(Some(message))) => {
                if let Some(msg_type) = message.get_field(35)
                    && msg_type == "8"
                {
                    // ExecutionReport
                    debug!("📊 Received ExecutionReport: {:?}", message);

                    if let Some(recv_cl_ord_id) = message.get_field(11)
                        && recv_cl_ord_id == &order_id
                        && let Some(ord_status) = message.get_field(39)
                        && ord_status == "4"
                    {
                        // Canceled
                        info!("✅ Order status confirmed as Canceled: {}", ord_status);
                        order_canceled_confirmed = true;

                        // Additional validation for canceled order
                        if let Some(exec_type) = message.get_field(150) {
                            assert_eq!(exec_type, "4", "ExecType should be Canceled (4)");
                            info!("✅ ExecType confirmed as Canceled: {}", exec_type);
                        }

                        if let Some(recv_symbol) = message.get_field(55) {
                            assert_eq!(recv_symbol, &symbol, "Symbol should match");
                            info!("✅ Symbol confirmed: {}", recv_symbol);
                        }
                    }
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

    assert!(
        order_canceled_confirmed,
        "Expected ExecutionReport with Canceled order status was not received"
    );

    // Clean up
    client.disconnect().await.ok();
    info!("✅ Test completed successfully - Order cancellation confirmed");

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_order_cancellation_unknown_order() -> Result<()> {
    // Setup logging for test visibility
    unsafe {
        std::env::set_var("LOGLEVEL", "debug");
    }
    setup_logger();

    info!("=== Integration Test: Order Cancellation - Unknown Order ===");

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

    // Step 4: Try to cancel a non-existent order
    info!("🚫 Attempting to cancel non-existent order...");
    let fake_order_id = format!("FAKE_ORDER_{}", chrono::Utc::now().timestamp_millis());

    // This should either fail or we should receive an OrderCancelReject
    let cancel_result = client.cancel_order(fake_order_id.clone()).await;
    info!("📤 Cancel request sent for fake OrderID: {}", fake_order_id);

    // Step 5: Wait for OrderCancelReject message
    info!("👁️ Waiting for OrderCancelReject message...");
    let mut cancel_reject_received = false;
    let monitor_duration = Duration::from_secs(30);
    let start_time = std::time::Instant::now();

    while start_time.elapsed() < monitor_duration && !cancel_reject_received {
        match timeout(Duration::from_millis(500), client.receive_message()).await {
            Ok(Ok(Some(message))) => {
                if let Some(msg_type) = message.get_field(35)
                    && msg_type == "9"
                {
                    // OrderCancelReject
                    info!("📨 Received OrderCancelReject: {:?}", message);
                    cancel_reject_received = true;

                    // Validate OrderCancelReject fields
                    if let Some(cxl_rej_reason) = message.get_field(102) {
                        info!("✅ CxlRejReason: {}", cxl_rej_reason);
                        assert!(
                            !cxl_rej_reason.is_empty(),
                            "CxlRejReason should not be empty"
                        );
                    }

                    if let Some(text) = message.get_field(58) {
                        info!("✅ Rejection text: {}", text);
                        assert!(
                            !text.is_empty(),
                            "Text field should contain rejection reason"
                        );
                    }
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

    // Note: Some implementations might reject the cancel request immediately,
    // others might send OrderCancelReject message
    if !cancel_reject_received && cancel_result.is_err() {
        info!("✅ Cancel request properly rejected at client level");
    } else {
        assert!(
            cancel_reject_received,
            "Expected OrderCancelReject message for unknown order was not received"
        );
    }

    // Clean up
    client.disconnect().await.ok();
    info!("✅ Test completed successfully - Unknown order cancellation handled correctly");

    Ok(())
}
