use std::time::{Duration as StdDuration, Instant};

use chrono::Duration;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal,
};

use crate::util::duration;

pub fn run(pause_on_lap: bool) -> Result<String, String> {
    let mut result = Ok(String::new());
    terminal::enable_raw_mode().map_err(|e| e.to_string())?;
    print!("\rPress space to start or q to quit\r\n");
    loop {
        if let Ok(k) = wait_for_key() {
            match k {
                KeyCode::Char(' ') => {
                    result = run_loop(pause_on_lap);
                    break;
                },
                KeyCode::Char('q') => {
                    break;
                },
                _ => {
                    continue;
                }
            }
        }
    }
    terminal::disable_raw_mode().map_err(|e| e.to_string())?;
    result
}

fn run_loop(pause_on_lap: bool) -> Result<String, String> {
    let mut laps: Vec<Duration> = Vec::new();

    loop {
        let lap_start = Instant::now();
        let k = wait_for_event()?;
        match k {
            KeyCode::Char(' ') => {
                let lap_duration = Duration::from_std(lap_start.elapsed())
                    .map_err(|e| e.to_string())?;
                print!("Lap {}: {}\r\n", laps.len() + 1, duration::format_duration("", lap_duration));
                laps.push(lap_duration);
                if pause_on_lap {
                    let k = wait_for_event()?;
                    match k {
                        KeyCode::Char(' ') => continue,
                        KeyCode::Char('q') => break,
                        _ => continue
                    }
                }
            },
            KeyCode::Char('q') => {
                break;
            },
            _ => {
                continue;
            },
        }
    }
    let min = laps.iter().min().unwrap();
    let max = laps.iter().max().unwrap();
    
    let total = laps.iter().fold(Duration::zero(), |acc, d| acc + *d);
    let avg = total / (laps.len() as i32);

    Ok(format!(
        "Min: {}   Avg: {}   Max: {}",
        duration::format_duration("", *min),
        duration::format_duration("", avg),
        duration::format_duration("", *max)
    ))
}

fn wait_for_event() -> Result<KeyCode, String> {
    loop {
        if event::poll(POLL_INTERVAL).map_err(|e| e.to_string())? {
            if let Event::Key(key) = event::read().map_err(|e| e.to_string())? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                return Ok(key.code);
            }
        }
    }
}

fn wait_for_key() -> Result<KeyCode, String> {
    loop {
        if let Event::Key(key) = event::read().map_err(|e| e.to_string())? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            return Ok(key.code);
        }
    }
}

const POLL_INTERVAL: StdDuration = StdDuration::from_micros(500);
