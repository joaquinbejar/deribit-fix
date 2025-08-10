# TradingHours

## Overview

The `TradingHours` struct defines when trading is allowed for specific instruments, including regular hours, holidays, and special trading sessions.

## Purpose

- **Trading Schedule**: Defines regular trading hours and days
- **Holiday Management**: Specifies market holidays and closures
- **Session Types**: Differentiates between regular, extended, and special sessions
- **Time Zone Handling**: Manages trading times across different time zones

## Public Interface

### Struct Definition

```rust
pub struct TradingHours {
    pub timezone: String,
    pub regular_hours: RegularTradingHours,
    pub extended_hours: Option<ExtendedTradingHours>,
    pub holidays: Vec<Holiday>,
    pub special_sessions: Vec<SpecialSession>,
    pub is_24_7: bool,
    pub maintenance_windows: Vec<MaintenanceWindow>,
    pub trading_days: Vec<Weekday>,
}
```

### Key Methods

```rust
impl TradingHours {
    /// Create default trading hours (24/7)
    pub fn default() -> Self

    /// Create traditional market hours (9:30 AM - 4:00 PM EST)
    pub fn traditional_market() -> Self

    /// Create crypto exchange hours (24/7 with maintenance windows)
    pub fn crypto_exchange() -> Self

    /// Check if trading is currently allowed
    pub fn is_trading_open(&self, timestamp: DateTime<Utc>) -> bool

    /// Get next trading session start time
    pub fn next_session_start(&self, from: DateTime<Utc>) -> Option<DateTime<Utc>>

    /// Get next trading session end time
    pub fn next_session_end(&self, from: DateTime<Utc>) -> Option<DateTime<Utc>>

    /// Check if specific date is a holiday
    pub fn is_holiday(&self, date: DateTime<Utc>) -> bool

    /// Get maintenance windows for a time period
    pub fn get_maintenance_windows(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<&MaintenanceWindow>

    /// Validate trading hours configuration
    pub fn validate(&self) -> Result<(), TradingHoursError>

    /// Convert time to exchange timezone
    pub fn to_exchange_time(&self, utc_time: DateTime<Utc>) -> DateTime<FixedOffset>
}
```

## Usage Examples

### Creating Trading Hours

```rust
use deribit_fix::types::{TradingHours, RegularTradingHours, ExtendedTradingHours, Holiday, SpecialSession, MaintenanceWindow, Weekday};
use chrono::{TimeZone, Utc, FixedOffset};

// Create 24/7 trading hours (default for crypto)
let crypto_hours = TradingHours::default();

// Create traditional market hours
let traditional_hours = TradingHours::traditional_market();

// Create custom trading hours
let custom_hours = TradingHours {
    timezone: "UTC".to_string(),
    regular_hours: RegularTradingHours {
        start_time: "09:00:00".to_string(),
        end_time: "17:00:00".to_string(),
        days: vec![Weekday::Monday, Weekday::Tuesday, Weekday::Wednesday, Weekday::Thursday, Weekday::Friday],
    },
    extended_hours: Some(ExtendedTradingHours {
        pre_market_start: "08:00:00".to_string(),
        post_market_end: "18:00:00".to_string(),
    }),
    holidays: vec![
        Holiday {
            date: "2024-01-01".to_string(),
            description: "New Year's Day".to_string(),
            is_trading_holiday: true,
        },
        Holiday {
            date: "2024-12-25".to_string(),
            description: "Christmas Day".to_string(),
            is_trading_holiday: true,
        },
    ],
    special_sessions: vec![
        SpecialSession {
            name: "Earnings Season".to_string(),
            start_date: "2024-01-15".to_string(),
            end_date: "2024-02-15".to_string(),
            extended_hours: true,
        },
    ],
    is_24_7: false,
    maintenance_windows: vec![
        MaintenanceWindow {
            day: Weekday::Sunday,
            start_time: "02:00:00".to_string(),
            duration_minutes: 120,
            description: "Weekly maintenance".to_string(),
        },
    ],
    trading_days: vec![
        Weekday::Monday, Weekday::Tuesday, Weekday::Wednesday, 
        Weekday::Thursday, Weekday::Friday
    ],
};
```

### Trading Status Checks

```rust
// Check if trading is currently open
let now = Utc::now();
if trading_hours.is_trading_open(now) {
    println!("Market is open for trading");
} else {
    println!("Market is closed");
}

// Get next trading session
if let Some(next_start) = trading_hours.next_session_start(now) {
    println!("Next trading session starts at: {}", next_start);
}

if let Some(next_end) = trading_hours.next_session_end(now) {
    println!("Current session ends at: {}", next_end);
}

// Check for holidays
let christmas = Utc.ymd(2024, 12, 25).and_hms(12, 0, 0);
if trading_hours.is_holiday(christmas) {
    println!("Christmas is a trading holiday");
}
```

### Maintenance Window Management

```rust
// Get maintenance windows for next week
let start = Utc::now();
let end = start + chrono::Duration::days(7);

let maintenance_windows = trading_hours.get_maintenance_windows(start, end);
for window in maintenance_windows {
    println!("Maintenance: {} on {} at {} for {} minutes", 
        window.description, 
        window.day, 
        window.start_time, 
        window.duration_minutes
    );
}

// Check if currently in maintenance
let now = Utc::now();
let current_maintenance = trading_hours.get_maintenance_windows(now, now);
if !current_maintenance.is_empty() {
    println!("Currently in maintenance window");
}
```

### Time Zone Handling

```rust
// Convert UTC time to exchange timezone
let utc_time = Utc::now();
let exchange_time = trading_hours.to_exchange_time(utc_time);

println!("UTC: {}", utc_time);
println!("Exchange time ({}): {}", trading_hours.timezone, exchange_time);

// Create timezone-aware trading hours
let est_hours = TradingHours {
    timezone: "America/New_York".to_string(),
    regular_hours: RegularTradingHours {
        start_time: "09:30:00".to_string(),
        end_time: "16:00:00".to_string(),
        days: vec![Weekday::Monday, Weekday::Tuesday, Weekday::Wednesday, Weekday::Thursday, Weekday::Friday],
    },
    // ... other fields
    ..TradingHours::default()
};
```

## Module Dependencies

### Direct Dependencies

- **`types`**: `RegularTradingHours`, `ExtendedTradingHours`, `Holiday`, `SpecialSession`, `MaintenanceWindow`, `Weekday`
- **`chrono`**: `DateTime<Utc>`, `DateTime<FixedOffset>`, `TimeZone`
- **`error`**: `TradingHoursError`

### Related Types

- **`RegularTradingHours`**: Standard trading session times
- **`ExtendedTradingHours`**: Pre-market and post-market sessions
- **`Holiday`**: Market holidays and closures
- **`SpecialSession`**: Special trading sessions (earnings, etc.)
- **`MaintenanceWindow`**: Scheduled maintenance periods
- **`Weekday`**: Days of the week for trading
- **`TradingHoursError`**: Specific error types for trading hours

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_default_trading_hours() {
        let hours = TradingHours::default();
        assert!(hours.is_24_7);
        assert_eq!(hours.timezone, "UTC");
    }

    #[test]
    fn test_traditional_market_hours() {
        let hours = TradingHours::traditional_market();
        assert!(!hours.is_24_7);
        assert_eq!(hours.timezone, "America/New_York");
        assert_eq!(hours.regular_hours.start_time, "09:30:00");
        assert_eq!(hours.regular_hours.end_time, "16:00:00");
    }

    #[test]
    fn test_trading_status_check() {
        let hours = TradingHours::default();
        let now = Utc::now();
        
        // 24/7 trading should always be open
        assert!(hours.is_trading_open(now));
    }

    #[test]
    fn test_holiday_check() {
        let mut hours = TradingHours::default();
        hours.holidays = vec![
            Holiday {
                date: "2024-01-01".to_string(),
                description: "New Year's Day".to_string(),
                is_trading_holiday: true,
            },
        ];

        let new_year = Utc.ymd(2024, 1, 1).and_hms(12, 0, 0);
        assert!(hours.is_holiday(new_year));
    }

    #[test]
    fn test_maintenance_window_retrieval() {
        let hours = TradingHours::default();
        let start = Utc::now();
        let end = start + chrono::Duration::days(1);

        let windows = hours.get_maintenance_windows(start, end);
        // Should return maintenance windows within the specified period
        assert!(windows.len() >= 0);
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_trading_hours_validation() {
    let hours = TradingHours::default();
    let result = hours.validate();
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_next_session_calculation() {
    let hours = TradingHours::traditional_market();
    let now = Utc::now();

    let next_start = hours.next_session_start(now);
    let next_end = hours.next_session_end(now);

    // Both should be Some for valid trading hours
    assert!(next_start.is_some());
    assert!(next_end.is_some());
}
```

## Performance Considerations

- **Caching**: Cache trading status results for frequently accessed time periods
- **Efficient Date Operations**: Use optimized date/time calculations
- **Memory Usage**: Minimize memory footprint for large holiday/maintenance lists
- **Time Zone Conversions**: Cache timezone conversion results

## Security Considerations

- **Time Validation**: Validate all time inputs and prevent timezone manipulation
- **Holiday Data**: Ensure holiday data is accurate and up-to-date
- **Maintenance Windows**: Secure access to maintenance schedule modifications
- **Audit Logging**: Log all trading hours configuration changes
