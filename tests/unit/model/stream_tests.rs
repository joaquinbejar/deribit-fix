// Unit tests for Stream functionality

use deribit_fix::error::DeribitFixError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_error() {
        let error = DeribitFixError::Io(std::io::Error::new(
            std::io::ErrorKind::BrokenPipe,
            "Stream broken",
        ));
        let display_str = format!("{error}");
        assert!(display_str.contains("I/O error"));
        assert!(display_str.contains("Stream broken"));
    }

    #[test]
    fn test_stream_timeout_error() {
        let error = DeribitFixError::Timeout("Stream read timeout".to_string());
        let display_str = format!("{error}");
        assert!(display_str.contains("Timeout error"));
        assert!(display_str.contains("Stream read timeout"));
    }

    #[test]
    fn test_stream_connection_error() {
        let error = DeribitFixError::Connection("Stream connection lost".to_string());
        let display_str = format!("{error}");
        assert!(display_str.contains("Connection error"));
        assert!(display_str.contains("Stream connection lost"));
    }

    #[test]
    fn test_stream_protocol_error() {
        let error = DeribitFixError::Protocol("Invalid stream data".to_string());
        let display_str = format!("{error}");
        assert!(display_str.contains("Protocol error"));
        assert!(display_str.contains("Invalid stream data"));
    }

    #[test]
    fn test_stream_message_parsing_error() {
        let error = DeribitFixError::MessageParsing("Cannot parse stream message".to_string());
        let display_str = format!("{error}");
        assert!(display_str.contains("Message parsing error"));
        assert!(display_str.contains("Cannot parse stream message"));
    }
}
