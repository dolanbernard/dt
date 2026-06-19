use crate::util::{
    datetime,
    duration::parse_duration,
};

pub fn run(
    use_local_tz: bool,
    timezone: Option<String>,
    format_str: Option<String>,
    delta: String,
) -> Result<String, String> {
    let duration = parse_duration(&delta)?;

    Ok(format!(
        "{}",
        datetime::current_time_plus(
            duration,
            use_local_tz,
            timezone.as_deref(),
            format_str.as_deref(),
        )
    ))
}
