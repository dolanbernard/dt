use crate::util::{datetime_diff, duration};

pub fn run(
    ref_uses_local: bool,
    ref_timezone: Option<String>,
    end_uses_local: bool,
    end_timezone: Option<String>,
    start: String,
    end: Option<String>,
) {
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
        panic!("End time is before start");
    }

    let result = end - start;

    println!("{}", duration::format_duration("", result));
}
