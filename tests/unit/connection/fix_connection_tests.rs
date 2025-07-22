// Unit tests for Connection

use deribit_fix::config::DeribitFixConfig;
use deribit_fix::error::DeribitFixError;

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
    }

    #[test]
    fn test_protocol_error() {
        let error = DeribitFixError::Protocol("Invalid FIX message".to_string());
        let display_str = format!("{error}");
        assert!(display_str.contains("Protocol error"));
        assert!(display_str.contains("Invalid FIX message"));
    }
}
