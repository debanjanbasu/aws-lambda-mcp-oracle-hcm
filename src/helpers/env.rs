use anyhow::{anyhow, Result};
use std::env;

// Attempts to load an environment variable, returning an error if it's not set or is empty.
pub fn load_env_var(key: &str) -> Result<String> {
    let value = env::var(key)?;
    if value.is_empty() {
        Err(anyhow!("Environment variable {key} is empty"))
    } else {
        Ok(value)
    }
}

// Loads an environment variable with a fallback default value if not set.
pub fn load_env_var_or(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}