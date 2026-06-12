use chrono::Duration;

pub fn parse_duration(s: &str) -> Result<Duration, String> {
    if !s.chars().any(|c| c.is_ascii_alphabetic()) {
        return parse_timestamp_duration(s);
    }

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
            "us" | "u" => value,
            "ms" => value * 1_000.0,
            "s" => value * 1_000_000.0,
            "m" => value * 60.0 * 1_000_000.0,
            "h" => value * 60.0 * 60.0 * 1_000_000.0,
            "d" => value * 24.0 * 60.0 * 60.0 * 1_000_000.0,
            "w" => value * 7.0 * 24.0 * 60.0 * 60.0 * 1_000_000.0,
            "M" | "mon" | "month" =>
                value * 30.0 * 24.0 * 60.0 * 60.0 * 1_000_000.0,
            "y" | "yr" =>
                value * 365.0 * 24.0 * 60.0 * 60.0 * 1_000_000.0,
            "c" | "cen" =>
                value * 100.0 * 365.0 * 24.0 * 60.0 * 60.0 * 1_000_000.0,
            _ => return Err(format!("Unknown unit '{unit}'")),
        };

        total_micros += micros.round() as i64;
    }

    Ok(Duration::microseconds(total_micros))
}

fn parse_timestamp_duration(s: &str) -> Result<Duration, String> {
    let (time_part, millis_part) = match s.split_once(';') {
        Some((time, millis)) => (time, Some(millis)),
        None => (s, None),
    };

    let fields: Vec<&str> = time_part.split(':').collect();

    if fields.is_empty() || fields.len() > 4 {
        return Err(format!("Invalid duration timestamp '{s}'"));
    }

    let mut days = 0i64;
    let mut hours = 0i64;
    let mut minutes = 0i64;
    let seconds: i64;

    match fields.len() {
        1 => {
            seconds = fields[0]
                .parse()
                .map_err(|_| format!("Invalid seconds '{}'", fields[0]))?;
        }
        2 => {
            minutes = fields[0]
                .parse()
                .map_err(|_| format!("Invalid minutes '{}'", fields[0]))?;

            seconds = fields[1]
                .parse()
                .map_err(|_| format!("Invalid seconds '{}'", fields[1]))?;
        }
        3 => {
            hours = fields[0]
                .parse()
                .map_err(|_| format!("Invalid hours '{}'", fields[0]))?;

            minutes = fields[1]
                .parse()
                .map_err(|_| format!("Invalid minutes '{}'", fields[1]))?;

            seconds = fields[2]
                .parse()
                .map_err(|_| format!("Invalid seconds '{}'", fields[2]))?;
        }
        4 => {
            days = fields[0]
                .parse()
                .map_err(|_| format!("Invalid days '{}'", fields[0]))?;

            hours = fields[1]
                .parse()
                .map_err(|_| format!("Invalid hours '{}'", fields[1]))?;

            minutes = fields[2]
                .parse()
                .map_err(|_| format!("Invalid minutes '{}'", fields[2]))?;

            seconds = fields[3]
                .parse()
                .map_err(|_| format!("Invalid seconds '{}'", fields[3]))?;
        }
        _ => unreachable!(),
    }

    let millis = match millis_part {
        Some(ms) => ms
            .parse::<i64>()
            .map_err(|_| format!("Invalid milliseconds '{ms}'"))?,
        None => 0,
    };

    Ok(
        Duration::days(days)
            + Duration::hours(hours)
            + Duration::minutes(minutes)
            + Duration::seconds(seconds)
            + Duration::milliseconds(millis),
    )
}

pub fn format_duration(duration: Duration) -> String {
    let mut remaining = duration.num_milliseconds();

    let units = [
        ("centur", 100 * 365 * 24 * 60 * 60 * 1000_i64),
        ("year",    365 * 24 * 60 * 60 * 1000_i64),
        ("month",    30 * 24 * 60 * 60 * 1000_i64),
        ("week",      7 * 24 * 60 * 60 * 1000_i64),
        ("day",          24 * 60 * 60 * 1000_i64),
        ("hour",              60 * 60 * 1000_i64),
        ("minute",                 60 * 1000_i64),
        ("second",                      1000_i64),
        ("millisecond",                    1_i64),
    ];

    let mut parts = Vec::new();

    for (name, unit_ms) in units {
        let count = remaining / unit_ms;
        if count > 0 {
            parts.push(format!(
                "{} {}{}",
                count,
                name,
                if count == 1 {
                    if name.starts_with("c") {
                        "y"
                    } else {
                        ""
                    }
                } else {
                    if name.starts_with("c") {
                        "ies"
                    } else {
                        "s"
                    }
                }
            ));
            remaining %= unit_ms;
        }
    }

    if parts.is_empty() {
        "0 milliseconds".to_string()
    } else {
        parts.join(" ")
    }
}
