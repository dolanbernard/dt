use chrono::Duration;

use crate::util::duration;

pub fn run(format_str: Option<String>, duration_strings: &Vec<String>) -> Result<String, String> {
    let total = sum(duration_strings)?;
    Ok(format!("{}", 
        duration::format_duration(format_str.unwrap_or("".to_string()).as_ref(), total)))
}

fn sum(duration_strings: &Vec<String>) -> Result<Duration, String> {
    duration_strings.iter()
        .try_fold(Duration::zero(), |acc, duration_string| {
            let d = duration::parse_duration(duration_string)?;
            Ok(acc + d)
        })
}
