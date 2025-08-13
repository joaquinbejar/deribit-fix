//! TEST 40: SECURITY LIST REQUEST
//!
//! This test covers querying for available instruments:
//! 1. Send a `SecurityListRequest` (x).
//! 2. Receive the `SecurityList` (y) message(s).
//! 3. Validate that the response contains a list of securities and their definitions.
//! 4. Parse key details for at least one instrument (e.g., symbol, tick size).
