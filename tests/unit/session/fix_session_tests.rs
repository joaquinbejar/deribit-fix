// Unit tests for Session functionality

use deribit_fix::config::DeribitFixConfig;
use deribit_fix::error::DeribitFixError;
use deribit_fix::session::SessionState;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_state_values() {
        // Test session state enum values
        assert_eq!(format!("{:?}", SessionState::Disconnected), "Disconnected");
        assert_eq!(format!("{:?}", SessionState::LogonSent), "LogonSent");
        assert_eq!(format!("{:?}", SessionState::LoggedOn), "LoggedOn");
        assert_eq!(format!("{:?}", SessionState::LogoutSent), "LogoutSent");
    }

    #[test]
    fn test_session_state_equality() {
        assert_eq!(SessionState::Disconnected, SessionState::Disconnected);
        assert_eq!(SessionState::LoggedOn, SessionState::LoggedOn);
        assert_ne!(SessionState::Disconnected, SessionState::LoggedOn);
    }

    #[test]
    fn test_session_state_clone() {
        let state = SessionState::LoggedOn;
        let cloned = state;
        assert_eq!(state, cloned);
    }

    #[test]
    fn test_session_state_copy() {
        let state = SessionState::LogonSent;
        let copied = state;
        assert_eq!(state, copied);
    }

    #[test]
    fn test_session_error() {
        let error = DeribitFixError::Session("Session not established".to_string());
        let display_str = format!("{error}");
        assert!(display_str.contains("Session error"));
        assert!(display_str.contains("Session not established"));
    }

    #[test]
    fn test_session_authentication_error() {
        let error = DeribitFixError::Authentication("Login failed".to_string());
        let display_str = format!("{error}");
        assert!(display_str.contains("Authentication error"));
        assert!(display_str.contains("Login failed"));
    }

    #[test]
    fn test_config_for_session() {
        let config = DeribitFixConfig::new()
            .with_credentials("test_user".to_string(), "test_pass".to_string());

        // Test that config can be used for session setup
        assert!(!config.username.is_empty());
        assert!(!config.password.is_empty());
        assert!(!config.host.is_empty());
        assert!(config.port > 0);
    }

    #[test]
    fn test_session_timeout_error() {
        let error = DeribitFixError::Timeout("Session timeout".to_string());
        let display_str = format!("{error}");
        assert!(display_str.contains("Timeout error"));
        assert!(display_str.contains("Session timeout"));
    }

    #[test]
    fn test_session_protocol_error() {
        let error = DeribitFixError::Protocol("Invalid session message".to_string());
        let display_str = format!("{error}");
        assert!(display_str.contains("Protocol error"));
        assert!(display_str.contains("Invalid session message"));
    }

    #[test]
    fn test_session_message_construction_error() {
        let error =
            DeribitFixError::MessageConstruction("Failed to build logon message".to_string());
        let display_str = format!("{error}");
        assert!(display_str.contains("Message construction error"));
        assert!(display_str.contains("Failed to build logon message"));
    }
}
