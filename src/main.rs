use clap::Parser;

use args::Command;

mod args;
mod commands;
mod util;

fn main() {
    let config = args::Args::parse();

    match config.command {
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
        Command::Diff {
            //
        } => commands::diff::run(
            //
        )
    }
}
