use clap::Parser;
use regex::Regex;

use crate::entry::Entry;

#[derive(Debug, Parser)]
#[clap(
    name = "find",
    version = "0.1.0",
    about = "A simple implementation of the `find` command"
)]
pub struct Cli {
    #[arg(value_name = "PATH", help = "Search paths", default_value = ".")]
    pub paths: Vec<String>,
    #[arg(short = 'n', long = "name", value_name = "NAME", help = "File name", value_parser = validate_regex)]
    pub names: Vec<Regex>,
    #[arg(
        short = 't',
        long = "type",
        value_name = "TYPE",
        help = "Entry type",
        value_enum
    )]
    pub entry_types: Vec<Entry>,
}

fn validate_regex(val: &str) -> anyhow::Result<Regex> {
    Regex::new(&val).map_err(|_| anyhow::anyhow!("Invalid --name regex: {}", val))
}
