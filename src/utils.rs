/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 6/3/26
******************************************************************************/

//! Utility functions for the Deribit FIX client

use std::env;
use std::sync::Once;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

static INIT: Once = Once::new();

/// Sets up the logger for the application.
///
/// The logger level is determined by the `DERIBIT_LOG_LEVEL` environment variable.
/// If the variable is not set, it defaults to `INFO`.
///
/// This function can be called multiple times safely - it will only initialize
/// the logger on the first call.
///
/// # Example
///
/// ```rust,no_run
/// use deribit_fix::utils::setup_logger;
///
/// fn main() {
///     setup_logger();
///     // Application code here
/// }
/// ```
pub fn setup_logger() {
    INIT.call_once(|| {
        let log_level = env::var("DERIBIT_LOG_LEVEL")
            .unwrap_or_else(|_| "INFO".to_string())
            .to_uppercase();

        let level = match log_level.as_str() {
            "DEBUG" => Level::DEBUG,
            "ERROR" => Level::ERROR,
            "WARN" => Level::WARN,
            "TRACE" => Level::TRACE,
            _ => Level::INFO,
        };

        let subscriber = FmtSubscriber::builder().with_max_level(level).finish();

        if tracing::subscriber::set_global_default(subscriber).is_ok() {
            tracing::debug!("Log level set to: {}", level);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup_logger_can_be_called_multiple_times() {
        setup_logger();
        setup_logger();
        // Should not panic
    }
}
