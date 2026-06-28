use std::{
    io::{self, Write},
    thread,
    time::{Duration, Instant},
};

pub const MIN_MINUTES: u64 = 1;
pub const MAX_MINUTES: u64 = 480;

pub fn validate_minutes(minutes: u64) -> Result<(), String> {
    if !(MIN_MINUTES..=MAX_MINUTES).contains(&minutes) {
        return Err(format!(
            "minutes must be between {MIN_MINUTES} and {MAX_MINUTES}"
        ));
    }

    Ok(())
}

pub fn start(minutes: u64, show_complete_message: bool) -> io::Result<()> {
    validate_minutes(minutes)
        .map_err(|message| io::Error::new(io::ErrorKind::InvalidInput, message))?;

    let total_seconds = minutes * 60;
    println!("🍅 Local coding session started for {minutes} minute(s). Press Ctrl+C to stop it.");

    let start = Instant::now();
    loop {
        let elapsed = start.elapsed().as_secs();
        if elapsed >= total_seconds {
            break;
        }

        let remaining = total_seconds - elapsed;
        print!("\r⏳ Session remaining: {} ", format_duration(remaining));
        io::stdout().flush()?;
        thread::sleep(Duration::from_millis(250));
    }

    println!("\r✅ Session complete.                    ");
    if show_complete_message {
        println!("Stretch, drink water, and commit something you are proud of. 🚀");
    }

    Ok(())
}

pub fn format_duration(total_seconds: u64) -> String {
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{minutes:02}:{seconds:02}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_a_duration() {
        assert_eq!(format_duration(65), "01:05");
        assert_eq!(format_duration(3_600), "60:00");
    }

    #[test]
    fn rejects_invalid_minutes() {
        assert!(validate_minutes(0).is_err());
        assert!(validate_minutes(481).is_err());
        assert!(validate_minutes(25).is_ok());
    }
}
