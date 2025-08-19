//! Position Management Example
//!
//! This example demonstrates how to:
//! 1. Connect to the Deribit FIX server
//! 2. Create a position with a market order
//! 3. Request position information
//! 4. Display position details
//! 5. Close the position at the end

use chrono;
use deribit_base::prelude::{NewOrderRequest, OrderSide, OrderType, TimeInForce, setup_logger};
use deribit_fix::prelude::*;
use deribit_fix::session::SessionState;
use std::time::Duration;
use tokio::time::{sleep, timeout};
use tracing::{debug, error, info};

#[tokio::main]
async fn main() -> Result<()> {
    // Setup logging
    unsafe {
        std::env::set_var("DERIBIT_LOG_LEVEL", "DEBUG");
    }
    setup_logger();

    info!("=== Position Management Example ===");

    // Create configuration for test environment
    let config = DeribitFixConfig::default()
        .with_heartbeat_interval(30)
        .with_cancel_on_disconnect(true)
        .with_logging(true, "info".to_string());

    // Validate configuration
    if let Err(e) = config.validate() {
        error!("Configuration validation failed: {}", e);
        return Err(e);
    }

    info!("Creating Deribit FIX client...");
    let mut client = DeribitFixClient::new(config).await?;

    info!("Connecting to Deribit FIX server...");
    client.connect().await?;
    info!("Connection established");

    // Wait for logon confirmation
    info!("Waiting for logon confirmation...");
    let logon_timeout = Duration::from_secs(30);

    let logon_result = timeout(logon_timeout, async {
        loop {
            if let Ok(Some(_message)) = client.receive_message().await {
                // Process any initial messages
            }

            if let Some(state) = client.get_session_state().await {
                if state == SessionState::LoggedOn {
                    return Ok::<(), DeribitFixError>(());
                }
            }

            sleep(Duration::from_millis(100)).await;
        }
    })
    .await;

    match logon_result {
        Ok(_) => info!("Logon confirmed - session is active"),
        Err(_) => {
            error!("Logon confirmation timeout");
            client.disconnect().await.ok();
            return Err(DeribitFixError::Timeout("Logon timeout".to_string()));
        }
    }

    // Step 1: Create a position with a market order
    info!("=== Step 1: Creating Position with Market Order ===");
    let instrument = "BTC-PERPETUAL";
    let quantity = 10.0;

    let market_order_request = NewOrderRequest {
        instrument_name: instrument.to_string(),
        side: OrderSide::Buy,
        order_type: OrderType::Market,
        amount: quantity,
        price: None,
        time_in_force: TimeInForce::ImmediateOrCancel,
        post_only: Some(false),
        reduce_only: Some(false),
        client_order_id: Some(format!(
            "MARKET_BUY_{}",
            chrono::Utc::now().timestamp_millis()
        )),
        label: Some(format!(
            "MARKET_BUY_{}",
            chrono::Utc::now().timestamp_millis()
        )),
        stop_price: None,
        trigger: None,
        advanced: None,
        max_show: None,
        reject_post_only: None,
        valid_until: None,
    };

    info!(
        "Sending market buy order: {} {} at market price",
        quantity, instrument
    );

    let market_order_id = match client.send_order(market_order_request).await {
        Ok(order_id) => {
            info!("Market buy order sent successfully with ID: {}", order_id);
            Some(order_id)
        }
        Err(e) => {
            error!("Failed to send market buy order: {}", e);
            None
        }
    };

    // Wait for order processing and execution
    if market_order_id.is_some() {
        info!("Waiting for order execution...");
        let mut fill_confirmed = false;
        let start_time = std::time::Instant::now();
        let order_timeout = Duration::from_secs(10);

        while start_time.elapsed() < order_timeout && !fill_confirmed {
            match timeout(Duration::from_millis(1000), client.receive_message()).await {
                Ok(Ok(Some(message))) => {
                    if let Some(msg_type) = message.get_field(35)
                        && msg_type == "8"
                    {
                        // ExecutionReport
                        if let Some(recv_cl_ord_id) = message.get_field(11)
                            && recv_cl_ord_id == market_order_id.as_ref().unwrap()
                            && let Some(ord_status) = message.get_field(39)
                            && (ord_status == "2" || ord_status == "1")
                        {
                            // Filled or PartiallyFilled
                            info!("✅ Market order executed successfully!");
                            fill_confirmed = true;

                            if let Some(last_px) = message.get_field(31) {
                                info!("📊 Fill price: {}", last_px);
                            }
                            if let Some(last_qty) = message.get_field(32) {
                                info!("📊 Fill quantity: {}", last_qty);
                            }
                        }
                    }
                }
                Ok(Ok(None)) => {
                    debug!("⏳ No message received, continuing to wait...");
                }
                Ok(Err(e)) => {
                    debug!("❌ Error receiving message: {:?}", e);
                }
                Err(_) => {
                    debug!("⏰ Timeout waiting for message, continuing...");
                }
            }
        }

        if !fill_confirmed {
            info!("ℹ️ Fill confirmation timeout, but continuing with position check");
        }
    }

    sleep(Duration::from_secs(2)).await;

    // Step 2: Request current positions
    info!("=== Step 2: Requesting Current Positions ===");

    match client.get_positions().await {
        Ok(positions) => {
            info!("Retrieved {} positions", positions.len());

            if positions.is_empty() {
                info!("No positions found");
            } else {
                for (index, position) in positions.iter().enumerate() {
                    info!(
                        "Position {}: {} - Qty: {}, Avg Price: {}, Unrealized PnL: {:?}, Realized PnL: {:?}",
                        index + 1,
                        position.instrument_name,
                        position.size,
                        position.average_price,
                        position.unrealized_profit_loss,
                        position.realized_profit_loss
                    );
                }
            }
        }
        Err(e) => {
            error!("Failed to get positions: {}", e);
        }
    }

    // Keep connection alive for a bit to receive any position updates
    info!("Monitoring for position updates for 10 seconds...");
    sleep(Duration::from_secs(10)).await;

    // Step 3: Close the position by placing an opposite market order
    info!("=== Step 3: Closing Position with Reduce-Only Market Order ===");

    // Check current positions to determine what needs to be closed
    match client.get_positions().await {
        Ok(positions) => {
            let target_position = positions
                .iter()
                .find(|pos| pos.instrument_name == instrument && pos.size != 0.0);

            if let Some(position) = target_position {
                info!(
                    "Found position to close: {} with size {}",
                    position.instrument_name, position.size
                );

                // Determine the opposite side and quantity to close
                let close_side = if position.size > 0.0 {
                    OrderSide::Sell // Close long position
                } else {
                    OrderSide::Buy // Close short position
                };
                let close_quantity = position.size.abs();

                let close_order_request = NewOrderRequest {
                    instrument_name: instrument.to_string(),
                    side: close_side,
                    order_type: OrderType::Market,
                    amount: close_quantity,
                    price: None,
                    time_in_force: TimeInForce::ImmediateOrCancel,
                    post_only: Some(false),
                    reduce_only: Some(true), // This ensures we only reduce the position
                    client_order_id: Some(format!(
                        "MARKET_CLOSE_{}",
                        chrono::Utc::now().timestamp_millis()
                    )),
                    label: Some(format!(
                        "MARKET_CLOSE_{}",
                        chrono::Utc::now().timestamp_millis()
                    )),
                    stop_price: None,
                    trigger: None,
                    advanced: None,
                    max_show: None,
                    reject_post_only: None,
                    valid_until: None,
                };

                info!(
                    "Sending market {:?} order to close position: {} {} (reduce-only)",
                    close_side, close_quantity, instrument
                );

                match client.send_order(close_order_request).await {
                    Ok(close_order_id) => {
                        info!(
                            "Position close order sent successfully with ID: {}",
                            close_order_id
                        );

                        // Wait for close order execution
                        info!("Waiting for position close execution...");
                        let mut close_confirmed = false;
                        let start_time = std::time::Instant::now();
                        let close_timeout = Duration::from_secs(30);

                        while start_time.elapsed() < close_timeout && !close_confirmed {
                            match timeout(Duration::from_millis(1000), client.receive_message())
                                .await
                            {
                                Ok(Ok(Some(message))) => {
                                    if let Some(msg_type) = message.get_field(35)
                                        && msg_type == "8"
                                    {
                                        // ExecutionReport
                                        if let Some(recv_cl_ord_id) = message.get_field(11)
                                            && recv_cl_ord_id == &close_order_id
                                            && let Some(ord_status) = message.get_field(39)
                                            && (ord_status == "2" || ord_status == "1")
                                        {
                                            // Filled or PartiallyFilled
                                            info!("✅ Position close order executed successfully!");
                                            close_confirmed = true;

                                            if let Some(last_px) = message.get_field(31) {
                                                info!("📊 Close price: {}", last_px);
                                            }
                                            if let Some(last_qty) = message.get_field(32) {
                                                info!("📊 Close quantity: {}", last_qty);
                                            }
                                        }
                                    }
                                }
                                Ok(Ok(None)) => {
                                    debug!("⏳ No message received, continuing to wait...");
                                }
                                Ok(Err(e)) => {
                                    debug!("❌ Error receiving message: {:?}", e);
                                }
                                Err(_) => {
                                    debug!("⏰ Timeout waiting for message, continuing...");
                                }
                            }
                        }

                        if !close_confirmed {
                            info!("ℹ️ Close confirmation timeout, but continuing");
                        }
                    }
                    Err(e) => {
                        error!("Failed to send position close order: {}", e);
                    }
                }
            } else {
                info!("No position found to close for instrument: {}", instrument);
            }
        }
        Err(e) => {
            error!("Failed to get positions for closing: {}", e);
        }
    }

    sleep(Duration::from_secs(2)).await;

    // Step 4: Final position check
    info!("=== Step 4: Final Position Verification ===");
    match client.get_positions().await {
        Ok(positions) => {
            let final_position = positions
                .iter()
                .find(|pos| pos.instrument_name == instrument);

            if let Some(position) = final_position {
                if position.size == 0.0 {
                    info!("✅ Position successfully closed! Final size: 0");
                } else {
                    info!("📊 Remaining position size: {}", position.size);
                }
            } else {
                info!("✅ No position found - successfully closed!");
            }
        }
        Err(e) => {
            error!("Failed to get final positions: {}", e);
        }
    }

    // Disconnect
    info!("Disconnecting...");
    client.disconnect().await?;

    info!("Position management example completed successfully!");
    Ok(())
}
