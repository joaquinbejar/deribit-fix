# Enhanced Features

This document describes the recently enhanced features in the deribit-fix crate.

## Market Data Snapshot Enhancements

The `MarketDataSnapshotFullRefresh` message now includes additional snapshot-only optional fields as per the Deribit FIX specification:

### New MdEntry Fields

The `MdEntry` struct now supports the following snapshot-only fields:

- **`price`** (`Option<f64>`) - Index price at trade moment (FIX tag 44)
- **`text`** (`Option<String>`) - Trade sequence number (FIX tag 58)  
- **`ord_status`** (`Option<char>`) - Order status (FIX tag 39)
- **`deribit_label`** (`Option<String>`) - User-defined order label (FIX tag 100010)
- **`deribit_liquidation`** (`Option<String>`) - Liquidation indicator (FIX tag 100091)
- **`trd_match_id`** (`Option<String>`) - Block trade ID (FIX tag 880)

These fields are automatically included in the FIX message output when present.

### Usage Example

```rust
use deribit_fix::message::market_data::{MarketDataSnapshotFullRefresh, MdEntry, MdEntryType};

let mut entry = MdEntry::trade(50000.0, 1.5, '1', "12345".to_string(), Utc::now());
entry.deribit_label = Some("my_label".to_string());
entry.ord_status = Some('2'); // Filled
entry.trd_match_id = Some("block_123".to_string());

let snapshot = MarketDataSnapshotFullRefresh::new("BTC-PERPETUAL".to_string())
    .with_entries(vec![entry]);
```

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