use clap::Parser;
use cli::Cli;
use walkdir::WalkDir;

mod cli;
mod entry;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    for path in cli.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => println!("{}", entry.path().display()),
            }
        }
    }

    Ok(())
}
