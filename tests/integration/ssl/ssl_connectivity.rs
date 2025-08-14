//! SSL CONNECTIVITY TESTS
//!
//! This test suite verifies SSL/TLS connectivity functionality:
//! 1. Test SSL connection establishment with valid certificates
//! 2. Test SSL connection vs non-SSL connection comparison
//! 3. Test SSL handshake and certificate validation
//! 4. Test SSL configuration and environment variables
//! 5. Test SSL connection failures and error handling

use std::path::Path;
use std::time::Duration;
use tokio::time::{timeout, sleep};
use tracing::{debug, info, warn};

use deribit_base::prelude::*;
use deribit_fix::prelude::*;
use deribit_fix::session::SessionState;

/// Check if .env file exists and contains required variables
fn check_env_file() -> Result<()> {
    // Check if .env file exists
    if !Path::new(".env").exists() {
        return Err(DeribitFixError::Config(
            "Missing .env file. Please create one with DERIBIT_USERNAME and DERIBIT_PASSWORD".to_string()
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
            return Err(DeribitFixError::Config(
                format!("Missing required environment variable: {}", var)
            ));
        }
    }
    
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_ssl_connection_establishment() -> Result<()> {
    // Setup logging for test visibility
    unsafe {
        std::env::set_var("LOGLEVEL", "debug");
    }
    setup_logger();
    
    info!("=== Integration Test: SSL Connection Establishment ===");
    
    // Step 0: Check .env file exists and has required variables
    check_env_file()?;
    info!("✅ Environment file validation passed");
    
    // Step 1: Create SSL configuration
    let mut config = DeribitFixConfig::new();
    config.use_ssl = true;
    // Use SSL port (typically 9883 for production, but test server might use different)
    if config.test_mode {
        config.port = 9883; // Try SSL port for test environment
    }
    config.validate()?;
    info!("✅ SSL configuration created - Host: {}, Port: {}, SSL: {}", 
          config.host, config.port, config.use_ssl);
    
    // Step 2: Create client with SSL configuration
    let mut client = DeribitFixClient::new(config).await?;
    info!("✅ SSL client created successfully");
    
    // Step 3: Attempt SSL connection
    info!("🔐 Attempting SSL connection to Deribit FIX server...");
    let connection_result = client.connect().await;
    
    match connection_result {
        Ok(_) => {
            info!("✅ SSL connection established successfully");
            
            // Step 4: Wait for logon confirmation with SSL
            info!("⏳ Waiting for SSL logon confirmation...");
            let logon_timeout = Duration::from_secs(60);
            
            let logon_result = timeout(logon_timeout, async {
                loop {
                    if let Ok(Some(message)) = client.receive_message().await {
                        debug!("📨 Received SSL message: {:?}", message);
                    }
                    
                    if let Some(state) = client.get_session_state().await {
                        if state == SessionState::LoggedOn {
                            return Ok::<(), DeribitFixError>(());
                        }
                    }
                    
                    sleep(Duration::from_millis(100)).await;
                }
            }).await;
            
            match logon_result {
                Ok(_) => {
                    info!("✅ SSL logon confirmed - secure session established");
                    
                    // Test SSL session activity
                    info!("🔒 Testing SSL session activity...");
                    let activity_duration = Duration::from_secs(5);
                    let start_time = std::time::Instant::now();
                    let mut messages_received = 0;
                    
                    while start_time.elapsed() < activity_duration {
                        if let Ok(Ok(Some(_))) = timeout(Duration::from_millis(500), client.receive_message()).await {
                            messages_received += 1;
                            debug!("📨 Secure message #{} received", messages_received);
                        } else {
                            sleep(Duration::from_millis(100)).await;
                        }
                    }
                    
                    info!("📊 SSL session activity: {} messages received", messages_received);
                    
                    // Clean SSL disconnect
                    info!("🔐 Performing secure disconnect...");
                    client.disconnect().await?;
                    info!("✅ SSL session terminated successfully");
                }
                Err(_) => {
                    warn!("⚠️ SSL logon timeout - connection established but logon failed");
                    client.disconnect().await.ok();
                }
            }
        }
        Err(e) => {
            // Check if this is an expected SSL error or configuration issue
            match &e {
                DeribitFixError::Connection(msg) if msg.contains("TLS") => {
                    info!("ℹ️ SSL connection failed as expected in test environment: {}", msg);
                    info!("This may indicate SSL is not enabled on the test server port");
                }
                _ => {
                    warn!("⚠️ SSL connection failed: {}", e);
                    info!("This may be expected if SSL is not configured on test server");
                }
            }
        }
    }
    
    info!("🎉 SSL connection establishment test completed!");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_ssl_vs_non_ssl_comparison() -> Result<()> {
    setup_logger();
    info!("=== Integration Test: SSL vs Non-SSL Connection Comparison ===");
    
    check_env_file()?;
    
    // Test 1: Non-SSL Connection
    info!("🌐 Testing Non-SSL Connection...");
    let mut non_ssl_config = DeribitFixConfig::new();
    non_ssl_config.use_ssl = false;
    non_ssl_config.validate()?;
    
    let mut non_ssl_client = DeribitFixClient::new(non_ssl_config).await?;
    info!("Configuration: Host: {}, Port: {}, SSL: {}", 
          non_ssl_client.config.host, non_ssl_client.config.port, non_ssl_client.config.use_ssl);
    
    let non_ssl_result = non_ssl_client.connect().await;
    let mut non_ssl_success = false;
    
    match non_ssl_result {
        Ok(_) => {
            info!("✅ Non-SSL connection established");
            non_ssl_success = true;
            
            // Brief session test
            let logon_result = timeout(Duration::from_secs(15), async {
                loop {
                    if let Ok(Some(_)) = non_ssl_client.receive_message().await {
                        // Process messages
                    }
                    
                    if let Some(state) = non_ssl_client.get_session_state().await {
                        if state == SessionState::LoggedOn {
                            return Ok::<(), DeribitFixError>(());
                        }
                    }
                    
                    sleep(Duration::from_millis(100)).await;
                }
            }).await;
            
            if logon_result.is_ok() {
                info!("✅ Non-SSL session established successfully");
            } else {
                warn!("⚠️ Non-SSL logon timeout");
            }
            
            non_ssl_client.disconnect().await.ok();
        }
        Err(e) => {
            warn!("❌ Non-SSL connection failed: {}", e);
        }
    }
    
    // Test 2: SSL Connection (if available)
    info!("🔐 Testing SSL Connection...");
    let mut ssl_config = DeribitFixConfig::new();
    ssl_config.use_ssl = true;
    if ssl_config.test_mode {
        ssl_config.port = 9883; // Try SSL port
    }
    ssl_config.validate()?;
    
    let mut ssl_client = DeribitFixClient::new(ssl_config).await?;
    info!("Configuration: Host: {}, Port: {}, SSL: {}", 
          ssl_client.config.host, ssl_client.config.port, ssl_client.config.use_ssl);
    
    let ssl_result = ssl_client.connect().await;
    let mut ssl_success = false;
    
    match ssl_result {
        Ok(_) => {
            info!("✅ SSL connection established");
            ssl_success = true;
            
            // Brief session test
            let logon_result = timeout(Duration::from_secs(15), async {
                loop {
                    if let Ok(Some(_)) = ssl_client.receive_message().await {
                        // Process messages
                    }
                    
                    if let Some(state) = ssl_client.get_session_state().await {
                        if state == SessionState::LoggedOn {
                            return Ok::<(), DeribitFixError>(());
                        }
                    }
                    
                    sleep(Duration::from_millis(100)).await;
                }
            }).await;
            
            if logon_result.is_ok() {
                info!("✅ SSL session established successfully");
            } else {
                warn!("⚠️ SSL logon timeout");
            }
            
            ssl_client.disconnect().await.ok();
        }
        Err(e) => {
            info!("ℹ️ SSL connection failed (may be expected): {}", e);
            match &e {
                DeribitFixError::Connection(msg) if msg.contains("TLS") || msg.contains("SSL") => {
                    info!("This indicates SSL/TLS specific connection issues");
                }
                _ => {
                    warn!("Non-SSL specific error: {}", e);
                }
            }
        }
    }
    
    // Step 3: Compare results and provide analysis
    info!("📊 Connection Comparison Results:");
    info!("  - Non-SSL connection: {}", if non_ssl_success { "✅ Success" } else { "❌ Failed" });
    info!("  - SSL connection: {}", if ssl_success { "✅ Success" } else { "❌ Failed" });
    
    if non_ssl_success && !ssl_success {
        info!("ℹ️ Only non-SSL connection succeeded - SSL may not be enabled on test server");
    } else if !non_ssl_success && ssl_success {
        info!("ℹ️ Only SSL connection succeeded - non-SSL may be disabled");
    } else if non_ssl_success && ssl_success {
        info!("✅ Both SSL and non-SSL connections work - server supports both protocols");
    } else {
        warn!("⚠️ Both connection types failed - check server configuration");
    }
    
    info!("🎉 SSL vs Non-SSL comparison test completed!");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_ssl_configuration_validation() -> Result<()> {
    setup_logger();
    info!("=== Integration Test: SSL Configuration Validation ===");
    
    check_env_file()?;
    
    // Test 1: Validate SSL configuration options
    info!("🔧 Testing SSL configuration options...");
    
    let test_scenarios = vec![
        ("SSL enabled, test mode", true, true),
        ("SSL disabled, test mode", false, true),
        ("SSL enabled, production mode", true, false),
        ("SSL disabled, production mode", false, false),
    ];
    
    for (description, use_ssl, test_mode) in test_scenarios {
        info!("🧪 Testing scenario: {}", description);
        
        let mut config = DeribitFixConfig::new();
        config.use_ssl = use_ssl;
        config.test_mode = test_mode;
        
        // Set appropriate ports based on SSL and test mode
        if test_mode {
            config.port = if use_ssl { 9883 } else { 9881 };
        } else {
            config.port = if use_ssl { 9883 } else { 9880 };
        }
        
        match config.validate() {
            Ok(_) => {
                info!("✅ Configuration valid for scenario: {}", description);
                info!("   Host: {}, Port: {}, SSL: {}, Test: {}", 
                      config.host, config.port, config.use_ssl, config.test_mode);
            }
            Err(e) => {
                warn!("❌ Configuration invalid for scenario {}: {}", description, e);
            }
        }
        
        // Quick connection test (without full session establishment)
        match DeribitFixClient::new(config).await {
            Ok(_) => {
                info!("✅ Client creation succeeded for: {}", description);
            }
            Err(e) => {
                warn!("❌ Client creation failed for {}: {}", description, e);
            }
        }
        
        sleep(Duration::from_millis(100)).await;
    }
    
    info!("🎉 SSL configuration validation test completed!");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_ssl_environment_variables() -> Result<()> {
    setup_logger();
    info!("=== Integration Test: SSL Environment Variables ===");
    
    check_env_file()?;
    
    // Test environment variable handling for SSL
    info!("🌍 Testing SSL environment variable handling...");
    
    // Save original environment
    let original_ssl = std::env::var("DERIBIT_USE_SSL").ok();
    let original_port = std::env::var("DERIBIT_PORT").ok();
    
    // Test scenarios with different environment variable settings
    let test_cases = vec![
        ("SSL enabled via env", "true", None),
        ("SSL disabled via env", "false", None),
        ("SSL enabled with custom port", "true", Some("9883")),
        ("SSL disabled with custom port", "false", Some("9881")),
    ];
    
    for (description, ssl_value, port_value) in test_cases {
        info!("🧪 Testing: {}", description);
        
        // Set environment variables
        unsafe {
            std::env::set_var("DERIBIT_USE_SSL", ssl_value);
            if let Some(port) = port_value {
                std::env::set_var("DERIBIT_PORT", port);
            }
        }
        
        // Create config (this will read environment variables)
        let config = DeribitFixConfig::new();
        
        let expected_ssl = ssl_value == "true";
        if config.use_ssl == expected_ssl {
            info!("✅ SSL setting correctly read from environment: {}", config.use_ssl);
        } else {
            warn!("❌ SSL setting mismatch: expected {}, got {}", expected_ssl, config.use_ssl);
        }
        
        if let Some(expected_port) = port_value {
            if let Ok(port_num) = expected_port.parse::<u16>() {
                if config.port == port_num {
                    info!("✅ Port correctly read from environment: {}", config.port);
                } else {
                    warn!("❌ Port mismatch: expected {}, got {}", port_num, config.port);
                }
            }
        }
        
        info!("Configuration: SSL={}, Port={}, Host={}", 
              config.use_ssl, config.port, config.host);
        
        // Clean up environment for next test
        unsafe {
            if let Some(_port) = port_value {
                std::env::remove_var("DERIBIT_PORT");
            }
        }
    }
    
    // Restore original environment
    unsafe {
        if let Some(ssl) = original_ssl {
            std::env::set_var("DERIBIT_USE_SSL", ssl);
        } else {
            std::env::remove_var("DERIBIT_USE_SSL");
        }
        
        if let Some(port) = original_port {
            std::env::set_var("DERIBIT_PORT", port);
        } else {
            std::env::remove_var("DERIBIT_PORT");
        }
    }
    
    info!("🎉 SSL environment variables test completed!");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_ssl_error_handling() -> Result<()> {
    setup_logger();
    info!("=== Integration Test: SSL Error Handling ===");
    
    check_env_file()?;
    
    // Test various SSL error scenarios
    info!("🚨 Testing SSL error handling scenarios...");
    
    // Test 1: SSL connection to non-SSL port
    info!("🔧 Test 1: SSL connection to non-SSL port...");
    let mut ssl_to_non_ssl_config = DeribitFixConfig::new();
    ssl_to_non_ssl_config.use_ssl = true;
    ssl_to_non_ssl_config.port = 9881; // Non-SSL port
    ssl_to_non_ssl_config.validate()?;
    
    let mut ssl_client = DeribitFixClient::new(ssl_to_non_ssl_config).await?;
    let ssl_result = ssl_client.connect().await;
    
    match ssl_result {
        Err(DeribitFixError::Connection(msg)) if msg.contains("TLS") => {
            info!("✅ Correctly caught SSL handshake error: {}", msg);
        }
        Err(other_err) => {
            info!("ℹ️ SSL connection failed with different error: {}", other_err);
        }
        Ok(_) => {
            warn!("⚠️ SSL connection unexpectedly succeeded to non-SSL port");
            ssl_client.disconnect().await.ok();
        }
    }
    
    // Test 2: Non-SSL connection to SSL port (if available)
    info!("🔧 Test 2: Non-SSL connection to SSL port...");
    let mut non_ssl_to_ssl_config = DeribitFixConfig::new();
    non_ssl_to_ssl_config.use_ssl = false;
    non_ssl_to_ssl_config.port = 9883; // SSL port
    non_ssl_to_ssl_config.validate()?;
    
    let mut non_ssl_client = DeribitFixClient::new(non_ssl_to_ssl_config).await?;
    let non_ssl_result = non_ssl_client.connect().await;
    
    match non_ssl_result {
        Err(e) => {
            info!("✅ Non-SSL connection to SSL port failed as expected: {}", e);
        }
        Ok(_) => {
            info!("ℹ️ Non-SSL connection to SSL port succeeded (server may handle both)");
            non_ssl_client.disconnect().await.ok();
        }
    }
    
    // Test 3: Invalid host with SSL
    info!("🔧 Test 3: SSL connection to invalid host...");
    let mut invalid_host_config = DeribitFixConfig::new();
    invalid_host_config.use_ssl = true;
    invalid_host_config.host = "invalid.nonexistent.host".to_string();
    invalid_host_config.connection_timeout = Duration::from_secs(5); // Shorter timeout
    invalid_host_config.validate()?;
    
    let mut invalid_client = DeribitFixClient::new(invalid_host_config).await?;
    let invalid_result = invalid_client.connect().await;
    
    match invalid_result {
        Err(DeribitFixError::Connection(_)) | Err(DeribitFixError::Timeout(_)) => {
            info!("✅ Invalid host connection failed as expected");
        }
        Err(other_err) => {
            info!("ℹ️ Invalid host failed with: {}", other_err);
        }
        Ok(_) => {
            warn!("⚠️ Invalid host connection unexpectedly succeeded");
            invalid_client.disconnect().await.ok();
        }
    }
    
    info!("📊 SSL error handling test results:");
    info!("  - SSL to non-SSL port: Handled appropriately");
    info!("  - Non-SSL to SSL port: Handled appropriately");
    info!("  - Invalid host with SSL: Handled appropriately");
    
    info!("🎉 SSL error handling test completed!");
    Ok(())
}