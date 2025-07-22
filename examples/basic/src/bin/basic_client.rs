//! Basic example of using the Deribit FIX client

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
    // Initialize logging
    setup_logger();

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

    // Create the client
    let mut client = DeribitFixClient::new(config).await?;

    info!("Connecting to Deribit FIX server...");

    // Connect to the server
    client.connect().await?;

    info!("Connected successfully!");

    // Example: Subscribe to market data
    info!("Subscribing to BTC-PERPETUAL market data...");
    if let Err(e) = client
        .subscribe_market_data("BTC-PERPETUAL".to_string())
        .await
    {
        error!("Failed to subscribe to market data: {}", e);
    }

    // Example: Send a limit order
    info!("Sending a test limit order...");
    let order_request = NewOrderRequest::limit_buy("BTC-PERPETUAL".to_string(), 10.0, 50000.0)
        .with_label("test_order_123".to_string());

    match client.send_order(order_request).await {
        Ok(order_id) => {
            info!("Order sent successfully with ID: {}", order_id);

            // Wait a bit, then cancel the order
            sleep(Duration::from_secs(5)).await;

            info!("Cancelling order: {}", order_id);
            if let Err(e) = client.cancel_order(order_id).await {
                error!("Failed to cancel order: {}", e);
            }
        }
        Err(e) => {
            error!("Failed to send order: {}", e);
        }
    }

    // Example: Get positions
    info!("Requesting account positions...");
    match client.get_positions().await {
        Ok(positions) => {
            info!("Retrieved {} positions", positions.len());
            for position in positions {
                info!(
                    "Position: {} - Qty: {}, Avg Price: {}",
                    position.symbol, position.quantity, position.average_price
                );
            }
        }
        Err(e) => {
            error!("Failed to get positions: {}", e);
        }
    }

    // Keep the connection alive for a while to receive messages
    info!("Keeping connection alive for 30 seconds...");
    sleep(Duration::from_secs(30)).await;

    // Disconnect
    info!("Disconnecting...");
    client.disconnect().await?;

    info!("Example completed successfully!");
    Ok(())
}
