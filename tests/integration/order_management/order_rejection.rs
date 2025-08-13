//! TEST 12: ORDER CANCEL REJECTION
//!
//! This test validates the `OrderCancelReject` (9) message flow:
//! 1. Submit a new order and wait for it to be fully filled.
//! 2. Attempt to cancel the filled order.
//! 3. Expect an `OrderCancelReject` message with the correct `CxlRejReason`.
//! 4. Attempt to cancel an order using a non-existent `ClOrdID`.
//! 5. Expect an `OrderCancelReject` message.
