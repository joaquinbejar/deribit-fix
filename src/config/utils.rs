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

/// Generates a unique, random identifier using a custom character set.
///
/// This function creates a 30-character-long identifier composed of
/// uppercase English letters ('A'-'Z') and digits ('0'-'9'). The generated
/// identifier is suitable for use in scenarios where a non-collision and
/// easy-to-read identifier is required, such as database keys, tokens,
/// or URLs.
///
/// # Returns
///
/// A `String` containing the randomly generated identifier.
///
/// # Example
///
/// ```rust
/// use deribit_fix::config::gen_id;
/// let id = gen_id();
/// println!("Generated ID: {}", id); // Example output: "A1B2C3D4E5F6G7H8I9J0KLMNOPQRSTU"
/// ```
///
/// # Dependencies
///
/// This function uses the `nanoid` crate to generate the random identifier.
pub fn gen_id() -> String {
    let alphabet: [char; 36] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
        'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    nanoid::nanoid!(30, &alphabet)
}
