use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    name = "echo",
    version = "0.1.0",
    about = "Description: A simple `echo` command implementation"
)]
pub struct Cli {
    #[arg(short('t'), long, help = "Input text", required = true, num_args = 1..)]
    pub text: Vec<String>,
    #[arg(short('n'), long, help = "Do not output the trailing newline")]
    pub omit_newline: bool,
}
