# FIX Protocol Enums

This document describes the FIX protocol related enums available in the `deribit-fix` crate.

## Overview

FIX protocol enums define the various message types, field types, and protocol-specific values used in FIX message handling. These enums ensure proper FIX protocol compliance and message formatting.

## FIX Message Type

Defines the type of FIX message.

```rust
pub enum FixMsgType {
    // Session messages
    Logon,
    Logout,
    Heartbeat,
    TestRequest,
    ResendRequest,
    Reject,
    SequenceReset,
    
    // Trading messages
    NewOrderSingle,
    OrderCancelRequest,
    OrderCancelReplaceRequest,
    OrderCancelReject,
    ExecutionReport,
    OrderStatusRequest,
    
    // Market data messages
    MarketDataRequest,
    MarketDataSnapshotFullRefresh,
    MarketDataIncrementalRefresh,
    MarketDataRequestReject,
    
    // Security messages
    SecurityListRequest,
    SecurityList,
    SecurityDefinitionRequest,
    SecurityDefinition,
}
```

**Values:**
- **Session Messages**: `Logon`, `Logout`, `Heartbeat`, `TestRequest`, `ResendRequest`, `Reject`, `SequenceReset`
- **Trading Messages**: `NewOrderSingle`, `OrderCancelRequest`, `OrderCancelReplaceRequest`, `OrderCancelReject`, `ExecutionReport`, `OrderStatusRequest`
- **Market Data Messages**: `MarketDataRequest`, `MarketDataSnapshotFullRefresh`, `MarketDataIncrementalRefresh`, `MarketDataRequestReject`
- **Security Messages**: `SecurityListRequest`, `SecurityList`, `SecurityDefinitionRequest`, `SecurityDefinition`

**Example:**
```rust
let msg_type = FixMsgType::NewOrderSingle;
match msg_type {
    FixMsgType::Logon => println!("Logon message"),
    FixMsgType::NewOrderSingle => println!("New order message"),
    FixMsgType::ExecutionReport => println!("Execution report"),
    _ => println!("Other message type"),
}
```

## FIX Field Type

Defines the data type of a FIX field.

```rust
pub enum FixFieldType {
    String,
    Integer,
    Float,
    Boolean,
    DateTime,
    Date,
    Time,
    Currency,
    Exchange,
    SecurityType,
    Side,
    OrderType,
    TimeInForce,
}
```

**Values:**
- `String` - String value
- `Integer` - Integer value
- `Float` - Floating point value
- `Boolean` - Boolean value (Y/N)
- `DateTime` - Date and time value
- `Date` - Date value only
- `Time` - Time value only
- `Currency` - Currency code
- `Exchange` - Exchange code
- `SecurityType` - Security type code
- `Side` - Order side
- `OrderType` - Order type
- `TimeInForce` - Time in force

**Example:**
```rust
let field_type = FixFieldType::String;
match field_type {
    FixFieldType::String => println!("String field"),
    FixFieldType::Integer => println!("Integer field"),
    FixFieldType::Float => println!("Float field"),
    FixFieldType::Boolean => println!("Boolean field"),
    _ => println!("Other field type"),
}
```

## FIX Tag

Defines the standard FIX protocol tags.

```rust
pub enum FixTag {
    // Header tags
    BeginString = 8,
    BodyLength = 9,
    MsgType = 35,
    MsgSeqNum = 34,
    SenderCompID = 49,
    TargetCompID = 56,
    SendingTime = 52,
    
    // Logon tags
    EncryptMethod = 98,
    HeartBtInt = 108,
    ResetSeqNumFlag = 141,
    
    // Order tags
    ClOrdID = 11,
    Side = 54,
    OrdType = 40,
    OrderQty = 38,
    Price = 44,
    TimeInForce = 59,
    Symbol = 55,
    
    // Execution tags
    ExecID = 17,
    ExecType = 150,
    OrdStatus = 39,
    LeavesQty = 151,
    CumQty = 14,
    AvgPx = 6,
    
    // Market data tags
    MDReqID = 262,
    SubscriptionRequestType = 263,
    MarketDepth = 264,
    MDUpdateType = 265,
    NoMDEntries = 268,
    MDEntryType = 269,
    MDEntryPx = 270,
    MDEntrySize = 271,
}
```

**Values:**
- **Header Tags**: `BeginString`, `BodyLength`, `MsgType`, `MsgSeqNum`, `SenderCompID`, `TargetCompID`, `SendingTime`
- **Logon Tags**: `EncryptMethod`, `HeartBtInt`, `ResetSeqNumFlag`
- **Order Tags**: `ClOrdID`, `Side`, `OrdType`, `OrderQty`, `Price`, `TimeInForce`, `Symbol`
- **Execution Tags**: `ExecID`, `ExecType`, `OrdStatus`, `LeavesQty`, `CumQty`, `AvgPx`
- **Market Data Tags**: `MDReqID`, `SubscriptionRequestType`, `MarketDepth`, `MDUpdateType`, `NoMDEntries`, `MDEntryType`, `MDEntryPx`, `MDEntrySize`

**Example:**
```rust
let tag = FixTag::MsgType;
println!("Tag number: {}", tag as u32);
println!("Tag name: {:?}", tag);
```

## FIX Side

Defines the FIX protocol side values.

```rust
pub enum FixSide {
    Buy = 1,
    Sell = 2,
    BuyMinus = 3,
    SellPlus = 4,
    SellShort = 5,
    SellShortExempt = 6,
    Undisclosed = 7,
    Cross = 8,
    CrossShort = 9,
}
```

**Values:**
- `Buy = 1` - Buy order
- `Sell = 2` - Sell order
- `BuyMinus = 3` - Buy minus
- `SellPlus = 4` - Sell plus
- `SellShort = 5` - Sell short
- `SellShortExempt = 6` - Sell short exempt
- `Undisclosed = 7` - Undisclosed
- `Cross = 8` - Cross order
- `CrossShort = 9` - Cross short order

**Example:**
```rust
let side = FixSide::Buy;
println!("Side value: {}", side as u32);
match side {
    FixSide::Buy => println!("Buy order"),
    FixSide::Sell => println!("Sell order"),
    _ => println!("Other side"),
}
```

## FIX Order Type

Defines the FIX protocol order type values.

```rust
pub enum FixOrderType {
    Market = 1,
    Limit = 2,
    Stop = 3,
    StopLimit = 4,
    MarketOnClose = 5,
    WithOrWithout = 6,
    LimitOrBetter = 7,
    LimitWithOrWithout = 8,
    OnBasis = 9,
    OnClose = 10,
    LimitOnClose = 11,
    ForexMarket = 12,
    PreviouslyQuoted = 13,
    PreviouslyIndicated = 14,
    ForexLimit = 15,
    ForexSwap = 16,
    ForexPreviouslyQuoted = 17,
    Funari = 18,
    MarketIfTouched = 19,
    MarketWithLeftoverAsLimit = 20,
    PreviousFundValuationPoint = 21,
    NextFundValuationPoint = 22,
    Pegged = 23,
    CounterOrderSelection = 24,
}
```

**Values:**
- `Market = 1` - Market order
- `Limit = 2` - Limit order
- `Stop = 3` - Stop order
- `StopLimit = 4` - Stop limit order
- `MarketOnClose = 5` - Market on close
- `WithOrWithout = 6` - With or without
- `LimitOrBetter = 7` - Limit or better
- `LimitWithOrWithout = 8` - Limit with or without
- `OnBasis = 9` - On basis
- `OnClose = 10` - On close
- `LimitOnClose = 11` - Limit on close
- `ForexMarket = 12` - Forex market
- `PreviouslyQuoted = 13` - Previously quoted
- `PreviouslyIndicated = 14` - Previously indicated
- `ForexLimit = 15` - Forex limit
- `ForexSwap = 16` - Forex swap
- `ForexPreviouslyQuoted = 17` - Forex previously quoted
- `Funari = 18` - Funari
- `MarketIfTouched = 19` - Market if touched
- `MarketWithLeftoverAsLimit = 20` - Market with leftover as limit
- `PreviousFundValuationPoint = 21` - Previous fund valuation point
- `NextFundValuationPoint = 22` - Next fund valuation point
- `Pegged = 23` - Pegged order
- `CounterOrderSelection = 24` - Counter order selection

**Example:**
```rust
let order_type = FixOrderType::Limit;
println!("Order type value: {}", order_type as u32);
match order_type {
    FixOrderType::Market => println!("Market order"),
    FixOrderType::Limit => println!("Limit order"),
    FixOrderType::Stop => println!("Stop order"),
    _ => println!("Other order type"),
}
```

## FIX Time In Force

Defines the FIX protocol time in force values.

```rust
pub enum FixTimeInForce {
    Day = 0,
    GoodTillCanceled = 1,
    AtTheOpening = 2,
    ImmediateOrCancel = 3,
    FillOrKill = 4,
    GoodTillCrossing = 5,
    GoodTillDate = 6,
    AtTheClose = 7,
}
```

**Values:**
- `Day = 0` - Day order
- `GoodTillCanceled = 1` - Good till canceled
- `AtTheOpening = 2` - At the opening
- `ImmediateOrCancel = 3` - Immediate or cancel
- `FillOrKill = 4` - Fill or kill
- `GoodTillCrossing = 5` - Good till crossing
- `GoodTillDate = 6` - Good till date
- `AtTheClose = 7` - At the close

**Example:**
```rust
let tif = FixTimeInForce::GoodTillCanceled;
println!("Time in force value: {}", tif as u32);
match tif {
    FixTimeInForce::Day => println!("Day order"),
    FixTimeInForce::GoodTillCanceled => println!("Good till canceled"),
    FixTimeInForce::ImmediateOrCancel => println!("Immediate or cancel"),
    _ => println!("Other time in force"),
}
```

## FIX Exec Type

Defines the FIX protocol execution type values.

```rust
pub enum FixExecType {
    New = 0,
    PartialFill = 1,
    Fill = 2,
    DoneForDay = 3,
    Canceled = 4,
    Replace = 5,
    PendingCancel = 6,
    Stopped = 7,
    Rejected = 8,
    Suspended = 9,
    PendingNew = 10,
    Calculated = 11,
    Expired = 12,
    Restated = 13,
    PendingReplace = 14,
    Trade = 15,
    TradeCorrect = 16,
    TradeCancel = 17,
    OrderStatus = 18,
}
```

**Values:**
- `New = 0` - New
- `PartialFill = 1` - Partial fill
- `Fill = 2` - Fill
- `DoneForDay = 3` - Done for day
- `Canceled = 4` - Canceled
- `Replace = 5` - Replace
- `PendingCancel = 6` - Pending cancel
- `Stopped = 7` - Stopped
- `Rejected = 8` - Rejected
- `Suspended = 9` - Suspended
- `PendingNew = 10` - Pending new
- `Calculated = 11` - Calculated
- `Expired = 12` - Expired
- `Restated = 13` - Restated
- `PendingReplace = 14` - Pending replace
- `Trade = 15` - Trade
- `TradeCorrect = 16` - Trade correct
- `TradeCancel = 17` - Trade cancel
- `OrderStatus = 18` - Order status

**Example:**
```rust
let exec_type = FixExecType::PartialFill;
println!("Exec type value: {}", exec_type as u32);
match exec_type {
    FixExecType::New => println!("New execution"),
    FixExecType::PartialFill => println!("Partial fill"),
    FixExecType::Fill => println!("Complete fill"),
    _ => println!("Other execution type"),
}
```

## Usage Examples

### Creating FIX Messages

```rust
let msg_type = FixMsgType::NewOrderSingle;
let side = FixSide::Buy;
let order_type = FixOrderType::Limit;
let tif = FixTimeInForce::GoodTillCanceled;

let fix_message = FixMessage::new()
    .with_msg_type(msg_type)
    .with_field(FixTag::Side, side as u32)
    .with_field(FixTag::OrdType, order_type as u32)
    .with_field(FixTag::TimeInForce, tif as u32);
```

### Parsing FIX Messages

```rust
let fix_string = "8=FIX.4.2|9=123|35=D|11=ORDER001|54=1|40=2|38=1000|44=50000|59=1|";
let message = FixMessage::from_fix_string(fix_string)?;

let msg_type = message.get_msg_type()?;
match msg_type {
    FixMsgType::NewOrderSingle => println!("New order message"),
    FixMsgType::ExecutionReport => println!("Execution report"),
    _ => println!("Other message type"),
}
```

### Field Type Validation

```rust
let field_type = FixFieldType::Integer;
let value = "12345";

match field_type {
    FixFieldType::Integer => {
        let int_value: i64 = value.parse()?;
        println!("Integer value: {}", int_value);
    }
    FixFieldType::Float => {
        let float_value: f64 = value.parse()?;
        println!("Float value: {}", float_value);
    }
    _ => println!("Other field type"),
}
```

## Best Practices

1. **Use Enum Values**: Always use enum values instead of magic numbers
2. **Validation**: Validate FIX field values against expected types
3. **Error Handling**: Handle all possible enum values in match statements
4. **Documentation**: Document FIX tag meanings and usage
5. **Compliance**: Ensure FIX protocol compliance with exchange requirements

## See Also

- [FixMessage Struct](../structs/fix_message.md)
- [MessageHandler Trait](../traits/message_handler.md)
- [FIX Protocol Documentation](https://www.fixtrading.org/)
- [Message Handling](../../01_project_overview/architecture/data_flow.md)
