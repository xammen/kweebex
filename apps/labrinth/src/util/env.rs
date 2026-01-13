use std::str::FromStr;

use eyre::{Context, eyre};

pub fn env_var(key: &str) -> eyre::Result<String> {
    dotenvy::var(key)
        .wrap_err_with(|| eyre!("missing environment variable `{key}`"))
}

pub fn parse_var<T: FromStr>(var: &str) -> Option<T> {
    dotenvy::var(var).ok().and_then(|i| i.parse().ok())
}
pub fn parse_strings_from_var(var: &'static str) -> Option<Vec<String>> {
    dotenvy::var(var).ok().and_then(|s| {
        // Try parsing as-is first
        if let Ok(v) = serde_json::from_str::<Vec<String>>(&s) {
            return Some(v);
        }
        // Handle escaped quotes from Docker/Coolify (e.g., [\"value\"] -> ["value"])
        let unescaped = s.replace("\\\"", "\"");
        if let Ok(v) = serde_json::from_str::<Vec<String>>(&unescaped) {
            return Some(v);
        }
        // Handle empty case
        if s.is_empty() || s == "[]" {
            return Some(Vec::new());
        }
        None
    })
}
