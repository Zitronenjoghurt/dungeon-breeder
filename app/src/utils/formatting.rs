use chrono::{DateTime, Local, Utc};
use humantime::format_duration;
use std::time::Duration;

pub fn format_seconds(secs: u64) -> String {
    format_duration(Duration::from_secs(secs)).to_string()
}

pub fn format_bytes(bytes: u64) -> String {
    if bytes < 1_000 {
        format!("{} B", bytes)
    } else if bytes < 1_000_000 {
        format!("{:.2} KB", bytes as f64 / 1_000.0)
    } else if bytes < 1_000_000_000 {
        format!("{:.2} MB", bytes as f64 / 1_000_000.0)
    } else {
        format!("{:.2} GB", bytes as f64 / 1_000_000_000.0)
    }
}

pub fn format_date(date: DateTime<Utc>) -> String {
    date.with_timezone(&Local)
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}
