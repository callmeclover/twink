use iced::time::Duration;

/// Converts a `Duration` to a `String` with the form `HH:mm:ss`.
pub fn format_duration(duration: Duration) -> String {
    let total_seconds: u64 = duration.as_secs();
    let hours: u64 = total_seconds / 3600;
    let minutes: u64 = (total_seconds % 3600) / 60;
    let seconds: u64 = total_seconds % 60;

    format!("{hours:02}:{minutes:02}:{seconds:02}")
}
