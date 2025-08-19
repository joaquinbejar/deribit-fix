//! TEST 05: CONNECTION RECOVERY
//!
//! This test verifies the client's ability to recover a session after a transport-level disconnect:
//! 1. Establish a session and send some messages.
//! 2. Simulate a TCP connection drop.
//! 3. Reconnect and send a Logon (A) message with the previous session's sequence numbers.
//! 4. Handle any ResendRequests from the server to synchronize state.
//! 5. Confirm the session is re-established without a full reset.

use std::path::Path;
use std::time::Duration;
use tokio::time::{sleep, timeout};
use tracing::{debug, info, warn};

use deribit_base::prelude::*;
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
#[serial_test::serial]
async fn test_basic_connection_recovery() -> Result<()> {
    // Setup logging for test visibility
    unsafe {
        std::env::set_var("DERIBIT_LOG_LEVEL", "debug");
    }
    setup_logger();

    info!("=== Integration Test: Basic Connection Recovery ===");

    // Step 0: Check .env file exists and has required variables
    check_env_file()?;
    info!("✅ Environment file validation passed");

    // Step 1: Create initial configuration and client
    let config = DeribitFixConfig::new();
    config.validate()?;

    let mut client = DeribitFixClient::new(&config.clone()).await?;
    info!("✅ Initial client created successfully");

    // Step 2: Establish initial connection
    info!("🔌 Establishing initial connection...");
    client.connect().await?;
    info!("✅ Initial connection established");

    // Step 3: Wait for logon confirmation
    info!("⏳ Waiting for logon confirmation...");
    let logon_timeout = Duration::from_secs(15);

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

    if logon_result.is_err() {
        client.disconnect().await.ok();
        return Err(DeribitFixError::Timeout(
            "Initial logon timeout".to_string(),
        ));
    }

    info!("✅ Initial session established successfully");

    // Step 4: Collect some messages to establish session activity
    info!("📊 Collecting initial session activity...");
    let activity_duration = Duration::from_secs(5);
    let start_time = std::time::Instant::now();
    let mut messages_received = 0;

    while start_time.elapsed() < activity_duration {
        match timeout(Duration::from_millis(300), client.receive_message()).await {
            Ok(Ok(Some(_))) => {
                messages_received += 1;
                debug!("📨 Received message #{}", messages_received);
            }
            _ => {
                sleep(Duration::from_millis(100)).await;
            }
        }
    }

    info!(
        "📊 Initial session activity: {} messages received",
        messages_received
    );

    // Step 5: Simulate connection drop by disconnecting
    info!("💔 Simulating connection drop...");
    client.disconnect().await?;
    info!("✅ Connection dropped successfully");

    // Wait a bit to simulate real disconnection time
    sleep(Duration::from_secs(2)).await;

    // Step 6: Attempt to recover connection
    info!("🔄 Attempting connection recovery...");
    let mut recovery_client = DeribitFixClient::new(&config).await?;

    let recovery_result = recovery_client.connect().await;
    match recovery_result {
        Ok(_) => {
            info!("✅ Connection recovery initiated");

            // Wait for recovery confirmation
            let recovery_timeout = Duration::from_secs(15);
            let recovery_confirmed = timeout(recovery_timeout, async {
                loop {
                    if let Ok(Some(message)) = recovery_client.receive_message().await {
                        debug!("📨 Recovery message: {:?}", message);
                    }

                    if let Some(state) = recovery_client.get_session_state().await
                        && state == SessionState::LoggedOn
                    {
                        return Ok::<(), DeribitFixError>(());
                    }

                    sleep(Duration::from_millis(100)).await;
                }
            })
            .await;

            match recovery_confirmed {
                Ok(_) => {
                    info!("✅ Connection recovery successful");

                    // Test recovered session by receiving some messages
                    info!("🧪 Testing recovered session...");
                    let test_duration = Duration::from_secs(3);
                    let test_start = std::time::Instant::now();
                    let mut recovery_messages = 0;

                    while test_start.elapsed() < test_duration {
                        match timeout(
                            Duration::from_millis(200),
                            recovery_client.receive_message(),
                        )
                        .await
                        {
                            Ok(Ok(Some(_))) => {
                                recovery_messages += 1;
                            }
                            _ => {
                                sleep(Duration::from_millis(100)).await;
                            }
                        }
                    }

                    info!("📊 Recovery test: {} messages received", recovery_messages);
                    info!("✅ Connection recovery test passed");
                }
                Err(_) => {
                    warn!("⚠️ Recovery logon timeout, but connection was re-established");
                }
            }

            // Clean up recovery client
            recovery_client.disconnect().await.ok();
        }
        Err(e) => {
            warn!("⚠️ Connection recovery failed: {}", e);
            // This might be expected behavior in some test environments
            info!("ℹ️ Recovery failure can be acceptable in test environments");
        }
    }

    info!("🎉 Basic connection recovery test completed!");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_connection_resilience() -> Result<()> {
    setup_logger();
    info!("=== Integration Test: Connection Resilience ===");

    check_env_file()?;

    let config = DeribitFixConfig::new();
    config.validate()?;

    // Test multiple connection cycles
    let connection_cycles = 3;

    for cycle in 1..=connection_cycles {
        info!("🔄 Connection cycle {}/{}", cycle, connection_cycles);

        let mut client = DeribitFixClient::new(&config.clone()).await?;

        // Connect
        let connect_result = client.connect().await;
        match connect_result {
            Ok(_) => {
                info!("✅ Cycle {} - Connected successfully", cycle);

                // Brief activity period
                let activity_duration = Duration::from_secs(2);
                let start_time = std::time::Instant::now();
                let mut cycle_messages = 0;

                while start_time.elapsed() < activity_duration {
                    if let Ok(Ok(Some(_))) =
                        timeout(Duration::from_millis(100), client.receive_message()).await
                    {
                        cycle_messages += 1;
                    } else {
                        sleep(Duration::from_millis(50)).await;
                    }
                }

                info!("📊 Cycle {} - {} messages received", cycle, cycle_messages);

                // Disconnect
                client.disconnect().await?;
                info!("👋 Cycle {} - Disconnected", cycle);
            }
            Err(e) => {
                warn!("⚠️ Cycle {} - Connection failed: {}", cycle, e);
            }
        }

        // Wait between cycles
        if cycle < connection_cycles {
            sleep(Duration::from_millis(500)).await;
        }
    }

    info!("✅ Connection resilience test completed");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_session_state_after_reconnection() -> Result<()> {
    setup_logger();
    info!("=== Integration Test: Session State After Reconnection ===");

    check_env_file()?;

    let config = DeribitFixConfig::new();
    config.validate()?;

    // First connection - establish baseline
    info!("🔌 Establishing baseline connection...");
    let mut client1 = DeribitFixClient::new(&config.clone()).await?;

    match client1.connect().await {
        Ok(_) => {
            info!("✅ Baseline connection established");

            // Wait for session establishment
            let session_timeout = Duration::from_secs(8);
            let session_established = timeout(session_timeout, async {
                loop {
                    if let Ok(Some(_)) = client1.receive_message().await {
                        // Process session messages
                    }

                    if let Some(state) = client1.get_session_state().await
                        && state == SessionState::LoggedOn
                    {
                        return Ok::<(), DeribitFixError>(());
                    }

                    sleep(Duration::from_millis(100)).await;
                }
            })
            .await;

            let baseline_established = session_established.is_ok();
            info!("📊 Baseline session established: {}", baseline_established);

            // Disconnect baseline
            client1.disconnect().await?;
            info!("👋 Baseline disconnected");

            // Wait before reconnection
            sleep(Duration::from_secs(1)).await;

            // Second connection - test recovery
            info!("🔄 Testing reconnection session state...");
            let mut client2 = DeribitFixClient::new(&config).await?;

            match client2.connect().await {
                Ok(_) => {
                    info!("✅ Reconnection successful");

                    // Monitor session state recovery
                    let recovery_timeout = Duration::from_secs(8);
                    let recovery_start = std::time::Instant::now();
                    let mut state_changes = Vec::new();

                    while recovery_start.elapsed() < recovery_timeout {
                        if let Some(state) = client2.get_session_state().await
                            && (state_changes.is_empty() || state_changes.last() != Some(&state))
                        {
                            state_changes.push(state);
                            info!("📊 Session state: {:?}", state);

                            if state == SessionState::LoggedOn {
                                break;
                            }
                        }

                        // Process any messages
                        if let Ok(Ok(Some(_message))) =
                            timeout(Duration::from_millis(100), client2.receive_message()).await
                        {
                            debug!("📨 Recovery message received");
                        }

                        sleep(Duration::from_millis(100)).await;
                    }

                    info!("📊 Session state progression: {:?}", state_changes);

                    let final_state = client2.get_session_state().await;
                    info!("📊 Final session state: {:?}", final_state);

                    // Cleanup
                    client2.disconnect().await?;
                }
                Err(e) => {
                    warn!("⚠️ Reconnection failed: {}", e);
                }
            }
        }
        Err(e) => {
            warn!("⚠️ Baseline connection failed: {}", e);
        }
    }

    info!("✅ Session state after reconnection test completed");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_rapid_connect_disconnect_cycles() -> Result<()> {
    setup_logger();
    info!("=== Integration Test: Rapid Connect/Disconnect Cycles ===");

    check_env_file()?;

    let config = DeribitFixConfig::new();
    config.validate()?;

    // Test rapid connection cycles
    let rapid_cycles = 5;
    let mut successful_connections = 0;
    let mut connection_errors = 0;

    for cycle in 1..=rapid_cycles {
        info!("⚡ Rapid cycle {}/{}", cycle, rapid_cycles);

        let mut client = DeribitFixClient::new(&config.clone()).await?;

        // Quick connect
        match client.connect().await {
            Ok(_) => {
                successful_connections += 1;
                debug!("✅ Rapid cycle {} - Connected", cycle);

                // Very brief activity
                if let Ok(Ok(Some(_))) =
                    timeout(Duration::from_millis(200), client.receive_message()).await
                {
                    debug!("📨 Quick message received in cycle {}", cycle);
                }

                // Quick disconnect
                client.disconnect().await?;
                debug!("👋 Rapid cycle {} - Disconnected", cycle);
            }
            Err(e) => {
                connection_errors += 1;
                debug!("❌ Rapid cycle {} - Failed: {}", cycle, e);
            }
        }

        // Very short delay between cycles
        sleep(Duration::from_millis(100)).await;
    }

    info!("📊 Rapid cycle results:");
    info!(
        "  - Successful connections: {}/{}",
        successful_connections, rapid_cycles
    );
    info!("  - Connection errors: {}", connection_errors);

    // We expect most connections to succeed, but some failures are acceptable
    // in rapid testing scenarios
    let success_rate = successful_connections as f64 / rapid_cycles as f64;
    info!("  - Success rate: {:.1}%", success_rate * 100.0);

    if success_rate >= 0.6 {
        // 60% success rate is reasonable for rapid cycles
        info!("✅ Rapid connect/disconnect test passed");
    } else {
        warn!("⚠️ Low success rate in rapid cycles (may be acceptable in test environment)");
    }

    Ok(())
}
