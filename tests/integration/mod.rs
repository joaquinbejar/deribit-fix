//! Integration tests for Deribit FIX client
//!
//! These tests verify the complete functionality of FIX message construction,
//! serialization, and validation across different message types.

//! Deribit FIX Integration Tests

pub mod error_handling;
pub mod market_data;
pub mod order_management;
pub mod position_management;
pub mod reference_data;
pub mod scenarios;
pub mod session;
pub mod ssl;