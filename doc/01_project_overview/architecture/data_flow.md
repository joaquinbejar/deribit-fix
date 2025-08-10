# Data Flow

This document provides detailed explanations of how data flows through the `deribit-fix` crate, including step-by-step diagrams and sequence flows for various operations.

## Overview

The data flow in `deribit-fix` follows a layered architecture where each layer has specific responsibilities and data transformations. Data flows bidirectionally between the client application and Deribit's FIX gateway.

## Data Flow Layers

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Application   │    │   FIX Client    │    │  Deribit FIX   │
│     Layer      │◄──►│     Layer       │◄──►│     Gateway     │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## Connection Establishment Flow

### 1. Client Initialization
```
Application → DeribitFixClient::new()
    ↓
Load configuration from environment/files
    ↓
Validate configuration parameters
    ↓
Create internal components (session, connection)
    ↓
Return configured client instance
```

### 2. Connection Setup
```
DeribitFixClient::connect()
    ↓
SessionManager::establish_connection()
    ↓
ConnectionManager::connect_tcp()
    ↓
Establish TCP connection to Deribit
    ↓
Enable SSL/TLS if configured
    ↓
Set connection timeout and keepalive
    ↓
Return connection status
```

### 3. FIX Session Establishment
```
Connection established
    ↓
Send FIX Logon message
    ├── SenderCompID: Client identifier
    ├── TargetCompID: "DERIBIT"
    ├── HeartBtInt: Heartbeat interval
    ├── Username: API key
    ├── Password: API secret hash
    └── ResetSeqNumFlag: Y (for new sessions)
    ↓
Wait for Logon acknowledgment
    ↓
Start heartbeat timer
    ↓
Session state: LoggedIn
```

## Order Placement Flow

### 1. Order Creation
```
Application creates Order struct
    ↓
Validate order parameters
    ├── Symbol exists and is tradable
    ├── Quantity is valid
    ├── Price is within limits (if limit order)
    └── Client order ID is unique
    ↓
Convert to FIX NewOrderSingle message
    ├── MsgType: D (New Order Single)
    ├── ClOrdID: Client order ID
    ├── Symbol: Trading instrument
    ├── Side: Buy/Sell
    ├── OrdType: Market/Limit/Stop
    ├── OrderQty: Quantity
    ├── Price: Limit price (if applicable)
    └── TimeInForce: Order validity
    ↓
Send to Deribit via FIX session
```

### 2. Order Processing
```
FIX message sent
    ↓
Deribit processes order
    ↓
Send ExecutionReport (MsgType: 8)
    ├── ExecType: 0 (New)
    ├── OrdStatus: 0 (New)
    ├── OrderID: Deribit order ID
    ├── ClOrdID: Client order ID
    └── Other order details
    ↓
Client receives execution report
    ↓
Update internal order state
    ↓
Notify application of order status
```

### 3. Order Execution
```
Order matched in market
    ↓
Deribit sends ExecutionReport
    ├── ExecType: 1 (Partial Fill) or 2 (Fill)
    ├── OrdStatus: 1 (Partially Filled) or 2 (Filled)
    ├── CumQty: Cumulative filled quantity
    ├── AvgPx: Average execution price
    └── LastQty: Last fill quantity
    ↓
Client processes execution report
    ↓
Update position and order state
    ↓
Send execution notification to application
```

## Market Data Flow

### 1. Market Data Subscription
```
Application requests market data
    ↓
Create MarketDataRequest message
    ├── MsgType: V (Market Data Request)
    ├── MDReqID: Request identifier
    ├── SubscriptionRequestType: 1 (Subscribe)
    ├── MarketDepth: Number of levels
    ├── MDUpdateType: 0 (Full refresh)
    └── NoMDEntryTypes: Entry types
    ↓
Send to Deribit
    ↓
Deribit acknowledges subscription
    ↓
Start receiving market data updates
```

### 2. Market Data Updates
```
Market data changes occur
    ↓
Deribit sends MarketDataSnapshotFullRefresh
    ├── MsgType: W (Market Data Snapshot Full Refresh)
    ├── MDReqID: Request identifier
    ├── NoMDEntries: Number of entries
    └── MDEntryType: Bid/Ask/Trade
    ↓
Client parses market data
    ↓
Update internal order book
    ↓
Notify application of changes
    ↓
Trigger any dependent operations
```

## Position Management Flow

### 1. Position Query
```
Application requests positions
    ↓
Send SecurityStatusRequest
    ├── MsgType: e (Security Status Request)
    ├── SecurityReqID: Request identifier
    └── Symbol: Trading instrument
    ↓
Deribit responds with positions
    ↓
Parse position data
    ├── Symbol
    ├── Side (Long/Short)
    ├── Size
    ├── Average entry price
    └── Unrealized P&L
    ↓
Update internal position state
    ↓
Return position information to application
```

## Error Handling Flow

### 1. Error Detection
```
Error occurs at any layer
    ↓
Create appropriate error type
    ├── Connection errors
    ├── FIX protocol errors
    ├── Business logic errors
    └── Configuration errors
    ↓
Log error with context
    ↓
Determine error severity
    ↓
Choose error handling strategy
```

### 2. Error Recovery
```
Non-critical error
    ↓
Attempt automatic recovery
    ├── Retry with exponential backoff
    ├── Reconnect if connection lost
    └── Reset sequence numbers if needed
    ↓
Notify application of recovery attempt
    ↓
Continue normal operation
```

### 3. Critical Error
```
Critical error detected
    ↓
Log error details
    ↓
Close FIX session gracefully
    ↓
Disconnect from Deribit
    ↓
Notify application of error
    ↓
Require manual intervention
```

## Heartbeat and Keepalive Flow

### 1. Heartbeat Timer
```
Session established
    ↓
Start heartbeat timer (configurable interval)
    ↓
Timer expires
    ↓
Send Heartbeat message
    ├── MsgType: 0 (Heartbeat)
    ├── TestReqID: Test request identifier
    └── Other required fields
    ↓
Wait for acknowledgment
    ↓
Reset timer
    ↓
Continue monitoring
```

### 2. Test Request Handling
```
Receive TestRequest from Deribit
    ├── MsgType: 1 (Test Request)
    └── TestReqID: Request identifier
    ↓
Send Heartbeat response
    ├── MsgType: 0 (Heartbeat)
    ├── TestReqID: Echo received ID
    └── Other required fields
    ↓
Maintain session health
```

## Session Termination Flow

### 1. Graceful Shutdown
```
Application requests disconnect
    ↓
Send Logout message
    ├── MsgType: 5 (Logout)
    └── Text: Reason for logout
    ↓
Wait for Logout acknowledgment
    ↓
Stop heartbeat timer
    ↓
Close TCP connection
    ↓
Reset session state
    ↓
Notify application of disconnect
```

### 2. Emergency Shutdown
```
Critical error or timeout
    ↓
Immediately close TCP connection
    ↓
Stop all timers
    ↓
Reset session state
    ↓
Log shutdown reason
    ↓
Notify application of emergency shutdown
```

## Data Transformation Examples

### Order to FIX Message
```rust
// Rust Order struct
let order = Order {
    symbol: "BTC-PERPETUAL".to_string(),
    side: OrderSide::Buy,
    order_type: OrderType::Limit,
    quantity: 1.0,
    price: Some(50000.0),
    time_in_force: TimeInForce::GoodTillCancel,
    client_order_id: "order_123".to_string(),
};

// Converted to FIX message
let fix_message = FixMessage {
    msg_type: "D".to_string(), // New Order Single
    sender_comp_id: "CLIENT_001".to_string(),
    target_comp_id: "DERIBIT".to_string(),
    msg_seq_num: 1,
    sending_time: Utc::now(),
    fields: HashMap::from([
        ("11".to_string(), "order_123".to_string()), // ClOrdID
        ("55".to_string(), "BTC-PERPETUAL".to_string()), // Symbol
        ("54".to_string(), "1".to_string()), // Side (1=Buy)
        ("40".to_string(), "2".to_string()), // OrdType (2=Limit)
        ("38".to_string(), "1.0".to_string()), // OrderQty
        ("44".to_string(), "50000.0".to_string()), // Price
        ("59".to_string(), "1".to_string()), // TimeInForce (1=GTC)
    ]),
};
```

### FIX Message to Execution Report
```rust
// Incoming FIX message
let fix_message = FixMessage {
    msg_type: "8".to_string(), // Execution Report
    // ... other fields
    fields: HashMap::from([
        ("37".to_string(), "deribit_order_456".to_string()), // OrderID
        ("11".to_string(), "order_123".to_string()), // ClOrdID
        ("17".to_string(), "exec_789".to_string()), // ExecID
        ("20".to_string(), "0".to_string()), // ExecTransType
        ("39".to_string(), "0".to_string()), // OrdStatus (0=New)
        ("55".to_string(), "BTC-PERPETUAL".to_string()), // Symbol
        ("54".to_string(), "1".to_string()), // Side (1=Buy)
        ("38".to_string(), "1.0".to_string()), // OrderQty
        ("151".to_string(), "0.0".to_string()), // LeavesQty
        ("14".to_string(), "0.0".to_string()), // CumQty
    ]),
};

// Converted to Rust struct
let execution_report = ExecutionReport {
    order_id: "deribit_order_456".to_string(),
    client_order_id: "order_123".to_string(),
    exec_id: "exec_789".to_string(),
    exec_type: ExecType::New,
    ord_status: OrdStatus::New,
    symbol: "BTC-PERPETUAL".to_string(),
    side: OrderSide::Buy,
    leaves_qty: 0.0,
    cum_qty: 0.0,
    avg_px: 0.0,
};
```

## Performance Considerations

### Data Flow Optimization
- **Message Batching**: Group multiple orders in single FIX message when possible
- **Connection Pooling**: Reuse connections for multiple operations
- **Async Processing**: Non-blocking I/O for all network operations
- **Memory Management**: Efficient serialization/deserialization
- **Caching**: Cache frequently accessed data (symbols, positions)

### Monitoring Points
- **Latency**: Measure time from application request to response
- **Throughput**: Track messages per second
- **Error Rates**: Monitor failed operations and recovery times
- **Connection Health**: Track connection stability and reconnection frequency
