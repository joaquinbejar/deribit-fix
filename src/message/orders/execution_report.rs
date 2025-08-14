/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 10/8/25
******************************************************************************/

//! Execution Report FIX Message Implementation

use super::*;
use crate::error::Result as DeribitFixResult;
use crate::message::builder::MessageBuilder;
use crate::model::types::{ExecType, MsgType};
use chrono::{DateTime, Utc};
use deribit_base::{impl_json_debug_pretty, impl_json_display};
use serde::{Deserialize, Serialize};

/// Execution Report message (MsgType = '8')
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionReport {
    /// Order ID
    pub order_id: String,
    /// Client order ID
    pub cl_ord_id: String,
    /// Original client order ID (for replace/cancel operations)
    pub orig_cl_ord_id: Option<String>,
    /// Execution ID
    pub exec_id: String,
    /// Execution type
    pub exec_type: ExecType,
    /// Order status
    pub ord_status: OrderStatus,
    /// Instrument symbol
    pub symbol: String,
    /// Side of order
    pub side: OrderSide,
    /// Quantity open for further execution
    pub leaves_qty: f64,
    /// Total quantity filled
    pub cum_qty: f64,
    /// Average price of all fills on this order
    pub avg_px: Option<f64>,
    /// Price of this fill
    pub last_px: Option<f64>,
    /// Quantity of shares bought/sold on this fill
    pub last_qty: Option<f64>,
    /// Order quantity
    pub order_qty: f64,
    /// Price
    pub price: Option<f64>,
    /// Transaction time
    pub transact_time: DateTime<Utc>,
    /// Text
    pub text: Option<String>,
    /// Order reject reason (if applicable)
    pub ord_rej_reason: Option<OrderRejectReason>,
    /// Custom label
    pub deribit_label: Option<String>,
    /// Secondary execution ID
    pub secondary_exec_id: Option<String>,
    /// Order type
    pub ord_type: Option<OrderType>,
    /// Commission (deprecated, always 0)
    pub commission: Option<f64>,
    /// Security exchange
    pub security_exchange: Option<String>,
    /// Quantity type
    pub qty_type: Option<QuantityType>,
    /// Contract multiplier
    pub contract_multiplier: Option<f64>,
    /// Display quantity
    pub display_qty: Option<f64>,
    /// Advanced order type for options
    pub deribit_adv_order_type: Option<char>,
    /// Volatility for implied volatility orders
    pub volatility: Option<f64>,
    /// Fixed USD price for USD orders
    pub pegged_price: Option<f64>,
    /// Trade match ID
    pub trd_match_id: Option<String>,
    /// Market Maker Protection flag
    pub deribit_mm_protection: Option<bool>,
    /// MMP Group
    pub mmp_group: Option<String>,
    /// Quote Set ID (for orders from Mass Quote)
    pub quote_set_id: Option<String>,
    /// Quote ID (for orders from Mass Quote)
    pub quote_id: Option<String>,
    /// Quote Entry ID (for orders from Mass Quote)
    pub quote_entry_id: Option<String>,
    /// Execution instruction
    pub exec_inst: Option<String>,
    /// Stop price
    pub stop_px: Option<f64>,
    /// Condition trigger method
    pub condition_trigger_method: Option<i32>,
    /// Last liquidity indicator (1=Added Liquidity, 2=Removed Liquidity)
    pub last_liquidity_ind: Option<i32>,
}

impl ExecutionReport {
    /// Create a new execution report for a new order
    #[allow(clippy::too_many_arguments)]
    pub fn new_order(
        order_id: String,
        cl_ord_id: String,
        exec_id: String,
        symbol: String,
        side: OrderSide,
        order_qty: f64,
        leaves_qty: f64,
        price: Option<f64>,
    ) -> Self {
        Self {
            order_id,
            cl_ord_id,
            orig_cl_ord_id: None,
            exec_id,
            exec_type: ExecType::New,
            ord_status: OrderStatus::New,
            symbol,
            side,
            leaves_qty,
            cum_qty: 0.0,
            avg_px: None,
            last_px: None,
            last_qty: None,
            order_qty,
            price,
            transact_time: Utc::now(),
            text: None,
            ord_rej_reason: None,
            deribit_label: None,
            secondary_exec_id: None,
            ord_type: None,
            commission: None,
            security_exchange: None,
            qty_type: None,
            contract_multiplier: None,
            display_qty: None,
            deribit_adv_order_type: None,
            volatility: None,
            pegged_price: None,
            trd_match_id: None,
            deribit_mm_protection: None,
            mmp_group: None,
            quote_set_id: None,
            quote_id: None,
            quote_entry_id: None,
            exec_inst: None,
            stop_px: None,
            condition_trigger_method: None,
            last_liquidity_ind: None,
        }
    }

    /// Create a fill execution report
    #[allow(clippy::too_many_arguments)]
    pub fn fill(
        order_id: String,
        cl_ord_id: String,
        exec_id: String,
        symbol: String,
        side: OrderSide,
        order_qty: f64,
        leaves_qty: f64,
        cum_qty: f64,
        last_px: f64,
        last_qty: f64,
        avg_px: f64,
    ) -> Self {
        Self {
            order_id,
            cl_ord_id,
            orig_cl_ord_id: None,
            exec_id,
            exec_type: ExecType::Trade,
            ord_status: if leaves_qty > 0.0 {
                OrderStatus::PartiallyFilled
            } else {
                OrderStatus::Filled
            },
            symbol,
            side,
            leaves_qty,
            cum_qty,
            avg_px: Some(avg_px),
            last_px: Some(last_px),
            last_qty: Some(last_qty),
            order_qty,
            price: Some(last_px),
            transact_time: Utc::now(),
            text: None,
            ord_rej_reason: None,
            deribit_label: None,
            secondary_exec_id: None,
            ord_type: None,
            commission: None,
            security_exchange: None,
            qty_type: None,
            contract_multiplier: None,
            display_qty: None,
            deribit_adv_order_type: None,
            volatility: None,
            pegged_price: None,
            trd_match_id: None,
            deribit_mm_protection: None,
            mmp_group: None,
            quote_set_id: None,
            quote_id: None,
            quote_entry_id: None,
            exec_inst: None,
            stop_px: None,
            condition_trigger_method: None,
            last_liquidity_ind: None,
        }
    }

    /// Create a rejection execution report
    pub fn reject(
        cl_ord_id: String,
        symbol: String,
        side: OrderSide,
        order_qty: f64,
        reason: OrderRejectReason,
        text: Option<String>,
    ) -> Self {
        Self {
            order_id: String::new(),
            cl_ord_id,
            orig_cl_ord_id: None,
            exec_id: format!("REJ{}", Utc::now().timestamp_millis()),
            exec_type: ExecType::Rejected,
            ord_status: OrderStatus::Rejected,
            symbol,
            side,
            leaves_qty: 0.0,
            cum_qty: 0.0,
            avg_px: None,
            last_px: None,
            last_qty: None,
            order_qty,
            price: None,
            transact_time: Utc::now(),
            text,
            ord_rej_reason: Some(reason),
            deribit_label: None,
            secondary_exec_id: None,
            ord_type: None,
            commission: None,
            security_exchange: None,
            qty_type: None,
            contract_multiplier: None,
            display_qty: None,
            deribit_adv_order_type: None,
            volatility: None,
            pegged_price: None,
            trd_match_id: None,
            deribit_mm_protection: None,
            mmp_group: None,
            quote_set_id: None,
            quote_id: None,
            quote_entry_id: None,
            exec_inst: None,
            stop_px: None,
            condition_trigger_method: None,
            last_liquidity_ind: None,
        }
    }

    /// Set custom label
    pub fn with_label(mut self, label: String) -> Self {
        self.deribit_label = Some(label);
        self
    }

    /// Convert to FIX message
    pub fn to_fix_message(
        &self,
        sender_comp_id: &str,
        target_comp_id: &str,
        msg_seq_num: u32,
    ) -> DeribitFixResult<String> {
        let mut builder = MessageBuilder::new()
            .msg_type(MsgType::ExecutionReport)
            .sender_comp_id(sender_comp_id.to_string())
            .target_comp_id(target_comp_id.to_string())
            .msg_seq_num(msg_seq_num)
            .sending_time(Utc::now());

        // Required fields
        builder = builder
            .field(37, self.order_id.clone()) // OrderID
            .field(11, self.cl_ord_id.clone()) // ClOrdID
            .field(17, self.exec_id.clone()) // ExecID
            .field(150, char::from(self.exec_type).to_string()) // ExecType
            .field(39, char::from(self.ord_status).to_string()) // OrdStatus
            .field(55, self.symbol.clone()) // Symbol
            .field(54, char::from(self.side).to_string()) // Side
            .field(151, self.leaves_qty.to_string()) // LeavesQty
            .field(14, self.cum_qty.to_string()) // CumQty
            .field(38, self.order_qty.to_string()) // OrderQty
            .field(
                60,
                self.transact_time.format("%Y%m%d-%H:%M:%S%.3f").to_string(),
            ); // TransactTime

        // Optional fields
        if let Some(orig_cl_ord_id) = &self.orig_cl_ord_id {
            builder = builder.field(41, orig_cl_ord_id.clone());
        }

        if let Some(avg_px) = &self.avg_px {
            builder = builder.field(6, avg_px.to_string());
        }

        if let Some(last_px) = &self.last_px {
            builder = builder.field(31, last_px.to_string());
        }

        if let Some(last_qty) = &self.last_qty {
            builder = builder.field(32, last_qty.to_string());
        }

        if let Some(price) = &self.price {
            builder = builder.field(44, price.to_string());
        }

        if let Some(text) = &self.text {
            builder = builder.field(58, text.clone());
        }

        if let Some(reason) = &self.ord_rej_reason {
            builder = builder.field(103, i32::from(*reason).to_string());
        }

        if let Some(deribit_label) = &self.deribit_label {
            builder = builder.field(100010, deribit_label.clone());
        }

        // Additional optional fields from specification
        if let Some(secondary_exec_id) = &self.secondary_exec_id {
            builder = builder.field(527, secondary_exec_id.clone());
        }

        if let Some(ord_type) = &self.ord_type {
            builder = builder.field(40, char::from(*ord_type).to_string());
        }

        if let Some(commission) = &self.commission {
            builder = builder.field(12, commission.to_string());
        }

        if let Some(security_exchange) = &self.security_exchange {
            builder = builder.field(207, security_exchange.clone());
        }

        if let Some(qty_type) = &self.qty_type {
            builder = builder.field(854, i32::from(*qty_type).to_string());
        }

        if let Some(contract_multiplier) = &self.contract_multiplier {
            builder = builder.field(231, contract_multiplier.to_string());
        }

        if let Some(display_qty) = &self.display_qty {
            builder = builder.field(1138, display_qty.to_string());
        }

        if let Some(deribit_adv_order_type) = &self.deribit_adv_order_type {
            builder = builder.field(100012, deribit_adv_order_type.to_string());
        }

        if let Some(volatility) = &self.volatility {
            builder = builder.field(1188, volatility.to_string());
        }

        if let Some(pegged_price) = &self.pegged_price {
            builder = builder.field(839, pegged_price.to_string());
        }

        if let Some(trd_match_id) = &self.trd_match_id {
            builder = builder.field(880, trd_match_id.clone());
        }

        if let Some(deribit_mm_protection) = &self.deribit_mm_protection {
            builder = builder.field(
                9008,
                if *deribit_mm_protection { "Y" } else { "N" }.to_string(),
            );
        }

        if let Some(mmp_group) = &self.mmp_group {
            builder = builder.field(9019, mmp_group.clone());
        }

        if let Some(quote_set_id) = &self.quote_set_id {
            builder = builder.field(302, quote_set_id.clone());
        }

        if let Some(quote_id) = &self.quote_id {
            builder = builder.field(117, quote_id.clone());
        }

        if let Some(quote_entry_id) = &self.quote_entry_id {
            builder = builder.field(299, quote_entry_id.clone());
        }

        if let Some(exec_inst) = &self.exec_inst {
            builder = builder.field(18, exec_inst.clone());
        }

        if let Some(stop_px) = &self.stop_px {
            builder = builder.field(99, stop_px.to_string());
        }

        if let Some(condition_trigger_method) = &self.condition_trigger_method {
            builder = builder.field(5127, condition_trigger_method.to_string());
        }

        if let Some(last_liquidity_ind) = &self.last_liquidity_ind {
            builder = builder.field(851, last_liquidity_ind.to_string());
        }

        Ok(builder.build()?.to_string())
    }
}

impl_json_display!(ExecutionReport);
impl_json_debug_pretty!(ExecutionReport);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_report_new_order() {
        let report = ExecutionReport::new_order(
            "ORD123".to_string(),
            "CLORD123".to_string(),
            "EXEC123".to_string(),
            "BTC-PERPETUAL".to_string(),
            OrderSide::Buy,
            10.0,
            10.0,
            Some(50000.0),
        );

        assert_eq!(report.order_id, "ORD123");
        assert_eq!(report.cl_ord_id, "CLORD123");
        assert_eq!(report.exec_type, ExecType::New);
        assert_eq!(report.ord_status, OrderStatus::New);
        assert_eq!(report.symbol, "BTC-PERPETUAL");
        assert_eq!(report.side, OrderSide::Buy);
        assert_eq!(report.order_qty, 10.0);
        assert_eq!(report.leaves_qty, 10.0);
        assert_eq!(report.cum_qty, 0.0);
        assert_eq!(report.price, Some(50000.0));
    }

    #[test]
    fn test_execution_report_fill() {
        let report = ExecutionReport::fill(
            "ORD123".to_string(),
            "CLORD123".to_string(),
            "EXEC123".to_string(),
            "BTC-PERPETUAL".to_string(),
            OrderSide::Buy,
            10.0,
            5.0,
            5.0,
            50000.0,
            5.0,
            50000.0,
        );

        assert_eq!(report.exec_type, ExecType::Trade);
        assert_eq!(report.ord_status, OrderStatus::PartiallyFilled);
        assert_eq!(report.cum_qty, 5.0);
        assert_eq!(report.leaves_qty, 5.0);
        assert_eq!(report.last_px, Some(50000.0));
        assert_eq!(report.last_qty, Some(5.0));
        assert_eq!(report.avg_px, Some(50000.0));
    }

    #[test]
    fn test_execution_report_fill_complete() {
        let report = ExecutionReport::fill(
            "ORD123".to_string(),
            "CLORD123".to_string(),
            "EXEC123".to_string(),
            "BTC-PERPETUAL".to_string(),
            OrderSide::Sell,
            10.0,
            0.0, // No leaves qty means fully filled
            10.0,
            49500.0,
            10.0,
            49500.0,
        );

        assert_eq!(report.exec_type, ExecType::Trade);
        assert_eq!(report.ord_status, OrderStatus::Filled);
        assert_eq!(report.cum_qty, 10.0);
        assert_eq!(report.leaves_qty, 0.0);
    }

    #[test]
    fn test_execution_report_reject() {
        let report = ExecutionReport::reject(
            "CLORD123".to_string(),
            "BTC-PERPETUAL".to_string(),
            OrderSide::Buy,
            10.0,
            OrderRejectReason::OrderExceedsLimit,
            Some("Insufficient margin".to_string()),
        );

        assert_eq!(report.exec_type, ExecType::Rejected);
        assert_eq!(report.ord_status, OrderStatus::Rejected);
        assert_eq!(
            report.ord_rej_reason,
            Some(OrderRejectReason::OrderExceedsLimit)
        );
        assert_eq!(report.text, Some("Insufficient margin".to_string()));
        assert_eq!(report.leaves_qty, 0.0);
        assert_eq!(report.cum_qty, 0.0);
    }

    #[test]
    fn test_execution_report_with_label() {
        let report = ExecutionReport::new_order(
            "ORD123".to_string(),
            "CLORD123".to_string(),
            "EXEC123".to_string(),
            "BTC-PERPETUAL".to_string(),
            OrderSide::Buy,
            10.0,
            10.0,
            Some(50000.0),
        )
        .with_label("my-strategy".to_string());

        assert_eq!(report.deribit_label, Some("my-strategy".to_string()));
    }

    #[test]
    fn test_execution_report_to_fix_message() {
        let report = ExecutionReport::new_order(
            "ORD123".to_string(),
            "CLORD123".to_string(),
            "EXEC123".to_string(),
            "BTC-PERPETUAL".to_string(),
            OrderSide::Buy,
            10.0,
            10.0,
            Some(50000.0),
        );

        let fix_message = report.to_fix_message("SENDER", "TARGET", 1).unwrap();

        // Check that the message contains required fields
        assert!(fix_message.contains("35=8")); // MsgType
        assert!(fix_message.contains("37=ORD123")); // OrderID
        assert!(fix_message.contains("11=CLORD123")); // ClOrdID
        assert!(fix_message.contains("17=EXEC123")); // ExecID
        assert!(fix_message.contains("150=0")); // ExecType=New
        assert!(fix_message.contains("39=0")); // OrdStatus=New
        assert!(fix_message.contains("55=BTC-PERPETUAL")); // Symbol
        assert!(fix_message.contains("54=1")); // Side=Buy
        assert!(fix_message.contains("151=10")); // LeavesQty
        assert!(fix_message.contains("14=0")); // CumQty
        assert!(fix_message.contains("38=10")); // OrderQty
        assert!(fix_message.contains("44=50000")); // Price
    }
}
