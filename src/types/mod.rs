//! Type definitions for the Deribit FIX framework


use serde::{Deserialize, Serialize};

/// FIX message type identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MsgType {
    /// Heartbeat (0)
    Heartbeat,
    /// Test Request (1)
    TestRequest,
    /// Resend Request (2)
    ResendRequest,
    /// Reject (3)
    Reject,
    /// Sequence Reset (4)
    SequenceReset,
    /// Logout (5)
    Logout,
    /// Execution Report (8)
    ExecutionReport,
    /// Order Cancel Reject (9)
    OrderCancelReject,
    /// Logon (A)
    Logon,
    /// New Order Single (D)
    NewOrderSingle,
    /// Order Cancel Request (F)
    OrderCancelRequest,
    /// Order Cancel/Replace Request (G)
    OrderCancelReplaceRequest,
    /// Quote Request (R)
    QuoteRequest,
    /// Market Data Request (V)
    MarketDataRequest,
    /// Market Data Snapshot/Full Refresh (W)
    MarketDataSnapshotFullRefresh,
    /// Market Data Incremental Refresh (X)
    MarketDataIncrementalRefresh,
    /// Market Data Request Reject (Y)
    MarketDataRequestReject,
    /// Quote Cancel (Z)
    QuoteCancel,
    /// Mass Quote Acknowledgement (b)
    MassQuoteAcknowledgement,
    /// Security Definition Request (c)
    SecurityDefinitionRequest,
    /// Security Definition (d)
    SecurityDefinition,
    /// Security Status Request (e)
    SecurityStatusRequest,
    /// Security Status (f)
    SecurityStatus,
    /// Mass Quote (i)
    MassQuote,
    /// Order Mass Cancel Request (q)
    OrderMassCancelRequest,
    /// Order Mass Cancel Report (r)
    OrderMassCancelReport,
    /// Security List Request (x)
    SecurityListRequest,
    /// Security List (y)
    SecurityList,
    /// Quote Status Report (AI)
    QuoteStatusReport,
    /// RFQ Request (AH)
    RfqRequest,
    /// Quote Request Reject (AG)
    QuoteRequestReject,
    /// Trade Capture Report Request (AD)
    TradeCaptureReportRequest,
    /// Trade Capture Report (AE)
    TradeCaptureReport,
    /// Trade Capture Report Request Ack (AQ)
    TradeCaptureReportRequestAck,
    /// Order Mass Status Request (AF)
    OrderMassStatusRequest,
    /// Request For Positions (AN)
    RequestForPositions,
    /// Position Report (AP)
    PositionReport,
    /// User Request (BE)
    UserRequest,
    /// User Response (BF)
    UserResponse,
    /// MM Protection Limits (MM)
    MmProtectionLimits,
    /// MM Protection Limits Result/Reject (MR)
    MmProtectionLimitsResult,
    /// MM Protection Reset (MZ)
    MmProtectionReset,
}

impl MsgType {
    /// Convert to FIX message type string
    pub fn as_str(&self) -> &'static str {
        match self {
            MsgType::Heartbeat => "0",
            MsgType::TestRequest => "1",
            MsgType::ResendRequest => "2",
            MsgType::Reject => "3",
            MsgType::SequenceReset => "4",
            MsgType::Logout => "5",
            MsgType::ExecutionReport => "8",
            MsgType::OrderCancelReject => "9",
            MsgType::Logon => "A",
            MsgType::NewOrderSingle => "D",
            MsgType::OrderCancelRequest => "F",
            MsgType::OrderCancelReplaceRequest => "G",
            MsgType::QuoteRequest => "R",
            MsgType::MarketDataRequest => "V",
            MsgType::MarketDataSnapshotFullRefresh => "W",
            MsgType::MarketDataIncrementalRefresh => "X",
            MsgType::MarketDataRequestReject => "Y",
            MsgType::QuoteCancel => "Z",
            MsgType::MassQuoteAcknowledgement => "b",
            MsgType::SecurityDefinitionRequest => "c",
            MsgType::SecurityDefinition => "d",
            MsgType::SecurityStatusRequest => "e",
            MsgType::SecurityStatus => "f",
            MsgType::MassQuote => "i",
            MsgType::OrderMassCancelRequest => "q",
            MsgType::OrderMassCancelReport => "r",
            MsgType::SecurityListRequest => "x",
            MsgType::SecurityList => "y",
            MsgType::QuoteStatusReport => "AI",
            MsgType::RfqRequest => "AH",
            MsgType::QuoteRequestReject => "AG",
            MsgType::TradeCaptureReportRequest => "AD",
            MsgType::TradeCaptureReport => "AE",
            MsgType::TradeCaptureReportRequestAck => "AQ",
            MsgType::OrderMassStatusRequest => "AF",
            MsgType::RequestForPositions => "AN",
            MsgType::PositionReport => "AP",
            MsgType::UserRequest => "BE",
            MsgType::UserResponse => "BF",
            MsgType::MmProtectionLimits => "MM",
            MsgType::MmProtectionLimitsResult => "MR",
            MsgType::MmProtectionReset => "MZ",
        }
    }

    /// Parse from FIX message type string
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "0" => Some(MsgType::Heartbeat),
            "1" => Some(MsgType::TestRequest),
            "2" => Some(MsgType::ResendRequest),
            "3" => Some(MsgType::Reject),
            "4" => Some(MsgType::SequenceReset),
            "5" => Some(MsgType::Logout),
            "8" => Some(MsgType::ExecutionReport),
            "9" => Some(MsgType::OrderCancelReject),
            "A" => Some(MsgType::Logon),
            "D" => Some(MsgType::NewOrderSingle),
            "F" => Some(MsgType::OrderCancelRequest),
            "G" => Some(MsgType::OrderCancelReplaceRequest),
            "R" => Some(MsgType::QuoteRequest),
            "V" => Some(MsgType::MarketDataRequest),
            "W" => Some(MsgType::MarketDataSnapshotFullRefresh),
            "X" => Some(MsgType::MarketDataIncrementalRefresh),
            "Y" => Some(MsgType::MarketDataRequestReject),
            "Z" => Some(MsgType::QuoteCancel),
            "b" => Some(MsgType::MassQuoteAcknowledgement),
            "c" => Some(MsgType::SecurityDefinitionRequest),
            "d" => Some(MsgType::SecurityDefinition),
            "e" => Some(MsgType::SecurityStatusRequest),
            "f" => Some(MsgType::SecurityStatus),
            "i" => Some(MsgType::MassQuote),
            "q" => Some(MsgType::OrderMassCancelRequest),
            "r" => Some(MsgType::OrderMassCancelReport),
            "x" => Some(MsgType::SecurityListRequest),
            "y" => Some(MsgType::SecurityList),
            "AI" => Some(MsgType::QuoteStatusReport),
            "AH" => Some(MsgType::RfqRequest),
            "AG" => Some(MsgType::QuoteRequestReject),
            "AD" => Some(MsgType::TradeCaptureReportRequest),
            "AE" => Some(MsgType::TradeCaptureReport),
            "AQ" => Some(MsgType::TradeCaptureReportRequestAck),
            "AF" => Some(MsgType::OrderMassStatusRequest),
            "AN" => Some(MsgType::RequestForPositions),
            "AP" => Some(MsgType::PositionReport),
            "BE" => Some(MsgType::UserRequest),
            "BF" => Some(MsgType::UserResponse),
            "MM" => Some(MsgType::MmProtectionLimits),
            "MR" => Some(MsgType::MmProtectionLimitsResult),
            "MZ" => Some(MsgType::MmProtectionReset),
            _ => None,
        }
    }
}

/// FIX field tags commonly used in Deribit
pub mod tags {
    /// BeginString (8)
    pub const BEGIN_STRING: u32 = 8;
    /// BodyLength (9)
    pub const BODY_LENGTH: u32 = 9;
    /// CheckSum (10)
    pub const CHECKSUM: u32 = 10;
    /// ClOrdID (11)
    pub const CL_ORD_ID: u32 = 11;
    /// MsgSeqNum (34)
    pub const MSG_SEQ_NUM: u32 = 34;
    /// MsgType (35)
    pub const MSG_TYPE: u32 = 35;
    /// OrderQty (38)
    pub const ORDER_QTY: u32 = 38;
    /// OrdType (40)
    pub const ORD_TYPE: u32 = 40;
    /// OrigClOrdID (41)
    pub const ORIG_CL_ORD_ID: u32 = 41;
    /// Price (44)
    pub const PRICE: u32 = 44;
    /// SenderCompID (49)
    pub const SENDER_COMP_ID: u32 = 49;
    /// SendingTime (52)
    pub const SENDING_TIME: u32 = 52;
    /// Side (54)
    pub const SIDE: u32 = 54;
    /// Symbol (55)
    pub const SYMBOL: u32 = 55;
    /// TargetCompID (56)
    pub const TARGET_COMP_ID: u32 = 56;
    /// TimeInForce (59)
    pub const TIME_IN_FORCE: u32 = 59;
    /// RawDataLength (95)
    pub const RAW_DATA_LENGTH: u32 = 95;
    /// RawData (96)
    pub const RAW_DATA: u32 = 96;
    /// HeartBtInt (108)
    pub const HEART_BT_INT: u32 = 108;
    /// TestReqID (112)
    pub const TEST_REQ_ID: u32 = 112;
    /// NoRelatedSym (146)
    pub const NO_RELATED_SYM: u32 = 146;
    /// MDReqID (262)
    pub const MD_REQ_ID: u32 = 262;
    /// SubscriptionRequestType (263)
    pub const SUBSCRIPTION_REQUEST_TYPE: u32 = 263;
    /// MarketDepth (264)
    pub const MARKET_DEPTH: u32 = 264;
    /// Username (553)
    pub const USERNAME: u32 = 553;
    /// Password (554)
    pub const PASSWORD: u32 = 554;
    /// PosReqID (710)
    pub const POS_REQ_ID: u32 = 710;
    /// PosReqType (724)
    pub const POS_REQ_TYPE: u32 = 724;
    
    // Deribit custom tags
    /// CancelOnDisconnect (9001)
    pub const CANCEL_ON_DISCONNECT: u32 = 9001;
    /// DeribitAppId (9004)
    pub const DERIBIT_APP_ID: u32 = 9004;
    /// DeribitAppSig (9005)
    pub const DERIBIT_APP_SIG: u32 = 9005;
}

/// Order status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    DoneForDay,
    Canceled,
    Replaced,
    PendingCancel,
    Stopped,
    Rejected,
    Suspended,
    PendingNew,
    Calculated,
    Expired,
    AcceptedForBidding,
    PendingReplace,
}

/// Execution type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecType {
    New,
    DoneForDay,
    Canceled,
    Replaced,
    PendingCancel,
    Stopped,
    Rejected,
    Suspended,
    PendingNew,
    Calculated,
    Expired,
    Restated,
    PendingReplace,
    Trade,
    TradeCorrect,
    TradeCancel,
    OrderStatus,
}

/// Market data entry type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MDEntryType {
    Bid,
    Offer,
    Trade,
    IndexValue,
    OpeningPrice,
    ClosingPrice,
    SettlementPrice,
    TradingSessionHighPrice,
    TradingSessionLowPrice,
    TradingSessionVWAPPrice,
    Imbalance,
    TradeVolume,
    OpenInterest,
}

/// Security type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityType {
    Future,
    Option,
    Spot,
    Index,
}

/// Currency enumeration for Deribit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Currency {
    BTC,
    ETH,
    USD,
    USDC,
}

impl Currency {
    pub fn as_str(&self) -> &'static str {
        match self {
            Currency::BTC => "BTC",
            Currency::ETH => "ETH",
            Currency::USD => "USD",
            Currency::USDC => "USDC",
        }
    }
}

/// Instrument kind for Deribit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstrumentKind {
    Future,
    Option,
    Spot,
    FutureCombo,
    OptionCombo,
}

impl InstrumentKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            InstrumentKind::Future => "future",
            InstrumentKind::Option => "option",
            InstrumentKind::Spot => "spot",
            InstrumentKind::FutureCombo => "future_combo",
            InstrumentKind::OptionCombo => "option_combo",
        }
    }
}
