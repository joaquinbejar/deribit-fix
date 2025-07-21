/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/
use std::env;
use std::fmt::Debug;
use std::str::FromStr;
use tracing::error;

/// Get environment variable or return default value
pub fn get_env_or_default<T: FromStr>(env_var: &str, default: T) -> T
where
    <T as FromStr>::Err: Debug,
{
    match env::var(env_var) {
        Ok(val) => val.parse::<T>().unwrap_or_else(|_| {
            error!("Failed to parse {}: {}, using default", env_var, val);
            default
        }),
        Err(_) => default,
    }
}

/// Get optional environment variable
pub fn get_env_optional<T: FromStr>(env_var: &str) -> Option<T>
where
    <T as FromStr>::Err: Debug,
{
    match env::var(env_var) {
        Ok(val) => val.parse::<T>().ok(),
        Err(_) => None,
    }
}
