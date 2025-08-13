//! TEST 41: SECURITY STATUS
//!
//! This test covers querying the status of a specific instrument:
//! 1. Send a `SecurityStatusRequest` (e) for a known instrument.
//! 2. Receive and validate the `SecurityStatus` (f) message.
//! 3. Ensure the `TradingSessionID` and `SecurityTradingStatus` are present and valid.
