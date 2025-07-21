//! Connection management for Deribit FIX client

use crate::{
    config::Config,
    error::{DeribitFixError, Result},
    message::FixMessage,
};
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::timeout,
};
use tokio_native_tls::{TlsConnector, TlsStream};
use tracing::{debug, error, info, warn};

/// Connection wrapper for both TCP and TLS streams
pub enum Stream {
    Tcp(TcpStream),
    Tls(TlsStream<TcpStream>),
}

impl Stream {
    async fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Stream::Tcp(stream) => stream.read(buf).await,
            Stream::Tls(stream) => stream.read(buf).await,
        }
    }

    async fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        match self {
            Stream::Tcp(stream) => stream.write_all(buf).await,
            Stream::Tls(stream) => stream.write_all(buf).await,
        }
    }

    async fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Stream::Tcp(stream) => stream.flush().await,
            Stream::Tls(stream) => stream.flush().await,
        }
    }
}

/// TCP/TLS connection to Deribit FIX server
pub struct Connection {
    stream: Stream,
    config: Config,
    buffer: Vec<u8>,
    connected: bool,
}

impl Connection {
    /// Create a new connection to the Deribit FIX server
    pub async fn new(config: &Config) -> Result<Self> {
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
    async fn connect_tcp(config: &Config) -> Result<Stream> {
        info!("Connecting to {}:{} via TCP", config.host, config.port);
        
        let addr = format!("{}:{}", config.host, config.port);
        let stream = timeout(config.connection_timeout, TcpStream::connect(&addr))
            .await
            .map_err(|_| DeribitFixError::Timeout(format!("Connection timeout to {}", addr)))?
            .map_err(|e| DeribitFixError::Connection(format!("Failed to connect to {}: {}", addr, e)))?;

        info!("Successfully connected via TCP");
        Ok(Stream::Tcp(stream))
    }

    /// Connect using TLS
    async fn connect_tls(config: &Config) -> Result<Stream> {
        info!("Connecting to {}:{} via TLS", config.host, config.port);
        
        let addr = format!("{}:{}", config.host, config.port);
        let tcp_stream = timeout(config.connection_timeout, TcpStream::connect(&addr))
            .await
            .map_err(|_| DeribitFixError::Timeout(format!("Connection timeout to {}", addr)))?
            .map_err(|e| DeribitFixError::Connection(format!("Failed to connect to {}: {}", addr, e)))?;

        let connector = TlsConnector::from(
            native_tls::TlsConnector::builder()
                .build()
                .map_err(|e| DeribitFixError::Connection(format!("TLS connector creation failed: {}", e)))?
        );

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
            return Err(DeribitFixError::Connection("Connection is closed".to_string()));
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
            return Err(DeribitFixError::Connection("Connection is closed".to_string()));
        }

        // Read data into buffer
        let mut temp_buffer = [0u8; 4096];
        let bytes_read = self.stream
            .read(&mut temp_buffer)
            .await
            .map_err(|e| DeribitFixError::Io(e))?;

        if bytes_read == 0 {
            // Connection closed by peer
            self.connected = false;
            return Ok(None);
        }

        self.buffer.extend_from_slice(&temp_buffer[..bytes_read]);

        // Try to parse a complete FIX message
        if let Some(message) = self.try_parse_message()? {
            debug!("Received FIX message: {}", message);
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
            let message_str = String::from_utf8(message_bytes)
                .map_err(|e| DeribitFixError::MessageParsing(format!("Invalid UTF-8: {}", e)))?;
            
            let message = FixMessage::parse(&message_str)?;
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
