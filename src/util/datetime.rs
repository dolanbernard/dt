use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Tz;

pub fn current_time(
    use_local_tz: bool,
    timezone: Option<&str>,
    format_str: Option<&str>,
) -> String {
    current_time_plus(
        Duration::zero(),
        use_local_tz,
        timezone,
        format_str,
    )
}

pub fn current_time_plus(
    delta: Duration,
    use_local_tz: bool,
    timezone: Option<&str>,
    format_str: Option<&str>,
) -> String {
    if use_local_tz {
        let dt = Local::now() + delta;
        format_datetime(dt, format_str)
    } else if let Some(tz_name) = timezone {
        let tz = parse_timezone(tz_name)
            .expect("Error parsing timezone: {tz_name}");

        let dt = Utc::now()
            .with_timezone(&tz)
            + delta;

        format_datetime(dt, format_str)
    } else {
        let dt = Utc::now() + delta;
        format_datetime(dt, format_str)
    }
}

fn format_datetime<Tz>(
    dt: chrono::DateTime<Tz>,
    format_str: Option<&str>,
) -> String
where
    Tz: chrono::TimeZone,
    Tz::Offset: std::fmt::Display,
{
    match format_str {
        Some(fmt) => dt.format(fmt).to_string(),
        None => dt.to_string(),
        //None => dt.format("%F %T-%Z"),
    }
}

pub fn parse_timezone(timezone_str: &str) -> Result<Tz, chrono_tz::ParseError> {
    let timezone_str = timezone_str.trim();
    let uppercase_timezone_str = timezone_str.to_uppercase();
    let iana_name = match uppercase_timezone_str.as_str() {
        "EST" | "EDT" => Some("America/New_York"),
        "CST" | "CDT" => Some("America/Chicago"),
        "MST" | "MDT" => Some("America/Denver"),
        "PST" | "PDT" => Some("America/Los_Angeles"),

        "CET" | "CEST" => Some("Europe/Paris"),
        "GMT" | "BST" => Some("Europe/London"),

        "JST" => Some("Asia/Tokyo"),
        "AEST" | "AEDT" => Some("Australia/Sydney"),
        _ => None
    };
    if let Some(tz) = iana_name {
        return tz.parse();
    }

    timezone_str.parse()
        .or_else(|_| timezone_str.replace(" ", "_").parse())
        .or_else(|_| uppercase_timezone_str.replace(" ", "_").parse())
        .or_else(|_| uppercase_timezone_str.to_lowercase().replace(" ", "_").parse())
}

pub fn parse_datetime<Tz>(
    timezone: Tz,
    dt_string: &str,
) -> Result<DateTime<Tz>, String>
where
    Tz: TimeZone + Clone,
    Tz::Offset: std::fmt::Display,
{
    // RFC3339
    if let Ok(dt) = DateTime::parse_from_rfc3339(dt_string) {
        return Ok(dt.with_timezone(&timezone));
    }

    // RFC2822
    if let Ok(dt) = DateTime::parse_from_rfc2822(dt_string) {
        return Ok(dt.with_timezone(&timezone));
    }

    // Unix timestamp (seconds)
    if let Ok(ts) = dt_string.parse::<i64>() {
        if let Some(dt) = timezone.timestamp_opt(ts, 0).single() {
            return Ok(dt);
        }
    }

    // Unix timestamp (milliseconds)
    if dt_string.chars().all(|c| c.is_ascii_digit()) && dt_string.len() >= 13 {
        if let Ok(ms) = dt_string.parse::<i64>() {
            let secs = ms / 1000;
            let nanos = ((ms % 1000) * 1_000_000) as u32;

            if let Some(dt) = timezone.timestamp_opt(secs, nanos).single() {
                return Ok(dt);
            }
        }
    }

    const TZ_FORMATS: &[&str] = &[
        "%Y-%m-%d %H:%M:%S %:z",
        "%Y-%m-%d %H:%M:%S%.f %:z",
        "%Y-%m-%d %H:%M %:z",
        "%Y-%m-%dT%H:%M:%S%:z",
        "%Y-%m-%dT%H:%M:%S%.f%:z",
        "%Y-%m-%dT%H:%M%:z",
        "%a %b %d %H:%M:%S %Y %z",
    ];

    for fmt in TZ_FORMATS {
        if let Ok(dt) = DateTime::parse_from_str(dt_string, fmt) {
            return Ok(dt.with_timezone(&timezone));
        }
    }

    const DATETIME_FORMATS: &[&str] = &[
        // ISO-like
        "%Y-%m-%d %H:%M:%S%.f",
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%d %H:%M",
        "%Y-%m-%dT%H:%M:%S%.f",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%dT%H:%M",

        // US
        "%m/%d/%Y %H:%M:%S",
        "%m/%d/%Y %H:%M",
        "%m/%d/%Y %I:%M:%S %p",
        "%m/%d/%Y %I:%M %p",

        // Alternate separators
        "%Y/%m/%d %H:%M:%S",
        "%Y/%m/%d %H:%M",
        "%d-%m-%Y %H:%M:%S",
        "%d-%m-%Y %H:%M",

        // Textual months
        "%b %d %Y %H:%M:%S",
        "%b %d %Y %H:%M",
        "%B %d %Y %H:%M:%S",
        "%B %d %Y %H:%M",
    ];

    for fmt in DATETIME_FORMATS {
        if let Ok(naive) = NaiveDateTime::parse_from_str(dt_string, fmt) {
            return timezone
                .from_local_datetime(&naive)
                .single()
                .ok_or_else(|| {
                    format!(
                        "Ambiguous or nonexistent local time '{}'",
                        dt_string
                    )
                });
        }
    }

    const DATE_FORMATS: &[&str] = &[
        "%Y-%m-%d",
        "%Y/%m/%d",
        "%m/%d/%Y",
        "%d/%m/%Y",
        "%b %d %Y",
        "%B %d %Y",
    ];

    for fmt in DATE_FORMATS {
        if let Ok(date) = NaiveDate::parse_from_str(dt_string, fmt) {
            let naive = date.and_hms_opt(0, 0, 0).unwrap();

            return timezone
                .from_local_datetime(&naive)
                .single()
                .ok_or_else(|| {
                    format!(
                        "Ambiguous or nonexistent local time '{}'",
                        dt_string,
                    )
                });
        }
    }

    Err(format!(
        "Unable to parse datetime '{}'. Supported formats include RFC3339, RFC2822, Unix timestamps, ISO-8601, YYYY-MM-DD HH:MM:SS, MM/DD/YYYY HH:MM[:SS], and common textual month formats.",
        dt_string
    ))
}

pub fn now<Tz>(timezone: Tz) -> DateTime<Tz>
where
    Tz: TimeZone,
{
    Utc::now().with_timezone(&timezone)
}

pub fn parse_datetime_utc<Tz>(
    timezone: Tz,
    value: &str,
) -> Result<DateTime<Utc>, String>
where
    Tz: TimeZone + Clone,
    Tz::Offset: std::fmt::Display,
{
    Ok(parse_datetime(timezone, value)?.with_timezone(&Utc))
}

/*enum FormatPreset {
    Iso8601,
    Rfc3339,
    DayMonthYear,
    DayMonthYearShort,
    MonthDayYear,
    FullText,
    ShortText,
}

impl FormatPreset {
    fn get_format_str(&self) -> &'static str {
        match self {
            FormatPreset::Iso8601 | FormatPreset::Rfc3339 => "%+",
            FormatPreset::DayMonthYear => "%e-%B-%Y",
            FormatPreset::DayMonthYearShort => "%e-%b-%Y",
            FormatPreset::MonthDayYear => "%D",
            FormatPreset::FullText => "%A, %B %e, %Y",
            FormatPreset::ShortText => "%a, %b %e, %Y",
            _ => "%+"
        }
    }
    fn try_parse_fuzzy(s: impl AsRef<str>) -> Option<Self> {
        Some(FormatPreset::Iso8601)
    }
}*/
