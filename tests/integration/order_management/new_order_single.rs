//! TEST 10: NEW ORDER SINGLE
//!
//! This test covers the submission of new orders:
//! 1. Submit a valid limit buy order (NewOrderSingle - D).
//! 2. Receive and validate the `ExecutionReport` (8) confirming the order is `New` (pending).
//! 3. Submit a valid market sell order.
//! 4. Receive and validate the `ExecutionReport` confirming the order is `Filled` or `PartiallyFilled`.
