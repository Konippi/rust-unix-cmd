use clap::Parser;
use cli::Cli;
use entry::Entry;
use walkdir::{DirEntry, WalkDir};

mod cli;
mod entry;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    for path in &cli.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => {
                    if filter_type(&cli, &entry) && filter_name(&cli, &entry) {
                        println!("{}", entry.path().display());
                    }
                }
            }
        }
    }

    Ok(())
}

fn filter_type(cli: &Cli, entry: &DirEntry) -> bool {
    cli.entry_types.is_empty()
        || cli.entry_types.iter().any(|entry_type| match entry_type {
            Entry::Dir => entry.file_type().is_dir(),
            Entry::File => entry.file_type().is_file(),
            Entry::Link => entry.file_type().is_symlink(),
        })
}

fn filter_name(cli: &Cli, entry: &DirEntry) -> bool {
    cli.names.is_empty()
        || cli
            .names
            .iter()
            .any(|name| name.is_match(&entry.file_name().to_string_lossy()))
}
