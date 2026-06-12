use chrono::{Duration, Local, Utc};
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

fn parse_timezone(timezone_str: &str) -> Result<Tz, chrono_tz::ParseError> {
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

enum FormatPreset {
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
}
