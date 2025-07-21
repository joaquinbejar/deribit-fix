//! Utility functions for the Deribit FIX framework
pub(crate) mod logger;

use chrono::{DateTime, Utc};
use base64::prelude::*;
use rand::{rng, Rng};
use std::time::{SystemTime, UNIX_EPOCH};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Setup logging with configurable level
pub fn setup_logger() {
    let log_level = std::env::var("DERIBIT_LOG_LEVEL")
        .unwrap_or_else(|_| "info".to_string())
        .to_lowercase();
    
    let level = match log_level.as_str() {
        "trace" => tracing::Level::TRACE,
        "debug" => tracing::Level::DEBUG,
        "info" => tracing::Level::INFO,
        "warn" => tracing::Level::WARN,
        "error" => tracing::Level::ERROR,
        _ => tracing::Level::INFO,
    };
    
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(level.to_string()))
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

/// Generate a cryptographically secure random nonce
pub fn generate_nonce(length: usize) -> String {
    let mut rng = rng();
    let bytes: Vec<u8> = (0..length).map(|_| rng.random()).collect();
    base64::prelude::BASE64_STANDARD.encode(bytes)
}

/// Generate a timestamp in milliseconds since Unix epoch
pub fn generate_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

/// Format a DateTime for FIX SendingTime field (YYYYMMDD-HH:MM:SS.sss)
pub fn format_fix_time(time: DateTime<Utc>) -> String {
    time.format("%Y%m%d-%H:%M:%S%.3f").to_string()
}

/// Parse a FIX time string to DateTime
pub fn parse_fix_time(time_str: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    DateTime::parse_from_str(&format!("{}+00:00", time_str), "%Y%m%d-%H:%M:%S%.3f%z")
        .map(|dt| dt.with_timezone(&Utc))
}

/// Calculate FIX checksum for a message
pub fn calculate_checksum(message: &str) -> u8 {
    let sum: u32 = message.bytes().map(|b| b as u32).sum();
    (sum % 256) as u8
}

/// Validate FIX checksum
pub fn validate_checksum(message: &str) -> bool {
    if let Some(checksum_pos) = message.rfind("10=") {
        let message_without_checksum = &message[..checksum_pos];
        let expected_checksum = calculate_checksum(message_without_checksum);
        
        if let Some(checksum_str) = message[checksum_pos + 3..].split('\x01').next() {
            if let Ok(actual_checksum) = checksum_str.parse::<u8>() {
                return expected_checksum == actual_checksum;
            }
        }
    }
    false
}

/// Generate a unique client order ID
pub fn generate_client_order_id(prefix: &str) -> String {
    format!("{}_{}", prefix, generate_timestamp())
}

/// Convert price to FIX decimal format
pub fn format_price(price: f64, precision: usize) -> String {
    format!("{:.precision$}", price, precision = precision)
}

/// Convert quantity to FIX decimal format
pub fn format_quantity(quantity: f64, precision: usize) -> String {
    format!("{:.precision$}", quantity, precision = precision)
}

/// Parse FIX decimal string to f64
pub fn parse_decimal(decimal_str: &str) -> Result<f64, std::num::ParseFloatError> {
    decimal_str.parse::<f64>()
}

/// Escape special characters in FIX field values
pub fn escape_fix_value(value: &str) -> String {
    value.replace('\x01', "\\001") // SOH
}

/// Unescape special characters in FIX field values
pub fn unescape_fix_value(value: &str) -> String {
    value.replace("\\001", "\x01") // SOH
}

/// Generate a random request ID
pub fn generate_request_id(prefix: &str) -> String {
    let mut rng = rng();
    let random_part: u32 = rng.random();
    format!("{}_{}", prefix, random_part)
}

/// Convert side enum to FIX side value
pub fn side_to_fix(side: crate::client::OrderSide) -> &'static str {
    match side {
        crate::client::OrderSide::Buy => "1",
        crate::client::OrderSide::Sell => "2",
    }
}

/// Convert order type enum to FIX order type value
pub fn order_type_to_fix(order_type: crate::client::OrderType) -> &'static str {
    match order_type {
        crate::client::OrderType::Market => "1",
        crate::client::OrderType::Limit => "2",
        crate::client::OrderType::Stop => "3",
        crate::client::OrderType::StopLimit => "4",
    }
}

/// Convert time in force enum to FIX time in force value
pub fn time_in_force_to_fix(tif: crate::client::TimeInForce) -> &'static str {
    match tif {
        crate::client::TimeInForce::Day => "0",
        crate::client::TimeInForce::GoodTillCancel => "1",
        crate::client::TimeInForce::ImmediateOrCancel => "3",
        crate::client::TimeInForce::FillOrKill => "4",
    }
}

/// Validate instrument name format for Deribit
pub fn validate_instrument_name(instrument: &str) -> bool {
    // Basic validation for Deribit instrument naming convention
    // Examples: BTC-PERPETUAL, ETH-25DEC20-600-C, BTC-25DEC20
    if instrument.is_empty() {
        return false;
    }
    
    // Must contain at least one dash
    if !instrument.contains('-') {
        return false;
    }
    
    // Must start with a valid currency
    let valid_currencies = ["BTC", "ETH", "USD", "USDC"];
    let starts_with_valid_currency = valid_currencies.iter()
        .any(|&currency| instrument.starts_with(currency));
    
    starts_with_valid_currency
}

/// Extract currency from instrument name
pub fn extract_currency_from_instrument(instrument: &str) -> Option<&str> {
    if let Some(dash_pos) = instrument.find('-') {
        Some(&instrument[..dash_pos])
    } else {
        None
    }
}

/// Format instrument name for Deribit
pub fn format_deribit_instrument(
    currency: &str,
    expiry: Option<&str>,
    strike: Option<f64>,
    option_type: Option<&str>,
) -> String {
    let mut instrument = currency.to_string();
    
    if let Some(exp) = expiry {
        instrument.push('-');
        instrument.push_str(exp);
        
        if let Some(strike_price) = strike {
            instrument.push('-');
            instrument.push_str(&strike_price.to_string());
            
            if let Some(opt_type) = option_type {
                instrument.push('-');
                instrument.push_str(opt_type);
            }
        }
    } else {
        // Perpetual contract
        instrument.push_str("-PERPETUAL");
    }
    
    instrument
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_nonce() {
        let nonce1 = generate_nonce(32);
        let nonce2 = generate_nonce(32);
        
        assert_ne!(nonce1, nonce2);
        assert!(!nonce1.is_empty());
        assert!(!nonce2.is_empty());
    }

    #[test]
    fn test_checksum_calculation() {
        let message = "8=FIX.4.4\x019=61\x0135=A\x0149=CLIENT\x0156=DERIBITSERVER\x0134=1\x01";
        let checksum = calculate_checksum(message);
        assert!(checksum <= 255); 
    }

    #[test]
    fn test_instrument_validation() {
        assert!(validate_instrument_name("BTC-PERPETUAL"));
        assert!(validate_instrument_name("ETH-25DEC20-600-C"));
        assert!(validate_instrument_name("BTC-25DEC20"));
        assert!(!validate_instrument_name("INVALID"));
        assert!(!validate_instrument_name(""));
    }

    #[test]
    fn test_currency_extraction() {
        assert_eq!(extract_currency_from_instrument("BTC-PERPETUAL"), Some("BTC"));
        assert_eq!(extract_currency_from_instrument("ETH-25DEC20-600-C"), Some("ETH"));
        assert_eq!(extract_currency_from_instrument("INVALID"), None);
    }

    #[test]
    fn test_instrument_formatting() {
        assert_eq!(
            format_deribit_instrument("BTC", None, None, None),
            "BTC-PERPETUAL"
        );
        assert_eq!(
            format_deribit_instrument("ETH", Some("25DEC20"), Some(600.0), Some("C")),
            "ETH-25DEC20-600-C"
        );
        assert_eq!(
            format_deribit_instrument("BTC", Some("25DEC20"), None, None),
            "BTC-25DEC20"
        );
    }
}
