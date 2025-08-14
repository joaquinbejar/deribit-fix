# Enhanced Features

This document describes the recently enhanced features in the deribit-fix crate.

## Market Data Snapshot Enhancements

The `MarketDataSnapshotFullRefresh` now includes additional fields to better represent Deribit-specific features:

- `TradeVolume24h`: 24h traded volume
- `MarkPrice`: Mark price for instruments
- `CurrentFunding` and `Funding8h`: Funding rate metrics for perpetual swaps
- `UnderlyingPx` and `ContractMultiplier`: Underlying/index price and contract size
- `PutOrCall`: Option type when applicable

These fields are emitted in the snapshot (`35=W`) message and parsed accordingly into market data structures.

## Position Report Enhancements

A dedicated `PositionReport` builder and parser has been implemented to fully support emission and consumption of `35=AP` messages, including:

- Position size, average price, and side
- Mark price, index/underlying price
- Realized/unrealized P&L, margin, and leverage

## Mass Quote Acknowledgement Enhancements

The `MassQuoteAcknowledgement` message now supports both standard FIX repeating groups and simplified custom tags for backward compatibility.

### Standard FIX Repeating Groups

When enabled, the message uses proper FIX repeating group structure with tags:
- `295` - NoQuoteEntries 
- `299` - QuoteEntryID
- `9020` - QuoteEntryType
- `1167` - QuoteEntryStatus
- `132/133` - BidPx/OfferPx
- `134/135` - BidSize/OfferSize
- `368` - QuoteEntryRejectReason

### Usage Example

```rust
use deribit_fix::message::quotes::mass_quote_acknowledgement::{MassQuoteAcknowledgement, QuoteAckStatus};

let ack = MassQuoteAcknowledgement::accepted(
    "quote_123".to_string(),
    "set_456".to_string(), 
    vec![]
)
.enable_standard_repeating_groups(); // Enable standard FIX repeating groups

// Or use simplified custom tags (default)
let ack_simple = MassQuoteAcknowledgement::accepted(
    "quote_123".to_string(),
    "set_456".to_string(),
    vec![]
)
.disable_standard_repeating_groups(); // Use custom tags for backward compatibility
```

## Testing

All enhanced features are covered by unit tests. Run tests with:

```bash
make test
```

The enhanced features maintain backward compatibility while providing additional functionality required by the Deribit FIX specification.