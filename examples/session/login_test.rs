//! Login test example for Deribit FIX API
//!
//! This example demonstrates how to perform a proper login to the Deribit FIX API
//! according to their documentation. It includes proper authentication with SHA256
//! hashing and nonce generation.

use deribit_base::prelude::*;
use deribit_fix::prelude::*;
use tokio::time::{Duration, sleep};
use tracing::{debug, error, info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    // Setup logging with debug level to see all messages
    unsafe {
        std::env::set_var("LOGLEVEL", "debug");
    }
    setup_logger();

    info!("=== Deribit FIX Login Test ===");

    // Create configuration from environment variables
    let config = DeribitFixConfig::new();

    // Validate configuration
    if let Err(e) = config.validate() {
        error!("Configuration validation failed: {}", e);
        error!("Make sure you have set DERIBIT_USERNAME and DERIBIT_PASSWORD in your .env file");
        return Err(e);
    }

    info!("Configuration loaded successfully:");
    info!("  Host: {}", config.host);
    info!("  Port: {}", config.port);
    info!("  SSL: {}", config.use_ssl);
    info!("  Test Mode: {}", config.test_mode);
    info!("  Username: {}", config.username);
    info!("  Heartbeat Interval: {}s", config.heartbeat_interval);
    info!("  Sender Comp ID: {}", config.sender_comp_id);
    info!("  Target Comp ID: {}", config.target_comp_id);

    if let Some(app_id) = &config.app_id {
        info!("  App ID: {}", app_id);
    }

    // Create the client
    let mut client = DeribitFixClient::new(&config).await?;

    info!("Attempting to connect to Deribit FIX server...");

    // Connect to the server - this will automatically perform login
    match client.connect().await {
        Ok(_) => {
            info!("✅ Successfully connected and logged in to Deribit FIX server!");

            // Wait for logon confirmation and read server messages
            info!("Waiting for logon confirmation...");
            let mut logged_on = false;
            let start_time = std::time::Instant::now();
            let timeout_duration = Duration::from_secs(10); // 10 second timeout

            while start_time.elapsed() < timeout_duration {
                // Try to receive messages from server with longer timeout
                match tokio::time::timeout(Duration::from_millis(500), client.receive_message())
                    .await
                {
                    Ok(Ok(Some(message))) => {
                        info!("📨 Received message from server: {:?}", message);
                        // Message received and processed
                    }
                    Ok(Ok(None)) => {
                        // No message available right now
                        debug!("No message available from server");
                    }
                    Ok(Err(e)) => {
                        error!("❌ Error receiving message: {}", e);
                        break;
                    }
                    Err(_) => {
                        // Timeout - no message received in 500ms
                        debug!("Timeout waiting for server message");
                    }
                }

                // Check if we're logged on
                if let Some(state) = client.get_session_state().await {
                    debug!("Current session state: {:?}", state);
                    if state == deribit_fix::session::SessionState::LoggedOn {
                        logged_on = true;
                        break;
                    }
                } else {
                    debug!("Unable to get session state (session locked)");
                }

                sleep(Duration::from_millis(200)).await;
            }

            if logged_on {
                info!("✅ Logon confirmed by server!");
            } else {
                error!("❌ Timed out waiting for logon confirmation from server.");
                client.disconnect().await?;
                return Err(DeribitFixError::Timeout(
                    "Logon confirmation timeout".to_string(),
                ));
            }

            // Test if we're still connected
            if client.is_connected() {
                info!("✅ Connection is still active");
            } else {
                warn!("⚠️ Connection appears to have been lost");
            }

            // Gracefully disconnect
            info!("Disconnecting...");
            client.disconnect().await?;
            info!("✅ Disconnected successfully");
        }
        Err(e) => {
            error!("❌ Failed to connect/login: {}", e);
            error!("Common issues:");
            error!("  - Check your username and password in .env file");
            error!("  - Verify network connectivity to {}", client.config.host);
            error!("  - Ensure you're using the correct environment (test vs production)");
            error!("  - Check if your account has FIX API access enabled");
            return Err(e);
        }
    }

    info!("Login test completed successfully!");
    Ok(())
}
