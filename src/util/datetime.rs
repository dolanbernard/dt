use chrono::{Duration, Local, Utc};

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
        let tz: chrono_tz::Tz = tz_name
            .parse()
            .unwrap_or_else(|_| panic!("Invalid timezone: {tz_name}"));

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
