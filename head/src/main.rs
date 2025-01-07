use clap::Parser;
use cli::Cli;
use std::io::{BufRead, Read};

mod cli;
mod file;
mod parser;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let num_files = cli.files.len();

    for (file_num, filename) in cli.files.iter().enumerate() {
        match file::open(&filename) {
            Err(e) => eprintln!("Failed to open {}: {}", filename, e),
            Ok(mut f) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    );
                }

                if let Some(num_bytes) = cli.bytes {
                    let mut handle = f.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..cli.lines {
                        let bytes = f.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            }
        }
    }

    Ok(())
}
