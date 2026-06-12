use chrono::Duration;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
    
    //#[arg(short='d', action = clap::ArgAction::SetFalse, default_value_t = true)]
    //pub asdf: bool,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /*Dump {
        #[arg(short='c', long="cols", required = false, default_value_t=2)]
        columns: usize,
    },*/
    Now {
        #[arg(short='l', long="local", required=false)]
        use_local_tz: bool,
        #[arg(short='z', long="tz", required=false)]
        timezone: Option<String>,
        #[arg(short='f', long="format", required=false)]
        format_str: Option<String>,
    },
    Add {
        /// Examples: 30s, 15m, 2h, 7d
        #[arg(short='l', long="local")]
        use_local_tz: bool,
        #[arg(short='z', long="tz")]
        timezone: Option<String>,
        #[arg(short='f', long="format")]
        format_str: Option<String>,
        delta: String,
    },
}