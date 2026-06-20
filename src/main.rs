use clap::Parser;

use args::Command;

mod args;
mod commands;
mod util;

fn main() {
    let config = args::Args::parse();

    let output = match config.command {
        Command::Now {
            use_local_tz,
            timezone,
            format_str,
        } => commands::now::run(
            use_local_tz,
            timezone,
            format_str,
        ),
        Command::Add {
            use_local_tz,
            timezone,
            format_str,
            delta,
        } => commands::add::run(
            use_local_tz,
            timezone,
            format_str,
            delta,
        ),
        Command::Sub {
            use_local_tz,
            timezone,
            format_str,
            delta,
        } => commands::sub::run(
            use_local_tz,
            timezone,
            format_str,
            delta,
        ),
        Command::Since {
            ref_uses_local,
            ref_timezone,
            end_uses_local,
            end_timezone,
            start,
            end,
        } => {
            commands::since::run(
                ref_uses_local,
                ref_timezone,
                end_uses_local,
                end_timezone,
                end,
                start
            )
        },
        Command::Until {
            ref_uses_local,
            ref_timezone,
            end_uses_local,
            end_timezone,
            start,
            end
        } => {
            commands::until::run(
                ref_uses_local,
                ref_timezone,
                end_uses_local,
                end_timezone,
                start,
                end
            )
        },
        Command::Sum {
            format_str,
            durations,
        } => {
            commands::sum::run(format_str, &durations)
        },
        Command::Timer {
            durations,
        } => {
            commands::timer::run(&durations)
        },
        Command::Chrono {
            pause_on_lap,
            force_ascii_timer
        } => {
            commands::chronograph::run(pause_on_lap, force_ascii_timer)
        },
    };
    println!("{}", output.unwrap());
}
