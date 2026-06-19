use crate::util::duration;

pub fn run(format_str: Option<String>, duration_strings: &Vec<String>) -> Result<String, String> {
    let result = duration::sum_durations(duration_strings)?;
    Ok(format!("{}", 
        duration::format_duration(format_str.unwrap_or(String::new()).as_ref(), result)))
}
