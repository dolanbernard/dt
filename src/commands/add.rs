use crate::util::duration::parse_duration;

pub fn run(
    use_local_tz: bool,
    timezone: Option<String>,
    format_str: Option<String>,
    delta: String,
) {
    let duration = parse_duration(&delta)
        .unwrap_or_else(|e| panic!("{e}"));

    let output = if use_local_tz {
        let dt = chrono::Local::now() + duration;

        match &format_str {
            Some(fmt) => dt.format(fmt).to_string(),
            None => dt.to_string(),
        }
    } else if let Some(tz_name) = timezone {
        let tz: chrono_tz::Tz = tz_name
            .parse()
            .unwrap_or_else(|_| panic!("Invalid timezone: {tz_name}"));

        let dt = chrono::Utc::now()
            .with_timezone(&tz)
            + duration;

        match &format_str {
            Some(fmt) => dt.format(fmt).to_string(),
            None => dt.to_string(),
        }
    } else {
        let dt = chrono::Utc::now() + duration;

        match &format_str {
            Some(fmt) => dt.format(fmt).to_string(),
            None => dt.to_string(),
        }
    };

    println!("{output}");
}
