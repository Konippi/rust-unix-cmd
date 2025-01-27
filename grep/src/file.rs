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

#[cfg(test)]
mod tests {
    use io::Cursor;
    use regex::RegexBuilder;

    use super::*;

    #[test]
    fn test_find_files() {
        let files = find_files(&["./tests/inputs/sample.txt".to_string()], false);
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].as_ref().unwrap(), "./tests/inputs/sample.txt");

        let files = find_files(&["./tests/inputs".to_string()], false);
        assert_eq!(files.len(), 1);
        if let Err(e) = &files[0] {
            assert_eq!(e.to_string(), "./tests/inputs is a directory");
        }

        let res = find_files(&["./tests/inputs".to_string()], true);
        let mut files: Vec<String> = res
            .iter()
            .map(|r| r.as_ref().unwrap().replace("\\", "/"))
            .collect();
        files.sort();
        assert_eq!(files.len(), 3);
        assert_eq!(
            files,
            vec![
                "./tests/inputs/empty.txt",
                "./tests/inputs/sample.txt",
                "./tests/inputs/sample2.txt"
            ]
        );

        let bad = find_files(&["./tests/inputs/bad.txt".to_string()], false);
        assert_eq!(bad.len(), 1);
        if let Err(e) = &bad[0] {
            assert_eq!(
                e.to_string(),
                "./tests/inputs/bad.txt: No such file or directory (os error 2)"
            );
        }
    }

    #[test]
    fn test_find_lines() {
        let text = b"Lorem\nIpsum\r\nDOLOR";
        let re1 = Regex::new("or").unwrap();
        let matches = find_lines(Cursor::new(&text), &re1, false);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 1);

        let matches = find_lines(Cursor::new(&text), &re1, true);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 2);

        let re2 = RegexBuilder::new("or")
            .case_insensitive(true)
            .build()
            .unwrap();

        let matches = find_lines(Cursor::new(&text), &re2, false);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 2);

        let matches = find_lines(Cursor::new(&text), &re2, true);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 1);
    }
}
