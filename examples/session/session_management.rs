//! Session Management Example
//!
//! This example demonstrates how to:
//! 1. Connect to the Deribit FIX server
//! 2. Handle logon process and session state changes
//! 3. Monitor session lifecycle
//! 4. Handle session disconnection

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
        std::env::set_var("DERIBIT_LOG_LEVEL", "info");
    }
    setup_logger();

    info!("=== Session Management Example ===");

    // Example 1: Basic session lifecycle
    info!("=== Example 1: Basic Session Lifecycle ===");

    let config = DeribitFixConfig::default()
        .with_heartbeat_interval(10) // Shorter heartbeat for demonstration
        .with_cancel_on_disconnect(true)
        .with_logging(true, "info".to_string());

    if let Err(e) = config.validate() {
        error!("Configuration validation failed: {}", e);
        return Err(e);
    }

    info!("Creating Deribit FIX client...");
    let mut client = DeribitFixClient::new(&config).await?;

    // Monitor session state during connection
    info!(
        "Initial session state: {:?}",
        client.get_session_state().await
    );

    info!("Connecting to Deribit FIX server...");
    client.connect().await?;
    info!("Connection established");

    info!(
        "Session state after connection: {:?}",
        client.get_session_state().await
    );

    // Wait for logon and monitor state changes
    info!("Waiting for logon confirmation...");
    let logon_timeout = Duration::from_secs(30);

    let logon_result = timeout(logon_timeout, async {
        loop {
            if let Ok(Some(_message)) = client.receive_message().await {
                // Process messages but don't handle complex patterns
                info!("Received message during logon");
            }

            let current_state = client.get_session_state().await;
            if let Some(state) = current_state {
                if state == SessionState::LoggedOn {
                    info!("Session state changed to LoggedOn!");
                    return Ok::<(), DeribitFixError>(());
                }
            }

            sleep(Duration::from_millis(100)).await;
        }
    })
    .await;

    match logon_result {
        Ok(_) => info!("Logon confirmed successfully"),
        Err(_) => {
            error!("Logon confirmation timeout");
            client.disconnect().await.ok();
            return Err(DeribitFixError::Timeout("Logon timeout".to_string()));
        }
    }

    // Example 2: Session monitoring
    info!("=== Example 2: Session State Monitoring ===");
    info!("Monitoring session for 30 seconds...");

    let monitor_duration = Duration::from_secs(30);
    let start_time = std::time::Instant::now();

    while start_time.elapsed() < monitor_duration {
        // Check session state periodically
        if let Some(state) = client.get_session_state().await {
            if state != SessionState::LoggedOn {
                error!("Session is no longer logged on: {:?}", state);
                break;
            }
        }

        // Keep the connection alive
        sleep(Duration::from_secs(5)).await;
        info!(
            "Session monitoring active... State: {:?}",
            client.get_session_state().await
        );
    }

    info!("Session monitoring completed");

    // Example 3: Disconnect and reconnect
    info!("=== Example 3: Disconnect and Reconnect ===");

    info!(
        "Current session state before disconnect: {:?}",
        client.get_session_state().await
    );

    info!("Disconnecting from server...");
    client.disconnect().await?;

    info!(
        "Session state after disconnect: {:?}",
        client.get_session_state().await
    );

    sleep(Duration::from_secs(3)).await;

    info!("Attempting to reconnect...");

    match client.connect().await {
        Ok(_) => {
            info!("Reconnection successful");

            // Wait for logon again
            info!("Waiting for logon after reconnection...");
            let reconnect_logon_result = timeout(Duration::from_secs(30), async {
                loop {
                    if let Ok(Some(_message)) = client.receive_message().await {
                        // Process reconnection messages
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

            match reconnect_logon_result {
                Ok(_) => {
                    info!("Reconnection and logon successful!");
                    info!(
                        "Final session state: {:?}",
                        client.get_session_state().await
                    );
                }
                Err(_) => {
                    error!("Logon timeout after reconnection");
                }
            }
        }
        Err(e) => {
            error!("Reconnection failed: {}", e);
        }
    }

    // Final cleanup
    info!("Performing final disconnect...");
    client.disconnect().await?;

    info!(
        "Final session state: {:?}",
        client.get_session_state().await
    );

    info!("Session management example completed successfully!");
    Ok(())
}
