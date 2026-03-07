/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 7/3/26
******************************************************************************/

//! Common exports for easy importing
//!
//! This module re-exports the most commonly used types from the deribit-fix
//! crate for convenient importing in client applications.
//!
//! # Example
//!
//! ```rust,no_run
//! use deribit_fix::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let config = DeribitFixConfig::default()
//!         .with_credentials("your_key".to_string(), "your_secret".to_string());
//!
//!     let mut client = DeribitFixClient::new(&config).await?;
//!     client.connect().await?;
//!     client.disconnect().await?;
//!     Ok(())
//! }
//! ```

#![allow(ambiguous_glob_reexports)]

// Client exports
pub use crate::client::DeribitFixClient;

// Configuration exports
pub use crate::config::{DeribitFixConfig, gen_id};

// Error handling exports
pub use crate::error::{DeribitFixError, Result};

// Message exports - all message types for FIX protocol communication
pub use crate::message::{
    admin::*, builder::*, market_data::*, orders::*, positions::*, quotes::*, risk::*,
    security_definition::*, security_list::*, security_status::*, trade::*, user::*,
};

// Model exports - data structures and types
pub use crate::model::*;

// Session exports - session management
pub use crate::session::{Session, SessionState};

// Utility exports
pub use crate::utils::setup_logger;
