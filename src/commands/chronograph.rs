use std::{time::{Duration as StdDuration, Instant}};

use chrono::Duration;
use crossterm::{
    cursor, event::{self, Event, KeyCode, KeyEventKind}, execute, terminal
};

use crate::util::duration;

pub fn run(pause_on_lap: bool, force_ascii_timer: bool) -> Result<String, String> {
    let mut result = Ok(String::new());
    terminal::enable_raw_mode().map_err(|e| e.to_string())?;
    execute!(std::io::stdout(), cursor::Hide).map_err(|e| e.to_string())?;
    println_raw(format!("\rPress space to start or q to quit"));
    loop {
        if let Ok(k) = wait_for_key() {
            match k {
                KeyCode::Char(' ') => {
                    result = run_loop(pause_on_lap, force_ascii_timer);
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
    execute!(std::io::stdout(), cursor::Show).map_err(|e| e.to_string())?;
    terminal::disable_raw_mode().map_err(|e| e.to_string())?;
    result
}

fn run_loop(pause_on_lap: bool, force_ascii_timer: bool) -> Result<String, String> {
    let mut laps: Vec<Duration> = Vec::new();

    loop {
        let lap_start = Instant::now();
        let k = wait_for_event(lap_start, force_ascii_timer)?;
        match k {
            KeyCode::Char(' ') => {
                let lap_duration = Duration::from_std(lap_start.elapsed())
                    .map_err(|e| e.to_string())?;
                println_raw(format!("Lap {}: {}", laps.len() + 1, duration::format_duration("", lap_duration)));
                laps.push(lap_duration);
                if pause_on_lap {
                    let k = wait_for_event(None, force_ascii_timer)?;
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
    let default_duration = Duration::zero();
    let min = laps.iter().min().unwrap_or(&default_duration);
    let max = laps.iter().max().unwrap_or(&default_duration);
    
    let total = laps.iter().fold(Duration::zero(), |acc, d| acc + *d);
    let avg = if laps.len() > 0 {
        total / (laps.len() as i32)
    } else {
        default_duration
    };

    execute!(std::io::stdout(), cursor::MoveToColumn(0)).unwrap();

    Ok(format!(
        "Min: {}   Avg: {}   Max: {}",
        duration::format_duration("", *min),
        duration::format_duration("", avg),
        duration::format_duration("", *max)
    ))
}

fn println_raw(s: impl AsRef<str>) {
    execute!(std::io::stdout(), cursor::MoveToColumn(0)).unwrap();
    println!("{}", s.as_ref());
    execute!(std::io::stdout(), cursor::MoveToColumn(0)).unwrap();
    //execute!(std::io::stdout(), cursor::MoveToNextLine(1)).unwrap();
}

const ASCII_ANIMATION_FRAMES: [&'static str;4] = ["|", "/", "-", "\\"];
const ANIMATION_FRAMES: [&'static str;10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
//const ANIMATION_FRAMES: [&'static str;8] = ["⣾", "⣷", "⣯", "⣟", "⡿", "⢿", "⣻", "⣽"];
const ANIMATION_DELAY_MS: usize = 100;

fn animate_timer(start_time: Instant, force_ascii_timer: bool) {
    let next_frame = if force_ascii_timer {
        ASCII_ANIMATION_FRAMES[(start_time.elapsed().as_millis() as usize / ANIMATION_DELAY_MS) % ASCII_ANIMATION_FRAMES.len()]
    } else {
        ANIMATION_FRAMES[(start_time.elapsed().as_millis() as usize / ANIMATION_DELAY_MS) % ANIMATION_FRAMES.len()]
    };
    execute!(std::io::stdout(), cursor::MoveToColumn(0)).unwrap();
    print!("{}", next_frame);
}

fn wait_for_event(animation_start_time: impl Into<Option<Instant>>, force_ascii_timer: bool) -> Result<KeyCode, String> {
    let animation_start_time = animation_start_time.into();
    loop {
        if let Some(start_time) = animation_start_time {
            animate_timer(start_time, force_ascii_timer);
        }
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
