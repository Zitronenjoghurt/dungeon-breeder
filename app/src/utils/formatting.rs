use chrono::{DateTime, Local, Utc};
use humantime::format_duration;
use num_traits::ToPrimitive;
use std::fmt::Display;
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

pub fn format_number<T>(value: T) -> String
where
    T: ToPrimitive + Display,
{
    const SUFFIXES: &[&str] = &["", "k", "M", "B", "T", "Q"];
    const THRESHOLD: f64 = 1_000.0;
    const EPSILON: f64 = 0.01;

    let Some(num_f64) = value.to_f64() else {
        return value.to_string();
    };

    if num_f64 < THRESHOLD {
        return value.to_string();
    }

    let mut num = num_f64;
    let mut suffix_index = 0;

    while num >= THRESHOLD - EPSILON && suffix_index < SUFFIXES.len() - 1 {
        num /= THRESHOLD;
        suffix_index += 1;
    }

    if num >= 100.0 - EPSILON {
        format!("{:.0}{}", num, SUFFIXES[suffix_index])
    } else if num >= 10.0 - EPSILON {
        format!("{:.1}{}", num, SUFFIXES[suffix_index])
    } else {
        format!("{:.2}{}", num, SUFFIXES[suffix_index])
    }
}
