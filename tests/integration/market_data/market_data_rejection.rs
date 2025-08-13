//! TEST 23: MARKET DATA REJECTION
//!
//! This test ensures `MarketDataRequestReject` (Y) is handled correctly:
//! 1. Request market data for a non-existent instrument.
//! 2. Expect a `MarketDataRequestReject` message with the appropriate reason.
//! 3. Request market data with insufficient permissions (if possible to simulate).
//! 4. Expect a `MarketDataRequestReject` message.
