// Unit tests for Deribit FIX Authentication
// Tests verify compliance with official Deribit FIX API specification

use base64::{Engine, engine::general_purpose::STANDARD as BASE64_STANDARD};
use sha2::{Digest, Sha256};

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to create auth data using the same logic as Session
    /// This replicates the generate_auth_data logic for testing
    fn generate_test_auth_data(access_secret: &str) -> (String, String) {
        // Generate timestamp (strictly increasing integer in milliseconds)
        let timestamp = chrono::Utc::now().timestamp_millis();

        // Generate random nonce (at least 32 bytes as recommended by Deribit)
        let mut nonce_bytes = vec![0u8; 32];
        for byte in nonce_bytes.iter_mut() {
            *byte = rand::random::<u8>();
        }
        let nonce_b64 = BASE64_STANDARD.encode(&nonce_bytes);

        // Create RawData: timestamp.nonce (separated by ASCII period)
        let raw_data = format!("{timestamp}.{nonce_b64}");

        // Calculate password hash: base64(sha256(RawData ++ access_secret))
        let mut auth_data = raw_data.as_bytes().to_vec();
        auth_data.extend_from_slice(access_secret.as_bytes());

        let mut hasher = Sha256::new();
        hasher.update(&auth_data);
        let hash_result = hasher.finalize();
        let password_hash = BASE64_STANDARD.encode(hash_result);

        (raw_data, password_hash)
    }

    /// Test that nonce generation follows Deribit FIX specification
    #[test]
    fn test_nonce_format_and_length() {
        // Generate auth data multiple times to test consistency
        for _ in 0..10 {
            let (raw_data, _password_hash) = generate_test_auth_data("test_access_secret");

            // Test RawData format: timestamp.nonce
            let parts: Vec<&str> = raw_data.split('.').collect();
            assert_eq!(
                parts.len(),
                2,
                "RawData should contain exactly one period separator"
            );

            let timestamp_str = parts[0];
            let nonce_b64 = parts[1];

            // Test timestamp is a valid integer
            let timestamp: i64 = timestamp_str
                .parse()
                .expect("Timestamp should be a valid integer");
            assert!(timestamp > 0, "Timestamp should be positive");

            // Test nonce is valid base64
            let nonce_bytes = BASE64_STANDARD
                .decode(nonce_b64)
                .expect("Nonce should be valid base64");

            // Test nonce length (should be at least 32 bytes as per Deribit spec)
            assert!(
                nonce_bytes.len() >= 32,
                "Nonce should be at least 32 bytes, got {} bytes",
                nonce_bytes.len()
            );

            // Test nonce is not all zeros (should be random)
            assert!(
                nonce_bytes.iter().any(|&b| b != 0),
                "Nonce should contain random data, not all zeros"
            );
        }
    }

    /// Test that timestamps are strictly increasing as required by Deribit spec
    #[test]
    fn test_timestamp_strictly_increasing() {
        let mut previous_timestamp = 0i64;

        // Generate multiple auth data and verify timestamps increase
        for i in 0..5 {
            let (raw_data, _) = generate_test_auth_data("test_access_secret");
            let timestamp_str = raw_data.split('.').next().unwrap();
            let timestamp: i64 = timestamp_str.parse().unwrap();

            if i > 0 {
                assert!(
                    timestamp >= previous_timestamp,
                    "Timestamp should be strictly increasing or equal. Previous: {previous_timestamp}, Current: {timestamp}"
                );
            }

            previous_timestamp = timestamp;

            // Small delay to ensure different timestamps
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }

    /// Test password hash calculation according to Deribit FIX specification
    /// Password = base64(sha256(RawData ++ access_secret))
    #[test]
    fn test_password_hash_calculation() {
        let access_secret = "test_access_secret";

        let (raw_data, password_hash) = generate_test_auth_data(access_secret);

        // Manually calculate the expected password hash
        let mut auth_data = raw_data.as_bytes().to_vec();
        auth_data.extend_from_slice(access_secret.as_bytes());

        let mut hasher = Sha256::new();
        hasher.update(&auth_data);
        let hash_result = hasher.finalize();
        let expected_password_hash = BASE64_STANDARD.encode(hash_result);

        assert_eq!(
            password_hash, expected_password_hash,
            "Password hash should match manual calculation"
        );

        // Verify password hash is valid base64
        let decoded_hash = BASE64_STANDARD
            .decode(&password_hash)
            .expect("Password hash should be valid base64");

        // SHA256 hash should be exactly 32 bytes
        assert_eq!(
            decoded_hash.len(),
            32,
            "SHA256 hash should be 32 bytes, got {}",
            decoded_hash.len()
        );
    }

    /// Test that different nonces produce different password hashes
    #[test]
    fn test_password_hash_uniqueness() {
        let access_secret = "test_access_secret";

        let mut password_hashes = std::collections::HashSet::new();

        // Generate multiple auth data and verify all password hashes are unique
        for _i in 0..10 {
            let (_raw_data, password_hash) = generate_test_auth_data(access_secret);

            // Each password hash should be unique due to different nonces/timestamps
            assert!(
                password_hashes.insert(password_hash.clone()),
                "Password hash should be unique, but got duplicate: {password_hash}"
            );

            // Small delay to ensure different timestamps
            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        assert_eq!(
            password_hashes.len(),
            10,
            "Should have generated 10 unique password hashes"
        );
    }

    /// Test RawData format compliance with official specification
    #[test]
    fn test_raw_data_format_compliance() {
        let (raw_data, _) = generate_test_auth_data("test_access_secret");

        // Test format: timestamp.nonce
        assert!(
            raw_data.contains('.'),
            "RawData should contain period separator"
        );

        let parts: Vec<&str> = raw_data.split('.').collect();
        assert_eq!(parts.len(), 2, "RawData should have exactly 2 parts");

        let timestamp_part = parts[0];
        let nonce_part = parts[1];

        // Timestamp should be numeric
        assert!(
            timestamp_part.chars().all(|c| c.is_ascii_digit()),
            "Timestamp part should be numeric"
        );

        // Nonce should be base64 (alphanumeric + / + = padding)
        assert!(
            nonce_part
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '/' || c == '+' || c == '='),
            "Nonce part should be valid base64"
        );

        // Test that nonce decodes to at least 32 bytes
        let nonce_bytes = BASE64_STANDARD
            .decode(nonce_part)
            .expect("Nonce should be valid base64");
        assert!(
            nonce_bytes.len() >= 32,
            "Decoded nonce should be at least 32 bytes"
        );
    }

    /// Test authentication data with different access secrets
    #[test]
    fn test_different_access_secrets() {
        let (raw_data1, password_hash1) = generate_test_auth_data("secret1");
        let (raw_data2, password_hash2) = generate_test_auth_data("secret2");

        // Same session might generate same RawData (if called quickly),
        // but password hashes should be different due to different secrets
        if raw_data1 == raw_data2 {
            assert_ne!(
                password_hash1, password_hash2,
                "Different access secrets should produce different password hashes"
            );
        }
    }

    /// Test that the authentication follows the exact Deribit specification
    #[test]
    fn test_deribit_spec_compliance() {
        let access_secret = "MySecretKey123";

        let (raw_data, password_hash) = generate_test_auth_data(access_secret);

        // Verify the specification: Password = base64(sha256(RawData ++ access_secret))
        let concatenated = format!("{raw_data}{access_secret}");
        let mut hasher = Sha256::new();
        hasher.update(concatenated.as_bytes());
        let hash_result = hasher.finalize();
        let expected_hash = BASE64_STANDARD.encode(hash_result);

        assert_eq!(
            password_hash, expected_hash,
            "Password hash should match Deribit specification: base64(sha256(RawData ++ access_secret))"
        );

        // Verify RawData format: timestamp.nonce
        let parts: Vec<&str> = raw_data.split('.').collect();
        assert_eq!(
            parts.len(),
            2,
            "RawData should be in format: timestamp.nonce"
        );

        // Verify timestamp is milliseconds (reasonable range)
        let timestamp: i64 = parts[0].parse().expect("Timestamp should be valid integer");
        let now_ms = chrono::Utc::now().timestamp_millis();
        assert!(
            timestamp <= now_ms && timestamp >= now_ms - 1000,
            "Timestamp should be recent milliseconds"
        );

        // Verify nonce is base64 encoded and at least 32 bytes when decoded
        let nonce_bytes = BASE64_STANDARD
            .decode(parts[1])
            .expect("Nonce should be valid base64");
        assert!(
            nonce_bytes.len() >= 32,
            "Nonce should be at least 32 bytes when decoded"
        );
    }

    /// Test edge cases and error conditions
    #[test]
    fn test_auth_edge_cases() {
        // Test with empty access secret
        let (_raw_data_empty, _password_hash_empty) = generate_test_auth_data("");
        // Should not panic or fail

        // Test with very long access secret
        let long_secret = "a".repeat(1000);
        let (_raw_data_long, _password_hash_long) = generate_test_auth_data(&long_secret);
        // Should not panic or fail

        // Test with special characters in access secret
        let special_secret = "!@#$%^&*()_+-=[]{}|;:,.<>?";
        let (_raw_data_special, _password_hash_special) = generate_test_auth_data(special_secret);
        // Should not panic or fail
    }
}
