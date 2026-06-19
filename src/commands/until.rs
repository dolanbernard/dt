use crate::util::duration;
use crate::util::datetime_diff;

pub fn run(
    ref_uses_local: bool,
    ref_timezone: Option<String>,
    end_uses_local: bool,
    end_timezone: Option<String>,
    start: Option<String>,
    end: String,
) -> Result<String, String> {
    let start = match start {
        Some(start_str) => datetime_diff::parse_endpoint(
            ref_uses_local,
            ref_timezone.as_deref(),
            &start_str,
        ),
        None => datetime_diff::current_endpoint(
            ref_uses_local,
            ref_timezone.as_deref(),
        ),
    };

    let end = datetime_diff::parse_endpoint(
        end_uses_local,
        end_timezone.as_deref(),
        &end,
    );

    if end < start {
        return Err("End time is before start".to_owned());
    }

    Ok(format!(
        "{}",
        duration::format_duration("", end - start)
    ))
}
