use anyhow::anyhow;
use clap::Parser;
use cli::Cli;
use regex::RegexBuilder;

mod cli;
mod file;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let pattern = RegexBuilder::new(&cli.pattern)
        .case_insensitive(cli.insensitive)
        .build()
        .map_err(|_| anyhow!(r#"Invalid pattern {}"#, cli.pattern))?;
    let entries = file::find_files(&cli.files, cli.recursive);
    let num_files = entries.len();
    let print = |filename: &str, val: &str| {
        if num_files > 1 {
            println!("{filename}:{val}");
        } else {
            println!("{val}");
        }
    };

    for entry in entries {
        match entry {
            Err(e) => eprintln!("{e}"),
            Ok(filename) => match file::open(&filename) {
                Err(e) => eprint!("{filename}: {e}"),
                Ok(f) => match file::find_lines(f, &pattern, cli.invert) {
                    Err(e) => eprint!("{e}"),
                    Ok(matcehes) => {
                        if cli.count {
                            print(&filename, &format!("{}\n", matcehes.len()));
                        } else {
                            for line in &matcehes {
                                print(&filename, line);
                            }
                        }
                    }
                },
            },
        }
    }

    Ok(())
}
