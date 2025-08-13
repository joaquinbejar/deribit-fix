//! TEST 02: AUTHENTICATION FAILURES
//!
//! This test covers various logon failure scenarios:
//! 1. Attempt logon with an incorrect `RawData` signature (wrong password).
//! 2. Attempt logon with a stale timestamp.
//! 3. Attempt logon with an invalid `SenderCompID` (API Key).
//! 4. In each case, expect a Logout (5) message with a descriptive text field
//!    and a forceful disconnect from the server.
