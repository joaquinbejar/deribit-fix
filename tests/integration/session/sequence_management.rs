//! TEST 04: SEQUENCE NUMBER MANAGEMENT
//!
//! This test covers the handling of message sequence numbers:
//! 1. Send a message with a sequence number lower than expected; expect a Logout.
//! 2. Send a message with a sequence number higher than expected; expect a ResendRequest (2).
//! 3. Respond to the ResendRequest with a SequenceReset-GapFill (4) message.
//! 4. Ensure the session recovers and continues.
