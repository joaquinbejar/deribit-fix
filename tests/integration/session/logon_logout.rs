//! TEST 01: LOGON AND LOGOUT
//!
//! This test covers the most fundamental FIX session flow:
//! 1. Establish a TCP connection.
//! 2. Send a valid Logon (A) message.
//! 3. Receive and validate the server's Logon (A) confirmation.
//! 4. Confirm the session becomes `Active`.
//! 5. Send a Logout (5) message.
//! 6. Receive and validate the server's Logout (5) confirmation.
//! 7. Ensure the connection is terminated gracefully.
