use std::thread;

use crate::util::duration;

pub fn run(durations: &Vec<String>) -> Result<String, String> {
    if let Ok(delay) = duration::sum_durations(durations)?.to_std() {
        thread::sleep(delay);
        Ok(String::new())
    } else {
        Err("Can't wait for a negative amount of time".to_owned())
    }
}
