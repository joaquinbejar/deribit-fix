# Utility Functions

This document describes the utility functions available in the `deribit-fix` crate.

## Overview

Utility functions provide helper functionality for common operations such as data validation, formatting, calculations, and conversions. These functions are designed to be reusable across different parts of the system.

## Data Validation Functions

### `validate_order()`
Validates an order before submission to ensure it meets exchange requirements.

```rust
pub fn validate_order(order: &Order, config: &OrderValidationConfig) -> Result<(), OrderValidationError>
```

**Parameters:**
- `order: &Order` - The order to validate
- `config: &OrderValidationConfig` - Validation configuration

**Returns:** `Result<(), OrderValidationError>`

**Example:**
```rust
let config = OrderValidationConfig::default();
if let Err(e) = validate_order(&order, &config) {
    eprintln!("Order validation failed: {}", e);
    return Err(e);
}
```

### `validate_instrument()`
Validates if an instrument is tradeable and active.

```rust
pub fn validate_instrument(instrument: &str) -> Result<(), ValidationError>
```

**Parameters:**
- `instrument: &str` - The instrument symbol to validate

**Returns:** `Result<(), ValidationError>`

**Example:**
```rust
if let Err(e) = validate_instrument("BTC-PERPETUAL") {
    eprintln!("Invalid instrument: {}", e);
    return Err(e);
}
```

### `validate_price()`
Validates if a price is within acceptable bounds.

```rust
pub fn validate_price(price: f64, min_price: f64, max_price: f64) -> Result<(), ValidationError>
```

**Parameters:**
- `price: f64` - The price to validate
- `min_price: f64` - Minimum acceptable price
- `max_price: f64` - Maximum acceptable price

**Returns:** `Result<(), ValidationError>`

**Example:**
```rust
if let Err(e) = validate_price(50000.0, 1000.0, 100000.0) {
    eprintln!("Invalid price: {}", e);
    return Err(e);
}
```

## Formatting Functions

### `format_fix_message()`
Formats a FIX message for transmission.

```rust
pub fn format_fix_message(message: &FixMessage) -> Result<String, FixSerializationError>
```

**Parameters:**
- `message: &FixMessage` - The FIX message to format

**Returns:** `Result<String, FixSerializationError>`

**Example:**
```rust
let fix_string = format_fix_message(&fix_message)?;
println!("FIX message: {}", fix_string);
```

### `format_order_book()`
Formats an order book for display.

```rust
pub fn format_order_book(order_book: &OrderBook, depth: usize) -> String
```

**Parameters:**
- `order_book: &OrderBook` - The order book to format
- `depth: usize` - Number of levels to display

**Returns:** `String`

**Example:**
```rust
let formatted = format_order_book(&order_book, 5);
println!("Order Book:\n{}", formatted);
```

### `format_position()`
Formats a position for display.

```rust
pub fn format_position(position: &Position) -> String
```

**Parameters:**
- `position: &Position` - The position to format

**Returns:** `String`

**Example:**
```rust
let formatted = format_position(&position);
println!("Position: {}", formatted);
```

## Calculation Functions

### `calculate_margin_requirement()`
Calculates the required margin for a position.

```rust
pub fn calculate_margin_requirement(
    position_size: f64,
    price: f64,
    leverage: f64,
    margin_type: MarginType
) -> f64
```

**Parameters:**
- `position_size: f64` - Size of the position
- `price: f64` - Current price
- `leverage: f64` - Leverage used
- `margin_type: MarginType` - Type of margin calculation

**Returns:** `f64` - Required margin amount

**Example:**
```rust
let required_margin = calculate_margin_requirement(
    1000.0,    // 1000 contracts
    50000.0,   // $50,000 per contract
    10.0,      // 10x leverage
    MarginType::Isolated
);
println!("Required margin: ${}", required_margin);
```

### `calculate_pnl()`
Calculates the profit and loss for a position.

```rust
pub fn calculate_pnl(
    entry_price: f64,
    current_price: f64,
    position_size: f64,
    position_side: PositionSide
) -> f64
```

**Parameters:**
- `entry_price: f64` - Entry price of the position
- `current_price: f64` - Current market price
- `position_size: f64` - Size of the position
- `position_side: PositionSide` - Long or short position

**Returns:** `f64` - Profit and loss amount

**Example:**
```rust
let pnl = calculate_pnl(
    50000.0,           // Entry at $50,000
    52000.0,           // Current price $52,000
    1000.0,            // 1000 contracts
    PositionSide::Long
);
println!("PnL: ${}", pnl);
```

### `calculate_spread()`
Calculates the bid-ask spread.

```rust
pub fn calculate_spread(bid: f64, ask: f64) -> f64
```

**Parameters:**
- `bid: f64` - Best bid price
- `ask: f64` - Best ask price

**Returns:** `f64` - Spread amount

**Example:**
```rust
let spread = calculate_spread(49999.0, 50001.0);
println!("Spread: ${}", spread);
```

## Conversion Functions

### `convert_currency()`
Converts between different currencies.

```rust
pub fn convert_currency(
    amount: f64,
    from_currency: &str,
    to_currency: &str,
    exchange_rate: f64
) -> Result<f64, ConversionError>
```

**Parameters:**
- `amount: f64` - Amount to convert
- `from_currency: &str` - Source currency
- `to_currency: &str` - Target currency
- `exchange_rate: f64` - Exchange rate

**Returns:** `Result<f64, ConversionError>`

**Example:**
```rust
let usd_amount = convert_currency(
    1000.0,
    "EUR",
    "USD",
    1.18
)?;
println!("1000 EUR = {} USD", usd_amount);
```

### `convert_timezone()`
Converts timestamps between different timezones.

```rust
pub fn convert_timezone(
    timestamp: DateTime<Utc>,
    target_timezone: &str
) -> Result<DateTime<FixedOffset>, TimezoneError>
```

**Parameters:**
- `timestamp: DateTime<Utc>` - UTC timestamp
- `target_timezone: &str` - Target timezone

**Returns:** `Result<DateTime<FixedOffset>, TimezoneError>`

**Example:**
```rust
let utc_time = Utc::now();
let est_time = convert_timezone(utc_time, "America/New_York")?;
println!("UTC: {}, EST: {}", utc_time, est_time);
```

## Time and Date Functions

### `is_trading_hours()`
Checks if the current time is within trading hours.

```rust
pub fn is_trading_hours(
    current_time: DateTime<Utc>,
    trading_hours: &TradingHours
) -> bool
```

**Parameters:**
- `current_time: DateTime<Utc>` - Current time
- `trading_hours: &TradingHours` - Trading hours configuration

**Returns:** `bool`

**Example:**
```rust
let trading_hours = TradingHours::default();
if is_trading_hours(Utc::now(), &trading_hours) {
    println!("Market is open");
} else {
    println!("Market is closed");
}
```

### `calculate_time_until_open()`
Calculates time until the market opens.

```rust
pub fn calculate_time_until_open(
    current_time: DateTime<Utc>,
    trading_hours: &TradingHours
) -> Option<Duration>
```

**Parameters:**
- `current_time: DateTime<Utc>` - Current time
- `trading_hours: &TradingHours` - Trading hours configuration

**Returns:** `Option<Duration>`

**Example:**
```rust
if let Some(time_until_open) = calculate_time_until_open(Utc::now(), &trading_hours) {
    println!("Market opens in: {:?}", time_until_open);
}
```

## Logging and Debug Functions

### `log_message()`
Logs a message with appropriate level and context.

```rust
pub fn log_message(
    level: log::Level,
    target: &str,
    message: &str,
    context: Option<&str>
)
```

**Parameters:**
- `level: log::Level` - Log level
- `target: &str` - Log target
- `message: &str` - Message to log
- `context: Option<&str>` - Optional context

**Example:**
```rust
log_message(
    log::Level::Info,
    "order_placement",
    "Order placed successfully",
    Some("BTC-PERPETUAL")
);
```

### `debug_fix_message()`
Provides debug information for a FIX message.

```rust
pub fn debug_fix_message(message: &FixMessage) -> String
```

**Parameters:**
- `message: &FixMessage` - The FIX message to debug

**Returns:** `String` - Debug information

**Example:**
```rust
let debug_info = debug_fix_message(&fix_message);
println!("FIX Message Debug:\n{}", debug_info);
```

## Error Handling

All utility functions that can fail return `Result<T, E>` where `E` is an appropriate error type. Common error types include:

- `ValidationError` - Data validation failures
- `ConversionError` - Data conversion failures
- `FixSerializationError` - FIX message formatting errors
- `TimezoneError` - Timezone conversion errors

## Performance Considerations

- **Validation Functions**: These are designed to be fast and lightweight
- **Formatting Functions**: May allocate memory for string formatting
- **Calculation Functions**: Use efficient mathematical operations
- **Conversion Functions**: May involve database lookups or API calls

## Best Practices

1. **Input Validation**: Always validate inputs before processing
2. **Error Handling**: Handle all potential errors from utility functions
3. **Performance**: Cache expensive calculations when possible
4. **Logging**: Use appropriate log levels for different types of operations
5. **Testing**: Write unit tests for all utility functions

## See Also

- [OrderValidationConfig Struct](../structs/order_validation_config.md)
- [TradingHours Struct](../structs/trading_hours.md)
- [FixMessage Struct](../structs/fix_message.md)
- [Error Handling](../../01_project_overview/architecture/error_handling.md)
