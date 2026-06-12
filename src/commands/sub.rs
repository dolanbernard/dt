use crate::{
    util::{
        datetime,
        duration::parse_duration,
    }
};

pub fn run(
    use_local_tz: bool,
    timezone: Option<String>,
    format_str: Option<String>,
    delta: String,
) {
    let duration = parse_duration(&delta)
        .unwrap_or_else(|e| panic!("{e}"));

    println!(
        "{}",
        datetime::current_time_plus(
            -duration,
            use_local_tz,
            timezone.as_deref(),
            format_str.as_deref(),
        )
    );
}
