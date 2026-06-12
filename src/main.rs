use clap::Parser;
use chrono::Duration;

use args::Command;

mod args;

fn main() {
    let config = args::Args::parse();
    match config.command {
        Command::Now {
            use_local_tz,
            timezone,
            format_str,
        } => {
            let output = if use_local_tz {
                let now = chrono::Local::now();
    
                match &format_str {
                    Some(fmt) => now.format(fmt).to_string(),
                    None => now.to_string(),
                }
            } else if let Some(tz_name) = timezone {
                let tz: chrono_tz::Tz = tz_name
                    .parse()
                    .unwrap_or_else(|_| panic!("Invalid timezone: {tz_name}"));
    
                let now = chrono::Utc::now().with_timezone(&tz);
    
                match &format_str {
                    Some(fmt) => now.format(fmt).to_string(),
                    None => now.to_string(),
                }
            } else {
                let now = chrono::Utc::now();
    
                match &format_str {
                    Some(fmt) => now.format(fmt).to_string(),
                    None => now.to_string(),
                }
            };
            println!("{output}");
        },
        Command::Add {
            delta,
            use_local_tz,
            timezone,
            format_str,
        } => {
            let duration = parse_duration(&delta)
                .unwrap_or_else(|e| panic!("{e}"));
        
            let output = if use_local_tz {
                let dt = chrono::Local::now() + duration;
        
                match &format_str {
                    Some(fmt) => dt.format(fmt).to_string(),
                    None => dt.to_string(),
                }
            } else if let Some(tz_name) = timezone {
                let tz: chrono_tz::Tz = tz_name
                    .parse()
                    .unwrap_or_else(|_| panic!("Invalid timezone: {tz_name}"));
        
                let dt = chrono::Utc::now()
                    .with_timezone(&tz)
                    + duration;
        
                match &format_str {
                    Some(fmt) => dt.format(fmt).to_string(),
                    None => dt.to_string(),
                }
            } else {
                let dt = chrono::Utc::now() + duration;
        
                match &format_str {
                    Some(fmt) => dt.format(fmt).to_string(),
                    None => dt.to_string(),
                }
            };
        
            println!("{output}");
        }
    }
}

fn parse_duration(s: &str) -> Result<Duration, String> {
    let mut total_micros: i64 = 0;

    for part in s.split_whitespace() {
        let unit_start = part
            .find(|c: char| c.is_ascii_alphabetic())
            .ok_or_else(|| format!("Missing unit in '{part}'"))?;

        let (value_str, unit) = part.split_at(unit_start);

        let value: f64 = value_str
            .parse()
            .map_err(|_| format!("Invalid number '{value_str}'"))?;

        let micros = match unit {
            "us" | "u" => value * 1.0,
            "ms" => value * 1_000.0,
            "s" => value * 1_000_000.0,
            "m" => value * 60.0 * 1_000_000.0,
            "h" => value * 60.0 * 60.0 * 1_000_000.0,
            "d" => value * 24.0 * 60.0 * 60.0 * 1_000_000.0,
            "w" => value * 7.0 * 24.0 * 60.0 * 60.0 * 1_000_000.0,
            // TODO: best way to handle months/years?
            "M" | "mon" | "month"  => value * 30.0 * 7.0 * 24.0 * 60.0 * 60.0 * 1_000_000.0,
            "y" | "yr" => value * 356.0 * 30.0 * 7.0 * 24.0 * 60.0 * 60.0 * 1_000_000.0,
            "c" | "cen" => value * 100.0 * 356.0 * 30.0 * 7.0 * 24.0 * 60.0 * 60.0 * 1_000_000.0,
            _ => return Err(format!("Unknown unit '{unit}'")),
        };

        total_micros += micros.round() as i64;
    }
    Ok(Duration::microseconds(total_micros))
}
