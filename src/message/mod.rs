//! FIX message parsing and construction
//!
//! This module provides functionality for creating, parsing, and manipulating
//! FIX protocol messages used in communication with Deribit.

/// Administrative messages (Heartbeat, Test Request, Resend Request, Reject)
pub mod admin;

/// Message builder implementation
pub mod builder;

pub use builder::*;
