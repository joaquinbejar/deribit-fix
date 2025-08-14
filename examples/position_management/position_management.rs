//! Position Management Example
//!
//! This example demonstrates how to:
//! 1. Connect to the Deribit FIX server
//! 2. Request position information
//! 3. Display position details

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

    // Request current positions
    info!("=== Requesting Current Positions ===");

    match client.get_positions().await {
        Ok(positions) => {
            info!("Retrieved {} positions", positions.len());

            if positions.is_empty() {
                info!("No positions found");
            } else {
                for (index, position) in positions.iter().enumerate() {
                    info!(
                        "Position {}: {} - Qty: {}, Avg Price: {}, Unrealized PnL: {}, Realized PnL: {}",
                        index + 1,
                        position.symbol,
                        position.quantity,
                        position.average_price,
                        position.unrealized_pnl,
                        position.realized_pnl
                    );
                }
            }
        }
        Err(e) => {
            error!("Failed to get positions: {}", e);
        }
    }

    // Keep connection alive for a bit to receive any position updates
    info!("Monitoring for position updates for 30 seconds...");
    sleep(Duration::from_secs(30)).await;

    // Disconnect
    info!("Disconnecting...");
    client.disconnect().await?;

    info!("Position management example completed successfully!");
    Ok(())
}
