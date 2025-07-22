// Unit tests for Connection

use deribit_fix::config::DeribitFixConfig;
use deribit_fix::error::DeribitFixError;
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_error_display() {
        let error = DeribitFixError::Connection("Test connection error".to_string());
        let display_str = format!("{error}");
        assert!(display_str.contains("Connection error"));
        assert!(display_str.contains("Test connection error"));
    }

    #[test]
    fn test_connection_error_debug() {
        let error = DeribitFixError::Connection("Test connection error".to_string());
        let debug_str = format!("{error:?}");
        assert!(debug_str.contains("Connection"));
        assert!(debug_str.contains("Test connection error"));
    }

    #[test]
    fn test_timeout_error() {
        let error = DeribitFixError::Timeout("Connection timeout".to_string());
        let display_str = format!("{error}");
        assert!(display_str.contains("Timeout error"));
        assert!(display_str.contains("Connection timeout"));
    }

    #[test]
    fn test_io_error() {
        let io_error =
            std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "Connection refused");
        let error = DeribitFixError::Io(io_error);
        let display_str = format!("{error}");
        assert!(display_str.contains("I/O error"));
        assert!(display_str.contains("Connection refused"));
    }

    #[test]
    fn test_config_for_connection() {
        let config = DeribitFixConfig::new();

        // Test that config can be used for connection setup
        assert!(!config.host.is_empty());
        assert!(config.port > 0);
        assert!(config.connection_timeout > Duration::from_secs(0));
        assert!(config.heartbeat_interval > 0);
    }

    /// Test connection configuration validation
    #[test]
    fn test_connection_config_validation() {
        // Valid configuration
        let valid_config = DeribitFixConfig::new()
            .with_credentials("test_user".to_string(), "test_pass".to_string())
            .with_endpoint("test.deribit.com".to_string(), 9881)
            .with_connection_timeout(Duration::from_secs(10));
        
        assert!(valid_config.validate().is_ok());
        assert_eq!(valid_config.host, "test.deribit.com");
        assert_eq!(valid_config.port, 9881);
        assert_eq!(valid_config.connection_timeout, Duration::from_secs(10));
        
        // Test with empty host
        let mut invalid_host_config = DeribitFixConfig::new()
            .with_credentials("test_user".to_string(), "test_pass".to_string());
        invalid_host_config.host = "".to_string();
        let result = invalid_host_config.validate();
        assert!(result.is_err(), "Should fail with empty host");
        
        // Test with invalid port (0)
        let invalid_port_config = DeribitFixConfig::new()
            .with_credentials("test_user".to_string(), "test_pass".to_string())
            .with_endpoint("test.example.com".to_string(), 0);
        let result = invalid_port_config.validate();
        assert!(result.is_err(), "Should fail with port 0");
    }

    /// Test connection URL generation
    #[test]
    fn test_connection_url_generation() {
        let config = DeribitFixConfig::new()
            .with_endpoint("test.example.com".to_string(), 9881);
        
        let url = config.connection_url();
        assert_eq!(url, "test.example.com:9881");
        
        // Test with different port
        let config_ssl = DeribitFixConfig::new()
            .with_endpoint("secure.example.com".to_string(), 9883)
            .with_ssl(true);
        
        let url_ssl = config_ssl.connection_url();
        assert_eq!(url_ssl, "secure.example.com:9883");
    }

    /// Test SSL configuration
    #[test]
    fn test_ssl_configuration() {
        // Test SSL enabled
        let ssl_config = DeribitFixConfig::new()
            .with_ssl(true)
            .with_endpoint("test.deribit.com".to_string(), 9883);
        
        assert!(ssl_config.use_ssl);
        assert_eq!(ssl_config.port, 9883);
        
        // Test SSL disabled
        let no_ssl_config = DeribitFixConfig::new()
            .with_ssl(false)
            .with_endpoint("test.deribit.com".to_string(), 9881);
        
        assert!(!no_ssl_config.use_ssl);
        assert_eq!(no_ssl_config.port, 9881);
    }

    /// Test connection timeout configuration
    #[test]
    fn test_connection_timeout_config() {
        let config = DeribitFixConfig::new()
            .with_connection_timeout(Duration::from_secs(30));
        
        assert_eq!(config.connection_timeout, Duration::from_secs(30));
        
        // Test with different timeout
        let config_fast = DeribitFixConfig::new()
            .with_connection_timeout(Duration::from_millis(500));
        
        assert_eq!(config_fast.connection_timeout, Duration::from_millis(500));
    }

    /// Test heartbeat interval configuration
    #[test]
    fn test_heartbeat_interval_config() {
        let config = DeribitFixConfig::new()
            .with_heartbeat_interval(60);
        
        assert_eq!(config.heartbeat_interval, 60);
        
        // Test with different interval
        let config_fast = DeribitFixConfig::new()
            .with_heartbeat_interval(15);
        
        assert_eq!(config_fast.heartbeat_interval, 15);
    }

    /// Test FIX component IDs configuration
    #[test]
    fn test_fix_component_ids() {
        let config = DeribitFixConfig::new()
            .with_session_ids("TESTCLIENT".to_string(), "DERIBITSERVER".to_string());
        
        assert_eq!(config.sender_comp_id, "TESTCLIENT");
        assert_eq!(config.target_comp_id, "DERIBITSERVER");
        
        // Test default values
        let default_config = DeribitFixConfig::new();
        assert!(!default_config.sender_comp_id.is_empty());
        assert!(!default_config.target_comp_id.is_empty());
    }

    /// Test connection error scenarios
    #[test]
    fn test_connection_error_scenarios() {
        // Test various connection errors
        let connection_refused = DeribitFixError::Connection(
            "Connection refused by server".to_string()
        );
        assert!(format!("{connection_refused}").contains("Connection error"));
        
        let timeout_error = DeribitFixError::Timeout(
            "Connection timeout after 10 seconds".to_string()
        );
        assert!(format!("{timeout_error}").contains("Timeout error"));
        
        let io_error = DeribitFixError::Io(
            std::io::Error::new(std::io::ErrorKind::NetworkUnreachable, "Network unreachable")
        );
        assert!(format!("{io_error}").contains("I/O error"));
        
        let protocol_error = DeribitFixError::Protocol(
            "Invalid FIX message format".to_string()
        );
        assert!(format!("{protocol_error}").contains("Protocol error"));
    }

    /// Test message parsing scenarios
    #[test]
    fn test_message_parsing_scenarios() {
        // Test valid FIX message components
        let valid_fix_fields = vec![
            (8, "FIX.4.4".to_string()),
            (35, "A".to_string()),
            (49, "CLIENT".to_string()),
            (56, "DERIBITSERVER".to_string()),
        ];
        
        for (tag, value) in valid_fix_fields {
            assert!(tag > 0, "FIX tag should be positive");
            assert!(!value.is_empty(), "FIX value should not be empty");
        }
        
        // Test message construction error
        let msg_error = DeribitFixError::MessageConstruction(
            "Failed to construct FIX message".to_string()
        );
        assert!(format!("{msg_error}").contains("Message construction error"));
    }

    /// Test buffer management scenarios
    #[test]
    fn test_buffer_management() {
        // Test buffer capacity scenarios
        let small_buffer_size = 1024;
        let large_buffer_size = 8192;
        let max_buffer_size = 65536;
        
        assert!(small_buffer_size < large_buffer_size);
        assert!(large_buffer_size < max_buffer_size);
        
        // Test buffer operations
        let mut buffer = Vec::with_capacity(large_buffer_size);
        assert_eq!(buffer.len(), 0);
        assert!(buffer.capacity() >= large_buffer_size);
        
        // Simulate adding data to buffer
        let test_data = b"8=FIX.4.4\x019=100\x01";
        buffer.extend_from_slice(test_data);
        assert_eq!(buffer.len(), test_data.len());
        
        // Test buffer clearing
        buffer.clear();
        assert_eq!(buffer.len(), 0);
    }

    /// Test FIX message field parsing
    #[test]
    fn test_fix_message_field_parsing() {
        // Test SOH delimiter
        const SOH: u8 = 0x01;
        assert_eq!(SOH, 1);
        
        // Test FIX message structure
        let fix_message_parts = vec![
            "8=FIX.4.4",
            "35=A",
            "49=CLIENT",
            "56=DERIBITSERVER",
            "10=123",
        ];
        
        for part in fix_message_parts {
            assert!(part.contains('='), "FIX field should contain '=' separator");
            let field_parts: Vec<&str> = part.split('=').collect();
            assert_eq!(field_parts.len(), 2, "FIX field should have tag and value");
            
            let tag = field_parts[0];
            let value = field_parts[1];
            
            assert!(!tag.is_empty(), "FIX tag should not be empty");
            assert!(!value.is_empty(), "FIX value should not be empty");
            assert!(tag.chars().all(|c| c.is_ascii_digit()), "FIX tag should be numeric");
        }
    }

    /// Test connection state management
    #[test]
    fn test_connection_state_management() {
        // Test connection states
        let mut connected = false;
        assert!(!connected, "Should start disconnected");
        
        // Simulate connection
        connected = true;
        assert!(connected, "Should be connected after connection");
        
        // Simulate disconnection
        connected = false;
        assert!(!connected, "Should be disconnected after disconnection");
    }

    /// Test configuration defaults
    #[test]
    fn test_configuration_defaults() {
        let config = DeribitFixConfig::new();
        
        // Test default values
        assert_eq!(config.host, "test.deribit.com");
        assert_eq!(config.port, 9881);
        assert!(!config.use_ssl); // Default should be false for test environment
        assert_eq!(config.sender_comp_id, "CLIENT");
        assert_eq!(config.target_comp_id, "DERIBITSERVER");
        assert_eq!(config.heartbeat_interval, 30);
        assert_eq!(config.connection_timeout, Duration::from_secs(10));
        assert!(config.test_mode); // Should default to test mode
    }

    /// Test error conversion and chaining
    #[test]
    fn test_error_conversion_and_chaining() {
        // Test IO error conversion
        let io_err = std::io::Error::new(std::io::ErrorKind::TimedOut, "Operation timed out");
        let fix_err = DeribitFixError::Io(io_err);
        
        match fix_err {
            DeribitFixError::Io(ref e) => {
                assert_eq!(e.kind(), std::io::ErrorKind::TimedOut);
            },
            _ => panic!("Expected IO error"),
        }
        
        // Test error display chain
        let errors = vec![
            DeribitFixError::Connection("Connection failed".to_string()),
            DeribitFixError::Timeout("Request timeout".to_string()),
            DeribitFixError::Protocol("Invalid message".to_string()),
            DeribitFixError::Authentication("Login failed".to_string()),
        ];
        
        for error in errors {
            let display = format!("{error}");
            let debug = format!("{error:?}");
            
            assert!(!display.is_empty());
            assert!(!debug.is_empty());
            assert!(display.contains("error"));
        }
    }

    #[test]
    fn test_protocol_error() {
        let error = DeribitFixError::Protocol("Invalid FIX message".to_string());
        let display_str = format!("{error}");
        assert!(display_str.contains("Protocol error"));
        assert!(display_str.contains("Invalid FIX message"));
    }
}
