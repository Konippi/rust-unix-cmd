use clap::Parser;

use crate::parser;

#[derive(Debug, Parser)]
#[clap(
    name = "head",
    version = "0.1.0",
    about = "Description: A simple `head` command implementation"
)]
pub struct Cli {
    #[arg(value_name = "FILE", help = "Input file(s)", default_value = "-")]
    pub files: Vec<String>,
    #[arg(
        short = 'n',
        long = "lines",
        help = "Number of lines",
        value_name = "LINES",
        default_value = "10",
        value_parser = parser::parse_positive_int
    )]
    pub lines: usize,
    #[arg(
        short = 'c',
        long = "bytes",
        help = "Number of bytes",
        value_name = "BYTES",
        conflicts_with = "lines",
        value_parser = parser::parse_positive_int
    )]
    pub bytes: Option<usize>,
}
