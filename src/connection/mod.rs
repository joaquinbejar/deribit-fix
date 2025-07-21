//! Connection management for Deribit FIX client

use crate::model::stream::Stream;
use crate::{
    config::DeribitFixConfig,
    error::{DeribitFixError, Result},
};
use tokio::{
    net::TcpStream,
    time::timeout,
};
use tokio_native_tls::TlsConnector;
use tracing::{debug, info};
use crate::model::message::FixMessage;

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
            .map_err(|_| DeribitFixError::Timeout(format!("Connection timeout to {}", addr)))?
            .map_err(|e| {
                DeribitFixError::Connection(format!("Failed to connect to {}: {}", addr, e))
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
            .map_err(|_| DeribitFixError::Timeout(format!("Connection timeout to {}", addr)))?
            .map_err(|e| {
                DeribitFixError::Connection(format!("Failed to connect to {}: {}", addr, e))
            })?;

        let connector =
            TlsConnector::from(native_tls::TlsConnector::builder().build().map_err(|e| {
                DeribitFixError::Connection(format!("TLS connector creation failed: {}", e))
            })?);

        let tls_stream = connector
            .connect(&config.host, tcp_stream)
            .await
            .map_err(|e| DeribitFixError::Connection(format!("TLS handshake failed: {}", e)))?;

        info!("Successfully connected via TLS");
        Ok(Stream::Tls(tls_stream))
    }

    /// Send a FIX message
    pub async fn send_message(&mut self, message: &FixMessage) -> Result<()> {
        if !self.connected {
            return Err(DeribitFixError::Connection(
                "Connection is closed".to_string(),
            ));
        }

        let raw_message = message.to_string();
        debug!("Sending FIX message: {}", raw_message);

        self.stream
            .write_all(raw_message.as_bytes())
            .await
            .map_err(|e| DeribitFixError::Io(e))?;

        self.stream
            .flush()
            .await
            .map_err(|e| DeribitFixError::Io(e))?;

        Ok(())
    }

    /// Receive a FIX message
    pub async fn receive_message(&mut self) -> Result<Option<FixMessage>> {
        if !self.connected {
            return Err(DeribitFixError::Connection(
                "Connection is closed".to_string(),
            ));
        }

        // Read data into buffer
        let mut temp_buffer = [0u8; 4096];
        debug!("Waiting to read from stream...");
        let bytes_read = self
            .stream
            .read(&mut temp_buffer)
            .await
            .map_err(|e| DeribitFixError::Io(e))?;
        debug!("Read {} bytes from stream", bytes_read);

        if bytes_read == 0 {
            // Connection closed by peer
            self.connected = false;
            return Ok(None);
        }

        self.buffer.extend_from_slice(&temp_buffer[..bytes_read]);

        // Try to parse a complete FIX message
        if let Some(message) = self.try_parse_message()? {
            return Ok(Some(message));
        }

        Ok(None)
    }

    /// Try to parse a complete FIX message from the buffer
    fn try_parse_message(&mut self) -> Result<Option<FixMessage>> {
        // Look for SOH (Start of Header) character which separates FIX fields
        const SOH: u8 = 0x01;

        // Find the end of a complete message
        let mut msg_end = None;

        for (i, window) in self.buffer.windows(3).enumerate() {
            if window == b"10=" {
                // Look for SOH after checksum (3 digits)
                if i + 6 < self.buffer.len() && self.buffer[i + 6] == SOH {
                    msg_end = Some(i + 7);
                    break;
                }
            }
        }

        if let Some(end_pos) = msg_end {
            let message_bytes = self.buffer.drain(..end_pos).collect::<Vec<u8>>();
            let response = String::from_utf8_lossy(&message_bytes).to_string();
            debug!("Received FIX message: {}", response);
            let message = FixMessage::parse(&response)?;
            return Ok(Some(message));
        }

        Ok(None)
    }

    /// Check if the connection is active
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Close the connection
    pub async fn close(&mut self) -> Result<()> {
        self.connected = false;
        info!("Connection closed");
        Ok(())
    }

    /// Reconnect to the server
    pub async fn reconnect(&mut self) -> Result<()> {
        info!("Attempting to reconnect...");

        let stream = if self.config.use_ssl {
            Self::connect_tls(&self.config).await?
        } else {
            Self::connect_tcp(&self.config).await?
        };

        self.stream = stream;
        self.connected = true;
        self.buffer.clear();

        info!("Successfully reconnected");
        Ok(())
    }
}
