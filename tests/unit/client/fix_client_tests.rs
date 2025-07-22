// Unit tests for FixClient functionality

use deribit_fix::config::DeribitFixConfig;
use deribit_fix::error::DeribitFixError;

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
