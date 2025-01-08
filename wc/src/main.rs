use clap::Parser;
use cli::Cli;

mod cli;
mod file;
mod format;

fn main() -> anyhow::Result<()> {
    let mut cli = Cli::parse();
    cli.set_defaults();

    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;

    for filename in &cli.files {
        match file::open(filename) {
            Err(e) => eprintln!("Failed to open {}: {}", filename, e),
            Ok(f) => {
                if let Ok(info) = file::count(f) {
                    println!(
                        "{}{}{}{}{}",
                        format::format_field(info.num_lines, cli.lines),
                        format::format_field(info.num_words, cli.words),
                        format::format_field(info.num_bytes, cli.bytes),
                        format::format_field(info.num_chars, cli.chars),
                        if filename == "-" {
                            "".to_string()
                        } else {
                            format!(" {}", filename)
                        },
                    );

                    total_lines += info.num_lines;
                    total_words += info.num_words;
                    total_bytes += info.num_bytes;
                    total_chars += info.num_chars;
                }
            }
        }

        if cli.files.len() > 1 {
            println!(
                "{}{}{}{} total",
                format::format_field(total_lines, cli.lines),
                format::format_field(total_words, cli.words),
                format::format_field(total_bytes, cli.bytes),
                format::format_field(total_chars, cli.chars)
            );
        }
    }

    Ok(())
}
