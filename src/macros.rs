/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 6/3/26
******************************************************************************/

//! Utility macros for Debug and Display implementations
//!
//! These macros provide JSON-based Debug and Display implementations for types
//! that implement Serialize.

/// Implements `Display` for types that implement `Serialize`.
/// The output is compact JSON format.
#[macro_export]
macro_rules! impl_json_display {
    ($($t:ty),+ $(,)?) => {
        $(
            impl std::fmt::Display for $t {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match serde_json::to_string(self) {
                        Ok(json) => write!(f, "{}", json),
                        Err(e) => write!(f, "<serialization error: {}>", e),
                    }
                }
            }
        )+
    };
}

/// Implements `Debug` for types that implement `Serialize`.
/// The output is pretty-printed JSON format.
#[macro_export]
macro_rules! impl_json_debug_pretty {
    ($($t:ty),+ $(,)?) => {
        $(
            impl std::fmt::Debug for $t {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match serde_json::to_string_pretty(self) {
                        Ok(json) => write!(f, "{}", json),
                        Err(e) => write!(f, "<serialization error: {}>", e),
                    }
                }
            }
        )+
    };
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    #[derive(Serialize)]
    struct TestStruct {
        name: String,
        value: i32,
    }

    impl_json_display!(TestStruct);
    impl_json_debug_pretty!(TestStruct);

    #[test]
    fn test_display() {
        let test = TestStruct {
            name: "test".to_string(),
            value: 42,
        };
        let display = format!("{}", test);
        assert!(display.contains("test"));
        assert!(display.contains("42"));
    }

    #[test]
    fn test_debug() {
        let test = TestStruct {
            name: "test".to_string(),
            value: 42,
        };
        let debug = format!("{:?}", test);
        assert!(debug.contains("test"));
        assert!(debug.contains("42"));
    }
}
