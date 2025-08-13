//! TEST 13: MASS ORDER OPERATIONS
//!
//! This test covers mass order messages:
//! 1. Place several open orders for a specific instrument.
//! 2. Send an `OrderMassCancelRequest` (q) to cancel all orders for that instrument.
//! 3. Receive and validate the `OrderMassCancelReport` (r) confirming the cancellation.
//! 4. Send an `OrderMassStatusRequest` (AF) and validate the `ExecutionReport` responses.
