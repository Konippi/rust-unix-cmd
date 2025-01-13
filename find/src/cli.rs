use clap::{Parser, ValueEnum};
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
        value_enum,
        value_parser = validate_entry_type
    )]
    pub entry_types: Vec<Entry>,
}

fn validate_regex(val: &str) -> anyhow::Result<Regex> {
    Regex::new(&val).map_err(|_| anyhow::anyhow!("Invalid value for '--name <NAME>': {}", val))
}

fn validate_entry_type(val: &str) -> anyhow::Result<Entry> {
    Entry::from_str(val, true)
        .map_err(|_| anyhow::anyhow!("Invalid value for '--type <TYPE>': {}", val))
}
