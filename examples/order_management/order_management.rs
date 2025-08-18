//! Order Management Example
//!
//! This example demonstrates how to:
//! 1. Connect to the Deribit FIX server
//! 2. Send different types of orders (limit, market)
//! 3. Cancel orders
//! 4. Handle order-related errors

use deribit_base::prelude::TimeInForce; // Explicit import to resolve ambiguity
use deribit_base::prelude::*;
use deribit_fix::prelude::*;
use deribit_fix::session::SessionState;
use std::time::Duration;
use tokio::time::{sleep, timeout};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    // Setup logging
    unsafe {
        std::env::set_var("LOGLEVEL", "info");
    }
    setup_logger();

    info!("=== Order Management Example ===");

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

    // Example 1: Send a limit buy order
    info!("=== Example 1: Limit Buy Order ===");
    let instrument = "BTC-PERPETUAL";
    let quantity = 10.0;
    let price = 50000.0;

    let limit_order = NewOrderRequest::limit_buy(instrument.to_string(), quantity, price)
        .with_label("example_limit_buy".to_string())
        .with_time_in_force(TimeInForce::GoodTilCancelled);

    info!(
        "Sending limit buy order: {} {} at ${}",
        quantity, instrument, price
    );

    match client.send_order(limit_order).await {
        Ok(order_id) => {
            info!("Limit buy order sent successfully with ID: {}", order_id);

            // Wait for order processing
            sleep(Duration::from_secs(2)).await;
            info!("Order processing completed");

            // Cancel the order after 5 seconds
            sleep(Duration::from_secs(5)).await;
            info!("Cancelling limit buy order...");

            match client.cancel_order(order_id.clone()).await {
                Ok(_) => info!("Order {} cancelled successfully", order_id),
                Err(e) => error!("Failed to cancel order {}: {}", order_id, e),
            }
        }
        Err(e) => {
            error!("Failed to send limit buy order: {}", e);
        }
    }

    sleep(Duration::from_secs(2)).await;

    // Example 2: Send a limit sell order
    info!("=== Example 2: Limit Sell Order ===");
    let sell_price = 55000.0;

    let sell_order = NewOrderRequest::limit_sell(instrument.to_string(), quantity, sell_price)
        .with_label("example_limit_sell".to_string())
        .with_time_in_force(TimeInForce::GoodTilCancelled);

    info!(
        "Sending limit sell order: {} {} at ${}",
        quantity, instrument, sell_price
    );

    match client.send_order(sell_order).await {
        Ok(order_id) => {
            info!("Limit sell order sent successfully with ID: {}", order_id);

            // Wait for order processing
            sleep(Duration::from_secs(2)).await;
            info!("Order processing completed");

            // Cancel the order
            sleep(Duration::from_secs(2)).await;
            info!("Cancelling limit sell order...");

            match client.cancel_order(order_id.clone()).await {
                Ok(_) => info!("Order {} cancelled successfully", order_id),
                Err(e) => error!("Failed to cancel order {}: {}", order_id, e),
            }
        }
        Err(e) => {
            error!("Failed to send limit sell order: {}", e);
        }
    }

    sleep(Duration::from_secs(2)).await;

    // Example 3: Demonstrate order rejection
    info!("=== Example 3: Order Rejection (Invalid Parameters) ===");

    let invalid_order = NewOrderRequest::limit_buy(
        "INVALID-INSTRUMENT".to_string(),
        -10.0, // Invalid negative quantity
        0.0,   // Invalid zero price
    );

    info!("Sending invalid order to demonstrate rejection handling...");

    match client.send_order(invalid_order).await {
        Ok(order_id) => {
            info!("Order sent (may be rejected): {}", order_id);
            sleep(Duration::from_secs(2)).await;
            info!("Order processing completed");
        }
        Err(e) => {
            info!("Order rejected as expected: {}", e);
        }
    }

    // Keep connection alive for a bit
    info!("Keeping connection alive for 10 seconds...");
    sleep(Duration::from_secs(10)).await;

    // Disconnect
    info!("Disconnecting...");
    client.disconnect().await?;

    info!("Order management example completed successfully!");
    Ok(())
}
