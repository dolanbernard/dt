use crate::util::{datetime_diff, duration};

pub fn run(
    ref_uses_local: bool,
    ref_timezone: Option<String>,
    end_uses_local: bool,
    end_timezone: Option<String>,
    end: Option<String>,
    start: String,
) -> Result<String, String> {
    let start = datetime_diff::parse_endpoint(
        ref_uses_local,
        ref_timezone.as_deref(),
        &start,
    );

    let end = match end {
        Some(end_str) => datetime_diff::parse_endpoint(
            end_uses_local,
            end_timezone.as_deref(),
            &end_str,
        ),
        None => datetime_diff::current_endpoint(
            end_uses_local,
            end_timezone.as_deref(),
        ),
    };

    if end < start {
        return Err("End time is before start".to_owned());
    }

    let result = end - start;

    Ok(format!("{}", duration::format_duration("", result)))
}
