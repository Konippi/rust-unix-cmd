use clap::Parser;
use cli::Cli;
use entry::Entry;
use walkdir::{DirEntry, WalkDir};

mod cli;
mod entry;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let type_filter = |entry: &DirEntry| {
        cli.entry_types.is_empty()
            || cli.entry_types.iter().any(|entry_type| match entry_type {
                Entry::Dir => entry.file_type().is_dir(),
                Entry::File => entry.file_type().is_file(),
                Entry::Link => entry.file_type().is_symlink(),
            })
    };

    let name_filter = |entry: &DirEntry| {
        cli.names.is_empty()
            || cli
                .names
                .iter()
                .any(|name| name.is_match(&entry.file_name().to_string_lossy()))
    };

    for path in cli.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => {
                    if type_filter(&entry) && name_filter(&entry) {
                        println!("{}", entry.path().display());
                    }
                }
            }
        }
    }

    Ok(())
}
