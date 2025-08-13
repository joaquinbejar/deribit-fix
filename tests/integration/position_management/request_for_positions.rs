//! TEST 30: REQUEST FOR POSITIONS
//!
//! This test covers requesting and receiving position information:
//! 1. After executing a trade to establish a position, send a `RequestForPositions` (AN).
//! 2. Receive and validate the `PositionReport` (AP) message.
//! 3. Ensure the report contains the correct instrument, quantity, and average price.
