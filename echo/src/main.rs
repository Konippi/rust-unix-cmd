use clap::Parser;
use cli::Cli;

mod cli;

fn main() {
    let cli = Cli::parse();
    let text = cli.text;
    print!(
        "{}{}",
        text.join(" "),
        if cli.omit_newline { "" } else { "\n" }
    );
}
