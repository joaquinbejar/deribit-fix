/******************************************************************************
    Author: Joaquín Béjar García
    Email: jb@taunais.com 
    Date: 21/7/25
 ******************************************************************************/

use crate::error::{DeribitFixError, Result};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, time::Duration};
use tracing::{debug};
use crate::config::utils::{get_env_optional, get_env_or_default};
use crate::constants::{DEFAULT_CONNECTION_TIMEOUT_SECS, DEFAULT_HEARTBEAT_INTERVAL, DEFAULT_LOG_LEVEL, DEFAULT_PROD_HOST, DEFAULT_PROD_PORT, DEFAULT_RECONNECT_ATTEMPTS, DEFAULT_RECONNECT_DELAY_SECS, DEFAULT_SENDER_COMP_ID, DEFAULT_SSL_PORT, DEFAULT_TARGET_COMP_ID, DEFAULT_TEST_HOST, DEFAULT_TEST_PORT};

/// Configuration for the Deribit FIX client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeribitFixConfig {
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

impl DeribitFixConfig {
    /// Create a new configuration from environment variables with optional username and password
    pub fn new() -> Self {
        // Load .env file if available
        match dotenv() {
            Ok(_) => debug!("Successfully loaded .env file"),
            Err(e) => debug!("Failed to load .env file: {}", e),
        }

        let test_mode = get_env_or_default("DERIBIT_TEST_MODE", true);
        let use_ssl = get_env_or_default("DERIBIT_USE_SSL", false);

        let (default_host, default_port) = if test_mode {
            if use_ssl {
                (DEFAULT_TEST_HOST, DEFAULT_SSL_PORT)
            } else {
                (DEFAULT_TEST_HOST, DEFAULT_TEST_PORT)
            }
        } else {
            if use_ssl {
                (DEFAULT_PROD_HOST, DEFAULT_SSL_PORT)
            } else {
                (DEFAULT_PROD_HOST, DEFAULT_PROD_PORT)
            }
        };

        Self {
            username: get_env_or_default("DERIBIT_USERNAME", String::new()),
            password: get_env_or_default("DERIBIT_PASSWORD", String::new()),
            host: get_env_or_default("DERIBIT_HOST", default_host.to_string()),
            port: get_env_or_default("DERIBIT_PORT", default_port),
            use_ssl,
            test_mode,
            heartbeat_interval: get_env_or_default("DERIBIT_HEARTBEAT_INTERVAL", DEFAULT_HEARTBEAT_INTERVAL),
            connection_timeout: Duration::from_secs(get_env_or_default("DERIBIT_CONNECTION_TIMEOUT", DEFAULT_CONNECTION_TIMEOUT_SECS)),
            reconnect_attempts: get_env_or_default("DERIBIT_RECONNECT_ATTEMPTS", DEFAULT_RECONNECT_ATTEMPTS),
            reconnect_delay: Duration::from_secs(get_env_or_default("DERIBIT_RECONNECT_DELAY", DEFAULT_RECONNECT_DELAY_SECS)),
            enable_logging: get_env_or_default("DERIBIT_ENABLE_LOGGING", true),
            log_level: get_env_or_default("DERIBIT_LOG_LEVEL", DEFAULT_LOG_LEVEL.to_string()),
            sender_comp_id: get_env_or_default("DERIBIT_SENDER_COMP_ID", DEFAULT_SENDER_COMP_ID.to_string()),
            target_comp_id: get_env_or_default("DERIBIT_TARGET_COMP_ID", DEFAULT_TARGET_COMP_ID.to_string()),
            cancel_on_disconnect: get_env_or_default("DERIBIT_CANCEL_ON_DISCONNECT", false),
            app_id: get_env_optional("DERIBIT_APP_ID"),
            app_secret: get_env_optional("DERIBIT_APP_SECRET"),
        }
    }

    /// Set credentials
    pub fn with_credentials(mut self, username: String, password: String) -> Self {
        self.username = username;
        self.password = password;
        self
    }

    /// Create configuration for production environment
    pub fn production() -> Self {
        let mut config = Self::new();
        config.test_mode = false;
        config.host = get_env_or_default("DERIBIT_HOST", DEFAULT_PROD_HOST.to_string());
        config.port = if config.use_ssl {
            get_env_or_default("DERIBIT_PORT", DEFAULT_SSL_PORT)
        } else {
            get_env_or_default("DERIBIT_PORT", DEFAULT_PROD_PORT)
        };
        config
    }

    /// Create configuration for production environment with credentials
    pub fn production_with_credentials(username: String, password: String) -> Self {
        let mut config = Self::production();
        config.username = username;
        config.password = password;
        config
    }

    /// Create configuration for production environment with SSL
    pub fn production_ssl() -> Self {
        let mut config = Self::production();
        config.use_ssl = true;
        config.port = get_env_or_default("DERIBIT_PORT", DEFAULT_SSL_PORT);
        config
    }

    /// Create configuration for test environment with SSL
    pub fn test_ssl() -> Self {
        let mut config = Self::new();
        config.use_ssl = true;
        config.port = get_env_or_default("DERIBIT_PORT", DEFAULT_SSL_PORT);
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

impl Default for DeribitFixConfig {
    fn default() -> Self {
        Self::new()
    }
}
