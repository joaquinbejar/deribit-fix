//! TEST 22: TRADE DATA STREAMING
//!
//! This test covers subscribing to the live trade feed:
//! 1. Send a `MarketDataRequest` (V) for the `Trade` entry type.
//! 2. Receive `MarketDataIncrementalRefresh` (X) messages representing new trades.
//! 3. Validate the trade messages contain price, quantity, and trade ID.
