//! Error Handling Example
//!
//! This example demonstrates how to:
//! 1. Handle different types of FIX protocol errors
//! 2. Manage connection failures and recovery
//! 3. Handle configuration validation errors
//! 4. Implement proper error logging strategies

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

    info!("=== Error Handling Example ===");

    // Example 1: Configuration validation errors
    info!("=== Example 1: Configuration Validation Errors ===");
    
    // Create an invalid configuration to demonstrate error handling
    let invalid_config = DeribitFixConfig::default()
        .with_heartbeat_interval(0) // Invalid: heartbeat must be > 0
        .with_connection_timeout(Duration::from_secs(0)); // Invalid: timeout must be > 0

    info!("Testing invalid configuration...");
    match invalid_config.validate() {
        Ok(_) => {
            error!("Expected validation error but validation passed");
        }
        Err(e) => {
            info!("Configuration validation failed as expected: {}", e);
            
            match e {
                DeribitFixError::Config(msg) => {
                    info!("Caught configuration error: {}", msg);
                }
                _ => {
                    info!("Caught different error type: {:?}", e);
                }
            }
        }
    }

    // Example 2: Connection failures
    info!("=== Example 2: Connection Failure Handling ===");
    
    // Create a configuration with an invalid host to demonstrate connection errors
    let invalid_host_config = DeribitFixConfig::default()
        .with_endpoint("invalid-host-that-does-not-exist.com".to_string(), 9999)
        .with_heartbeat_interval(30)
        .with_connection_timeout(Duration::from_secs(5)); // Short timeout

    match invalid_host_config.validate() {
        Ok(_) => {
            info!("Creating client with invalid host configuration...");
            match DeribitFixClient::new(invalid_host_config).await {
                Ok(mut client) => {
                    info!("Client created, attempting connection to invalid host...");
                    match client.connect().await {
                        Ok(_) => {
                            error!("Connection unexpectedly succeeded to invalid host");
                            client.disconnect().await.ok();
                        }
                        Err(e) => {
                            info!("Connection failed as expected: {}", e);
                            
                            match e {
                                DeribitFixError::Connection(msg) => {
                                    info!("Caught connection error: {}", msg);
                                }
                                DeribitFixError::Timeout(msg) => {
                                    info!("Caught timeout error: {}", msg);
                                }
                                _ => {
                                    info!("Caught different error type: {:?}", e);
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    info!("Client creation failed as expected: {}", e);
                }
            }
        }
        Err(e) => {
            error!("Configuration validation failed unexpectedly: {}", e);
        }
    }

    // Example 3: Valid connection with basic error scenarios
    info!("=== Example 3: Basic Error Handling ===");
    
    let config = DeribitFixConfig::default()
        .with_heartbeat_interval(30)
        .with_cancel_on_disconnect(true)
        .with_logging(true, "info".to_string());

    match config.validate() {
        Ok(_) => {
            info!("Creating client for basic error testing...");
            let mut client = DeribitFixClient::new(config).await?;

            info!("Connecting to server...");
            match client.connect().await {
                Ok(_) => {
                    info!("Connection established successfully");

                    // Wait for logon
                    info!("Waiting for logon...");
                    let logon_result = timeout(Duration::from_secs(30), async {
                        loop {
                            if let Ok(Some(_message)) = client.receive_message().await {
                                // Process logon messages
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
                        Ok(_) => {
                            info!("Logon successful, testing basic error scenarios...");
                            
                            // Error Scenario 1: Invalid order parameters
                            info!("--- Error Scenario 1: Invalid Order ---");
                            let invalid_order = NewOrderRequest::limit_buy(
                                "INVALID_SYMBOL_THAT_DOES_NOT_EXIST".to_string(),
                                -100.0, // Invalid negative quantity
                                0.0,    // Invalid zero price
                            );

                            match client.send_order(invalid_order).await {
                                Ok(order_id) => {
                                    info!("Order sent but may be rejected: {}", order_id);
                                    sleep(Duration::from_secs(2)).await;
                                    info!("Order processing completed");
                                }
                                Err(e) => {
                                    info!("Order rejected at client level as expected: {}", e);
                                }
                            }

                            sleep(Duration::from_secs(2)).await;

                            // Error Scenario 2: Cancel non-existent order
                            info!("--- Error Scenario 2: Cancel Non-Existent Order ---");
                            let fake_order_id = "fake_order_id_12345".to_string();
                            
                            match client.cancel_order(fake_order_id.clone()).await {
                                Ok(_) => {
                                    info!("Cancel request sent for non-existent order: {}", fake_order_id);
                                    sleep(Duration::from_secs(2)).await;
                                    info!("Cancel processing completed");
                                }
                                Err(e) => {
                                    info!("Cancel request rejected at client level: {}", e);
                                }
                            }

                            sleep(Duration::from_secs(2)).await;

                            // Error Scenario 3: Test invalid market data subscription
                            info!("--- Error Scenario 3: Invalid Market Data Subscription ---");
                            match client.subscribe_market_data("INVALID_INSTRUMENT_XYZ".to_string()).await {
                                Ok(_) => {
                                    info!("Market data subscription sent (may be rejected)");
                                    sleep(Duration::from_secs(2)).await;
                                    info!("Market data subscription processing completed");
                                }
                                Err(e) => {
                                    info!("Market data subscription rejected at client level: {}", e);
                                }
                            }
                        }
                        Err(_) => {
                            error!("Logon timeout - cannot test message-level errors");
                        }
                    }

                    // Clean disconnect
                    info!("Disconnecting after error testing...");
                    client.disconnect().await?;
                }
                Err(e) => {
                    error!("Connection failed: {}", e);
                    
                    // Demonstrate error classification
                    match e {
                        DeribitFixError::Connection(msg) => {
                            info!("This is a connection error: {}", msg);
                            info!("Recovery strategy: Check network, retry connection");
                        }
                        DeribitFixError::Timeout(msg) => {
                            info!("This is a timeout error: {}", msg);
                            info!("Recovery strategy: Increase timeout, check connection");
                        }
                        DeribitFixError::Authentication(msg) => {
                            info!("This is an authentication error: {}", msg);
                            info!("Recovery strategy: Check credentials, refresh tokens");
                        }
                        DeribitFixError::Config(msg) => {
                            info!("This is a configuration error: {}", msg);
                            info!("Recovery strategy: Fix configuration, validate settings");
                        }
                        _ => {
                            info!("This is a different error type: {:?}", e);
                            info!("Recovery strategy: Analyze error type and implement appropriate handling");
                        }
                    }
                }
            }
        }
        Err(e) => {
            error!("Configuration validation failed: {}", e);
        }
    }

    info!("=== Error Handling Best Practices Summary ===");
    info!("1. Always validate configuration before creating clients");
    info!("2. Handle connection errors gracefully with appropriate retries");
    info!("3. Monitor session state and handle disconnections");
    info!("4. Process message-level rejections and errors");
    info!("5. Use appropriate timeouts for operations");
    info!("6. Implement proper logging for error diagnosis");
    info!("7. Plan recovery strategies for different error types");

    info!("Error handling example completed successfully!");
    Ok(())
}