//! Deribit FIX client implementation

use crate::{
    config::DeribitFixConfig,
    connection::Connection,
    error::{DeribitFixError, Result},
    session::Session,
};
use deribit_base::prelude::{NewOrderRequest, Position};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

/// Main Deribit FIX client
pub struct DeribitFixClient {
    /// Client configuration
    pub config: DeribitFixConfig,
    connection: Option<Arc<Mutex<Connection>>>,
    session: Option<Arc<Mutex<Session>>>,
}

impl DeribitFixClient {
    /// Create a new Deribit FIX client
    pub async fn new(config: DeribitFixConfig) -> Result<Self> {
        config.validate()?;

        Ok(Self {
            config,
            connection: None,
            session: None,
        })
    }

    /// Connect to the Deribit FIX server
    pub async fn connect(&mut self) -> Result<()> {
        info!(
            "Connecting to Deribit FIX server at {}",
            self.config.connection_url()
        );

        // Create connection
        let connection = Connection::new(&self.config).await?;
        self.connection = Some(Arc::new(Mutex::new(connection)));

        // Create session
        let session = Session::new(&self.config, self.connection.as_ref().unwrap().clone())?;
        self.session = Some(Arc::new(Mutex::new(session)));

        // Perform logon
        self.logon().await?;

        info!("Successfully connected to Deribit FIX server");
        Ok(())
    }

    /// Disconnect from the server
    pub async fn disconnect(&mut self) -> Result<()> {
        info!("Disconnecting from Deribit FIX server");

        if let Some(session) = &self.session {
            let mut session_guard = session.lock().await;
            session_guard.logout().await?;
        }

        if let Some(connection) = &self.connection {
            let mut connection_guard = connection.lock().await;
            connection_guard.close().await?;
        }

        self.connection = None;
        self.session = None;

        info!("Successfully disconnected from Deribit FIX server");
        Ok(())
    }

    /// Check if the client is connected
    pub fn is_connected(&self) -> bool {
        self.connection.is_some() && self.session.is_some()
    }

    /// Get the current session state
    pub fn get_session_state(&self) -> Option<crate::session::SessionState> {
        if let Some(session) = &self.session {
            // Try to get the session state without blocking
            if let Ok(session_guard) = session.try_lock() {
                Some(session_guard.get_state())
            } else {
                // Session is locked, return None to indicate we can't check right now
                None
            }
        } else {
            None
        }
    }

    /// Perform FIX logon
    async fn logon(&self) -> Result<()> {
        if let Some(session) = &self.session {
            let mut session_guard = session.lock().await;
            session_guard.logon().await?;
        }
        Ok(())
    }

    /// Send a new order
    pub async fn send_order(&self, order: NewOrderRequest) -> Result<String> {
        if let Some(session) = &self.session {
            let mut session_guard = session.lock().await;
            session_guard.send_new_order(order)
        } else {
            Err(DeribitFixError::Session("Not connected".to_string()))
        }
    }

    /// Cancel an order
    pub async fn cancel_order(&self, order_id: String) -> Result<()> {
        if let Some(session) = &self.session {
            let mut session_guard = session.lock().await;
            session_guard.cancel_order(order_id)
        } else {
            Err(DeribitFixError::Session("Not connected".to_string()))
        }
    }

    /// Subscribe to market data
    pub async fn subscribe_market_data(&self, symbol: String) -> Result<()> {
        if let Some(session) = &self.session {
            let mut session_guard = session.lock().await;
            session_guard.subscribe_market_data(symbol)
        } else {
            Err(DeribitFixError::Session("Not connected".to_string()))
        }
    }

    /// Get account positions
    pub async fn get_positions(&self) -> Result<Vec<Position>> {
        if let Some(session) = &self.session {
            let mut session_guard = session.lock().await;
            session_guard.request_positions()
        } else {
            Err(DeribitFixError::Session("Not connected".to_string()))
        }
    }

    /// Receive and process a message from the server
    pub async fn receive_message(&self) -> Result<Option<crate::model::message::FixMessage>> {
        if let Some(session) = &self.session {
            let mut session_guard = session.lock().await;
            session_guard.receive_and_process_message().await
        } else {
            Err(DeribitFixError::Session("Not connected".to_string()))
        }
    }
}
