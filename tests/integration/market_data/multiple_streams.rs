//! TEST 24: MULTIPLE CONCURRENT STREAMS
//!
//! This test verifies the client can handle multiple market data subscriptions at once:
//! 1. Subscribe to the order book for 'BTC-PERPETUAL'.
//! 2. Subscribe to the trade feed for 'ETH-PERPETUAL'.
//! 3. Ensure the client correctly demultiplexes and processes messages for both streams.
