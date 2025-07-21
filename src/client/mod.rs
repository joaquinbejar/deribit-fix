//! Deribit FIX client implementation

use crate::{
    config::Config,
    connection::Connection,
    error::{DeribitFixError, Result},
    message::{FixMessage, MessageBuilder},
    session::Session,
    types::*,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, error, info, warn};

/// Main Deribit FIX client
pub struct DeribitFixClient {
    config: Config,
    connection: Option<Arc<Mutex<Connection>>>,
    session: Option<Arc<Mutex<Session>>>,
}

impl DeribitFixClient {
    /// Create a new Deribit FIX client
    pub async fn new(config: Config) -> Result<Self> {
        config.validate()?;
        
        Ok(Self {
            config,
            connection: None,
            session: None,
        })
    }

    /// Connect to the Deribit FIX server
    pub async fn connect(&mut self) -> Result<()> {
        info!("Connecting to Deribit FIX server at {}", self.config.connection_url());
        
        // Create connection
        let connection = Connection::new(&self.config).await?;
        self.connection = Some(Arc::new(Mutex::new(connection)));

        // Create session
        let session = Session::new(&self.config)?;
        self.session = Some(Arc::new(Mutex::new(session)));

        // Perform logon
        self.logon().await?;

        info!("Successfully connected to Deribit FIX server");
        Ok(())
    }

    /// Disconnect from the server
    pub async fn disconnect(&mut self) -> Result<()> {
        if let Some(session) = &self.session {
            let mut session_guard = session.lock().await;
            session_guard.logout().await?;
        }

        if let Some(connection) = &self.connection {
            let mut conn_guard = connection.lock().await;
            conn_guard.close().await?;
        }

        self.connection = None;
        self.session = None;

        info!("Disconnected from Deribit FIX server");
        Ok(())
    }

    /// Check if the client is connected
    pub fn is_connected(&self) -> bool {
        self.connection.is_some() && self.session.is_some()
    }

    /// Perform FIX logon
    async fn logon(&self) -> Result<()> {
        let session = self.session.as_ref()
            .ok_or_else(|| DeribitFixError::Session("Session not initialized".to_string()))?;
        
        let mut session_guard = session.lock().await;
        session_guard.logon().await?;
        
        Ok(())
    }

    /// Send a new order
    pub async fn send_order(&self, order: NewOrderRequest) -> Result<String> {
        let session = self.session.as_ref()
            .ok_or_else(|| DeribitFixError::Session("Not connected".to_string()))?;
        
        let mut session_guard = session.lock().await;
        session_guard.send_new_order(order).await
    }

    /// Cancel an order
    pub async fn cancel_order(&self, order_id: String) -> Result<()> {
        let session = self.session.as_ref()
            .ok_or_else(|| DeribitFixError::Session("Not connected".to_string()))?;
        
        let mut session_guard = session.lock().await;
        session_guard.cancel_order(order_id).await
    }

    /// Subscribe to market data
    pub async fn subscribe_market_data(&self, symbol: String) -> Result<()> {
        let session = self.session.as_ref()
            .ok_or_else(|| DeribitFixError::Session("Not connected".to_string()))?;
        
        let mut session_guard = session.lock().await;
        session_guard.subscribe_market_data(symbol).await
    }

    /// Get account positions
    pub async fn get_positions(&self) -> Result<Vec<Position>> {
        let session = self.session.as_ref()
            .ok_or_else(|| DeribitFixError::Session("Not connected".to_string()))?;
        
        let mut session_guard = session.lock().await;
        session_guard.request_positions().await
    }
}

/// New order request parameters
#[derive(Debug, Clone)]
pub struct NewOrderRequest {
    pub symbol: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub quantity: f64,
    pub price: Option<f64>,
    pub time_in_force: TimeInForce,
    pub client_order_id: Option<String>,
}

/// Order side enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderSide {
    Buy,
    Sell,
}

/// Order type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
}

/// Time in force enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeInForce {
    Day,
    GoodTillCancel,
    ImmediateOrCancel,
    FillOrKill,
}

/// Position information
#[derive(Debug, Clone)]
pub struct Position {
    pub symbol: String,
    pub quantity: f64,
    pub average_price: f64,
    pub unrealized_pnl: f64,
    pub realized_pnl: f64,
}
