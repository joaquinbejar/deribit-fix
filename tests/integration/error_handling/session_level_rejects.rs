//! TEST 50: SESSION-LEVEL REJECTS
//!
//! This test ensures the client correctly handles session-level `Reject` (3) messages:
//! 1. Send a message with a required tag missing.
//! 2. Send a message with an undefined tag.
//! 3. Send a message with an invalid value for a tag.
//! 4. In each case, expect a `Reject` (3) message with the correct `SessionRejectReason`.
