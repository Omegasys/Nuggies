use std::fmt;

#[derive(Debug)]
pub enum NuggiesError {
    ConfigError(String),
    SystemError(String),
    PermissionError(String),
    SandboxError(String),
    Unknown(String),
}

impl fmt::Display for NuggiesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NuggiesError::ConfigError(msg) => write!(f, "[config] {}", msg),
            NuggiesError::SystemError(msg) => write!(f, "[system] {}", msg),
            NuggiesError::PermissionError(msg) => write!(f, "[permission] {}", msg),
            NuggiesError::SandboxError(msg) => write!(f, "[sandbox] {}", msg),
            NuggiesError::Unknown(msg) => write!(f, "[unknown] {}", msg),
        }
    }
}

impl std::error::Error for NuggiesError {}
