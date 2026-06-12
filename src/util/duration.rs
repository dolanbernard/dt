use chrono::Duration;

pub fn parse_duration(s: &str) -> Result<Duration, String> {
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
            // TODO: best way to do month/year math?
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
