use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    name = "wc",
    version = "0.1.0",
    about = "Description: A simple `wc` command implementation"
)]
pub struct Cli {
    #[arg(value_name = "FILE", help = "Input file(s)", default_value = "-")]
    pub files: Vec<String>,
    #[arg(
        short = 'l',
        long = "lines",
        value_name = "LINES",
        help = "Number of lines"
    )]
    pub lines: bool,
    #[arg(
        short = 'w',
        long = "words",
        value_name = "WORDS",
        help = "Number of words"
    )]
    pub words: bool,
    #[arg(
        short = 'c',
        long = "bytes",
        value_name = "BYTES",
        help = "Number of bytes"
    )]
    pub bytes: bool,
    #[arg(
        short = 'm',
        long = "chars",
        value_name = "CHARS",
        help = "Number of characters",
        conflicts_with = "bytes"
    )]
    pub chars: bool,
}

impl Cli {
    pub fn set_defaults(&mut self) {
        if [self.lines, self.words, self.chars, self.bytes]
            .iter()
            .all(|&arg| !arg)
        {
            self.lines = true;
            self.words = true;
            self.bytes = true;
        }
    }
}
