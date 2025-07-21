//! Deribit FIX client implementation

use crate::{
    config::DeribitFixConfig,
    connection::Connection,
    error::{DeribitFixError, Result},
    session::Session,
};
use deribit_base::prelude::{Position, fix::NewOrderRequest};
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

        // Spawn a task to listen for incoming messages
        let session_clone = self.session.as_ref().unwrap().clone();
        tokio::spawn(async move {
            loop {
                let mut session_guard = session_clone.lock().await;
                match session_guard.receive_and_process_message().await {
                    Ok(Some(_)) => { /* Message processed */ }
                    Ok(None) => {
                        // Connection closed
                        info!("Message listener task: connection closed.");
                        break;
                    }
                    Err(e) => {
                        tracing::error!("Message listener task error: {}", e);
                        break;
                    }
                }
            }
        });

        info!("Successfully connected to Deribit FIX server and listener task spawned");
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

    /// Get the current session state
    pub async fn get_session_state(&self) -> Option<crate::session::SessionState> {
        if let Some(session) = &self.session {
            let session_guard = session.lock().await;
            return Some(session_guard.state());
        }
        None
    }

    /// Perform FIX logon
    async fn logon(&self) -> Result<()> {
        let session = self
            .session
            .as_ref()
            .ok_or_else(|| DeribitFixError::Session("Session not initialized".to_string()))?;

        let mut session_guard = session.lock().await;
        session_guard.logon().await?;

        Ok(())
    }

    /// Send a new order
    pub async fn send_order(&self, order: NewOrderRequest) -> Result<String> {
        let session = self
            .session
            .as_ref()
            .ok_or_else(|| DeribitFixError::Session("Not connected".to_string()))?;

        let mut session_guard = session.lock().await;
        session_guard.send_new_order(order).await
    }

    /// Cancel an order
    pub async fn cancel_order(&self, order_id: String) -> Result<()> {
        let session = self
            .session
            .as_ref()
            .ok_or_else(|| DeribitFixError::Session("Not connected".to_string()))?;

        let mut session_guard = session.lock().await;
        session_guard.cancel_order(order_id).await
    }

    /// Subscribe to market data
    pub async fn subscribe_market_data(&self, symbol: String) -> Result<()> {
        let session = self
            .session
            .as_ref()
            .ok_or_else(|| DeribitFixError::Session("Not connected".to_string()))?;

        let mut session_guard = session.lock().await;
        session_guard.subscribe_market_data(symbol).await
    }

    /// Get account positions
    pub async fn get_positions(&self) -> Result<Vec<Position>> {
        let session = self
            .session
            .as_ref()
            .ok_or_else(|| DeribitFixError::Session("Not connected".to_string()))?;

        let mut session_guard = session.lock().await;
        session_guard.request_positions().await
    }
}
