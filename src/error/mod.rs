//! Error types for the Deribit FIX framework

use std::fmt;

/// Result type alias for the Deribit FIX framework
pub type Result<T> = std::result::Result<T, DeribitFixError>;

/// Main error type for the Deribit FIX framework
#[derive(Debug)]
pub enum DeribitFixError {
    /// Connection-related errors
    Connection(String),
    /// Authentication errors
    Authentication(String),
    /// Message parsing errors
    MessageParsing(String),
    MessageConstruction(String),
    /// Session management errors
    Session(String),
    /// Network I/O errors
    Io(std::io::Error),
    /// JSON serialization/deserialization errors
    Json(serde_json::Error),
    /// HTTP request errors
    Http(reqwest::Error),
    /// Configuration errors
    Config(String),
    /// Timeout errors
    Timeout(String),
    /// Protocol violation errors
    Protocol(String),
    /// Generic errors
    Generic(String),
}

impl fmt::Display for DeribitFixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeribitFixError::Connection(msg) => write!(f, "Connection error: {}", msg),
            DeribitFixError::Authentication(msg) => write!(f, "Authentication error: {}", msg),
            DeribitFixError::MessageParsing(msg) => write!(f, "Message parsing error: {}", msg),
            DeribitFixError::MessageConstruction(msg) => write!(f, "Message construction error: {}", msg),
            DeribitFixError::Session(msg) => write!(f, "Session error: {}", msg),
            DeribitFixError::Io(err) => write!(f, "I/O error: {}", err),
            DeribitFixError::Json(err) => write!(f, "JSON error: {}", err),
            DeribitFixError::Http(err) => write!(f, "HTTP error: {}", err),
            DeribitFixError::Config(msg) => write!(f, "Configuration error: {}", msg),
            DeribitFixError::Timeout(msg) => write!(f, "Timeout error: {}", msg),
            DeribitFixError::Protocol(msg) => write!(f, "Protocol error: {}", msg),
            DeribitFixError::Generic(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for DeribitFixError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DeribitFixError::Io(err) => Some(err),
            DeribitFixError::Json(err) => Some(err),
            DeribitFixError::Http(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for DeribitFixError {
    fn from(err: std::io::Error) -> Self {
        DeribitFixError::Io(err)
    }
}

impl From<serde_json::Error> for DeribitFixError {
    fn from(err: serde_json::Error) -> Self {
        DeribitFixError::Json(err)
    }
}

impl From<reqwest::Error> for DeribitFixError {
    fn from(err: reqwest::Error) -> Self {
        DeribitFixError::Http(err)
    }
}
