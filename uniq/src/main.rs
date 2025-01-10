use std::{
    fs::File,
    io::{self, BufRead, Write},
};

use clap::Parser;
use cli::Cli;

mod cli;
mod file;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut input_file = file::open(&cli.input_file)
        .map_err(|e| anyhow::anyhow!("Failed to open {}: {}", cli.input_file, e))?;
    let mut output_file: Box<dyn Write> = match &cli.output_file {
        Some(f) => Box::new(File::create(f)?),
        _ => Box::new(io::stdout()),
    };

    let mut print = |count: u64, text: &str| -> anyhow::Result<()> {
        if count > 0 {
            if cli.count {
                write!(output_file, "{:>4} {}", count, text)?;
            } else {
                write!(output_file, "{}", text)?;
            }
        }
        Ok(())
    };

    let mut line = String::new();
    let mut previous = String::new();
    let mut count: u64 = 0;

    loop {
        let bytes = input_file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if line.trim_end() != previous.trim_end() {
            print(count, &previous)?;
            previous = line.clone();
            count = 0;
        }

        count += 1;
        line.clear();
    }

    print(count, &previous)?;

    Ok(())
}
