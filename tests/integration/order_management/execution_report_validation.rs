//! TEST 14: EXECUTION REPORT VALIDATION
//!
//! This test performs deep validation of `ExecutionReport` (8) fields:
//! 1. For a new order, validate `OrdStatus` is `New`.
//! 2. For a filled order, validate `OrdStatus` is `Filled`, and `LastPx`, `LastQty`, `CumQty` are correct.
//! 3. For a canceled order, validate `OrdStatus` is `Canceled`.
//! 4. For a rejected order, validate `OrdStatus` is `Rejected` and `Text` field contains a reason.
