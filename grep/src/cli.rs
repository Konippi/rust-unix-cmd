use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    name = "cut",
    version = "0.1.0",
    about = "Description: A simple `cut` command implementation"
)]
pub struct Cli {
    #[arg(value_name = "PATTERN", help = "Search pattern", required = true)]
    pub pattern: String,
    #[arg(value_name = "FILE", help = "Input file(s)", default_value = "-")]
    pub files: Vec<String>,
    #[arg(short, long, help = "Case insensitive")]
    pub insensitive: bool,
    #[arg(short, long, help = "Recursive search")]
    pub recursive: bool,
    #[arg(short, long, help = "Count occurrences")]
    pub count: bool,
    #[arg(short = 'v', long = "invert-match", help = "Invert match")]
    pub invert: bool,
}
