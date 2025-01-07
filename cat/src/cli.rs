use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    name = "cat",
    version = "0.1.0",
    about = "Description: A simple `cat` command implementation"
)]
pub struct Cli {
    #[arg(value_name = "FILE", help = "Input file(s)", default_value = "-")]
    pub files: Vec<String>,
    #[arg(
        short = 'n',
        long = "number-lines",
        help = "Number lines",
        value_name = "NUMBER_LINES",
        conflicts_with = "number_nonblank_lines"
    )]
    pub number_lines: bool,
    #[arg(
        short = 'b',
        long = "number-nonblank-lines",
        help = "Number nonblank lines",
        value_name = "NUMBER_NONBLANK_LINES"
    )]
    pub number_nonblank_lines: bool,
}
