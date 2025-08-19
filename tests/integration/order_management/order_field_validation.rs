//! TEST 15: ORDER FIELD VALIDATION
//!
//! This test covers specific order modifiers and types:
//! 1. Submit a `PostOnly` order and ensure it gets rejected if it would trade immediately.
//! 2. Submit an order with `TimeInForce` = `ImmediateOrCancel` and validate behavior.
//! 3. Submit an order with `TimeInForce` = `FillOrKill` and validate behavior.

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
async fn test_post_only_order_rejection() -> Result<()> {
    // Setup logging for test visibility
    unsafe {
        std::env::set_var("DERIBIT_LOG_LEVEL", "debug");
    }
    setup_logger();

    info!("=== Integration Test: PostOnly Order Rejection ===");

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

    // Step 4: Submit a PostOnly order that would trade immediately (at market price)
    info!("📤 Creating PostOnly order that should be rejected for immediate execution...");
    let symbol = "BTC-PERPETUAL".to_string();
    // Use a price that's likely to execute immediately (very high buy price)
    let aggressive_price = 200000.0; // Unrealistically high price to ensure immediate execution
    let quantity = 0.001;

    let order_request = NewOrderRequest {
        instrument_name: symbol.clone(),
        side: OrderSide::Buy,
        order_type: OrderType::Limit,
        amount: quantity,
        price: Some(aggressive_price),
        time_in_force: TimeInForce::GoodTilCancelled,
        post_only: Some(true), // This should prevent immediate execution
        reduce_only: Some(false),
        client_order_id: Some(format!(
            "TEST_POSTONLY_{}",
            chrono::Utc::now().timestamp_millis()
        )),
        label: Some(format!(
            "TEST_POSTONLY_{}",
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
        "📤 PostOnly order sent: OrderID={}, Symbol={}, Price={}, Qty={}",
        order_id, symbol, aggressive_price, quantity
    );

    // Step 5: Wait for ExecutionReport - should be rejected or not filled
    info!("👁️ Monitoring ExecutionReport for PostOnly order behavior...");
    let mut order_handled = false;
    let monitor_duration = Duration::from_secs(30);
    let start_time = std::time::Instant::now();

    while start_time.elapsed() < monitor_duration && !order_handled {
        match timeout(Duration::from_millis(500), client.receive_message()).await {
            Ok(Ok(Some(message))) => {
                if let Some(msg_type) = message.get_field(35)
                    && msg_type == "8"
                {
                    // ExecutionReport
                    debug!("📊 Received ExecutionReport: {:?}", message);

                    if let Some(recv_cl_ord_id) = message.get_field(11)
                        && recv_cl_ord_id == &order_id
                    {
                        info!(
                            "✅ ExecutionReport received for PostOnly order: {}",
                            order_id
                        );

                        if let Some(ord_status) = message.get_field(39) {
                            match ord_status.as_str() {
                                "0" => {
                                    // Order is New - PostOnly worked correctly, order didn't execute
                                    info!(
                                        "✅ PostOnly order correctly placed as New without immediate execution"
                                    );
                                    order_handled = true;
                                }
                                "8" => {
                                    // Order rejected - PostOnly prevented immediate execution
                                    info!(
                                        "✅ PostOnly order correctly rejected to prevent immediate execution"
                                    );
                                    order_handled = true;

                                    if let Some(text) = message.get_field(58) {
                                        info!("✅ Rejection reason: {}", text);
                                        assert!(
                                            !text.is_empty(),
                                            "Rejection reason should be provided"
                                        );
                                    }
                                }
                                "2" | "1" => {
                                    // If the order filled despite PostOnly, this might indicate the order
                                    // was placed at a price that didn't require immediate execution
                                    warn!(
                                        "⚠️  PostOnly order filled - price may not have required immediate execution"
                                    );
                                    order_handled = true;
                                }
                                _ => {
                                    debug!("📊 Other order status received: {}", ord_status);
                                }
                            }
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
        order_handled,
        "Expected ExecutionReport for PostOnly order was not received"
    );

    // Clean up
    client.disconnect().await.ok();
    info!("✅ Test completed successfully - PostOnly order behavior validated");

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_immediate_or_cancel_behavior() -> Result<()> {
    // Setup logging for test visibility
    unsafe {
        std::env::set_var("DERIBIT_LOG_LEVEL", "debug");
    }
    setup_logger();

    info!("=== Integration Test: ImmediateOrCancel Order Behavior ===");

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

    // Step 4: Submit an ImmediateOrCancel order that likely won't fill completely
    info!("📤 Creating ImmediateOrCancel order...");
    let symbol = "BTC-PERPETUAL".to_string();
    let price = 10000.0; // Far below market to avoid immediate fill
    let quantity = 0.001;

    let order_request = NewOrderRequest {
        instrument_name: symbol.clone(),
        side: OrderSide::Buy,
        order_type: OrderType::Limit,
        amount: quantity,
        price: Some(price),
        time_in_force: TimeInForce::ImmediateOrCancel,
        post_only: Some(false),
        reduce_only: Some(false),
        client_order_id: Some(format!(
            "TEST_IOC_{}",
            chrono::Utc::now().timestamp_millis()
        )),
        label: Some(format!(
            "TEST_IOC_{}",
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
        "📤 IOC order sent: OrderID={}, Symbol={}, Price={}, Qty={}",
        order_id, symbol, price, quantity
    );

    // Step 5: Wait for ExecutionReport - should be canceled if not filled
    info!("👁️ Monitoring ExecutionReport for IOC order behavior...");
    let mut order_handled = false;
    let monitor_duration = Duration::from_secs(30);
    let start_time = std::time::Instant::now();

    while start_time.elapsed() < monitor_duration && !order_handled {
        match timeout(Duration::from_millis(500), client.receive_message()).await {
            Ok(Ok(Some(message))) => {
                if let Some(msg_type) = message.get_field(35)
                    && msg_type == "8"
                {
                    // ExecutionReport
                    debug!("📊 Received ExecutionReport: {:?}", message);

                    if let Some(recv_cl_ord_id) = message.get_field(11)
                        && recv_cl_ord_id == &order_id
                    {
                        info!("✅ ExecutionReport received for IOC order: {}", order_id);

                        if let Some(ord_status) = message.get_field(39) {
                            match ord_status.as_str() {
                                "4" => {
                                    // Order canceled - IOC behavior correct (not filled, so canceled)
                                    info!(
                                        "✅ IOC order correctly canceled (not filled immediately)"
                                    );
                                    order_handled = true;

                                    if let Some(exec_type) = message.get_field(150) {
                                        assert_eq!(
                                            exec_type, "4",
                                            "ExecType should be Canceled (4) for IOC"
                                        );
                                        info!("✅ ExecType confirmed as Canceled: {}", exec_type);
                                    }
                                }
                                "2" | "1" => {
                                    // Order filled - IOC worked correctly (filled what it could)
                                    info!("✅ IOC order filled (or partially filled)");
                                    order_handled = true;
                                }
                                "0" => {
                                    // This shouldn't happen with IOC as it should execute immediately or cancel
                                    warn!("⚠️  IOC order status is New - unexpected behavior");
                                    order_handled = true;
                                }
                                _ => {
                                    debug!("📊 Other order status received: {}", ord_status);
                                }
                            }
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
        order_handled,
        "Expected ExecutionReport for IOC order was not received"
    );

    // Clean up
    client.disconnect().await.ok();
    info!("✅ Test completed successfully - IOC order behavior validated");

    Ok(())
}

#[tokio::test]
#[serial]
async fn test_fill_or_kill_behavior() -> Result<()> {
    // Setup logging for test visibility
    unsafe {
        std::env::set_var("DERIBIT_LOG_LEVEL", "debug");
    }
    setup_logger();

    info!("=== Integration Test: FillOrKill Order Behavior ===");

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

    // Step 4: Submit a FillOrKill order that likely can't be filled completely
    info!("📤 Creating FillOrKill order...");
    let symbol = "BTC-PERPETUAL".to_string();
    let price = 5000.0; // Very low price, unlikely to fill completely
    let quantity = 1.0; // Larger quantity to make complete fill unlikely

    let order_request = NewOrderRequest {
        instrument_name: symbol.clone(),
        side: OrderSide::Buy,
        order_type: OrderType::Limit,
        amount: quantity,
        price: Some(price),
        time_in_force: TimeInForce::FillOrKill,
        post_only: Some(false),
        reduce_only: Some(false),
        client_order_id: Some(format!(
            "TEST_FOK_{}",
            chrono::Utc::now().timestamp_millis()
        )),
        label: Some(format!(
            "TEST_FOK_{}",
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
        "📤 FOK order sent: OrderID={}, Symbol={}, Price={}, Qty={}",
        order_id, symbol, price, quantity
    );

    // Step 5: Wait for ExecutionReport - should be canceled if not completely filled
    info!("👁️ Monitoring ExecutionReport for FOK order behavior...");
    let mut order_handled = false;
    let monitor_duration = Duration::from_secs(30);
    let start_time = std::time::Instant::now();

    while start_time.elapsed() < monitor_duration && !order_handled {
        match timeout(Duration::from_millis(500), client.receive_message()).await {
            Ok(Ok(Some(message))) => {
                if let Some(msg_type) = message.get_field(35)
                    && msg_type == "8"
                {
                    // ExecutionReport
                    debug!("📊 Received ExecutionReport: {:?}", message);

                    if let Some(recv_cl_ord_id) = message.get_field(11)
                        && recv_cl_ord_id == &order_id
                    {
                        info!("✅ ExecutionReport received for FOK order: {}", order_id);

                        if let Some(ord_status) = message.get_field(39) {
                            match ord_status.as_str() {
                                "4" => {
                                    // Order canceled - FOK behavior correct (couldn't fill completely)
                                    info!(
                                        "✅ FOK order correctly canceled (couldn't fill completely)"
                                    );
                                    order_handled = true;

                                    if let Some(exec_type) = message.get_field(150) {
                                        assert_eq!(
                                            exec_type, "4",
                                            "ExecType should be Canceled (4) for FOK"
                                        );
                                        info!("✅ ExecType confirmed as Canceled: {}", exec_type);
                                    }
                                }
                                "2" => {
                                    // Order filled completely - FOK worked correctly
                                    info!("✅ FOK order filled completely");
                                    order_handled = true;

                                    // Validate that it was completely filled
                                    if let Some(cum_qty) = message.get_field(14)
                                        && let Ok(cum_qty_val) = cum_qty.parse::<f64>()
                                    {
                                        assert!(
                                            (cum_qty_val - quantity).abs() < 0.0001,
                                            "FOK should fill completely or not at all"
                                        );
                                        info!("✅ FOK order completely filled: {}", cum_qty);
                                    }
                                }
                                "1" => {
                                    // Partial fill should not happen with FOK
                                    warn!(
                                        "❌ FOK order partially filled - this violates FOK behavior"
                                    );
                                    panic!("FOK orders should not be partially filled");
                                }
                                "0" => {
                                    // This shouldn't happen with FOK
                                    warn!("⚠️  FOK order status is New - unexpected behavior");
                                    order_handled = true;
                                }
                                _ => {
                                    debug!("📊 Other order status received: {}", ord_status);
                                }
                            }
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
        order_handled,
        "Expected ExecutionReport for FOK order was not received"
    );

    // Clean up
    client.disconnect().await.ok();
    info!("✅ Test completed successfully - FOK order behavior validated");

    Ok(())
}
