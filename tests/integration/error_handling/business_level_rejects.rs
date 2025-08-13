//! TEST 51: BUSINESS-LEVEL REJECTS
//!
//! This test ensures the client correctly handles `BusinessMessageReject` (j) messages:
//! 1. Send a logically flawed message (e.g., an `OrderCancelRequest` for an unknown `MsgType`).
//! 2. Expect to receive a `BusinessMessageReject` with the appropriate `BusinessRejectReason`.
