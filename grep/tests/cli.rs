use std::{fs, path::Path};

use assert_cmd::Command;
use predicates::prelude::predicate;
use sys_info::os_type;

mod util;

const BIN: &str = "grep";
const EMPTY: &str = "tests/inputs/empty.txt";
const SAMPLE: &str = "tests/inputs/sample.txt";

#[test]
fn dies_no_args() -> anyhow::Result<()> {
    Command::cargo_bin(BIN)?
        .assert()
        .failure()
        .stderr(predicates::str::contains("Usage"));

    Ok(())
}

#[test]
fn dies_bad_pattern() -> anyhow::Result<()> {
    Command::cargo_bin(BIN)?
        .args(["*foo", SAMPLE])
        .assert()
        .failure()
        .stderr(predicate::str::contains(r#"Invalid pattern "*foo""#));

    Ok(())
}

#[test]
fn warns_bad_file() -> anyhow::Result<()> {
    let bad = util::gen_bad_file();
    let expected = format!("{bad}: .* [(]os error 2[)]");
    Command::cargo_bin(BIN)?
        .args(["foo", &bad])
        .assert()
        .stderr(predicate::str::is_match(expected)?);

    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> anyhow::Result<()> {
    let windows_file = format!("{expected_file}.windows");
    let expected_file = if os_type().unwrap() == "Windows" && Path::new(&windows_file).is_file() {
        &windows_file
    } else {
        expected_file
    };

    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin(BIN)?.args(args).output().expect("fail");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);
    Ok(())
}

#[test]
fn test_emty_file() -> anyhow::Result<()> {
    run(&["foo", EMPTY], "tests/expected/empty.foo")
}

#[test]
fn test_empty_regex() -> anyhow::Result<()> {
    run(&["", SAMPLE], "tests/expected/sample.empty_regex")
}

#[test]
fn test_insensitive() -> anyhow::Result<()> {
    run(
        &["--insensitive", "the", SAMPLE],
        "tests/expected/sample.insensitive",
    )
}

#[test]
fn test_recursive() -> anyhow::Result<()> {
    run(
        &["--recursive", "dog", "tests/inputs"],
        "tests/expected/sample.recursive",
    )
}

#[test]
fn test_count() -> anyhow::Result<()> {
    run(
        &["--recursive", "--count", "the", "tests/inputs"],
        "tests/expected/sample.count",
    )
}
