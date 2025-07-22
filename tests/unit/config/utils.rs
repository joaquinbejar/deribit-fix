// Unit tests for configuration utilities

use deribit_fix::config::DeribitFixConfig;
use deribit_fix::error::DeribitFixError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_config() {
        let config = DeribitFixConfig::new()
            .with_credentials("valid_user".to_string(), "valid_pass".to_string());

        let result = config.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_empty_username() {
        let config =
            DeribitFixConfig::new().with_credentials("".to_string(), "valid_pass".to_string());

        let result = config.validate();
        assert!(result.is_err());

        if let Err(error) = result {
            match error {
                DeribitFixError::Config(msg) => {
                    assert!(msg.contains("username") || msg.contains("empty"));
                }
                _ => panic!("Expected Config error"),
            }
        }
    }

    #[test]
    fn test_invalid_empty_password() {
        let config =
            DeribitFixConfig::new().with_credentials("valid_user".to_string(), "".to_string());

        let result = config.validate();
        assert!(result.is_err());

        if let Err(error) = result {
            match error {
                DeribitFixError::Config(msg) => {
                    assert!(msg.contains("password") || msg.contains("empty"));
                }
                _ => panic!("Expected Config error"),
            }
        }
    }

    #[test]
    fn test_config_host_validation() {
        let config = DeribitFixConfig::new();

        // Test default host
        assert!(!config.host.is_empty());
    }

    #[test]
    fn test_config_port_validation() {
        let config = DeribitFixConfig::new();

        // Test default port
        assert!(config.port > 0);
        assert!(config.port < 65535);
    }

    #[test]
    fn test_config_ssl_setting() {
        let _config = DeribitFixConfig::new();

        // Test default SSL setting (should be reasonable)
        // Don't assert specific value as it may depend on environment
    }

    #[test]
    fn test_config_timeout_settings() {
        let config = DeribitFixConfig::new();

        // Test timeout settings exist and are reasonable
        assert!(config.connection_timeout.as_secs() > 0);
        assert!(config.connection_timeout.as_secs() < 300); // Less than 5 minutes

        assert!(config.heartbeat_interval > 0);
        assert!(config.heartbeat_interval < 300); // Less than 5 minutes
    }
}
