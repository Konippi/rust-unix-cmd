use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    name = "uniq",
    version = "0.1.0",
    about = "Description: A simple `uniq` command implementation"
)]
pub struct Cli {
    #[arg(value_name = "INPUT_FILE", help = "Input file", default_value = "-")]
    pub input_file: String,
    #[arg(value_name = "OUTPUT_FILE", help = "Output file")]
    pub output_file: Option<String>,
    #[arg(
        short = 'c',
        long = "count",
        value_name = "COUNT",
        help = "Show counts"
    )]
    pub count: bool,
}
