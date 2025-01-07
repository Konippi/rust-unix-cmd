use clap::Parser;
use cli::Cli;
use std::io::BufRead;

mod cli;
mod file;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    for filename in cli.files {
        match file::open(&filename) {
            Err(e) => eprintln!("Failed to open {}: {}", filename, e),
            Ok(f) => {
                let mut last_num = 0;

                for (line_num, line) in f.lines().enumerate() {
                    let result = line?;

                    if cli.number_lines {
                        println!("{:>6}\t{}", line_num + 1, result);
                    } else if cli.number_nonblank_lines {
                        if !result.is_empty() {
                            last_num += 1;
                            println!("{:>6}\t{}", last_num, result);
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", result);
                    }
                }
            }
        }
    }

    Ok(())
}
