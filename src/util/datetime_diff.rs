use chrono::{DateTime, Local, Utc};

use crate::util::datetime;

pub fn parse_endpoint(
    use_local: bool,
    timezone: Option<&str>,
    value: &str,
) -> DateTime<Utc> {
    if use_local {
        datetime::parse_datetime_utc(Local, value)
            .unwrap_or_else(|e| panic!("{e}"))
    } else {
        datetime::parse_datetime_utc(resolve_tz(timezone), value)
            .unwrap_or_else(|e| panic!("{e}"))
    }
}

pub fn current_endpoint(
    use_local: bool,
    timezone: Option<&str>,
) -> DateTime<Utc> {
    if use_local {
        datetime::now(Local).with_timezone(&Utc)
    } else {
        datetime::now(resolve_tz(timezone))
            .with_timezone(&Utc)
    }
}

pub fn resolve_tz(timezone: Option<&str>) -> chrono_tz::Tz {
    timezone
        .map(|s| {
            datetime::parse_timezone(s)
                .unwrap_or_else(|_| panic!("Invalid timezone: {s}"))
        })
        .unwrap_or(chrono_tz::UTC)
}
