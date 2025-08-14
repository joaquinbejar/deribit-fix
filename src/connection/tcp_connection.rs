//! Connection management for Deribit FIX client

use crate::model::message::FixMessage;
use crate::model::stream::Stream;
use crate::{
    config::DeribitFixConfig,
    error::{DeribitFixError, Result},
};
use std::str::FromStr;
use tokio::{net::TcpStream, time::timeout};
use tokio_native_tls::TlsConnector;
use tracing::{debug, error, info};

/// TCP/TLS connection to Deribit FIX server
pub struct Connection {
    stream: Stream,
    config: DeribitFixConfig,
    buffer: Vec<u8>,
    connected: bool,
}

impl Connection {
    /// Create a new connection to the Deribit FIX server
    pub async fn new(config: &DeribitFixConfig) -> Result<Self> {
        let stream = if config.use_ssl {
            Self::connect_tls(config).await?
        } else {
            Self::connect_tcp(config).await?
        };

        Ok(Self {
            stream,
            config: config.clone(),
            buffer: Vec::with_capacity(8192),
            connected: true,
        })
    }

    /// Connect using raw TCP
    async fn connect_tcp(config: &DeribitFixConfig) -> Result<Stream> {
        info!("Connecting to {}:{} via TCP", config.host, config.port);

        let addr = format!("{}:{}", config.host, config.port);
        let stream = timeout(config.connection_timeout, TcpStream::connect(&addr))
            .await
            .map_err(|_| DeribitFixError::Timeout(format!("Connection timeout to {addr}")))?
            .map_err(|e| {
                DeribitFixError::Connection(format!("Failed to connect to {addr}: {e}"))
            })?;

        info!("Successfully connected via TCP");
        Ok(Stream::Tcp(stream))
    }

    /// Connect using TLS
    async fn connect_tls(config: &DeribitFixConfig) -> Result<Stream> {
        info!("Connecting to {}:{} via TLS", config.host, config.port);

        let addr = format!("{}:{}", config.host, config.port);
        let tcp_stream = timeout(config.connection_timeout, TcpStream::connect(&addr))
            .await
            .map_err(|_| DeribitFixError::Timeout(format!("Connection timeout to {addr}")))?
            .map_err(|e| {
                DeribitFixError::Connection(format!("Failed to connect to {addr}: {e}"))
            })?;

        let connector = TlsConnector::from(
            native_tls::TlsConnector::builder()
                .build()
                .map_err(|e| DeribitFixError::Connection(format!("TLS setup failed: {e}")))?,
        );

        let tls_stream = connector
            .connect(&config.host, tcp_stream)
            .await
            .map_err(|e| DeribitFixError::Connection(format!("TLS handshake failed: {e}")))?;

        info!("Successfully connected via TLS");
        Ok(Stream::Tls(tls_stream))
    }

    /// Send a FIX message
    pub async fn send_message(&mut self, message: &FixMessage) -> Result<()> {
        if !self.connected {
            return Err(DeribitFixError::Connection(
                "Connection is not active".to_string(),
            ));
        }

        let message_str = message.to_string();
        debug!("Sending FIX message: {}", message_str);

        match self.stream.write_all(message_str.as_bytes()).await {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to send message: {}", e);
                self.connected = false;
                return Err(DeribitFixError::Io(e));
            }
        }

        match self.stream.flush().await {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Failed to flush stream: {}", e);
                self.connected = false;
                Err(DeribitFixError::Io(e))
            }
        }
    }

    /// Receive a FIX message from the server
    pub async fn receive_message(&mut self) -> Result<Option<FixMessage>> {
        if !self.connected {
            return Err(DeribitFixError::Connection(
                "Not connected to server".to_string(),
            ));
        }

        // Try to parse any existing buffered data first
        if !self.buffer.is_empty()
            && let Some(message) = self.try_parse_message()?
        {
            return Ok(Some(message));
        }

        // Read data from the stream with timeout
        let mut temp_buffer = vec![0u8; 4096];

        // Use a timeout to avoid blocking indefinitely
        match tokio::time::timeout(
            std::time::Duration::from_millis(1000), // Increased to 1 second
            self.stream.read(&mut temp_buffer),
        )
        .await
        {
            Ok(Ok(0)) => {
                // Connection closed
                debug!("Connection closed by server");
                self.connected = false;
                Ok(None)
            }
            Ok(Ok(n)) => {
                debug!("Received {} bytes from server", n);
                debug!("Raw bytes: {:?}", &temp_buffer[..n]);
                self.buffer.extend_from_slice(&temp_buffer[..n]);

                // Try to parse the new data
                self.try_parse_message()
            }
            Ok(Err(e)) => {
                if e.kind() == std::io::ErrorKind::WouldBlock {
                    // No data available right now
                    return Ok(None);
                }
                error!("IO error reading from server: {}", e);
                // Mark connection as inactive on IO errors
                self.connected = false;
                Err(DeribitFixError::Io(e))
            }
            Err(_) => {
                // Timeout - no data available
                Ok(None)
            }
        }
    }

    /// Try to parse a complete FIX message from the buffer
    fn try_parse_message(&mut self) -> Result<Option<FixMessage>> {
        if !self.buffer.is_empty() {
            debug!(
                "Buffer contains {} bytes: {:?}",
                self.buffer.len(),
                String::from_utf8_lossy(&self.buffer)
            );
        }

        // Look for SOH (Start of Header) character which delimits FIX fields
        const SOH: u8 = 0x01;

        // Find the end of a complete message (looking for checksum field)
        let buffer_str = String::from_utf8_lossy(&self.buffer);
        if let Some(checksum_pos) = buffer_str.find("10=") {
            debug!("Found checksum field at position {}", checksum_pos);
            // Find the SOH after the checksum (3 digits + SOH)
            if let Some(end_pos) = buffer_str[checksum_pos..].find(char::from(SOH)) {
                let message_end = checksum_pos + end_pos + 1;
                let message_bytes = self.buffer.drain(..message_end).collect::<Vec<u8>>();
                let message_str = String::from_utf8_lossy(&message_bytes);

                debug!("Received FIX message: {}", message_str);

                // Parse the message
                match FixMessage::from_str(&message_str) {
                    Ok(message) => Ok(Some(message)),
                    Err(e) => Err(DeribitFixError::MessageParsing(format!(
                        "Failed to parse FIX message: {e}"
                    ))),
                }
            } else {
                // Incomplete message
                Ok(None)
            }
        } else {
            // No complete message yet
            Ok(None)
        }
    }

    /// Check if the connection is active
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Close the connection
    pub async fn close(&mut self) -> Result<()> {
        self.connected = false;
        self.stream.shutdown().await.map_err(DeribitFixError::Io)?;
        info!("Connection closed");
        Ok(())
    }

    /// Reconnect to the server
    pub async fn reconnect(&mut self) -> Result<()> {
        info!("Reconnecting to Deribit FIX server");

        // Close existing connection
        let _ = self.close().await;

        // Create new connection
        let stream = if self.config.use_ssl {
            Self::connect_tls(&self.config).await?
        } else {
            Self::connect_tcp(&self.config).await?
        };

        self.stream = stream;
        self.buffer.clear();
        self.connected = true;

        info!("Successfully reconnected");
        Ok(())
    }
}
