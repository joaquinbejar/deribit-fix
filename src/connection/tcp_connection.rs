//! Connection management for Deribit FIX client

use crate::model::message::FixMessage;
use crate::model::stream::Stream;
use crate::{
    config::DeribitFixConfig,
    error::{DeribitFixError, Result},
};
use std::collections::VecDeque;
use std::str::FromStr;
use tokio::{net::TcpStream, time::timeout};
use tokio_native_tls::TlsConnector;
use tracing::{debug, error, info, trace};

/// TCP/TLS connection to Deribit FIX server
pub struct Connection {
    stream: Stream,
    config: DeribitFixConfig,
    buffer: Vec<u8>,
    message_queue: VecDeque<FixMessage>,
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
            message_queue: VecDeque::new(),
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

        // Check if we have queued messages first
        if let Some(message) = self.message_queue.pop_front() {
            return Ok(Some(message));
        }

        // Try to parse any existing buffered data first
        self.parse_all_messages_from_buffer()?;
        if let Some(message) = self.message_queue.pop_front() {
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
                trace!("Received {} bytes from server", n);
                trace!("Raw bytes: {:?}", &temp_buffer[..n]);
                self.buffer.extend_from_slice(&temp_buffer[..n]);

                // Parse all complete messages from buffer and queue them
                self.parse_all_messages_from_buffer()?;

                // Return the first message from queue
                Ok(self.message_queue.pop_front())
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

    /// Parse all complete messages from buffer and add to queue
    fn parse_all_messages_from_buffer(&mut self) -> Result<()> {
        while let Some(message) = self.try_parse_message()? {
            self.message_queue.push_back(message);
        }
        Ok(())
    }

    /// Try to parse a complete FIX message from the buffer
    fn try_parse_message(&mut self) -> Result<Option<FixMessage>> {
        if !self.buffer.is_empty() {
            trace!(
                "Buffer contains {} bytes: {:?}",
                self.buffer.len(),
                String::from_utf8_lossy(&self.buffer)
            );
        }

        // Look for SOH (Start of Header) character which delimits FIX fields
        const SOH: u8 = 0x01;

        // Find the beginning of a FIX message (looking for BeginString field)
        let buffer_str = String::from_utf8_lossy(&self.buffer);

        // Look for the start of a FIX message with BeginString (8=FIX.4.4)
        if let Some(msg_start) = buffer_str.find("8=FIX.4.4") {
            // For FIX messages, we need to check the BodyLength (tag 9) to know the complete message size
            let message_from_start = &buffer_str[msg_start..];

            // Parse the BodyLength to determine the complete message size
            if let Some(body_length_start) = message_from_start.find("9=")
                && let Some(body_length_end) =
                    message_from_start[body_length_start + 2..].find(char::from(SOH))
            {
                let body_length_str = &message_from_start
                    [body_length_start + 2..body_length_start + 2 + body_length_end];
                if let Ok(body_length) = body_length_str.parse::<usize>() {
                    // Calculate the total message length:
                    // "8=FIX.4.4\x01" + body_length + checksum field
                    let header_length = body_length_start + 2 + body_length_end + 1; // Up to and including SOH after BodyLength
                    let expected_total_length = msg_start + header_length + body_length;

                    // Check if we have the complete message
                    if self.buffer.len() >= expected_total_length {
                        let message_bytes = self
                            .buffer
                            .drain(msg_start..expected_total_length)
                            .collect::<Vec<u8>>();
                        let message_str = String::from_utf8_lossy(&message_bytes);

                        debug!(
                            "Received complete FIX message ({} bytes): {}",
                            message_bytes.len(),
                            message_str
                        );

                        // Parse the message
                        match FixMessage::from_str(&message_str) {
                            Ok(message) => return Ok(Some(message)),
                            Err(e) => {
                                return Err(DeribitFixError::MessageParsing(format!(
                                    "Failed to parse FIX message: {e}"
                                )));
                            }
                        }
                    } else {
                        debug!(
                            "Incomplete message: have {} bytes, need {}",
                            self.buffer.len(),
                            expected_total_length
                        );
                        return Ok(None);
                    }
                }
            }

            // Fallback to old checksum-based parsing if BodyLength parsing fails
            if let Some(checksum_pos) = message_from_start.find("10=") {
                debug!(
                    "Found checksum field at position {}",
                    msg_start + checksum_pos
                );

                // Find the SOH after the checksum (should be 3 digits + SOH)
                let checksum_section = &message_from_start[checksum_pos..];
                if let Some(end_pos) = checksum_section.find(char::from(SOH)) {
                    // Make sure we have the full 3-digit checksum
                    if end_pos >= 4 {
                        // "10=" + 3 digits = 7 chars minimum, but we'll be more lenient
                        let message_end = msg_start + checksum_pos + end_pos + 1;
                        let message_bytes = self
                            .buffer
                            .drain(msg_start..message_end)
                            .collect::<Vec<u8>>();
                        let message_str = String::from_utf8_lossy(&message_bytes);

                        debug!("Received FIX message (fallback): {}", message_str);

                        // Parse the message
                        match FixMessage::from_str(&message_str) {
                            Ok(message) => Ok(Some(message)),
                            Err(e) => Err(DeribitFixError::MessageParsing(format!(
                                "Failed to parse FIX message: {e}"
                            ))),
                        }
                    } else {
                        // Incomplete checksum
                        Ok(None)
                    }
                } else {
                    // Incomplete message - checksum field found but no terminating SOH
                    Ok(None)
                }
            } else {
                // No complete message yet - found start but no checksum
                Ok(None)
            }
        } else {
            // No message start found yet - might be just leftover data or waiting for more
            // Clear any non-message data from the beginning of buffer
            if !buffer_str.is_empty() && !buffer_str.starts_with("8=FIX") {
                // Find if there's a message start somewhere in the buffer
                if let Some(start_pos) = buffer_str.find("8=FIX") {
                    // Remove garbage data before the message start
                    debug!(
                        "Removing {} bytes of garbage data before message start",
                        start_pos
                    );
                    self.buffer.drain(..start_pos);
                } else {
                    // No message start found, could be fragment - keep if small, discard if too large
                    if self.buffer.len() > 1000 {
                        debug!(
                            "Clearing large buffer ({} bytes) with no message start",
                            self.buffer.len()
                        );
                        self.buffer.clear();
                    } else if self.buffer.len() > 10 && !buffer_str.trim().is_empty() {
                        // Check if this looks like invalid data (not starting with FIX fields)
                        let trimmed = buffer_str.trim();
                        // Valid FIX fragments should contain field numbers like "10=", "35=", etc.
                        // or be very short (under certain threshold)
                        if !trimmed.contains('=') || (!trimmed.starts_with(char::is_numeric) && self.buffer.len() > 20) {
                            // This looks like invalid data, not a FIX message fragment
                            return Err(DeribitFixError::MessageParsing(format!(
                                "Failed to parse invalid message data: {}",
                                trimmed
                            )));
                        }
                    }
                    // Keep smaller fragments or those that look like valid FIX field fragments
                }
            }
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
        self.message_queue.clear();
        self.connected = true;

        info!("Successfully reconnected");
        Ok(())
    }
}
