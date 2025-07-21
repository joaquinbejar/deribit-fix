/******************************************************************************
    Author: Joaquín Béjar García
    Email: jb@taunais.com 
    Date: 21/7/25
 ******************************************************************************/

/// FIX field tags commonly used in Deribit
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