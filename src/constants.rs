/******************************************************************************
    Author: Joaquín Béjar García
    Email: jb@taunais.com 
    Date: 21/7/25
 ******************************************************************************/

// Default configuration constants
pub(crate) const DEFAULT_TEST_HOST: &str = "test.deribit.com";
pub(crate) const DEFAULT_PROD_HOST: &str = "www.deribit.com";
pub(crate) const DEFAULT_TEST_PORT: u16 = 9881;
pub(crate) const DEFAULT_PROD_PORT: u16 = 9880;
pub(crate) const DEFAULT_SSL_PORT: u16 = 9883;
pub(crate) const DEFAULT_HEARTBEAT_INTERVAL: u32 = 30;
pub(crate) const DEFAULT_CONNECTION_TIMEOUT_SECS: u64 = 10;
pub(crate) const DEFAULT_RECONNECT_ATTEMPTS: u32 = 3;
pub(crate) const DEFAULT_RECONNECT_DELAY_SECS: u64 = 5;
pub(crate) const DEFAULT_LOG_LEVEL: &str = "info";
pub(crate) const DEFAULT_SENDER_COMP_ID: &str = "CLIENT";
pub(crate) const DEFAULT_TARGET_COMP_ID: &str = "DERIBITSERVER";