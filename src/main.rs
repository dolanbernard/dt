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
            delta,
            use_local_tz,
            timezone,
            format_str,
        } => commands::add::run(
            use_local_tz,
            timezone,
            format_str,
            delta,
        ),
    }
}
