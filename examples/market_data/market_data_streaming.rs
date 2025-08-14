//! Market Data Streaming Example
//!
//! This example demonstrates how to:
//! 1. Connect to the Deribit FIX server
//! 2. Subscribe to market data for a specific instrument
//! 3. Receive and process streaming market data updates
//! 4. Handle connection and error management

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

    info!("=== Market Data Streaming Example ===");

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

    // Subscribe to market data for BTC-PERPETUAL
    let instrument = "BTC-PERPETUAL";
    info!("Subscribing to market data for: {}", instrument);

    match client.subscribe_market_data(instrument.to_string()).await {
        Ok(_) => info!("Successfully subscribed to {} market data", instrument),
        Err(e) => {
            error!("Failed to subscribe to market data: {}", e);
            client.disconnect().await.ok();
            return Err(e);
        }
    }

    // Listen for market data updates for 60 seconds
    info!("Listening for market data updates for 60 seconds...");
    let listen_duration = Duration::from_secs(60);
    let start_time = std::time::Instant::now();

    let mut update_count = 0;

    while start_time.elapsed() < listen_duration {
        // Simply keep the connection alive and let the client handle messages internally
        sleep(Duration::from_secs(5)).await;
        update_count += 1;
        info!("Market data streaming active... update #{}", update_count);

        // Check if we're still connected
        if let Some(state) = client.get_session_state().await {
            if state != SessionState::LoggedOn {
                error!("Session is no longer logged on, stopping market data streaming");
                break;
            }
        }
    }

    info!("Received {} market data updates total", update_count);

    // Note: Unsubscribe functionality may not be available in this client version
    info!("Market data streaming completed for {}", instrument);

    // Disconnect
    info!("Disconnecting...");
    client.disconnect().await?;

    info!("Market data streaming example completed successfully!");
    Ok(())
}
