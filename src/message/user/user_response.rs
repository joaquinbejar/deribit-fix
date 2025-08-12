/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 12/8/25
******************************************************************************/

//! User Response FIX Message Implementation

use crate::error::Result as DeribitFixResult;
use crate::message::builder::MessageBuilder;
use crate::model::types::MsgType;
use chrono::Utc;
use base64::{engine::general_purpose, Engine as _};
use deribit_base::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

// Re-export UserStatus from user_request module
pub use super::user_request::UserStatus;

/// User Response message (MsgType = 'BF')
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct UserResponse {
    /// User request ID
    pub user_request_id: String,
    /// Username
    pub username: String,
    /// User status
    pub user_status: UserStatus,
    /// User status text
    pub user_status_text: Option<String>,
    /// Raw data length
    pub raw_data_length: Option<i32>,
    /// Raw data
    pub raw_data: Option<Vec<u8>>,
    /// Custom label
    pub deribit_label: Option<String>,
}

impl UserResponse {
    /// Create a new user response
    pub fn new(user_request_id: String, username: String, user_status: UserStatus) -> Self {
        Self {
            user_request_id,
            username,
            user_status,
            user_status_text: None,
            raw_data_length: None,
            raw_data: None,
            deribit_label: None,
        }
    }

    /// Create a successful login response
    pub fn logged_in(user_request_id: String, username: String) -> Self {
        let mut response = Self::new(user_request_id, username, UserStatus::LoggedIn);
        response.user_status_text = Some("User logged in successfully".to_string());
        response
    }

    /// Create a successful logout response
    pub fn logged_out(user_request_id: String, username: String) -> Self {
        let mut response = Self::new(user_request_id, username, UserStatus::NotLoggedIn);
        response.user_status_text = Some("User logged out successfully".to_string());
        response
    }

    /// Create a password changed response
    pub fn password_changed(user_request_id: String, username: String) -> Self {
        let mut response = Self::new(user_request_id, username, UserStatus::PasswordChanged);
        response.user_status_text = Some("Password changed successfully".to_string());
        response
    }

    /// Create an error response for unrecognized user
    pub fn user_not_recognised(user_request_id: String, username: String) -> Self {
        let mut response = Self::new(user_request_id, username, UserStatus::UserNotRecognised);
        response.user_status_text = Some("User not recognised".to_string());
        response
    }

    /// Create an error response for incorrect password
    pub fn password_incorrect(user_request_id: String, username: String) -> Self {
        let mut response = Self::new(user_request_id, username, UserStatus::PasswordIncorrect);
        response.user_status_text = Some("Password incorrect".to_string());
        response
    }

    /// Create a generic error response
    pub fn error(user_request_id: String, username: String, error_text: String) -> Self {
        let mut response = Self::new(user_request_id, username, UserStatus::Other);
        response.user_status_text = Some(error_text);
        response
    }

    /// Set user status text
    pub fn with_user_status_text(mut self, user_status_text: String) -> Self {
        self.user_status_text = Some(user_status_text);
        self
    }

    /// Set raw data
    pub fn with_raw_data(mut self, raw_data: Vec<u8>) -> Self {
        self.raw_data_length = Some(raw_data.len() as i32);
        self.raw_data = Some(raw_data);
        self
    }

    /// Set custom label
    pub fn with_label(mut self, label: String) -> Self {
        self.deribit_label = Some(label);
        self
    }

    /// Convert to FIX message
    pub fn to_fix_message(
        &self,
        sender_comp_id: &str,
        target_comp_id: &str,
        msg_seq_num: u32,
    ) -> DeribitFixResult<String> {
        let mut builder = MessageBuilder::new()
            .msg_type(MsgType::UserResponse)
            .sender_comp_id(sender_comp_id.to_string())
            .target_comp_id(target_comp_id.to_string())
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now());

        // Required fields
        builder = builder
            .field(923, self.user_request_id.clone()) // UserRequestID
            .field(553, self.username.clone()) // Username
            .field(926, i32::from(self.user_status).to_string()); // UserStatus

        // Optional fields
        if let Some(user_status_text) = &self.user_status_text {
            builder = builder.field(927, user_status_text.clone());
        }

        if let Some(raw_data_length) = &self.raw_data_length {
            builder = builder.field(95, raw_data_length.to_string());
        }

        if let Some(raw_data) = &self.raw_data {
            // Convert raw data to base64 for FIX transmission
            let encoded_data = general_purpose::STANDARD.encode(raw_data);
            builder = builder.field(96, encoded_data);
        }

        if let Some(deribit_label) = &self.deribit_label {
            builder = builder.field(100010, deribit_label.clone());
        }

        Ok(builder.build()?.to_string())
    }
}

impl_json_display!(UserResponse);
impl_json_debug_pretty!(UserResponse);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_response_creation() {
        let response = UserResponse::new(
            "UR123".to_string(),
            "testuser".to_string(),
            UserStatus::LoggedIn,
        );

        assert_eq!(response.user_request_id, "UR123");
        assert_eq!(response.username, "testuser");
        assert_eq!(response.user_status, UserStatus::LoggedIn);
        assert!(response.user_status_text.is_none());
        assert!(response.raw_data.is_none());
    }

    #[test]
    fn test_user_response_logged_in() {
        let response = UserResponse::logged_in(
            "UR456".to_string(),
            "user1".to_string(),
        );

        assert_eq!(response.user_status, UserStatus::LoggedIn);
        assert_eq!(response.username, "user1");
        assert_eq!(response.user_status_text, Some("User logged in successfully".to_string()));
    }

    #[test]
    fn test_user_response_logged_out() {
        let response = UserResponse::logged_out(
            "UR789".to_string(),
            "user2".to_string(),
        );

        assert_eq!(response.user_status, UserStatus::NotLoggedIn);
        assert_eq!(response.username, "user2");
        assert_eq!(response.user_status_text, Some("User logged out successfully".to_string()));
    }

    #[test]
    fn test_user_response_password_changed() {
        let response = UserResponse::password_changed(
            "UR999".to_string(),
            "user3".to_string(),
        );

        assert_eq!(response.user_status, UserStatus::PasswordChanged);
        assert_eq!(response.username, "user3");
        assert_eq!(response.user_status_text, Some("Password changed successfully".to_string()));
    }

    #[test]
    fn test_user_response_user_not_recognised() {
        let response = UserResponse::user_not_recognised(
            "UR111".to_string(),
            "unknown_user".to_string(),
        );

        assert_eq!(response.user_status, UserStatus::UserNotRecognised);
        assert_eq!(response.username, "unknown_user");
        assert_eq!(response.user_status_text, Some("User not recognised".to_string()));
    }

    #[test]
    fn test_user_response_password_incorrect() {
        let response = UserResponse::password_incorrect(
            "UR222".to_string(),
            "user4".to_string(),
        );

        assert_eq!(response.user_status, UserStatus::PasswordIncorrect);
        assert_eq!(response.username, "user4");
        assert_eq!(response.user_status_text, Some("Password incorrect".to_string()));
    }

    #[test]
    fn test_user_response_error() {
        let response = UserResponse::error(
            "UR333".to_string(),
            "user5".to_string(),
            "System temporarily unavailable".to_string(),
        );

        assert_eq!(response.user_status, UserStatus::Other);
        assert_eq!(response.username, "user5");
        assert_eq!(response.user_status_text, Some("System temporarily unavailable".to_string()));
    }

    #[test]
    fn test_user_response_with_options() {
        let raw_data = vec![10, 20, 30, 40];
        let response = UserResponse::new(
            "UR444".to_string(),
            "user6".to_string(),
            UserStatus::LoggedIn,
        )
        .with_user_status_text("Custom login message".to_string())
        .with_raw_data(raw_data.clone())
        .with_label("test-user-response".to_string());

        assert_eq!(response.user_status_text, Some("Custom login message".to_string()));
        assert_eq!(response.raw_data, Some(raw_data));
        assert_eq!(response.raw_data_length, Some(4));
        assert_eq!(response.deribit_label, Some("test-user-response".to_string()));
    }

    #[test]
    fn test_user_response_to_fix_message() {
        let response = UserResponse::logged_in(
            "UR123".to_string(),
            "testuser".to_string(),
        )
        .with_label("test-label".to_string());

        let fix_message = response.to_fix_message("SENDER", "TARGET", 1).unwrap();

        // Check that the message contains required fields
        assert!(fix_message.contains("35=BF")); // MsgType
        assert!(fix_message.contains("923=UR123")); // UserRequestID
        assert!(fix_message.contains("553=testuser")); // Username
        assert!(fix_message.contains("926=1")); // UserStatus=LoggedIn
        assert!(fix_message.contains("927=User logged in successfully")); // UserStatusText
        assert!(fix_message.contains("100010=test-label")); // Custom label
    }

    #[test]
    fn test_user_response_minimal_fix_message() {
        let response = UserResponse::new(
            "UR456".to_string(),
            "user".to_string(),
            UserStatus::NotLoggedIn,
        );

        let fix_message = response.to_fix_message("SENDER", "TARGET", 2).unwrap();

        // Check required fields only
        assert!(fix_message.contains("35=BF")); // MsgType
        assert!(fix_message.contains("923=UR456")); // UserRequestID
        assert!(fix_message.contains("553=user")); // Username
        assert!(fix_message.contains("926=2")); // UserStatus=NotLoggedIn

        // Check optional fields are not present when not set
        // Use SOH character (\x01) to be more precise and avoid false matches
        assert!(!fix_message.contains("\x01927=")); // UserStatusText field not set
        assert!(!fix_message.contains("\x0195=")); // RawDataLength field not set
        assert!(!fix_message.contains("\x0196=")); // RawData field not set
    }

    #[test]
    fn test_user_response_with_raw_data() {
        let raw_data = vec![0xFF, 0xFE, 0xFD];
        let response = UserResponse::new(
            "UR789".to_string(),
            "datauser".to_string(),
            UserStatus::LoggedIn,
        )
        .with_raw_data(raw_data.clone());

        let fix_message = response.to_fix_message("SENDER", "TARGET", 3).unwrap();

        // Check that raw data fields are present
        assert!(fix_message.contains("95=3")); // RawDataLength
        assert!(fix_message.contains("96=")); // RawData field should be present (base64 encoded)
    }
}