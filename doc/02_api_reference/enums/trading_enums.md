# Trading Enums

This document describes the trading-related enums available in the `deribit-fix` crate.

## Overview

Trading enums define the various states, types, and options used in trading operations. These enums provide type safety and clear semantics for trading-related functionality.

## Order Side

Defines the side of an order (buy or sell).

```rust
pub enum OrderSide {
    Buy,
    Sell,
}
```

**Values:**
- `Buy` - Buy order (long position)
- `Sell` - Sell order (short position)

**Example:**
```rust
let side = OrderSide::Buy;
match side {
    OrderSide::Buy => println!("Placing buy order"),
    OrderSide::Sell => println!("Placing sell order"),
}
```

## Order Type

Defines the type of order to place.

```rust
pub enum OrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
    TrailingStop,
}
```

**Values:**
- `Market` - Market order (executed at current market price)
- `Limit` - Limit order (executed at specified price or better)
- `Stop` - Stop order (becomes market order when stop price is reached)
- `StopLimit` - Stop limit order (becomes limit order when stop price is reached)
- `TrailingStop` - Trailing stop order (stop price follows market price)

**Example:**
```rust
let order_type = OrderType::Limit;
match order_type {
    OrderType::Market => println!("Market order"),
    OrderType::Limit => println!("Limit order"),
    OrderType::Stop => println!("Stop order"),
    OrderType::StopLimit => println!("Stop limit order"),
    OrderType::TrailingStop => println!("Trailing stop order"),
}
```

## Time In Force

Defines how long an order should remain active.

```rust
pub enum TimeInForce {
    Day,
    GoodTillCanceled,
    ImmediateOrCancel,
    FillOrKill,
}
```

**Values:**
- `Day` - Order is valid for the current trading day
- `GoodTillCanceled` - Order remains active until canceled
- `ImmediateOrCancel` - Order must be filled immediately or canceled
- `FillOrKill` - Order must be filled completely or canceled

**Example:**
```rust
let tif = TimeInForce::GoodTillCanceled;
match tif {
    TimeInForce::Day => println!("Valid for today"),
    TimeInForce::GoodTillCanceled => println!("Valid until canceled"),
    TimeInForce::ImmediateOrCancel => println!("Fill immediately or cancel"),
    TimeInForce::FillOrKill => println!("Fill completely or cancel"),
}
```

## Order Status

Defines the current status of an order.

```rust
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
    Expired,
}
```

**Values:**
- `New` - Order has been accepted but not yet executed
- `PartiallyFilled` - Order has been partially executed
- `Filled` - Order has been completely executed
- `Canceled` - Order has been canceled
- `Rejected` - Order was rejected by the exchange
- `Expired` - Order has expired

**Example:**
```rust
let status = OrderStatus::PartiallyFilled;
match status {
    OrderStatus::New => println!("Order is new"),
    OrderStatus::PartiallyFilled => println!("Order partially filled"),
    OrderStatus::Filled => println!("Order completely filled"),
    OrderStatus::Canceled => println!("Order was canceled"),
    OrderStatus::Rejected => println!("Order was rejected"),
    OrderStatus::Expired => println!("Order has expired"),
}
```

## Exec Type

Defines the execution type of an order.

```rust
pub enum ExecType {
    New,
    PartialFill,
    Fill,
    Canceled,
    Replaced,
    Rejected,
    Expired,
}
```

**Values:**
- `New` - New order execution
- `PartialFill` - Partial fill execution
- `Fill` - Complete fill execution
- `Canceled` - Order cancellation
- `Replaced` - Order replacement
- `Rejected` - Order rejection
- `Expired` - Order expiration

**Example:**
```rust
let exec_type = ExecType::PartialFill;
match exec_type {
    ExecType::New => println!("New execution"),
    ExecType::PartialFill => println!("Partial fill"),
    ExecType::Fill => println!("Complete fill"),
    ExecType::Canceled => println!("Canceled"),
    ExecType::Replaced => println!("Replaced"),
    ExecType::Rejected => println!("Rejected"),
    ExecType::Expired => println!("Expired"),
}
```

## Position Side

Defines the side of a position.

```rust
pub enum PositionSide {
    Long,
    Short,
}
```

**Values:**
- `Long` - Long position (bought)
- `Short` - Short position (sold)

**Example:**
```rust
let side = PositionSide::Long;
match side {
    PositionSide::Long => println!("Long position"),
    PositionSide::Short => println!("Short position"),
}
```

## Instrument Type

Defines the type of financial instrument.

```rust
pub enum InstrumentType {
    Future,
    Option,
    Perpetual,
    Spot,
}
```

**Values:**
- `Future` - Futures contract
- `Option` - Options contract
- `Perpetual` - Perpetual futures contract
- `Spot` - Spot trading instrument

**Example:**
```rust
let instrument_type = InstrumentType::Perpetual;
match instrument_type {
    InstrumentType::Future => println!("Futures contract"),
    InstrumentType::Option => println!("Options contract"),
    InstrumentType::Perpetual => println!("Perpetual futures"),
    InstrumentType::Spot => println!("Spot instrument"),
}
```

## Option Type

Defines the type of option (for options instruments).

```rust
pub enum OptionType {
    Call,
    Put,
}
```

**Values:**
- `Call` - Call option (right to buy)
- `Put` - Put option (right to sell)

**Example:**
```rust
let option_type = OptionType::Call;
match option_type {
    OptionType::Call => println!("Call option"),
    OptionType::Put => println!("Put option"),
}
```

## Usage Examples

### Creating an Order with Enums

```rust
let order = Order::new(
    "BTC-PERPETUAL",
    OrderSide::Buy,
    OrderType::Limit,
    1000.0,        // quantity
    50000.0        // price
)?;

order.set_time_in_force(TimeInForce::GoodTillCanceled);
```

### Checking Order Status

```rust
let status = client.get_order_status("order_123").await?;
match status {
    OrderStatus::New => println!("Order is pending"),
    OrderStatus::PartiallyFilled => println!("Order partially filled"),
    OrderStatus::Filled => println!("Order completed"),
    OrderStatus::Canceled => println!("Order was canceled"),
    OrderStatus::Rejected => println!("Order was rejected"),
    OrderStatus::Expired => println!("Order expired"),
}
```

### Position Management

```rust
let position = client.get_position("BTC-PERPETUAL").await?;
if let Some(pos) = position {
    match pos.side {
        PositionSide::Long => println!("Long position: {}", pos.size),
        PositionSide::Short => println!("Short position: {}", pos.size),
    }
}
```

## Best Practices

1. **Use Pattern Matching**: Always use pattern matching with enums for exhaustive handling
2. **Type Safety**: Leverage enum type safety to prevent invalid states
3. **Documentation**: Document what each enum value represents
4. **Default Values**: Provide sensible default values where appropriate
5. **Serialization**: Ensure enums can be properly serialized for FIX messages

## See Also

- [Order Struct](../structs/order.md)
- [Position Struct](../structs/position.md)
- [Instrument Struct](../structs/instrument.md)
- [Trading Operations](../../01_project_overview/usage/advanced_example.md)
