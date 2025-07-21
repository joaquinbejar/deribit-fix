//! FIX session management

use crate::model::message::FixMessage;
use crate::model::types::MsgType;
use crate::{
    config::DeribitFixConfig,
    connection::Connection,
    error::{DeribitFixError, Result},
    message::MessageBuilder,
};
use base64::prelude::*;
use chrono::Utc;
use deribit_base::prelude::*;
use deribit_base::utils::{generate_nonce, generate_timestamp};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info};

/// FIX session state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Disconnected,
    LogonSent,
    LoggedOn,
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

    /// Perform FIX logon
    pub async fn logon(&mut self) -> Result<()> {
        let connection = self
            .connection
            .as_ref()
            .ok_or_else(|| DeribitFixError::Session("No connection available".to_string()))?;

        info!("Initiating FIX logon");
        self.state = SessionState::LogonSent;

        // Generate authentication data
        let timestamp = generate_timestamp();
        let nonce = generate_nonce(32);
        let raw_data = format!("{timestamp}.{nonce}");

        // Calculate password hash
        let password_hash = self.calculate_password_hash(&raw_data)?;

        // Build logon message
        let mut builder = MessageBuilder::new()
            .msg_type(MsgType::Logon)
            .sender_comp_id(self.config.sender_comp_id.clone())
            .target_comp_id(self.config.target_comp_id.clone())
            .msg_seq_num(self.outgoing_seq_num)
            .sending_time(Utc::now())
            .field(108, self.config.heartbeat_interval.to_string()) // HeartBtInt
            .field(553, self.config.username.clone()) // Username
            .field(554, password_hash) // Password
            .field(95, raw_data.len().to_string()) // RawDataLength
            .field(96, raw_data.clone()); // RawData

        // Add cancel on disconnect if configured
        if self.config.cancel_on_disconnect {
            builder = builder.field(9001, "Y".to_string()); // CancelOnDisconnect
        } else {
            builder = builder.field(9001, "N".to_string());
        }

        // Add application signature if configured
        if let (Some(app_id), Some(app_secret)) = (&self.config.app_id, &self.config.app_secret) {
            let app_sig = self.calculate_app_signature(&raw_data, app_secret)?;
            builder = builder
                .field(9004, app_id.clone()) // DeribitAppId
                .field(9005, app_sig); // DeribitAppSig
        }

        let logon_message = builder.build()?;

        let mut conn_guard = connection.lock().await;
        conn_guard.send_message(&logon_message).await?;

        self.outgoing_seq_num += 1;

        info!("Logon message sent");
        Ok(())
    }

    /// Perform FIX logout
    pub async fn logout(&mut self) -> Result<()> {
        let connection = self
            .connection
            .as_ref()
            .ok_or_else(|| DeribitFixError::Session("No connection available".to_string()))?;

        info!("Initiating FIX logout");
        self.state = SessionState::LogoutSent;

        let logout_message = MessageBuilder::new()
            .msg_type(MsgType::Logout)
            .sender_comp_id(self.config.sender_comp_id.clone())
            .target_comp_id(self.config.target_comp_id.clone())
            .msg_seq_num(self.outgoing_seq_num)
            .sending_time(Utc::now())
            .build()?;

        let mut conn_guard = connection.lock().await;
        conn_guard.send_message(&logout_message).await?;

        self.outgoing_seq_num += 1;
        self.state = SessionState::Disconnected;

        info!("Logout message sent");
        Ok(())
    }

    /// Send a heartbeat message
    pub async fn send_heartbeat(&mut self, test_req_id: Option<String>) -> Result<()> {
        let connection = self
            .connection
            .as_ref()
            .ok_or_else(|| DeribitFixError::Session("No connection available".to_string()))?;

        let mut builder = MessageBuilder::new()
            .msg_type(MsgType::Heartbeat)
            .sender_comp_id(self.config.sender_comp_id.clone())
            .target_comp_id(self.config.target_comp_id.clone())
            .msg_seq_num(self.outgoing_seq_num)
            .sending_time(Utc::now());

        if let Some(test_req_id) = test_req_id {
            builder = builder.field(112, test_req_id); // TestReqID
        }

        let heartbeat_message = builder.build()?;

        let mut conn_guard = connection.lock().await;
        conn_guard.send_message(&heartbeat_message).await?;

        self.outgoing_seq_num += 1;

        debug!("Heartbeat sent");
        Ok(())
    }

    /// Send a new order
    pub async fn send_new_order(&mut self, order: NewOrderRequest) -> Result<String> {
        let connection = self
            .connection
            .as_ref()
            .ok_or_else(|| DeribitFixError::Session("No connection available".to_string()))?;

        if self.state != SessionState::LoggedOn {
            return Err(DeribitFixError::Session("Not logged on".to_string()));
        }

        let client_order_id = order
            .client_order_id
            .unwrap_or_else(|| format!("ORDER_{}", generate_timestamp()));

        let mut builder = MessageBuilder::new()
            .msg_type(MsgType::NewOrderSingle)
            .sender_comp_id(self.config.sender_comp_id.clone())
            .target_comp_id(self.config.target_comp_id.clone())
            .msg_seq_num(self.outgoing_seq_num)
            .sending_time(Utc::now())
            .field(11, client_order_id.clone()) // ClOrdID
            .field(55, order.symbol) // Symbol
            .field(
                54,
                match order.side {
                    // Side
                    OrderSide::Buy => "1",
                    OrderSide::Sell => "2",
                }
                .to_string(),
            )
            .field(38, order.quantity.to_string()) // OrderQty
            .field(
                40,
                match order.order_type {
                    // OrdType
                    OrderType::Market => "1",
                    OrderType::Limit => "2",
                    OrderType::Stop => "3",
                    OrderType::StopLimit => "4",
                }
                .to_string(),
            )
            .field(
                59,
                match order.time_in_force {
                    // TimeInForce
                    TimeInForce::Day => "0",
                    TimeInForce::GoodTillCancel => "1",
                    TimeInForce::ImmediateOrCancel => "3",
                    TimeInForce::FillOrKill => "4",
                }
                .to_string(),
            );

        // Add price for limit orders
        if let Some(price) = order.price {
            builder = builder.field(44, price.to_string()); // Price
        }

        let order_message = builder.build()?;

        let mut conn_guard = connection.lock().await;
        conn_guard.send_message(&order_message).await?;

        self.outgoing_seq_num += 1;

        info!("New order sent: {}", client_order_id);
        Ok(client_order_id)
    }

    /// Cancel an order
    pub async fn cancel_order(&mut self, order_id: String) -> Result<()> {
        let connection = self
            .connection
            .as_ref()
            .ok_or_else(|| DeribitFixError::Session("No connection available".to_string()))?;

        if self.state != SessionState::LoggedOn {
            return Err(DeribitFixError::Session("Not logged on".to_string()));
        }

        let cancel_message = MessageBuilder::new()
            .msg_type(MsgType::OrderCancelRequest)
            .sender_comp_id(self.config.sender_comp_id.clone())
            .target_comp_id(self.config.target_comp_id.clone())
            .msg_seq_num(self.outgoing_seq_num)
            .sending_time(Utc::now())
            .field(11, format!("CANCEL_{}", generate_timestamp())) // ClOrdID
            .field(41, order_id) // OrigClOrdID
            .build()?;

        let mut conn_guard = connection.lock().await;
        conn_guard.send_message(&cancel_message).await?;

        self.outgoing_seq_num += 1;

        info!("Order cancel request sent");
        Ok(())
    }

    /// Subscribe to market data
    pub async fn subscribe_market_data(&mut self, symbol: String) -> Result<()> {
        let connection = self
            .connection
            .as_ref()
            .ok_or_else(|| DeribitFixError::Session("No connection available".to_string()))?;

        if self.state != SessionState::LoggedOn {
            return Err(DeribitFixError::Session("Not logged on".to_string()));
        }

        let md_request = MessageBuilder::new()
            .msg_type(MsgType::MarketDataRequest)
            .sender_comp_id(self.config.sender_comp_id.clone())
            .target_comp_id(self.config.target_comp_id.clone())
            .msg_seq_num(self.outgoing_seq_num)
            .sending_time(Utc::now())
            .field(262, format!("MD_{}", generate_timestamp())) // MDReqID
            .field(263, "1".to_string()) // SubscriptionRequestType (Snapshot + Updates)
            .field(264, "0".to_string()) // MarketDepth (Full Book)
            .field(146, "1".to_string()) // NoRelatedSym
            .field(55, symbol) // Symbol
            .build()?;

        let mut conn_guard = connection.lock().await;
        conn_guard.send_message(&md_request).await?;

        self.outgoing_seq_num += 1;

        info!("Market data subscription sent");
        Ok(())
    }

    /// Request positions
    pub async fn request_positions(&mut self) -> Result<Vec<Position>> {
        let connection = self
            .connection
            .as_ref()
            .ok_or_else(|| DeribitFixError::Session("No connection available".to_string()))?;

        if self.state != SessionState::LoggedOn {
            return Err(DeribitFixError::Session("Not logged on".to_string()));
        }

        let pos_request = MessageBuilder::new()
            .msg_type(MsgType::RequestForPositions)
            .sender_comp_id(self.config.sender_comp_id.clone())
            .target_comp_id(self.config.target_comp_id.clone())
            .msg_seq_num(self.outgoing_seq_num)
            .sending_time(Utc::now())
            .field(710, format!("POS_{}", generate_timestamp())) // PosReqID
            .field(724, "0".to_string()) // PosReqType (Positions)
            .build()?;

        let mut conn_guard = connection.lock().await;
        conn_guard.send_message(&pos_request).await?;

        self.outgoing_seq_num += 1;

        info!("Position request sent");

        // TODO: Implement position response parsing
        Ok(vec![])
    }

    /// Calculate password hash for authentication
    fn calculate_password_hash(&self, raw_data: &str) -> Result<String> {
        let mut hasher = Sha256::new();
        hasher.update(raw_data.as_bytes());
        hasher.update(self.config.password.as_bytes());
        let hash = hasher.finalize();
        Ok(BASE64_STANDARD.encode(hash))
    }

    /// Calculate application signature for registered apps
    fn calculate_app_signature(&self, raw_data: &str, app_secret: &str) -> Result<String> {
        let mut hasher = Sha256::new();
        hasher.update(raw_data.as_bytes());
        hasher.update(app_secret.as_bytes());
        let hash = hasher.finalize();
        Ok(BASE64_STANDARD.encode(hash))
    }

    /// Get current session state
    pub fn state(&self) -> SessionState {
        self.state
    }

    /// Set session state (for testing)
    pub fn set_state(&mut self, state: SessionState) {
        self.state = state;
    }

    async fn process_message(&mut self, message: &FixMessage) -> Result<()> {
        self.incoming_seq_num += 1;

        match message.msg_type() {
            Some(MsgType::Logon) => {
                self.state = SessionState::LoggedOn;
                info!("Session state changed to LoggedOn");
            }
            Some(MsgType::Heartbeat) => {
                debug!("Heartbeat received");
            }
            Some(MsgType::TestRequest) => {
                if let Some(test_req_id) = message.get_field(112) {
                    info!("TestRequest received, sending Heartbeat");
                    self.send_heartbeat(Some(test_req_id.to_string())).await?;
                } else {
                    return Err(DeribitFixError::MessageConstruction(
                        "TestRequest received without TestReqID".to_string(),
                    ));
                }
            }
            _ => {
                // Other message types are handled by the client application
            }
        }
        Ok(())
    }

    /// Receive and process a FIX message from the connection
    pub async fn receive_and_process_message(&mut self) -> Result<Option<FixMessage>> {
        let message = {
            let conn_arc = self
                .connection
                .as_ref()
                .ok_or_else(|| DeribitFixError::Session("No connection available".to_string()))?;
            let mut conn = conn_arc.lock().await;
            conn.receive_message().await?
        };

        if let Some(msg) = message.as_ref() {
            self.process_message(msg).await?;
            Ok(Some(msg.clone()))
        } else {
            self.state = SessionState::Disconnected;
            Ok(None)
        }
    }
}
