// Unit tests for DeribitFixError

use deribit_fix::error::{DeribitFixError, Result};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_error() {
        let error = DeribitFixError::Connection("Connection failed".to_string());
        let display_str = format!("{error}");
        assert!(display_str.contains("Connection error"));
        assert!(display_str.contains("Connection failed"));
    }

    #[test]
    fn test_authentication_error() {
        let error = DeribitFixError::Authentication("Invalid credentials".to_string());
        let display_str = format!("{error}");
        assert!(display_str.contains("Authentication error"));
        assert!(display_str.contains("Invalid credentials"));
    }

    #[test]
    fn test_message_parsing_error() {
        let error = DeribitFixError::MessageParsing("Invalid FIX format".to_string());
        let display_str = format!("{error}");
        assert!(display_str.contains("Message parsing error"));
        assert!(display_str.contains("Invalid FIX format"));
    }

    #[test]
    fn test_session_error() {
        let error = DeribitFixError::Session("Session not established".to_string());
        let display_str = format!("{error}");
        assert!(display_str.contains("Session error"));
        assert!(display_str.contains("Session not established"));
    }

    #[test]
    fn test_timeout_error() {
        let error = DeribitFixError::Timeout("Operation timed out".to_string());
        let display_str = format!("{error}");
        assert!(display_str.contains("Timeout error"));
        assert!(display_str.contains("Operation timed out"));
    }

    #[test]
    fn test_config_error() {
        let error = DeribitFixError::Config("Invalid configuration".to_string());
        let display_str = format!("{error}");
        assert!(display_str.contains("Configuration error"));
        assert!(display_str.contains("Invalid configuration"));
    }

    #[test]
    fn test_protocol_error() {
        let error = DeribitFixError::Protocol("Protocol violation".to_string());
        let display_str = format!("{error}");
        assert!(display_str.contains("Protocol error"));
        assert!(display_str.contains("Protocol violation"));
    }

    #[test]
    fn test_generic_error() {
        let error = DeribitFixError::Generic("Something went wrong".to_string());
        let display_str = format!("{error}");
        assert!(display_str.contains("Error"));
        assert!(display_str.contains("Something went wrong"));
    }

    #[test]
    fn test_io_error_conversion() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let error = DeribitFixError::Io(io_error);
        let display_str = format!("{error}");
        assert!(display_str.contains("I/O error"));
        assert!(display_str.contains("File not found"));
    }

    #[test]
    fn test_result_type() {
        let success: Result<i32> = Ok(42);
        assert!(success.is_ok());
        assert_eq!(success.ok(), Some(42));

        let failure: Result<i32> = Err(DeribitFixError::Generic("Test error".to_string()));
        assert!(failure.is_err());
    }

    #[test]
    fn test_error_debug() {
        let error = DeribitFixError::MessageConstruction("Failed to build message".to_string());
        let debug_str = format!("{error:?}");
        assert!(debug_str.contains("MessageConstruction"));
        assert!(debug_str.contains("Failed to build message"));
    }
}
