use assert_cmd::Command;
use predicates::prelude::predicate;

use std::{borrow::Cow, fs};

mod util;

const BIN: &str = "find";

#[test]
fn skips_bad_dir() -> anyhow::Result<()> {
    let bad = util::gen_bad_file();
    let expected = format!("{}: No such file or directory", bad);

    Command::cargo_bin(BIN)?
        .arg(bad)
        .assert()
        .success()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

#[test]
fn dies_bad_name() -> anyhow::Result<()> {
    let invalid_regex = "*.csv";

    Command::cargo_bin(BIN)?
        .args(["-n", invalid_regex])
        .assert()
        .failure()
        .stderr(predicate::str::contains(format!(
            "Invalid value for '--name <NAME>': {}",
            invalid_regex
        )));

    Ok(())
}

#[test]
fn dies_bad_type() -> anyhow::Result<()> {
    let invalid_type = "invalid";

    Command::cargo_bin(BIN)?
        .args(["-t", invalid_type])
        .assert()
        .failure()
        .stderr(predicate::str::contains(format!(
            "Invalid value for '--type <TYPE>': {}",
            invalid_type
        )));

    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> anyhow::Result<()> {
    let file = format_file_name(expected_file);
    let contents = fs::read_to_string(file.as_ref())?;
    let mut expected = split_by_newline(&contents);
    expected.sort();

    let cmd = Command::cargo_bin(BIN)?.args(args).assert().success();
    let output = cmd.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let mut lines = split_by_newline(&stdout);
    lines.sort();

    assert_eq!(lines, expected);

    Ok(())
}

#[cfg(windows)]
fn format_file_name(expected_file: &str) -> Cow<str> {
    format!("{}.windows", expected_file).into()
}

#[cfg(not(windows))]
fn format_file_name(expected_file: &str) -> Cow<str> {
    expected_file.into()
}

fn split_by_newline(s: &str) -> Vec<&str> {
    s.split('\n').filter(|s| !s.is_empty()).collect()
}

#[test]
fn test_find() -> anyhow::Result<()> {
    run(&["tests/inputs/dir"], "tests/expected/find.txt")
}

#[test]
fn test_find_name() -> anyhow::Result<()> {
    run(
        &["tests/inputs/dir", "-n", "file"],
        "tests/expected/name.txt",
    )
}

#[test]
fn test_find_entry_type() -> anyhow::Result<()> {
    run(
        &["tests/inputs/dir", "-t", "d"],
        "tests/expected/entry_type.txt",
    )
}
