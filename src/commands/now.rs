use crate::util::datetime;

pub fn run(
    use_local_tz: bool,
    timezone: Option<String>,
    format_str: Option<String>,
) {
    println!(
        "{}",
        datetime::current_time(
            use_local_tz,
            timezone.as_deref(),
            format_str.as_deref(),
        )
    );
}
