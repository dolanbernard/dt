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
    #[command(alias = "n")]
    Now {
        #[arg(short='l', long="local")]
        use_local_tz: bool,
        #[arg(short='z', long="tz")]
        timezone: Option<String>,
        #[arg(short='f', long="format")]
        format_str: Option<String>,
    },
    #[command(alias = "a")]
    Add {
        #[arg(short='l', long="local")]
        use_local_tz: bool,
        #[arg(short='z', long="tz")]
        timezone: Option<String>,
        #[arg(short='f', long="format")]
        format_str: Option<String>,
        /// Examples: 30s, 15m, 2h, 7d
        delta: String,
    },
    #[command(alias = "s")]
    Sub {
        #[arg(short='l', long="local")]
        use_local_tz: bool,
        #[arg(short='z', long="tz")]
        timezone: Option<String>,
        #[arg(short='f', long="format")]
        format_str: Option<String>,
        delta: String,
    },
    #[command(alias = "snc")]
    Since {
        #[arg(short='l', long="ref-local-tz")]
        ref_uses_local: bool,
        #[arg(short='z', long="ref-tz")]
        ref_timezone: Option<String>,
        #[arg(short='c', long="end-local-tz")]
        end_uses_local: bool,
        #[arg(short='t', long="end-tz")]
        end_timezone: Option<String>,
        #[arg(short='r', long="ref", required=true)]
        start: String,
        #[arg(short='e', long="end")]
        end: Option<String>,
    },
    #[command(alias = "utl")]
    Until {
        #[arg(short='l', long="ref-local-tz")]
        ref_uses_local: bool,
        #[arg(short='z', long="ref-tz")]
        ref_timezone: Option<String>,
        #[arg(short='c', long="end-local-tz")]
        end_uses_local: bool,
        #[arg(short='t', long="end-tz")]
        end_timezone: Option<String>,
        #[arg(short='r', long="ref")]
        start: Option<String>,
        #[arg(short='e', long="end", required=true)]
        end: String,
    },
    #[command(alias = "total")]
    Sum {
        #[arg(short='f', long="format")]
        format_str: Option<String>,
        #[arg(num_args = 1..)]
        durations: Vec<String>
    }
}