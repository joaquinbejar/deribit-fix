//! TEST 03: HEARTBEAT AND LIVENESS
//!
//! This test ensures the session keep-alive mechanism is working correctly:
//! 1. After a successful logon, wait for the server to send a TestRequest (1).
//! 2. Respond correctly with a Heartbeat (0) containing the `TestReqID`.
//! 3. Proactively send a Heartbeat (0) if no messages are sent for the configured interval.
//! 4. Ensure the session remains active throughout.
