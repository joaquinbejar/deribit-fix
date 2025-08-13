//! TEST 11: ORDER CANCELLATION
//!
//! This test covers the cancellation of existing orders:
//! 1. Submit a new limit order that will not be filled immediately.
//! 2. Send an `OrderCancelRequest` (F) using the order's `ClOrdID`.
//! 3. Receive and validate the `ExecutionReport` confirming the order state is `Canceled`.
//! 4. Verify the `OrderCancelReject` (9) is received if trying to cancel a filled or unknown order.
