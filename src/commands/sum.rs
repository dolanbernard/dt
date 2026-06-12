use chrono::Duration;

use crate::util::duration;

pub fn run(duration_strings: &Vec<String>) {
    let total = sum(duration_strings).unwrap();
    println!("{}", duration::format_duration(total));
}

fn sum(duration_strings: &Vec<String>) -> Result<Duration, String> {
    duration_strings.iter()
        .try_fold(Duration::zero(), |acc, duration_string| {
            let d = duration::parse_duration(duration_string)?;
            Ok(acc + d)
        })
}
