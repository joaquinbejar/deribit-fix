//! Configuration module for the Deribit FIX client

use crate::error::{DeribitFixError, Result};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Configuration for the Deribit FIX client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Deribit username
    pub username: String,
    /// Deribit password
    pub password: String,
    /// FIX server host (default: www.deribit.com for production, test.deribit.com for test)
    pub host: String,
    /// FIX server port (default: 9881 for test, 9880 for production)
    pub port: u16,
    /// Whether to use SSL connection (default: false for raw TCP)
    pub use_ssl: bool,
    /// Whether to use test environment
    pub test_mode: bool,
    /// Heartbeat interval in seconds (default: 30)
    pub heartbeat_interval: u32,
    /// Connection timeout in seconds (default: 10)
    pub connection_timeout: Duration,
    /// Reconnection attempts (default: 3)
    pub reconnect_attempts: u32,
    /// Reconnection delay in seconds (default: 5)
    pub reconnect_delay: Duration,
    /// Enable logging (default: true)
    pub enable_logging: bool,
    /// Log level filter
    pub log_level: String,
    /// Sender company ID for FIX messages
    pub sender_comp_id: String,
    /// Target company ID for FIX messages (DERIBITSERVER)
    pub target_comp_id: String,
    /// Cancel orders on disconnect (default: false)
    pub cancel_on_disconnect: bool,
    /// Application ID for registered applications
    pub app_id: Option<String>,
    /// Application secret for registered applications
    pub app_secret: Option<String>,
}

impl Config {
    /// Create a new configuration with username and password
    pub fn new(username: String, password: String) -> Self {
        Self {
            username,
            password,
            host: "test.deribit.com".to_string(),
            port: 9881, // Test environment by default
            use_ssl: false,
            test_mode: true,
            heartbeat_interval: 30,
            connection_timeout: Duration::from_secs(10),
            reconnect_attempts: 3,
            reconnect_delay: Duration::from_secs(5),
            enable_logging: true,
            log_level: "info".to_string(),
            sender_comp_id: "CLIENT".to_string(),
            target_comp_id: "DERIBITSERVER".to_string(),
            cancel_on_disconnect: false,
            app_id: None,
            app_secret: None,
        }
    }

    /// Create configuration for production environment
    pub fn production(username: String, password: String) -> Self {
        let mut config = Self::new(username, password);
        config.test_mode = false;
        config.host = "www.deribit.com".to_string();
        config.port = 9880; // Production port
        config
    }

    /// Create configuration for production environment with SSL
    pub fn production_ssl(username: String, password: String) -> Self {
        let mut config = Self::production(username, password);
        config.use_ssl = true;
        config.port = 9883; // Production SSL port
        config
    }

    /// Create configuration for test environment with SSL
    pub fn test_ssl(username: String, password: String) -> Self {
        let mut config = Self::new(username, password);
        config.use_ssl = true;
        config.port = 9883; // Test SSL port
        config
    }

    /// Set custom host and port
    pub fn with_endpoint(mut self, host: String, port: u16) -> Self {
        self.host = host;
        self.port = port;
        self
    }

    /// Enable or disable SSL
    pub fn with_ssl(mut self, use_ssl: bool) -> Self {
        self.use_ssl = use_ssl;
        self
    }

    /// Set heartbeat interval
    pub fn with_heartbeat_interval(mut self, interval: u32) -> Self {
        self.heartbeat_interval = interval;
        self
    }

    /// Set connection timeout
    pub fn with_connection_timeout(mut self, timeout: Duration) -> Self {
        self.connection_timeout = timeout;
        self
    }

    /// Set reconnection parameters
    pub fn with_reconnection(mut self, attempts: u32, delay: Duration) -> Self {
        self.reconnect_attempts = attempts;
        self.reconnect_delay = delay;
        self
    }

    /// Set logging configuration
    pub fn with_logging(mut self, enabled: bool, level: String) -> Self {
        self.enable_logging = enabled;
        self.log_level = level;
        self
    }

    /// Set FIX session identifiers
    pub fn with_session_ids(mut self, sender_comp_id: String, target_comp_id: String) -> Self {
        self.sender_comp_id = sender_comp_id;
        self.target_comp_id = target_comp_id;
        self
    }

    /// Set cancel on disconnect behavior
    pub fn with_cancel_on_disconnect(mut self, cancel_on_disconnect: bool) -> Self {
        self.cancel_on_disconnect = cancel_on_disconnect;
        self
    }

    /// Set application credentials for registered applications
    pub fn with_app_credentials(mut self, app_id: String, app_secret: String) -> Self {
        self.app_id = Some(app_id);
        self.app_secret = Some(app_secret);
        self
    }

    /// Get the connection URL
    pub fn connection_url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        if self.username.is_empty() {
            return Err(DeribitFixError::Config("Username cannot be empty".to_string()));
        }

        if self.password.is_empty() {
            return Err(DeribitFixError::Config("Password cannot be empty".to_string()));
        }

        if self.host.is_empty() {
            return Err(DeribitFixError::Config("Host cannot be empty".to_string()));
        }

        if self.port == 0 {
            return Err(DeribitFixError::Config("Port must be greater than 0".to_string()));
        }

        if self.heartbeat_interval == 0 {
            return Err(DeribitFixError::Config("Heartbeat interval must be greater than 0".to_string()));
        }

        if self.sender_comp_id.is_empty() {
            return Err(DeribitFixError::Config("Sender company ID cannot be empty".to_string()));
        }

        if self.target_comp_id.is_empty() {
            return Err(DeribitFixError::Config("Target company ID cannot be empty".to_string()));
        }

        // Validate app credentials if provided
        if self.app_id.is_some() && self.app_secret.is_none() {
            return Err(DeribitFixError::Config("Application secret is required when app ID is provided".to_string()));
        }

        if self.app_secret.is_some() && self.app_id.is_none() {
            return Err(DeribitFixError::Config("Application ID is required when app secret is provided".to_string()));
        }

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new("".to_string(), "".to_string())
    }
}
