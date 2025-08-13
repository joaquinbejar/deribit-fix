//! TEST 32: TRADE CAPTURE REPORT
//!
//! This test covers requesting historical trade data:
//! 1. Send a `TradeCaptureReportRequest` (AD).
//! 2. Receive one or more `TradeCaptureReport` (AE) messages.
//! 3. Validate the contents of the reports, ensuring they match recent trades.
