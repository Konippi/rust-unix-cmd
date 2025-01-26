use std::{
    fs,
    io::{self, BufRead, BufReader},
    mem,
};

use anyhow::anyhow;
use regex::Regex;
use walkdir::WalkDir;

pub fn open(filename: &str) -> anyhow::Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(fs::File::open(filename)?))),
    }
}

pub fn find_files(paths: &[String], recursive: bool) -> Vec<anyhow::Result<String>> {
    let mut files = vec![];

    for path in paths {
        match path.as_str() {
            "-" => files.push(Ok(path.to_string())),
            _ => match fs::metadata(path) {
                Err(e) => files.push(Err(anyhow!("{path}: {e}"))),
                Ok(metadata) => {
                    if metadata.is_dir() {
                        if recursive {
                            WalkDir::new(path)
                                .into_iter()
                                .flatten()
                                .filter(|e| e.file_type().is_file())
                                .for_each(|file| {
                                    files.push(Ok(file.path().to_string_lossy().to_string()));
                                });
                        } else {
                            files.push(Err(anyhow!("{path} is a directory")));
                        }
                    } else if metadata.is_file() {
                        files.push(Ok(path.to_string()));
                    }
                }
            },
        }
    }

    files
}

pub fn find_lines<R: BufRead>(
    mut file: R,
    pattern: &Regex,
    invert: bool,
) -> anyhow::Result<Vec<String>> {
    let mut matches = vec![];
    let mut line = String::new();

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        if pattern.is_match(&line) ^ invert {
            matches.push(mem::take(&mut line));
        }
        line.clear();
    }

    Ok(matches)
}
