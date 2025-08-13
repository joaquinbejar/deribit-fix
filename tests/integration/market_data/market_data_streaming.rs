//! TEST 21: MARKET DATA STREAMING
//!
//! This test covers subscribing to live market data updates:
//! 1. Send a `MarketDataRequest` (V) with `SubscriptionRequestType` = `SnapshotPlusUpdates`.
//! 2. Receive the initial `MarketDataSnapshotFullRefresh` (W).
//! 3. Subsequently receive `MarketDataIncrementalRefresh` (X) messages.
//! 4. Validate the content of the incremental updates (New, Change, Delete).
