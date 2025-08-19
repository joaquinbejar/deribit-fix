// Unit tests for FixClient functionality

use deribit_base::prelude::{NewOrderRequest, OrderSide, OrderType, TimeInForce};
use deribit_fix::client::DeribitFixClient;
use deribit_fix::config::DeribitFixConfig;
use deribit_fix::error::DeribitFixError;
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = DeribitFixConfig::new()
            .with_credentials("test_user".to_string(), "test_pass".to_string());
        // Test basic config creation - this should always work
        assert!(!config.username.is_empty());
        assert!(!config.password.is_empty());
    }

    #[test]
    fn test_config_validation() {
        let config = DeribitFixConfig::new()
            .with_credentials("test_user".to_string(), "test_pass".to_string());
        let result = config.validate();
        // This should succeed for valid config
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_invalid_empty_username() {
        let config =
            DeribitFixConfig::new().with_credentials("".to_string(), "test_pass".to_string());
        let result = config.validate();
        // This should fail for empty username
        assert!(result.is_err());
    }

    #[test]
    fn test_config_invalid_empty_password() {
        let config =
            DeribitFixConfig::new().with_credentials("test_user".to_string(), "".to_string());
        let result = config.validate();
        // This should fail for empty password
        assert!(result.is_err());
    }

    #[test]
    fn test_error_display() {
        let error = DeribitFixError::Connection("Test connection error".to_string());
        let display_str = format!("{error}");
        assert!(display_str.contains("Connection error"));
    }

    /// Test DeribitFixClient creation with valid config
    #[tokio::test]
    async fn test_client_creation_valid_config() {
        let config = DeribitFixConfig::new()
            .with_credentials(
                "test_client_id".to_string(),
                "test_access_secret".to_string(),
            )
            .with_endpoint("test.deribit.com".to_string(), 9881);

        let result = DeribitFixClient::new(&config).await;
        assert!(
            result.is_ok(),
            "Client creation should succeed with valid config"
        );

        let client = result.unwrap();
        assert!(!client.config.username.is_empty());
        assert!(!client.config.password.is_empty());
        assert_eq!(client.config.host, "test.deribit.com");
        assert_eq!(client.config.port, 9881);
    }

    /// Test DeribitFixClient creation with invalid config
    #[tokio::test]
    async fn test_client_creation_invalid_config() {
        let config = DeribitFixConfig::new()
            .with_credentials("".to_string(), "test_access_secret".to_string()); // Empty username

        let result = DeribitFixClient::new(&config).await;
        assert!(
            result.is_err(),
            "Client creation should fail with invalid config"
        );

        match result {
            Err(DeribitFixError::Config(_)) => {} // Expected error type
            _ => panic!("Expected Configuration error"),
        }
    }

    /// Test client connection state when not connected
    #[tokio::test]
    async fn test_client_not_connected_state() {
        let config = DeribitFixConfig::new().with_credentials(
            "test_client_id".to_string(),
            "test_access_secret".to_string(),
        );

        let client = DeribitFixClient::new(&config).await.unwrap();

        // Client should not be connected initially
        assert!(
            !client.is_connected(),
            "Client should not be connected initially"
        );

        // Session state should be None when not connected
        assert!(
            client.get_session_state().await.is_none(),
            "Session state should be None when not connected"
        );
    }

    /// Test client operations when not connected (should return errors)
    #[tokio::test]
    async fn test_client_operations_not_connected() {
        let config = DeribitFixConfig::new().with_credentials(
            "test_client_id".to_string(),
            "test_access_secret".to_string(),
        );

        let client = DeribitFixClient::new(&config).await.unwrap();

        // Test send_order when not connected
        let order = NewOrderRequest {
            instrument_name: "BTC-PERPETUAL".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            amount: 100.0,
            price: Some(50000.0),
            time_in_force: TimeInForce::GoodTilCancelled,
            post_only: Some(false),
            reduce_only: Some(false),
            client_order_id: Some("test_order_1".to_string()),
            label: None,
            stop_price: None,
            trigger: None,
            advanced: None,
            max_show: None,
            reject_post_only: None,
            valid_until: None,
        };

        let result = client.send_order(order).await;
        assert!(result.is_err(), "send_order should fail when not connected");
        match result {
            Err(DeribitFixError::Session(msg)) => {
                assert!(
                    msg.contains("Not connected"),
                    "Error should mention not connected"
                );
            }
            _ => panic!("Expected Session error"),
        }

        // Test cancel_order when not connected
        let result = client.cancel_order("test_order_1".to_string()).await;
        assert!(
            result.is_err(),
            "cancel_order should fail when not connected"
        );
        match result {
            Err(DeribitFixError::Session(msg)) => {
                assert!(
                    msg.contains("Not connected"),
                    "Error should mention not connected"
                );
            }
            _ => panic!("Expected Session error"),
        }

        // Test subscribe_market_data when not connected
        let result = client
            .subscribe_market_data("BTC-PERPETUAL".to_string())
            .await;
        assert!(
            result.is_err(),
            "subscribe_market_data should fail when not connected"
        );
        match result {
            Err(DeribitFixError::Session(msg)) => {
                assert!(
                    msg.contains("Not connected"),
                    "Error should mention not connected"
                );
            }
            _ => panic!("Expected Session error"),
        }

        // Test get_positions when not connected
        let result = client.get_positions().await;
        assert!(
            result.is_err(),
            "get_positions should fail when not connected"
        );
        match result {
            Err(DeribitFixError::Session(msg)) => {
                assert!(
                    msg.contains("Not connected"),
                    "Error should mention not connected"
                );
            }
            _ => panic!("Expected Session error"),
        }

        // Test receive_message when not connected
        let result = client.receive_message().await;
        assert!(
            result.is_err(),
            "receive_message should fail when not connected"
        );
        match result {
            Err(DeribitFixError::Session(msg)) => {
                assert!(
                    msg.contains("Not connected"),
                    "Error should mention not connected"
                );
            }
            _ => panic!("Expected Session error"),
        }
    }

    /// Test configuration validation edge cases
    #[test]
    fn test_config_validation_edge_cases() {
        // Test with whitespace-only username (current validation only checks empty, so this will pass)
        let config =
            DeribitFixConfig::new().with_credentials("   ".to_string(), "test_pass".to_string());
        let result = config.validate();
        // Current implementation only checks is_empty(), not whitespace-only
        assert!(
            result.is_ok(),
            "Whitespace-only username currently passes validation"
        );

        // Test with whitespace-only password (current validation only checks empty, so this will pass)
        let config =
            DeribitFixConfig::new().with_credentials("test_user".to_string(), "   ".to_string());
        let result = config.validate();
        // Current implementation only checks is_empty(), not whitespace-only
        assert!(
            result.is_ok(),
            "Whitespace-only password currently passes validation"
        );

        // Test with invalid port (0)
        let config = DeribitFixConfig::new()
            .with_credentials("test_user".to_string(), "test_pass".to_string())
            .with_endpoint("test.deribit.com".to_string(), 0);
        let result = config.validate();
        assert!(result.is_err(), "Should fail with port 0");

        // Test with invalid port (too high)
        let config = DeribitFixConfig::new()
            .with_credentials("test_user".to_string(), "test_pass".to_string())
            .with_endpoint("test.deribit.com".to_string(), 65535);
        let result = config.validate();
        // This should actually pass since 65535 is a valid port, so let's change the test
        assert!(result.is_ok(), "Port 65535 should be valid");

        // Test with empty host
        let mut config = DeribitFixConfig::new()
            .with_credentials("test_user".to_string(), "test_pass".to_string());
        config.host = "".to_string();
        let result = config.validate();
        assert!(result.is_err(), "Should fail with empty host");
    }

    /// Test configuration builder pattern
    #[test]
    fn test_config_builder_pattern() {
        let config = DeribitFixConfig::new()
            .with_credentials("test_client".to_string(), "test_secret".to_string())
            .with_endpoint("custom.host.com".to_string(), 9999)
            .with_ssl(true)
            .with_heartbeat_interval(60)
            .with_connection_timeout(Duration::from_secs(10))
            .with_session_ids("TESTCLIENT".to_string(), "TESTSERVER".to_string());

        assert_eq!(config.username, "test_client");
        assert_eq!(config.password, "test_secret");
        assert_eq!(config.host, "custom.host.com");
        assert_eq!(config.port, 9999);
        assert!(config.use_ssl);
        assert_eq!(config.heartbeat_interval, 60);
        assert_eq!(config.connection_timeout, Duration::from_secs(10));
        assert_eq!(config.sender_comp_id, "TESTCLIENT");
        assert_eq!(config.target_comp_id, "TESTSERVER");
    }

    /// Test configuration URL generation
    #[test]
    fn test_config_connection_url() {
        // Test HTTP URL
        let config = DeribitFixConfig::new()
            .with_credentials("test".to_string(), "test".to_string())
            .with_endpoint("test.example.com".to_string(), 9881)
            .with_ssl(false);

        let url = config.connection_url();
        assert_eq!(url, "test.example.com:9881");

        // Test HTTPS URL
        let config = DeribitFixConfig::new()
            .with_credentials("test".to_string(), "test".to_string())
            .with_endpoint("secure.example.com".to_string(), 9883)
            .with_ssl(true);

        let url = config.connection_url();
        assert_eq!(url, "secure.example.com:9883");
    }

    /// Test error types and their display
    #[test]
    fn test_error_types_display() {
        let errors = vec![
            DeribitFixError::Connection("Connection failed".to_string()),
            DeribitFixError::Authentication("Auth failed".to_string()),
            DeribitFixError::Session("Session error".to_string()),
            DeribitFixError::Protocol("Protocol error".to_string()),
            DeribitFixError::MessageConstruction("Message error".to_string()),
            DeribitFixError::Config("Config error".to_string()),
            DeribitFixError::Timeout("Timeout error".to_string()),
        ];

        for error in errors {
            let display_str = format!("{error}");
            assert!(!display_str.is_empty(), "Error display should not be empty");
            assert!(
                display_str.contains("error"),
                "Error display should contain 'error'"
            );
        }
    }

    /// Test NewOrderRequest creation and validation
    #[test]
    fn test_new_order_request_creation() {
        let order = NewOrderRequest {
            instrument_name: "BTC-PERPETUAL".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            amount: 100.0,
            price: Some(50000.0),
            time_in_force: TimeInForce::GoodTilCancelled,
            post_only: Some(true),
            reduce_only: Some(false),
            client_order_id: Some("test_order_123".to_string()),
            label: Some("test_label".to_string()),
            stop_price: None,
            trigger: None,
            advanced: None,
            max_show: None,
            reject_post_only: None,
            valid_until: None,
        };

        assert_eq!(order.instrument_name, "BTC-PERPETUAL");
        assert!(matches!(order.side, OrderSide::Buy));
        assert!(matches!(order.order_type, OrderType::Limit));
        assert_eq!(order.amount, 100.0);
        assert_eq!(order.price, Some(50000.0));
        assert_eq!(order.post_only, Some(true));
        assert_eq!(order.reduce_only, Some(false));
        assert_eq!(order.client_order_id, Some("test_order_123".to_string()));
        assert_eq!(order.label, Some("test_label".to_string()));
    }

    /// Test different order types and sides
    #[test]
    fn test_order_types_and_sides() {
        // Test Market Buy order
        let market_buy = NewOrderRequest {
            instrument_name: "ETH-PERPETUAL".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Market,
            amount: 50.0,
            price: None, // Market orders don't have price
            time_in_force: TimeInForce::ImmediateOrCancel,
            post_only: Some(false),
            reduce_only: Some(false),
            client_order_id: Some("market_buy_1".to_string()),
            label: None,
            stop_price: None,
            trigger: None,
            advanced: None,
            max_show: None,
            reject_post_only: None,
            valid_until: None,
        };

        assert!(matches!(market_buy.order_type, OrderType::Market));
        assert!(matches!(market_buy.side, OrderSide::Buy));
        assert_eq!(market_buy.price, None);

        // Test Limit Sell order
        let limit_sell = NewOrderRequest {
            instrument_name: "BTC-PERPETUAL".to_string(),
            side: OrderSide::Sell,
            order_type: OrderType::Limit,
            amount: 25.0,
            price: Some(55000.0),
            time_in_force: TimeInForce::GoodTilCancelled,
            post_only: Some(true),
            reduce_only: Some(true),
            client_order_id: Some("limit_sell_1".to_string()),
            label: Some("hedge_position".to_string()),
            stop_price: None,
            trigger: None,
            advanced: None,
            max_show: None,
            reject_post_only: None,
            valid_until: None,
        };

        assert!(matches!(limit_sell.order_type, OrderType::Limit));
        assert!(matches!(limit_sell.side, OrderSide::Sell));
        assert_eq!(limit_sell.price, Some(55000.0));
        assert_eq!(limit_sell.reduce_only, Some(true));
    }

    /// Test error display formatting
    #[test]
    fn test_error_display_formatting() {
        let error = DeribitFixError::Connection("Test connection error".to_string());
        let display_str = format!("{error}");
        assert!(display_str.contains("Test connection error"));

        let error = DeribitFixError::Authentication("Test auth error".to_string());
        let display_str = format!("{error}");
        assert!(display_str.contains("Authentication error"));
        assert!(display_str.contains("Test auth error"));
    }

    #[test]
    fn test_error_debug() {
        let error = DeribitFixError::Session("Test session error".to_string());
        let debug_str = format!("{error:?}");
        assert!(debug_str.contains("Session"));
        assert!(debug_str.contains("Test session error"));
    }
}
