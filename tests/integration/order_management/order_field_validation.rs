//! TEST 15: ORDER FIELD VALIDATION
//!
//! This test covers specific order modifiers and types:
//! 1. Submit a `PostOnly` order and ensure it gets rejected if it would trade immediately.
//! 2. Submit an order with `TimeInForce` = `ImmediateOrCancel` and validate behavior.
//! 3. Submit an order with `TimeInForce` = `FillOrKill` and validate behavior.
