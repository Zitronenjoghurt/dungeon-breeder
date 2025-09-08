use humantime::format_duration;
use std::time::Duration;

pub fn format_seconds(secs: u64) -> String {
    format_duration(Duration::from_secs(secs)).to_string()
}
