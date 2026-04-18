use std::time::{SystemTime, UNIX_EPOCH};

pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

pub fn log(level: LogLevel, message: &str) {
    let timestamp = now();

    let level_str = match level {
        LogLevel::Error => "ERROR",
        LogLevel::Warn => "WARN ",
        LogLevel::Info => "INFO ",
        LogLevel::Debug => "DEBUG",
    };

    println!("[{}] [{}] {}", timestamp, level_str, message);
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
