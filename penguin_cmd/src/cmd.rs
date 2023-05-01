//! Command Line arguments definitions.
use radicle_term as term;

struct Help {
    name: &'static str,
    description: &'static str,
    version: &'static str,
    usage: &'static str,
}

const HELP: Help = Help {
    name: "lampod-cli",
    description: "Lampo Deamon command line",
    version: env!("CARGO_PKG_VERSION"),
    usage: r#"
Usage

    penguin [<option> ...]

Options

    -c | --config    Override the default path of the config field.
    -h | --help      Print help.
    --dry-run        Run a fake run without commit any change to the disk.
    --term           Run the triage meeting and print the result on the terminal.
"#,
};

pub fn print_help() -> Result<(), lexopt::Error> {
    println!(
        "{}",
        term::format::secondary("Common `penguin` commands used to init the lampo deamon")
    );
    println!(
        "\n{} {}",
        term::format::bold("Usage:"),
        term::format::dim("penguin <command> [--help]")
    );
    println!();

    println!(
        "\t{} {}",
        term::format::bold(format!("{:-12} {}", HELP.name, HELP.version)),
        term::format::dim(HELP.description)
    );
    println!("{}", term::format::bold(HELP.usage));
    Ok(())
}

/// Simple program to greet a person
pub struct Args {
    /// configuration file
    pub conf: String,
    pub dry_run: bool,
    pub term: bool,
}

impl Args {
    pub fn parse() -> Result<Self, lexopt::Error> {
        use lexopt::prelude::*;

        let mut config: Option<String> = None;
        let mut dry_run = false;
        let mut term = false;

        let mut parser = lexopt::Parser::from_env();
        while let Some(arg) = parser.next()? {
            match arg {
                Short('c') | Long("config") => {
                    let val: String = parser.value()?.parse()?;
                    config = Some(val);
                }
                Long("dry-run") => {
                    dry_run = true;
                }
                Long("term") => {
                    term = true;
                }
                Long("help") => {
                    let _ = print_help();
                    std::process::exit(0);
                }
                _ => return Err(arg.unexpected()),
            }
        }

        Ok(Self {
            conf: config.expect("Configuration option need to be specified"),
            dry_run,
            term,
        })
    }
}
