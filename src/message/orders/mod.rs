/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 22/7/25
******************************************************************************/

//! Order Management FIX Messages Module

use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

pub mod cancel_reject;
pub mod cancel_replace_request;
pub mod cancel_request;
pub mod execution_report;
pub mod mass_cancel;
pub mod mass_status;
pub mod new_order;

pub use cancel_reject::*;
pub use cancel_replace_request::*;
pub use cancel_request::*;
pub use execution_report::*;
pub use mass_cancel::*;
pub use mass_status::*;
pub use new_order::*;

/// Order side enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    /// Buy order
    Buy,
    /// Sell order
    Sell,
}

impl From<OrderSide> for char {
    fn from(side: OrderSide) -> Self {
        match side {
            OrderSide::Buy => '1',
            OrderSide::Sell => '2',
        }
    }
}

impl TryFrom<char> for OrderSide {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '1' => Ok(OrderSide::Buy),
            '2' => Ok(OrderSide::Sell),
            _ => Err(format!("Invalid OrderSide: {value}")),
        }
    }
}

/// Order type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    /// Market order
    Market,
    /// Limit order
    Limit,
    /// Market with left over as limit (market limit)
    MarketLimit,
    /// Stop limit (trailing stop)
    StopLimit,
    /// Market if touched (stop limit with StopPx)
    MarketIfTouched,
    /// Stop limit on bid or offer (stop market with StopPx)
    StopLimitOnBidOffer,
}

impl From<OrderType> for char {
    fn from(order_type: OrderType) -> Self {
        match order_type {
            OrderType::Market => '1',
            OrderType::Limit => '2',
            OrderType::MarketLimit => 'K',
            OrderType::StopLimit => '4',
            OrderType::MarketIfTouched => 'J',
            OrderType::StopLimitOnBidOffer => 'S',
        }
    }
}

impl TryFrom<char> for OrderType {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '1' => Ok(OrderType::Market),
            '2' => Ok(OrderType::Limit),
            'K' => Ok(OrderType::MarketLimit),
            '4' => Ok(OrderType::StopLimit),
            'J' => Ok(OrderType::MarketIfTouched),
            'S' => Ok(OrderType::StopLimitOnBidOffer),
            _ => Err(format!("Invalid OrderType: {value}")),
        }
    }
}

/// Time in force enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeInForce {
    /// Good Till Day
    GoodTillDay,
    /// Good Till Cancelled
    GoodTillCancelled,
    /// Immediate or Cancel
    ImmediateOrCancel,
    /// Fill or Kill
    FillOrKill,
}

impl From<TimeInForce> for char {
    fn from(tif: TimeInForce) -> Self {
        match tif {
            TimeInForce::GoodTillDay => '0',
            TimeInForce::GoodTillCancelled => '1',
            TimeInForce::ImmediateOrCancel => '3',
            TimeInForce::FillOrKill => '4',
        }
    }
}

impl TryFrom<char> for TimeInForce {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(TimeInForce::GoodTillDay),
            '1' => Ok(TimeInForce::GoodTillCancelled),
            '3' => Ok(TimeInForce::ImmediateOrCancel),
            '4' => Ok(TimeInForce::FillOrKill),
            _ => Err(format!("Invalid TimeInForce: {value}")),
        }
    }
}

/// Order status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    /// New order
    New,
    /// Partially filled
    PartiallyFilled,
    /// Filled
    Filled,
    /// Cancelled
    Cancelled,
    /// Pending cancel
    PendingCancel,
    /// Rejected
    Rejected,
}

impl From<OrderStatus> for char {
    fn from(status: OrderStatus) -> Self {
        match status {
            OrderStatus::New => '0',
            OrderStatus::PartiallyFilled => '1',
            OrderStatus::Filled => '2',
            OrderStatus::Cancelled => '4',
            OrderStatus::PendingCancel => '6',
            OrderStatus::Rejected => '8',
        }
    }
}

impl TryFrom<char> for OrderStatus {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(OrderStatus::New),
            '1' => Ok(OrderStatus::PartiallyFilled),
            '2' => Ok(OrderStatus::Filled),
            '4' => Ok(OrderStatus::Cancelled),
            '6' => Ok(OrderStatus::PendingCancel),
            '8' => Ok(OrderStatus::Rejected),
            _ => Err(format!("Invalid OrderStatus: {value}")),
        }
    }
}

/// Order rejection reason enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderRejectReason {
    /// No reject (accepted)
    NoReject,
    /// Unknown symbol
    UnknownSymbol,
    /// Exchange closed
    ExchangeClosed,
    /// Order exceeds limit
    OrderExceedsLimit,
    /// Too late to enter
    TooLateToEnter,
    /// Unknown order
    UnknownOrder,
    /// Duplicate order
    DuplicateOrder,
    /// Duplicate of verbally communicated order
    DuplicateVerbalOrder,
    /// Stale order
    StaleOrder,
    /// Trade along required
    TradeAlongRequired,
    /// Invalid investor ID
    InvalidInvestorId,
    /// Unsupported order characteristic
    UnsupportedOrderCharacteristic,
    /// Surveillance option
    SurveillanceOption,
    /// Incorrect quantity
    IncorrectQuantity,
    /// Incorrect allocated quantity
    IncorrectAllocatedQuantity,
    /// Unknown account
    UnknownAccount,
    /// Price exceeds current price band
    PriceExceedsPriceBand,
    /// Invalid price increment
    InvalidPriceIncrement,
    /// Other
    Other,
}

impl From<OrderRejectReason> for i32 {
    fn from(reason: OrderRejectReason) -> Self {
        match reason {
            OrderRejectReason::NoReject => 0,
            OrderRejectReason::UnknownSymbol => 1,
            OrderRejectReason::ExchangeClosed => 2,
            OrderRejectReason::OrderExceedsLimit => 3,
            OrderRejectReason::TooLateToEnter => 4,
            OrderRejectReason::UnknownOrder => 5,
            OrderRejectReason::DuplicateOrder => 6,
            OrderRejectReason::DuplicateVerbalOrder => 7,
            OrderRejectReason::StaleOrder => 8,
            OrderRejectReason::TradeAlongRequired => 9,
            OrderRejectReason::InvalidInvestorId => 10,
            OrderRejectReason::UnsupportedOrderCharacteristic => 11,
            OrderRejectReason::SurveillanceOption => 12,
            OrderRejectReason::IncorrectQuantity => 13,
            OrderRejectReason::IncorrectAllocatedQuantity => 14,
            OrderRejectReason::UnknownAccount => 15,
            OrderRejectReason::PriceExceedsPriceBand => 16,
            OrderRejectReason::InvalidPriceIncrement => 18,
            OrderRejectReason::Other => 99,
        }
    }
}

impl TryFrom<i32> for OrderRejectReason {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OrderRejectReason::NoReject),
            1 => Ok(OrderRejectReason::UnknownSymbol),
            2 => Ok(OrderRejectReason::ExchangeClosed),
            3 => Ok(OrderRejectReason::OrderExceedsLimit),
            4 => Ok(OrderRejectReason::TooLateToEnter),
            5 => Ok(OrderRejectReason::UnknownOrder),
            6 => Ok(OrderRejectReason::DuplicateOrder),
            7 => Ok(OrderRejectReason::DuplicateVerbalOrder),
            8 => Ok(OrderRejectReason::StaleOrder),
            9 => Ok(OrderRejectReason::TradeAlongRequired),
            10 => Ok(OrderRejectReason::InvalidInvestorId),
            11 => Ok(OrderRejectReason::UnsupportedOrderCharacteristic),
            12 => Ok(OrderRejectReason::SurveillanceOption),
            13 => Ok(OrderRejectReason::IncorrectQuantity),
            14 => Ok(OrderRejectReason::IncorrectAllocatedQuantity),
            15 => Ok(OrderRejectReason::UnknownAccount),
            16 => Ok(OrderRejectReason::PriceExceedsPriceBand),
            18 => Ok(OrderRejectReason::InvalidPriceIncrement),
            99 => Ok(OrderRejectReason::Other),
            _ => Err(format!("Invalid OrderRejectReason: {value}")),
        }
    }
}

/// Mass cancel request type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MassCancelRequestType {
    /// Cancel orders by symbol
    BySymbol,
    /// Cancel orders by security type
    BySecurityType,
    /// Cancel all orders
    AllOrders,
    /// Cancel orders by Deribit label
    ByDeribitLabel,
}

impl From<MassCancelRequestType> for i32 {
    fn from(request_type: MassCancelRequestType) -> Self {
        match request_type {
            MassCancelRequestType::BySymbol => 1,
            MassCancelRequestType::BySecurityType => 5,
            MassCancelRequestType::AllOrders => 7,
            MassCancelRequestType::ByDeribitLabel => 10,
        }
    }
}

impl TryFrom<i32> for MassCancelRequestType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(MassCancelRequestType::BySymbol),
            5 => Ok(MassCancelRequestType::BySecurityType),
            7 => Ok(MassCancelRequestType::AllOrders),
            10 => Ok(MassCancelRequestType::ByDeribitLabel),
            _ => Err(format!("Invalid MassCancelRequestType: {value}")),
        }
    }
}

/// Mass status request type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MassStatusRequestType {
    /// Status of a specific order
    SpecificOrder,
    /// Status of all orders
    AllOrders,
}

impl From<MassStatusRequestType> for i32 {
    fn from(request_type: MassStatusRequestType) -> Self {
        match request_type {
            MassStatusRequestType::SpecificOrder => 1,
            MassStatusRequestType::AllOrders => 7,
        }
    }
}

impl TryFrom<i32> for MassStatusRequestType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(MassStatusRequestType::SpecificOrder),
            7 => Ok(MassStatusRequestType::AllOrders),
            _ => Err(format!("Invalid MassStatusRequestType: {value}")),
        }
    }
}

/// Mass status request ID type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MassStatusRequestIdType {
    /// Original client order ID
    OrigClOrdId,
    /// Client order ID
    ClOrdId,
    /// Deribit label
    DeribitLabel,
}

impl From<MassStatusRequestIdType> for i32 {
    fn from(id_type: MassStatusRequestIdType) -> Self {
        match id_type {
            MassStatusRequestIdType::OrigClOrdId => 0,
            MassStatusRequestIdType::ClOrdId => 1,
            MassStatusRequestIdType::DeribitLabel => 2,
        }
    }
}

impl TryFrom<i32> for MassStatusRequestIdType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MassStatusRequestIdType::OrigClOrdId),
            1 => Ok(MassStatusRequestIdType::ClOrdId),
            2 => Ok(MassStatusRequestIdType::DeribitLabel),
            _ => Err(format!("Invalid MassStatusRequestIdType: {value}")),
        }
    }
}

/// Quantity type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuantityType {
    /// Units (USD for perpetual/inverse futures, base currency for linear futures, contracts for options)
    Units,
    /// Contracts
    Contracts,
}

impl From<QuantityType> for i32 {
    fn from(qty_type: QuantityType) -> Self {
        match qty_type {
            QuantityType::Units => 0,
            QuantityType::Contracts => 1,
        }
    }
}

impl TryFrom<i32> for QuantityType {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(QuantityType::Units),
            1 => Ok(QuantityType::Contracts),
            _ => Err(format!("Invalid QuantityType: {value}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_side_conversion() {
        assert_eq!(char::from(OrderSide::Buy), '1');
        assert_eq!(char::from(OrderSide::Sell), '2');

        assert_eq!(OrderSide::try_from('1').unwrap(), OrderSide::Buy);
        assert_eq!(OrderSide::try_from('2').unwrap(), OrderSide::Sell);
        assert!(OrderSide::try_from('3').is_err());
    }

    #[test]
    fn test_order_type_conversion() {
        assert_eq!(char::from(OrderType::Market), '1');
        assert_eq!(char::from(OrderType::Limit), '2');
        assert_eq!(char::from(OrderType::MarketLimit), 'K');

        assert_eq!(OrderType::try_from('1').unwrap(), OrderType::Market);
        assert_eq!(OrderType::try_from('2').unwrap(), OrderType::Limit);
        assert_eq!(OrderType::try_from('K').unwrap(), OrderType::MarketLimit);
        assert!(OrderType::try_from('Z').is_err());
    }

    #[test]
    fn test_time_in_force_conversion() {
        assert_eq!(char::from(TimeInForce::GoodTillDay), '0');
        assert_eq!(char::from(TimeInForce::GoodTillCancelled), '1');
        assert_eq!(char::from(TimeInForce::ImmediateOrCancel), '3');
        assert_eq!(char::from(TimeInForce::FillOrKill), '4');

        assert_eq!(
            TimeInForce::try_from('0').unwrap(),
            TimeInForce::GoodTillDay
        );
        assert_eq!(
            TimeInForce::try_from('1').unwrap(),
            TimeInForce::GoodTillCancelled
        );
        assert_eq!(
            TimeInForce::try_from('3').unwrap(),
            TimeInForce::ImmediateOrCancel
        );
        assert_eq!(TimeInForce::try_from('4').unwrap(), TimeInForce::FillOrKill);
        assert!(TimeInForce::try_from('5').is_err());
    }

    #[test]
    fn test_order_status_conversion() {
        assert_eq!(char::from(OrderStatus::New), '0');
        assert_eq!(char::from(OrderStatus::PartiallyFilled), '1');
        assert_eq!(char::from(OrderStatus::Filled), '2');
        assert_eq!(char::from(OrderStatus::Cancelled), '4');

        assert_eq!(OrderStatus::try_from('0').unwrap(), OrderStatus::New);
        assert_eq!(
            OrderStatus::try_from('1').unwrap(),
            OrderStatus::PartiallyFilled
        );
        assert_eq!(OrderStatus::try_from('2').unwrap(), OrderStatus::Filled);
        assert_eq!(OrderStatus::try_from('4').unwrap(), OrderStatus::Cancelled);
        assert!(OrderStatus::try_from('9').is_err());
    }

    #[test]
    fn test_order_reject_reason_conversion() {
        assert_eq!(i32::from(OrderRejectReason::NoReject), 0);
        assert_eq!(i32::from(OrderRejectReason::UnknownSymbol), 1);
        assert_eq!(i32::from(OrderRejectReason::Other), 99);

        assert_eq!(
            OrderRejectReason::try_from(0).unwrap(),
            OrderRejectReason::NoReject
        );
        assert_eq!(
            OrderRejectReason::try_from(1).unwrap(),
            OrderRejectReason::UnknownSymbol
        );
        assert_eq!(
            OrderRejectReason::try_from(99).unwrap(),
            OrderRejectReason::Other
        );
        assert!(OrderRejectReason::try_from(100).is_err());
    }

    #[test]
    fn test_mass_cancel_request_type_conversion() {
        assert_eq!(i32::from(MassCancelRequestType::BySymbol), 1);
        assert_eq!(i32::from(MassCancelRequestType::BySecurityType), 5);
        assert_eq!(i32::from(MassCancelRequestType::AllOrders), 7);
        assert_eq!(i32::from(MassCancelRequestType::ByDeribitLabel), 10);

        assert_eq!(
            MassCancelRequestType::try_from(1).unwrap(),
            MassCancelRequestType::BySymbol
        );
        assert_eq!(
            MassCancelRequestType::try_from(5).unwrap(),
            MassCancelRequestType::BySecurityType
        );
        assert_eq!(
            MassCancelRequestType::try_from(7).unwrap(),
            MassCancelRequestType::AllOrders
        );
        assert_eq!(
            MassCancelRequestType::try_from(10).unwrap(),
            MassCancelRequestType::ByDeribitLabel
        );
        assert!(MassCancelRequestType::try_from(99).is_err());
    }

    #[test]
    fn test_quantity_type_conversion() {
        assert_eq!(i32::from(QuantityType::Units), 0);
        assert_eq!(i32::from(QuantityType::Contracts), 1);

        assert_eq!(QuantityType::try_from(0).unwrap(), QuantityType::Units);
        assert_eq!(QuantityType::try_from(1).unwrap(), QuantityType::Contracts);
        assert!(QuantityType::try_from(2).is_err());
    }

    #[test]
    fn test_mass_status_request_type_conversion() {
        assert_eq!(i32::from(MassStatusRequestType::SpecificOrder), 1);
        assert_eq!(i32::from(MassStatusRequestType::AllOrders), 7);

        assert_eq!(
            MassStatusRequestType::try_from(1).unwrap(),
            MassStatusRequestType::SpecificOrder
        );
        assert_eq!(
            MassStatusRequestType::try_from(7).unwrap(),
            MassStatusRequestType::AllOrders
        );
        assert!(MassStatusRequestType::try_from(99).is_err());
    }

    #[test]
    fn test_mass_status_request_id_type_conversion() {
        assert_eq!(i32::from(MassStatusRequestIdType::OrigClOrdId), 0);
        assert_eq!(i32::from(MassStatusRequestIdType::ClOrdId), 1);
        assert_eq!(i32::from(MassStatusRequestIdType::DeribitLabel), 2);

        assert_eq!(
            MassStatusRequestIdType::try_from(0).unwrap(),
            MassStatusRequestIdType::OrigClOrdId
        );
        assert_eq!(
            MassStatusRequestIdType::try_from(1).unwrap(),
            MassStatusRequestIdType::ClOrdId
        );
        assert_eq!(
            MassStatusRequestIdType::try_from(2).unwrap(),
            MassStatusRequestIdType::DeribitLabel
        );
        assert!(MassStatusRequestIdType::try_from(99).is_err());
    }
}
