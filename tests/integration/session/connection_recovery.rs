//! TEST 05: CONNECTION RECOVERY
//!
//! This test verifies the client's ability to recover a session after a transport-level disconnect:
//! 1. Establish a session and send some messages.
//! 2. Simulate a TCP connection drop.
//! 3. Reconnect and send a Logon (A) message with the previous session's sequence numbers.
//! 4. Handle any ResendRequests from the server to synchronize state.
//! 5. Confirm the session is re-established without a full reset.
