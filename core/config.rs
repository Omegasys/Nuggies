use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub debug: bool,
    pub log_level: String,
    pub default_format: String,
    pub enable_snap: bool,
    pub offline_mode: bool,
    pub strict_security: bool,
}

impl Config {
    pub fn load() -> Self {
        Self {
            debug: get_bool("NUGGIES_DEBUG", false),
            log_level: get_str("NUGGIES_LOG_LEVEL", "info"),
            default_format: get_str("NUGGIES_DEFAULT_FORMAT", "auto"),
            enable_snap: get_bool("NUGGIES_ENABLE_SNAP", false),
            offline_mode: get_bool("NUGGIES_OFFLINE_MODE", false),
            strict_security: get_bool("NUGGIES_STRICT_SECURITY", true),
        }
    }
}

fn get_str(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

fn get_bool(key: &str, default: bool) -> bool {
    match env::var(key) {
        Ok(val) => matches!(val.as_str(), "1" | "true" | "yes"),
        Err(_) => default,
    }
}
