/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/
use deribit_base::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// FIX message type identifiers
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MsgType {
    /// Heartbeat (0)
    Heartbeat,
    /// Test Request (1)
    TestRequest,
    /// Resend Request (2)
    ResendRequest,
    /// Reject (3)
    Reject,
    /// Business Message Reject (j)
    BusinessMessageReject,
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
            MsgType::BusinessMessageReject => "j",
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
}

/// Error type for parsing MsgType from string
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseMsgTypeError(pub String);

impl std::fmt::Display for ParseMsgTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown message type: {}", self.0)
    }
}

impl std::error::Error for ParseMsgTypeError {}

impl FromStr for MsgType {
    type Err = ParseMsgTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(MsgType::Heartbeat),
            "1" => Ok(MsgType::TestRequest),
            "2" => Ok(MsgType::ResendRequest),
            "3" => Ok(MsgType::Reject),
            "j" => Ok(MsgType::BusinessMessageReject),
            "4" => Ok(MsgType::SequenceReset),
            "5" => Ok(MsgType::Logout),
            "8" => Ok(MsgType::ExecutionReport),
            "9" => Ok(MsgType::OrderCancelReject),
            "A" => Ok(MsgType::Logon),
            "D" => Ok(MsgType::NewOrderSingle),
            "F" => Ok(MsgType::OrderCancelRequest),
            "G" => Ok(MsgType::OrderCancelReplaceRequest),
            "R" => Ok(MsgType::QuoteRequest),
            "V" => Ok(MsgType::MarketDataRequest),
            "W" => Ok(MsgType::MarketDataSnapshotFullRefresh),
            "X" => Ok(MsgType::MarketDataIncrementalRefresh),
            "Y" => Ok(MsgType::MarketDataRequestReject),
            "Z" => Ok(MsgType::QuoteCancel),
            "b" => Ok(MsgType::MassQuoteAcknowledgement),
            "c" => Ok(MsgType::SecurityDefinitionRequest),
            "d" => Ok(MsgType::SecurityDefinition),
            "e" => Ok(MsgType::SecurityStatusRequest),
            "f" => Ok(MsgType::SecurityStatus),
            "i" => Ok(MsgType::MassQuote),
            "q" => Ok(MsgType::OrderMassCancelRequest),
            "r" => Ok(MsgType::OrderMassCancelReport),
            "x" => Ok(MsgType::SecurityListRequest),
            "y" => Ok(MsgType::SecurityList),
            "AI" => Ok(MsgType::QuoteStatusReport),
            "AH" => Ok(MsgType::RfqRequest),
            "AG" => Ok(MsgType::QuoteRequestReject),
            "AD" => Ok(MsgType::TradeCaptureReportRequest),
            "AE" => Ok(MsgType::TradeCaptureReport),
            "AQ" => Ok(MsgType::TradeCaptureReportRequestAck),
            "AF" => Ok(MsgType::OrderMassStatusRequest),
            "AN" => Ok(MsgType::RequestForPositions),
            "AP" => Ok(MsgType::PositionReport),
            "BE" => Ok(MsgType::UserRequest),
            "BF" => Ok(MsgType::UserResponse),
            "MM" => Ok(MsgType::MmProtectionLimits),
            "MR" => Ok(MsgType::MmProtectionLimitsResult),
            "MZ" => Ok(MsgType::MmProtectionReset),
            _ => Err(ParseMsgTypeError(s.to_string())),
        }
    }
}

/// Execution type enumeration
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecType {
    /// New order
    New,
    /// Order done for day
    DoneForDay,
    /// Order canceled
    Canceled,
    /// Order replaced
    Replaced,
    /// Pending cancel
    PendingCancel,
    /// Order stopped
    Stopped,
    /// Order rejected
    Rejected,
    /// Order suspended
    Suspended,
    /// Pending new order
    PendingNew,
    /// Calculated
    Calculated,
    /// Order expired
    Expired,
    /// Order restated
    Restated,
    /// Pending replace
    PendingReplace,
    /// Trade execution
    Trade,
    /// Trade correction
    TradeCorrect,
    /// Trade cancellation
    TradeCancel,
    /// Order status update
    OrderStatus,
}

impl From<ExecType> for char {
    fn from(exec_type: ExecType) -> Self {
        match exec_type {
            ExecType::New => '0',
            ExecType::DoneForDay => '3',
            ExecType::Canceled => '4',
            ExecType::Replaced => '5',
            ExecType::PendingCancel => '6',
            ExecType::Stopped => '7',
            ExecType::Rejected => '8',
            ExecType::Suspended => '9',
            ExecType::PendingNew => 'A',
            ExecType::Calculated => 'B',
            ExecType::Expired => 'C',
            ExecType::Restated => 'D',
            ExecType::PendingReplace => 'E',
            ExecType::Trade => 'F',
            ExecType::TradeCorrect => 'G',
            ExecType::TradeCancel => 'H',
            ExecType::OrderStatus => 'I',
        }
    }
}

impl TryFrom<char> for ExecType {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(ExecType::New),
            '3' => Ok(ExecType::DoneForDay),
            '4' => Ok(ExecType::Canceled),
            '5' => Ok(ExecType::Replaced),
            '6' => Ok(ExecType::PendingCancel),
            '7' => Ok(ExecType::Stopped),
            '8' => Ok(ExecType::Rejected),
            '9' => Ok(ExecType::Suspended),
            'A' => Ok(ExecType::PendingNew),
            'B' => Ok(ExecType::Calculated),
            'C' => Ok(ExecType::Expired),
            'D' => Ok(ExecType::Restated),
            'E' => Ok(ExecType::PendingReplace),
            'F' => Ok(ExecType::Trade),
            'G' => Ok(ExecType::TradeCorrect),
            'H' => Ok(ExecType::TradeCancel),
            'I' => Ok(ExecType::OrderStatus),
            _ => Err(format!("Invalid ExecType: {}", value)),
        }
    }
}

/// Market data entry type
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MDEntryType {
    /// Bid price
    Bid,
    /// Offer/ask price
    Offer,
    /// Trade price
    Trade,
    /// Index value
    IndexValue,
    /// Opening price
    OpeningPrice,
    /// Closing price
    ClosingPrice,
    /// Settlement price
    SettlementPrice,
    /// Trading session high price
    TradingSessionHighPrice,
    /// Trading session low price
    TradingSessionLowPrice,
    /// Trading session VWAP price
    TradingSessionVWAPPrice,
    /// Order imbalance
    Imbalance,
    /// Trade volume
    TradeVolume,
    /// Open interest
    OpenInterest,
}

/// Security type enumeration
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityType {
    /// Future contract
    Future,
    /// Option contract
    Option,
    /// Spot trading
    Spot,
    /// Index instrument
    Index,
}

impl_json_debug_pretty!(MsgType, ExecType, MDEntryType, SecurityType);
impl_json_display!(MsgType, ExecType, MDEntryType, SecurityType);
