//! TEST 90: FULL TRADE LIFECYCLE (END-TO-END)
//!
//! This test simulates a complete trading session from start to finish:
//! 1. Logon successfully.
//! 2. Request the security list to find a tradable instrument.
//! 3. Subscribe to market data for that instrument.
//! 4. Place a limit order below the market price.
//! 5. Receive confirmation, then send a cancel request.
//! 6. Receive cancel confirmation.
//! 7. Place a market order.
//! 8. Receive fill confirmation.
//! 9. Request positions and verify the new position.
//! 10. Logout.
