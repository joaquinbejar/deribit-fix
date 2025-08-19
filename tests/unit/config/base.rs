// Unit tests for DeribitFixConfig

use deribit_fix::config::DeribitFixConfig;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new() {
        let config = DeribitFixConfig::new();

        // Test default values (may be set from environment)
        // Username and password can be empty or set from env vars
        // Just test that they are valid strings
        assert!(!config.host.is_empty());
        assert!(config.port > 0);
    }

    #[test]
    fn test_config_with_credentials() {
        let config = DeribitFixConfig::new()
            .with_credentials("test_user".to_string(), "test_pass".to_string());

        assert_eq!(config.username, "test_user");
        assert_eq!(config.password, "test_pass");

        let result = config.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_invalid_empty_username() {
        let config =
            DeribitFixConfig::new().with_credentials("".to_string(), "test_pass".to_string());

        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_invalid_empty_password() {
        let config =
            DeribitFixConfig::new().with_credentials("test_user".to_string(), "".to_string());

        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_config_clone() {
        let config = DeribitFixConfig::new()
            .with_credentials("test_user".to_string(), "test_pass".to_string());

        let cloned = config.clone();
        assert_eq!(config.username, cloned.username);
        assert_eq!(config.password, cloned.password);
        assert_eq!(config.host, cloned.host);
        assert_eq!(config.port, cloned.port);
    }

    #[test]
    fn test_config_debug() {
        let config = DeribitFixConfig::new();

        let debug_str = format!("{config:?}");
        // The debug output uses JSON format, so check for JSON structure
        assert!(debug_str.contains("{") && debug_str.contains("}"));
        assert!(debug_str.contains("username") || debug_str.contains("host"));
    }
}
