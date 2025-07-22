// Comprehensive unit tests for TCP Connection implementation

use deribit_fix::config::DeribitFixConfig;
use deribit_fix::connection::Connection;
use deribit_fix::error::DeribitFixError;
use deribit_fix::model::message::FixMessage;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to create a test config
    fn create_test_config() -> DeribitFixConfig {
        DeribitFixConfig::new()
            .with_credentials("test_user".to_string(), "test_pass".to_string())
            .with_endpoint("127.0.0.1".to_string(), 0) // Will be updated with actual port
            .with_session_ids("CLIENT".to_string(), "DERIBIT".to_string())
            .with_heartbeat_interval(30)
            .with_connection_timeout(Duration::from_millis(1000))
    }

    /// Helper function to create a mock FIX message
    fn create_test_message() -> FixMessage {
        use std::str::FromStr;
        FixMessage::from_str("8=FIX.4.4\x0135=0\x0149=CLIENT\x0156=DERIBIT\x0134=1\x0152=20240101-12:00:00.000\x0110=123\x01").unwrap()
    }

    #[tokio::test]
    async fn test_connection_creation_tcp() {
        // Start a mock TCP server
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        // Spawn server task
        tokio::spawn(async move {
            if let Ok((mut socket, _)) = listener.accept().await {
                // Just accept the connection and close it
                let _ = socket.shutdown().await;
            }
        });

        let mut config = create_test_config();
        config.host = addr.ip().to_string();
        config.port = addr.port();
        config.use_ssl = false;

        let connection = Connection::new(&config).await;
        assert!(connection.is_ok(), "TCP connection should succeed");

        let conn = connection.unwrap();
        assert!(
            conn.is_connected(),
            "Connection should be marked as connected"
        );
    }

    #[tokio::test]
    async fn test_connection_creation_timeout() {
        let mut config = create_test_config();
        config.host = "192.0.2.1".to_string(); // Non-routable IP for timeout test
        config.port = 12345;
        config.connection_timeout = Duration::from_millis(100);
        config.use_ssl = false;

        let connection = Connection::new(&config).await;
        assert!(connection.is_err(), "Connection should timeout");

        let error = connection.err().unwrap();
        match error {
            DeribitFixError::Timeout(msg) => {
                assert!(
                    msg.contains("Connection timeout"),
                    "Should be timeout error"
                );
            }
            _ => panic!("Expected timeout error"),
        }
    }

    #[tokio::test]
    async fn test_connection_creation_invalid_host() {
        let mut config = create_test_config();
        config.host = "invalid.host.that.does.not.exist".to_string();
        config.port = 12345;
        config.connection_timeout = Duration::from_millis(500);
        config.use_ssl = false;

        let connection = Connection::new(&config).await;
        assert!(
            connection.is_err(),
            "Connection to invalid host should fail"
        );

        let error = connection.err().unwrap();
        match error {
            DeribitFixError::Connection(msg) => {
                assert!(
                    msg.contains("Failed to connect"),
                    "Should be connection error"
                );
            }
            DeribitFixError::Timeout(_) => {
                // Also acceptable as DNS resolution might timeout
            }
            _ => panic!("Expected connection or timeout error"),
        }
    }

    #[tokio::test]
    async fn test_send_message_when_connected() {
        // Start a mock TCP server
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        // Spawn server task to receive data
        let received_data = std::sync::Arc::new(tokio::sync::Mutex::new(Vec::new()));
        let received_clone = received_data.clone();

        tokio::spawn(async move {
            if let Ok((mut socket, _)) = listener.accept().await {
                let mut buffer = vec![0u8; 1024];
                if let Ok(n) = socket.read(&mut buffer).await {
                    let mut data = received_clone.lock().await;
                    data.extend_from_slice(&buffer[..n]);
                }
            }
        });

        let mut config = create_test_config();
        config.host = addr.ip().to_string();
        config.port = addr.port();
        config.use_ssl = false;

        let mut connection = Connection::new(&config).await.unwrap();
        let message = create_test_message();

        let result = connection.send_message(&message).await;
        assert!(result.is_ok(), "Send message should succeed when connected");

        // Give some time for the server to receive data
        tokio::time::sleep(Duration::from_millis(100)).await;

        let data = received_data.lock().await;
        assert!(!data.is_empty(), "Server should have received data");
    }

    #[tokio::test]
    async fn test_send_message_when_disconnected() {
        let mut config = create_test_config();
        config.host = "127.0.0.1".to_string();
        config.port = 12345;
        config.use_ssl = false;

        // Create a connection but mark it as disconnected
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        config.port = addr.port();

        tokio::spawn(async move {
            if let Ok((mut socket, _)) = listener.accept().await {
                let _ = socket.shutdown().await;
            }
        });

        let mut connection = Connection::new(&config).await.unwrap();

        // Manually disconnect
        connection.close().await.unwrap();

        let message = create_test_message();
        let result = connection.send_message(&message).await;

        assert!(
            result.is_err(),
            "Send message should fail when disconnected"
        );
        match result.unwrap_err() {
            DeribitFixError::Connection(msg) => {
                assert!(
                    msg.contains("not active"),
                    "Should indicate connection not active"
                );
            }
            _ => panic!("Expected connection error"),
        }
    }

    #[tokio::test]
    async fn test_receive_message_when_disconnected() {
        let mut config = create_test_config();
        config.host = "127.0.0.1".to_string();
        config.port = 12345;
        config.use_ssl = false;

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        config.port = addr.port();

        tokio::spawn(async move {
            if let Ok((mut socket, _)) = listener.accept().await {
                let _ = socket.shutdown().await;
            }
        });

        let mut connection = Connection::new(&config).await.unwrap();

        // Manually disconnect
        connection.close().await.unwrap();

        let result = connection.receive_message().await;
        assert!(
            result.is_err(),
            "Receive message should fail when disconnected"
        );

        match result.unwrap_err() {
            DeribitFixError::Connection(msg) => {
                assert!(
                    msg.contains("Not connected"),
                    "Should indicate not connected"
                );
            }
            _ => panic!("Expected connection error"),
        }
    }

    #[tokio::test]
    async fn test_receive_message_timeout() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        // Server that accepts connection but sends no data
        tokio::spawn(async move {
            if let Ok((_socket, _)) = listener.accept().await {
                // Just keep the connection open without sending data
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        });

        let mut config = create_test_config();
        config.host = addr.ip().to_string();
        config.port = addr.port();
        config.use_ssl = false;

        let mut connection = Connection::new(&config).await.unwrap();

        // Should timeout and return None (no message available)
        let result = connection.receive_message().await;
        assert!(result.is_ok(), "Receive should not error on timeout");
        assert!(
            result.unwrap().is_none(),
            "Should return None when no data available"
        );
    }

    #[tokio::test]
    async fn test_receive_message_with_data() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        // Server that sends a FIX message
        tokio::spawn(async move {
            if let Ok((mut socket, _)) = listener.accept().await {
                let fix_message = "8=FIX.4.4\x0135=0\x0149=DERIBIT\x0156=CLIENT\x0134=1\x0152=20240101-12:00:00.000\x0110=123\x01";
                let _ = socket.write_all(fix_message.as_bytes()).await;
                let _ = socket.flush().await;
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        });

        let mut config = create_test_config();
        config.host = addr.ip().to_string();
        config.port = addr.port();
        config.use_ssl = false;

        let mut connection = Connection::new(&config).await.unwrap();

        // Should receive the message
        let result = connection.receive_message().await;
        assert!(result.is_ok(), "Receive should succeed");

        let message = result.unwrap();
        assert!(message.is_some(), "Should receive a message");
    }

    #[tokio::test]
    async fn test_receive_message_connection_closed() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        // Server that immediately closes connection
        tokio::spawn(async move {
            if let Ok((socket, _)) = listener.accept().await {
                drop(socket); // Close immediately
            }
        });

        let mut config = create_test_config();
        config.host = addr.ip().to_string();
        config.port = addr.port();
        config.use_ssl = false;

        let mut connection = Connection::new(&config).await.unwrap();

        // Give server time to close
        tokio::time::sleep(Duration::from_millis(50)).await;

        let result = connection.receive_message().await;
        assert!(result.is_ok(), "Should handle closed connection gracefully");
        assert!(
            result.unwrap().is_none(),
            "Should return None for closed connection"
        );
        assert!(
            !connection.is_connected(),
            "Connection should be marked as disconnected"
        );
    }

    #[tokio::test]
    async fn test_connection_close() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            if let Ok((mut socket, _)) = listener.accept().await {
                let _ = socket.shutdown().await;
            }
        });

        let mut config = create_test_config();
        config.host = addr.ip().to_string();
        config.port = addr.port();
        config.use_ssl = false;

        let mut connection = Connection::new(&config).await.unwrap();
        assert!(connection.is_connected(), "Should be connected initially");

        let result = connection.close().await;
        assert!(result.is_ok(), "Close should succeed");
        assert!(
            !connection.is_connected(),
            "Should be disconnected after close"
        );
    }

    #[tokio::test]
    async fn test_connection_reconnect() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        // Server that accepts multiple connections
        tokio::spawn(async move {
            loop {
                if let Ok((mut socket, _)) = listener.accept().await {
                    // Keep connection alive briefly then close
                    tokio::time::sleep(Duration::from_millis(50)).await;
                    let _ = socket.shutdown().await;
                }
            }
        });

        let mut config = create_test_config();
        config.host = addr.ip().to_string();
        config.port = addr.port();
        config.use_ssl = false;

        let mut connection = Connection::new(&config).await.unwrap();
        assert!(connection.is_connected(), "Should be connected initially");

        // Close the connection
        connection.close().await.unwrap();
        assert!(
            !connection.is_connected(),
            "Should be disconnected after close"
        );

        // Reconnect
        let result = connection.reconnect().await;
        assert!(result.is_ok(), "Reconnect should succeed");
        assert!(
            connection.is_connected(),
            "Should be connected after reconnect"
        );
    }

    #[tokio::test]
    async fn test_connection_reconnect_failure() {
        // Create a connection to a server that will close immediately
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        // Server that accepts one connection then stops
        tokio::spawn(async move {
            if let Ok((socket, _)) = listener.accept().await {
                drop(socket); // Close immediately
            }
            // Don't accept any more connections
        });

        let mut config = create_test_config();
        config.host = addr.ip().to_string();
        config.port = addr.port();
        config.use_ssl = false;
        config.connection_timeout = Duration::from_millis(100);

        let mut connection = Connection::new(&config).await.unwrap();

        // Wait for server to close
        tokio::time::sleep(Duration::from_millis(50)).await;

        // Try to reconnect - should fail since server is no longer accepting connections
        let result = connection.reconnect().await;
        assert!(
            result.is_err(),
            "Reconnect should fail when server is not accepting connections"
        );
    }

    #[tokio::test]
    async fn test_message_parsing_from_buffer() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        // Server that sends incomplete then complete message
        tokio::spawn(async move {
            if let Ok((mut socket, _)) = listener.accept().await {
                // Send partial message first
                let partial = "8=FIX.4.4\x0135=0\x01";
                let _ = socket.write_all(partial.as_bytes()).await;
                let _ = socket.flush().await;

                tokio::time::sleep(Duration::from_millis(50)).await;

                // Send rest of message
                let rest =
                    "49=DERIBIT\x0156=CLIENT\x0134=1\x0152=20240101-12:00:00.000\x0110=123\x01";
                let _ = socket.write_all(rest.as_bytes()).await;
                let _ = socket.flush().await;

                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        });

        let mut config = create_test_config();
        config.host = addr.ip().to_string();
        config.port = addr.port();
        config.use_ssl = false;

        let mut connection = Connection::new(&config).await.unwrap();

        // First receive should get partial message (None)
        let result1 = connection.receive_message().await;
        assert!(result1.is_ok(), "First receive should succeed");
        // May or may not get the message depending on timing

        // Second receive should get complete message
        let result2 = connection.receive_message().await;
        assert!(result2.is_ok(), "Second receive should succeed");

        // At least one of the receives should get the message
        let got_message = result1.unwrap().is_some() || result2.unwrap().is_some();
        assert!(
            got_message,
            "Should eventually receive the complete message"
        );
    }

    #[tokio::test]
    async fn test_invalid_message_parsing() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        // Server that sends invalid FIX message
        tokio::spawn(async move {
            if let Ok((mut socket, _)) = listener.accept().await {
                let invalid_message = "INVALID_FIX_MESSAGE\x0110=123\x01";
                let _ = socket.write_all(invalid_message.as_bytes()).await;
                let _ = socket.flush().await;
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        });

        let mut config = create_test_config();
        config.host = addr.ip().to_string();
        config.port = addr.port();
        config.use_ssl = false;

        let mut connection = Connection::new(&config).await.unwrap();

        let result = connection.receive_message().await;
        assert!(result.is_err(), "Should fail to parse invalid message");

        match result.unwrap_err() {
            DeribitFixError::MessageParsing(msg) => {
                assert!(msg.contains("Failed to parse"), "Should be parsing error");
            }
            _ => panic!("Expected message parsing error"),
        }
    }

    #[tokio::test]
    async fn test_connection_state_management() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            if let Ok((mut socket, _)) = listener.accept().await {
                let _ = socket.shutdown().await;
            }
        });

        let mut config = create_test_config();
        config.host = addr.ip().to_string();
        config.port = addr.port();
        config.use_ssl = false;

        let mut connection = Connection::new(&config).await.unwrap();

        // Test initial state
        assert!(connection.is_connected(), "Should be connected initially");

        // Test after close
        connection.close().await.unwrap();
        assert!(
            !connection.is_connected(),
            "Should be disconnected after close"
        );

        // Test operations on closed connection
        let message = create_test_message();
        let send_result = connection.send_message(&message).await;
        assert!(
            send_result.is_err(),
            "Send should fail on closed connection"
        );

        let receive_result = connection.receive_message().await;
        assert!(
            receive_result.is_err(),
            "Receive should fail on closed connection"
        );
    }
}
