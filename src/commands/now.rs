pub fn run(
    use_local_tz: bool,
    timezone: Option<String>,
    format_str: Option<String>,
) {
    let output = if use_local_tz {
        let now = chrono::Local::now();

        match &format_str {
            Some(fmt) => now.format(fmt).to_string(),
            None => now.to_string(),
        }
    } else if let Some(tz_name) = timezone {
        let tz: chrono_tz::Tz = tz_name
            .parse()
            .unwrap_or_else(|_| panic!("Invalid timezone: {tz_name}"));

        let now = chrono::Utc::now().with_timezone(&tz);

        match &format_str {
            Some(fmt) => now.format(fmt).to_string(),
            None => now.to_string(),
        }
    } else {
        let now = chrono::Utc::now();

        match &format_str {
            Some(fmt) => now.format(fmt).to_string(),
            None => now.to_string(),
        }
    };

    println!("{output}");
}
