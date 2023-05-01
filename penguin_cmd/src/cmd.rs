//! Command Line arguments definitions.
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// configuration file
    #[arg(short, long)]
    pub conf: String,
    #[arg(short, long, default_value = "false")]
    pub dry_run: bool,
}
