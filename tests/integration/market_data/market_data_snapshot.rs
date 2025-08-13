//! TEST 20: MARKET DATA SNAPSHOT
//!
//! This test covers requesting a market data snapshot:
//! 1. Send a `MarketDataRequest` (V) with `SubscriptionRequestType` = `Snapshot`.
//! 2. Receive a single `MarketDataSnapshotFullRefresh` (W) message.
//! 3. Validate the message contains order book entries (bids and asks).
//! 4. Ensure no further market data messages are received for this request.
