/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/
use crate::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

/// Time in force enumeration
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeInForce {
    Day,
    GoodTillCancel,
    ImmediateOrCancel,
    FillOrKill,
}

/// Order side enumeration
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

/// Order type enumeration
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
}

/// New order request structure
#[derive(Clone, Serialize, Deserialize)]
pub struct NewOrderRequest {
    pub symbol: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub quantity: f64,
    pub price: Option<f64>,
    pub time_in_force: TimeInForce,
    pub client_order_id: Option<String>,
}

/// Order status enumeration
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

impl_json_debug_pretty!(
    TimeInForce,
    OrderSide,
    OrderType,
    NewOrderRequest,
    OrderStatus
);
impl_json_display!(
    TimeInForce,
    OrderSide,
    OrderType,
    NewOrderRequest,
    OrderStatus
);
