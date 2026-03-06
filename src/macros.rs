/// Implements `std::fmt::Display` for one or more types using JSON serialization.
macro_rules! impl_json_display {
    ($($t:ty),+ $(,)?) => {
        $(
            impl std::fmt::Display for $t {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match serde_json::to_string(self) {
                        Ok(s) => write!(f, "{}", s),
                        Err(e) => write!(f, "{{\"error\": \"{}\"}}", e),
                    }
                }
            }
        )+
    };
}

/// Implements `std::fmt::Debug` for one or more types using pretty-printed JSON serialization.
macro_rules! impl_json_debug_pretty {
    ($($t:ty),+ $(,)?) => {
        $(
            impl std::fmt::Debug for $t {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match serde_json::to_string_pretty(self) {
                        Ok(s) => write!(f, "{}", s),
                        Err(e) => write!(f, "{{\"error\": \"{}\"}}", e),
                    }
                }
            }
        )+
    };
}
