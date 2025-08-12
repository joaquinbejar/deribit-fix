/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 12/8/25
******************************************************************************/

//! User Request FIX Message Implementation

use crate::error::Result as DeribitFixResult;
use crate::message::builder::MessageBuilder;
use crate::model::types::MsgType;
use chrono::Utc;
use deribit_base::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

/// User request type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRequestType {
    /// Log on user
    LogOnUser,
    /// Log off user
    LogOffUser,
    /// Change password for user
    ChangePasswordForUser,
    /// Request individual user status
    RequestIndividualUserStatus,
}

impl From<UserRequestType> for i32 {
    fn from(request_type: UserRequestType) -> Self {
        match request_type {
            UserRequestType::LogOnUser => 1,
            UserRequestType::LogOffUser => 2,
            UserRequestType::ChangePasswordForUser => 3,
            UserRequestType::RequestIndividualUserStatus => 4,
        }
    }
}

impl TryFrom<i32> for UserRequestType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(UserRequestType::LogOnUser),
            2 => Ok(UserRequestType::LogOffUser),
            3 => Ok(UserRequestType::ChangePasswordForUser),
            4 => Ok(UserRequestType::RequestIndividualUserStatus),
            _ => Err(format!("Invalid UserRequestType: {}", value)),
        }
    }
}

/// User status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserStatus {
    /// Logged in
    LoggedIn,
    /// Not logged in
    NotLoggedIn,
    /// User not recognised
    UserNotRecognised,
    /// Password incorrect
    PasswordIncorrect,
    /// Password changed
    PasswordChanged,
    /// Other
    Other,
}

impl From<UserStatus> for i32 {
    fn from(status: UserStatus) -> Self {
        match status {
            UserStatus::LoggedIn => 1,
            UserStatus::NotLoggedIn => 2,
            UserStatus::UserNotRecognised => 3,
            UserStatus::PasswordIncorrect => 4,
            UserStatus::PasswordChanged => 5,
            UserStatus::Other => 99,
        }
    }
}

impl TryFrom<i32> for UserStatus {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(UserStatus::LoggedIn),
            2 => Ok(UserStatus::NotLoggedIn),
            3 => Ok(UserStatus::UserNotRecognised),
            4 => Ok(UserStatus::PasswordIncorrect),
            5 => Ok(UserStatus::PasswordChanged),
            99 => Ok(UserStatus::Other),
            _ => Err(format!("Invalid UserStatus: {}", value)),
        }
    }
}

/// User Request message (MsgType = 'BE')
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct UserRequest {
    /// User request ID
    pub user_request_id: String,
    /// User request type
    pub user_request_type: UserRequestType,
    /// Username
    pub username: String,
    /// Password
    pub password: Option<String>,
    /// New password
    pub new_password: Option<String>,
    /// Raw data length
    pub raw_data_length: Option<i32>,
    /// Raw data
    pub raw_data: Option<Vec<u8>>,
    /// User status
    pub user_status: Option<UserStatus>,
    /// User status text
    pub user_status_text: Option<String>,
    /// Custom label
    pub deribit_label: Option<String>,
}

impl UserRequest {
    /// Create a new user request
    pub fn new(
        user_request_id: String,
        user_request_type: UserRequestType,
        username: String,
    ) -> Self {
        Self {
            user_request_id,
            user_request_type,
            username,
            password: None,
            new_password: None,
            raw_data_length: None,
            raw_data: None,
            user_status: None,
            user_status_text: None,
            deribit_label: None,
        }
    }

    /// Create a log on user request
    pub fn log_on_user(
        user_request_id: String,
        username: String,
        password: String,
    ) -> Self {
        let mut request = Self::new(user_request_id, UserRequestType::LogOnUser, username);
        request.password = Some(password);
        request
    }

    /// Create a log off user request
    pub fn log_off_user(user_request_id: String, username: String) -> Self {
        Self::new(user_request_id, UserRequestType::LogOffUser, username)
    }

    /// Create a change password request
    pub fn change_password(
        user_request_id: String,
        username: String,
        old_password: String,
        new_password: String,
    ) -> Self {
        let mut request = Self::new(user_request_id, UserRequestType::ChangePasswordForUser, username);
        request.password = Some(old_password);
        request.new_password = Some(new_password);
        request
    }

    /// Create a status request
    pub fn status_request(user_request_id: String, username: String) -> Self {
        Self::new(user_request_id, UserRequestType::RequestIndividualUserStatus, username)
    }

    /// Set password
    pub fn with_password(mut self, password: String) -> Self {
        self.password = Some(password);
        self
    }

    /// Set new password
    pub fn with_new_password(mut self, new_password: String) -> Self {
        self.new_password = Some(new_password);
        self
    }

    /// Set raw data
    pub fn with_raw_data(mut self, raw_data: Vec<u8>) -> Self {
        self.raw_data_length = Some(raw_data.len() as i32);
        self.raw_data = Some(raw_data);
        self
    }

    /// Set user status
    pub fn with_user_status(mut self, user_status: UserStatus) -> Self {
        self.user_status = Some(user_status);
        self
    }

    /// Set user status text
    pub fn with_user_status_text(mut self, user_status_text: String) -> Self {
        self.user_status_text = Some(user_status_text);
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
            .msg_type(MsgType::UserRequest)
            .sender_comp_id(sender_comp_id.to_string())
            .target_comp_id(target_comp_id.to_string())
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now());

        // Required fields
        builder = builder
            .field(923, self.user_request_id.clone()) // UserRequestID
            .field(924, i32::from(self.user_request_type).to_string()) // UserRequestType
            .field(553, self.username.clone()); // Username

        // Optional fields
        if let Some(password) = &self.password {
            builder = builder.field(554, password.clone());
        }

        if let Some(new_password) = &self.new_password {
            builder = builder.field(925, new_password.clone());
        }

        if let Some(raw_data_length) = &self.raw_data_length {
            builder = builder.field(95, raw_data_length.to_string());
        }

        if let Some(raw_data) = &self.raw_data {
            // Convert raw data to base64 for FIX transmission
            let encoded_data = base64::encode(raw_data);
            builder = builder.field(96, encoded_data);
        }

        if let Some(user_status) = &self.user_status {
            builder = builder.field(926, i32::from(*user_status).to_string());
        }

        if let Some(user_status_text) = &self.user_status_text {
            builder = builder.field(927, user_status_text.clone());
        }

        if let Some(deribit_label) = &self.deribit_label {
            builder = builder.field(100010, deribit_label.clone());
        }

        Ok(builder.build()?.to_string())
    }
}

impl_json_display!(UserRequest);
impl_json_debug_pretty!(UserRequest);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_request_creation() {
        let request = UserRequest::new(
            "UR123".to_string(),
            UserRequestType::RequestIndividualUserStatus,
            "testuser".to_string(),
        );

        assert_eq!(request.user_request_id, "UR123");
        assert_eq!(request.user_request_type, UserRequestType::RequestIndividualUserStatus);
        assert_eq!(request.username, "testuser");
        assert!(request.password.is_none());
        assert!(request.new_password.is_none());
    }

    #[test]
    fn test_user_request_log_on() {
        let request = UserRequest::log_on_user(
            "UR456".to_string(),
            "user1".to_string(),
            "password123".to_string(),
        );

        assert_eq!(request.user_request_type, UserRequestType::LogOnUser);
        assert_eq!(request.username, "user1");
        assert_eq!(request.password, Some("password123".to_string()));
    }

    #[test]
    fn test_user_request_log_off() {
        let request = UserRequest::log_off_user(
            "UR789".to_string(),
            "user2".to_string(),
        );

        assert_eq!(request.user_request_type, UserRequestType::LogOffUser);
        assert_eq!(request.username, "user2");
        assert!(request.password.is_none());
    }

    #[test]
    fn test_user_request_change_password() {
        let request = UserRequest::change_password(
            "UR999".to_string(),
            "user3".to_string(),
            "oldpass".to_string(),
            "newpass".to_string(),
        );

        assert_eq!(request.user_request_type, UserRequestType::ChangePasswordForUser);
        assert_eq!(request.username, "user3");
        assert_eq!(request.password, Some("oldpass".to_string()));
        assert_eq!(request.new_password, Some("newpass".to_string()));
    }

    #[test]
    fn test_user_request_status_request() {
        let request = UserRequest::status_request(
            "UR111".to_string(),
            "user4".to_string(),
        );

        assert_eq!(request.user_request_type, UserRequestType::RequestIndividualUserStatus);
        assert_eq!(request.username, "user4");
    }

    #[test]
    fn test_user_request_with_options() {
        let raw_data = vec![1, 2, 3, 4, 5];
        let request = UserRequest::new(
            "UR222".to_string(),
            UserRequestType::LogOnUser,
            "user5".to_string(),
        )
        .with_password("mypass".to_string())
        .with_raw_data(raw_data.clone())
        .with_user_status(UserStatus::LoggedIn)
        .with_user_status_text("User logged in successfully".to_string())
        .with_label("test-user-request".to_string());

        assert_eq!(request.password, Some("mypass".to_string()));
        assert_eq!(request.raw_data, Some(raw_data));
        assert_eq!(request.raw_data_length, Some(5));
        assert_eq!(request.user_status, Some(UserStatus::LoggedIn));
        assert_eq!(request.user_status_text, Some("User logged in successfully".to_string()));
        assert_eq!(request.deribit_label, Some("test-user-request".to_string()));
    }

    #[test]
    fn test_user_request_to_fix_message() {
        let request = UserRequest::log_on_user(
            "UR123".to_string(),
            "testuser".to_string(),
            "secret".to_string(),
        )
        .with_label("test-label".to_string());

        let fix_message = request.to_fix_message("SENDER", "TARGET", 1).unwrap();

        // Check that the message contains required fields
        assert!(fix_message.contains("35=BE")); // MsgType
        assert!(fix_message.contains("923=UR123")); // UserRequestID
        assert!(fix_message.contains("924=1")); // UserRequestType=LogOnUser
        assert!(fix_message.contains("553=testuser")); // Username
        assert!(fix_message.contains("554=secret")); // Password
        assert!(fix_message.contains("100010=test-label")); // Custom label
    }

    #[test]
    fn test_user_request_type_conversions() {
        assert_eq!(i32::from(UserRequestType::LogOnUser), 1);
        assert_eq!(i32::from(UserRequestType::LogOffUser), 2);
        assert_eq!(i32::from(UserRequestType::ChangePasswordForUser), 3);
        assert_eq!(i32::from(UserRequestType::RequestIndividualUserStatus), 4);

        assert_eq!(UserRequestType::try_from(1).unwrap(), UserRequestType::LogOnUser);
        assert_eq!(UserRequestType::try_from(2).unwrap(), UserRequestType::LogOffUser);
        assert_eq!(UserRequestType::try_from(3).unwrap(), UserRequestType::ChangePasswordForUser);
        assert_eq!(UserRequestType::try_from(4).unwrap(), UserRequestType::RequestIndividualUserStatus);

        assert!(UserRequestType::try_from(99).is_err());
    }

    #[test]
    fn test_user_status_conversions() {
        assert_eq!(i32::from(UserStatus::LoggedIn), 1);
        assert_eq!(i32::from(UserStatus::NotLoggedIn), 2);
        assert_eq!(i32::from(UserStatus::UserNotRecognised), 3);
        assert_eq!(i32::from(UserStatus::PasswordIncorrect), 4);
        assert_eq!(i32::from(UserStatus::PasswordChanged), 5);
        assert_eq!(i32::from(UserStatus::Other), 99);

        assert_eq!(UserStatus::try_from(1).unwrap(), UserStatus::LoggedIn);
        assert_eq!(UserStatus::try_from(2).unwrap(), UserStatus::NotLoggedIn);
        assert_eq!(UserStatus::try_from(3).unwrap(), UserStatus::UserNotRecognised);
        assert_eq!(UserStatus::try_from(4).unwrap(), UserStatus::PasswordIncorrect);
        assert_eq!(UserStatus::try_from(5).unwrap(), UserStatus::PasswordChanged);
        assert_eq!(UserStatus::try_from(99).unwrap(), UserStatus::Other);

        assert!(UserStatus::try_from(50).is_err());
    }
}