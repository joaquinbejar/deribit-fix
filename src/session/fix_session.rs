//! FIX session management

use crate::model::message::FixMessage;
use crate::model::types::MsgType;
use crate::{
    config::DeribitFixConfig,
    connection::Connection,
    error::{DeribitFixError, Result},
    message::{MessageBuilder, RequestForPositions, PositionReport},
};
use base64::prelude::*;
use chrono::Utc;
use deribit_base::prelude::*;
use rand;
use sha2::{Digest, Sha256};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info};

/// FIX session state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    /// Session is disconnected
    Disconnected,
    /// Logon message sent, waiting for response
    LogonSent,
    /// Session is logged on and active
    LoggedOn,
    /// Logout message sent, waiting for confirmation
    LogoutSent,
}

/// FIX session manager
pub struct Session {
    config: DeribitFixConfig,
    connection: Option<Arc<Mutex<Connection>>>,
    state: SessionState,
    outgoing_seq_num: u32,
    incoming_seq_num: u32,
}

impl Session {
    /// Create a new FIX session
    pub fn new(config: &DeribitFixConfig, connection: Arc<Mutex<Connection>>) -> Result<Self> {
        info!("Creating new FIX session");
        Ok(Self {
            config: config.clone(),
            state: SessionState::Disconnected,
            outgoing_seq_num: 1,
            incoming_seq_num: 1,
            connection: Some(connection),
        })
    }

    /// Set the connection for this session
    pub fn set_connection(&mut self, connection: Arc<Mutex<Connection>>) {
        self.connection = Some(connection);
    }

    /// Get the current session state
    pub fn get_state(&self) -> SessionState {
        self.state
    }

    /// Send a FIX message through the connection
    async fn send_message(&mut self, message: FixMessage) -> Result<()> {
        if let Some(connection) = &self.connection {
            let mut conn_guard = connection.lock().await;
            conn_guard.send_message(&message).await?;
            debug!("Sent FIX message: {}", message.to_string());
        } else {
            return Err(DeribitFixError::Connection(
                "No connection available".to_string(),
            ));
        }
        Ok(())
    }

    /// Perform FIX logon
    pub async fn logon(&mut self) -> Result<()> {
        info!("Performing FIX logon");

        // Generate RawData and password hash according to Deribit FIX spec
        let (raw_data, password_hash) = self.generate_auth_data(&self.config.password)?;

        let mut message_builder = MessageBuilder::new()
            .msg_type(MsgType::Logon)
            .sender_comp_id(self.config.sender_comp_id.clone())
            .target_comp_id(self.config.target_comp_id.clone())
            .msg_seq_num(self.outgoing_seq_num)
            .field(108, self.config.heartbeat_interval.to_string()) // HeartBtInt - Required
            .field(96, raw_data.clone()) // RawData - Required (timestamp.nonce)
            .field(553, self.config.username.clone()) // Username - Required
            .field(554, password_hash); // Password - Required

        // Add RawDataLength if needed (optional but recommended)
        message_builder = message_builder.field(95, raw_data.len().to_string()); // RawDataLength

        // Add optional Deribit-specific tags based on configuration
        if let Some(use_wordsafe_tags) = &self.config.use_wordsafe_tags {
            message_builder = message_builder.field(9002, if *use_wordsafe_tags { "Y" } else { "N" }.to_string()); // UseWordsafeTags
        }

        // CancelOnDisconnect - always include based on config
        message_builder = message_builder.field(9001, if self.config.cancel_on_disconnect { "Y" } else { "N" }.to_string()); // CancelOnDisconnect

        if let Some(app_id) = &self.config.app_id {
            message_builder = message_builder.field(9004, app_id.clone()); // DeribitAppId
        }

        if let Some(app_secret) = &self.config.app_secret
            && let Some(raw_data_str) = raw_data.split_once('.').map(|(timestamp, nonce)| format!("{}.{}", timestamp, nonce))
                && let Ok(app_sig) = self.calculate_app_signature(&raw_data_str, app_secret) {
                    message_builder = message_builder.field(9005, app_sig); // DeribitAppSig
                }

        if let Some(deribit_sequential) = &self.config.deribit_sequential {
            message_builder = message_builder.field(9007, if *deribit_sequential { "Y" } else { "N" }.to_string()); // DeribitSequential
        }

        if let Some(unsubscribe_exec_reports) = &self.config.unsubscribe_execution_reports {
            message_builder = message_builder.field(9009, if *unsubscribe_exec_reports { "Y" } else { "N" }.to_string()); // UnsubscribeExecutionReports
        }

        if let Some(connection_only_exec_reports) = &self.config.connection_only_execution_reports {
            message_builder = message_builder.field(9010, if *connection_only_exec_reports { "Y" } else { "N" }.to_string()); // ConnectionOnlyExecutionReports
        }

        if let Some(report_fills_as_exec_reports) = &self.config.report_fills_as_exec_reports {
            message_builder = message_builder.field(9015, if *report_fills_as_exec_reports { "Y" } else { "N" }.to_string()); // ReportFillsAsExecReports
        }

        if let Some(display_increment_steps) = &self.config.display_increment_steps {
            message_builder = message_builder.field(9018, if *display_increment_steps { "Y" } else { "N" }.to_string()); // DisplayIncrementSteps
        }

        // Add AppID if available - temporarily disabled for testing
        // if let Some(app_id) = &self.config.app_id {
        //     message_builder = message_builder.field(1128, app_id.clone()); // AppID
        // }

        let logon_message = message_builder.build()?;

        // Send the logon message
        self.send_message(logon_message).await?;
        self.state = SessionState::LogonSent;
        self.outgoing_seq_num += 1;

        info!("Logon message sent");
        Ok(())
    }

    /// Perform FIX logout
    pub async fn logout(&mut self) -> Result<()> {
        self.logout_with_options(None, None).await
    }

    /// Perform FIX logout with optional parameters
    pub async fn logout_with_options(&mut self, text: Option<String>, dont_cancel_on_disconnect: Option<bool>) -> Result<()> {
        info!("Performing FIX logout");

        let mut message_builder = MessageBuilder::new()
            .msg_type(MsgType::Logout)
            .sender_comp_id(self.config.sender_comp_id.clone())
            .target_comp_id(self.config.target_comp_id.clone())
            .msg_seq_num(self.outgoing_seq_num);

        // Add Text field (tag 58) - optional
        let logout_text = text.unwrap_or_else(|| "Normal logout".to_string());
        message_builder = message_builder.field(58, logout_text); // Text

        // Add DontCancelOnDisconnect field (tag 9003) - optional
        if let Some(dont_cancel) = dont_cancel_on_disconnect {
            message_builder = message_builder.field(9003, if dont_cancel { "Y" } else { "N" }.to_string()); // DontCancelOnDisconnect
        }

        let logout_message = message_builder.build()?;

        // Send the logout message
        self.send_message(logout_message).await?;
        self.state = SessionState::LogoutSent;
        self.outgoing_seq_num += 1;

        info!("Logout message sent");
        Ok(())
    }

    /// Send a heartbeat message
    pub async fn send_heartbeat(&mut self, test_req_id: Option<String>) -> Result<()> {
        debug!("Sending heartbeat message");

        let mut builder = MessageBuilder::new()
            .msg_type(MsgType::Heartbeat)
            .sender_comp_id(self.config.sender_comp_id.clone())
            .target_comp_id(self.config.target_comp_id.clone())
            .msg_seq_num(self.outgoing_seq_num);

        if let Some(test_req_id) = test_req_id {
            builder = builder.field(112, test_req_id); // TestReqID
        }

        let heartbeat_message = builder.build()?;

        // Send the heartbeat message
        self.send_message(heartbeat_message).await?;
        self.outgoing_seq_num += 1;

        debug!("Heartbeat message sent");
        Ok(())
    }

    /// Send a new order
    pub fn send_new_order(&mut self, order: NewOrderRequest) -> Result<String> {
        info!("Sending new order: {:?}", order);

        let order_id = format!("ORDER_{}", chrono::Utc::now().timestamp_millis());

        let _order_message = MessageBuilder::new()
            .msg_type(MsgType::NewOrderSingle)
            .sender_comp_id(self.config.sender_comp_id.clone())
            .target_comp_id(self.config.target_comp_id.clone())
            .msg_seq_num(self.outgoing_seq_num)
            .field(11, order_id.clone()) // ClOrdID
            .field(55, order.instrument_name.clone()) // Symbol
            .field(
                54,
                match order.side {
                    deribit_base::model::order::OrderSide::Buy => "1".to_string(),
                    deribit_base::model::order::OrderSide::Sell => "2".to_string(),
                },
            ) // Side
            .field(60, Utc::now().format("%Y%m%d-%H:%M:%S%.3f").to_string()) // TransactTime
            .field(38, order.amount.to_string()) // OrderQty
            .field(40, "2".to_string()) // OrdType (2 = Limit)
            .field(44, order.price.unwrap_or(0.0).to_string()) // Price
            .build()?;

        // In a real implementation, you would send this message
        self.outgoing_seq_num += 1;

        info!("New order message prepared with ID: {}", order_id);
        Ok(order_id)
    }

    /// Cancel an order
    pub fn cancel_order(&mut self, order_id: String) -> Result<()> {
        info!("Cancelling order: {}", order_id);

        let cancel_id = format!("CANCEL_{}", chrono::Utc::now().timestamp_millis());

        let _cancel_message = MessageBuilder::new()
            .msg_type(MsgType::OrderCancelRequest)
            .sender_comp_id(self.config.sender_comp_id.clone())
            .target_comp_id(self.config.target_comp_id.clone())
            .msg_seq_num(self.outgoing_seq_num)
            .field(11, cancel_id) // ClOrdID
            .field(41, order_id) // OrigClOrdID
            .field(60, Utc::now().format("%Y%m%d-%H:%M:%S%.3f").to_string()) // TransactTime
            .build()?;

        // In a real implementation, you would send this message
        self.outgoing_seq_num += 1;

        info!("Order cancel message prepared");
        Ok(())
    }

    /// Subscribe to market data
    pub async fn subscribe_market_data(&mut self, symbol: String) -> Result<()> {
        info!("Subscribing to market data for: {}", symbol);

        let request_id = format!("MDR_{}", chrono::Utc::now().timestamp_millis());

        let market_data_request = MessageBuilder::new()
            .msg_type(MsgType::MarketDataRequest)
            .sender_comp_id(self.config.sender_comp_id.clone())
            .target_comp_id(self.config.target_comp_id.clone())
            .msg_seq_num(self.outgoing_seq_num)
            .field(262, request_id.clone()) // MDReqID
            .field(263, "1".to_string()) // SubscriptionRequestType (1 = Snapshot + Updates)
            .field(264, "0".to_string()) // MarketDepth (0 = Full Book)
            .field(267, "2".to_string()) // NoMDEntryTypes
            .field(269, "0".to_string()) // MDEntryType (0 = Bid)
            .field(269, "1".to_string()) // MDEntryType (1 = Offer)
            .field(146, "1".to_string()) // NoRelatedSym
            .field(55, symbol.clone()) // Symbol
            .build()?;

        // Send the market data request
        self.send_message(market_data_request).await?;
        self.outgoing_seq_num += 1;

        info!("Market data subscription request sent for symbol: {} with ID: {}", symbol, request_id);
        Ok(())
    }

    /// Request positions asynchronously
    pub async fn request_positions(&mut self) -> Result<Vec<Position>> {
        use std::time::{Duration, Instant};
        use tracing::{debug, info, warn};

        info!("Requesting positions");

        let request_id = format!("POS_{}", chrono::Utc::now().timestamp_millis());
        
        // Create typed position request
        let position_request = RequestForPositions::all_positions(request_id.clone())
            .with_clearing_date(Utc::now().format("%Y%m%d").to_string());

        // Build the FIX message
        let fix_message = position_request.to_fix_message(
            self.config.sender_comp_id.clone(),
            self.config.target_comp_id.clone(),
            self.outgoing_seq_num,
        )?;
        
        // Send the position request
        self.send_message(fix_message).await?;
        self.outgoing_seq_num += 1;
        
        info!("Position request sent, awaiting responses for request ID: {}", request_id);

        // Collect position reports with correlation by PosReqID
        let mut positions = Vec::new();
        let timeout = Duration::from_secs(30); // 30 second timeout
        let start_time = Instant::now();

        loop {
            // Check for timeout
            if start_time.elapsed() > timeout {
                warn!("Position request timed out after {:?}", timeout);
                break;
            }

            // Receive and process messages
            match self.receive_and_process_message().await {
                Ok(Some(message)) => {
                    // Check if this is a PositionReport message
                    if let Some(msg_type_str) = message.get_field(35)
                        && msg_type_str == "AP" { // PositionReport
                            // Check if this position report matches our request ID
                            if let Some(pos_req_id) = message.get_field(710) {
                                if pos_req_id == &request_id {
                                    debug!("Received PositionReport for request: {}", request_id);
                                    
                                    match PositionReport::from_fix_message(&message) {
                                        Ok(position_report) => {
                                            let position = position_report.to_position();
                                            debug!("Parsed position: {} - Qty: {}, Avg Price: {}", 
                                                   position.symbol, position.quantity, position.average_price);
                                            positions.push(position);
                                        }
                                        Err(e) => {
                                            warn!("Failed to parse PositionReport: {}", e);
                                        }
                                    }
                                } else {
                                    debug!("Received PositionReport for different request: {}", pos_req_id);
                                }
                            }
                        }
                }
                Ok(None) => {
                    // No message received, continue loop
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
                Err(e) => {
                    warn!("Error receiving message: {}", e);
                    // Continue trying to receive more messages
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            }

            // For now, we'll break after receiving some positions or after a reasonable time
            // In a real implementation, we might wait for an end-of-transmission signal
            if !positions.is_empty() && start_time.elapsed() > Duration::from_secs(5) {
                debug!("Received {} positions, stopping collection", positions.len());
                break;
            }
        }

        info!("Position request completed, received {} positions", positions.len());
        Ok(positions)
    }

    /// Generate authentication data according to Deribit FIX specification
    /// Returns (raw_data, base64_password_hash)
    pub fn generate_auth_data(&self, access_secret: &str) -> Result<(String, String)> {
        // Generate timestamp (strictly increasing integer in milliseconds)
        let timestamp = chrono::Utc::now().timestamp_millis();

        // Generate random nonce (at least 32 bytes as recommended by Deribit)
        let mut nonce_bytes = vec![0u8; 32];
        for byte in nonce_bytes.iter_mut() {
            *byte = rand::random::<u8>();
        }
        let nonce_b64 = BASE64_STANDARD.encode(&nonce_bytes);

        // Create RawData: timestamp.nonce (separated by ASCII period)
        let raw_data = format!("{timestamp}.{nonce_b64}");

        // Calculate password hash: base64(sha256(RawData ++ access_secret))
        let mut auth_data = raw_data.as_bytes().to_vec();
        auth_data.extend_from_slice(access_secret.as_bytes());

        debug!("Timestamp: {}", timestamp);
        debug!("Nonce length: {} bytes", nonce_bytes.len());
        debug!("Nonce (base64): {}", nonce_b64);
        debug!("RawData: {}", raw_data);
        debug!("Access secret: {}", access_secret);
        debug!("Auth data length: {} bytes", auth_data.len());

        let mut hasher = Sha256::new();
        hasher.update(&auth_data);
        let hash_result = hasher.finalize();
        let password_hash = BASE64_STANDARD.encode(hash_result);

        debug!("Password hash: {}", password_hash);

        Ok((raw_data, password_hash))
    }

    /// Calculate application signature for registered apps
    #[allow(dead_code)]
    fn calculate_app_signature(&self, raw_data: &str, app_secret: &str) -> Result<String> {
        let mut hasher = Sha256::new();
        hasher.update(format!("{raw_data}{app_secret}").as_bytes());
        let result = hasher.finalize();
        Ok(BASE64_STANDARD.encode(result))
    }

    /// Get current session state
    pub fn state(&self) -> SessionState {
        self.state
    }

    /// Set session state (for testing)
    pub fn set_state(&mut self, state: SessionState) {
        self.state = state;
    }

    /// Process incoming FIX message
    async fn process_message(&mut self, message: &FixMessage) -> Result<()> {
        debug!("Processing FIX message: {:?}", message);

        // Get message type
        let msg_type_str = message.get_field(35).unwrap_or(&String::new()).clone();
        let msg_type = MsgType::from_str(&msg_type_str).map_err(|_| {
            DeribitFixError::MessageParsing(format!("Unknown message type: {msg_type_str}"))
        })?;

        match msg_type {
            MsgType::Logon => {
                info!("Received logon response");
                self.state = SessionState::LoggedOn;
            }
            MsgType::Logout => {
                info!("Received logout message");
                self.state = SessionState::Disconnected;
            }
            MsgType::Heartbeat => {
                debug!("Received heartbeat");
            }
            MsgType::TestRequest => {
                debug!("Received test request, sending heartbeat response");
                let test_req_id = message.get_field(112);
                self.send_heartbeat(test_req_id.cloned()).await?;
            }
            _ => {
                debug!("Received message type: {:?}", msg_type);
            }
        }

        self.incoming_seq_num += 1;
        Ok(())
    }

    /// Receive and process a FIX message from the connection
    pub async fn receive_and_process_message(&mut self) -> Result<Option<FixMessage>> {
        let message = if let Some(connection) = &self.connection {
            let mut conn_guard = connection.lock().await;
            conn_guard.receive_message().await?
        } else {
            None
        };

        if let Some(message) = message {
            self.process_message(&message).await?;
            Ok(Some(message))
        } else {
            Ok(None)
        }
    }
}
