use std::fs;

use assert_cmd::Command;
use predicates::prelude::predicate;

mod util;

const BIN: &str = "cut";
const CSV: &str = "tests/inputs/movies.csv";

#[test]
fn skips_bad_file() -> anyhow::Result<()> {
    let bad = util::gen_bad_file();
    let output = Command::cargo_bin(BIN)?.arg(bad).output()?;
    assert!(!output.status.success());
    Ok(())
}

fn dies(args: &[&str], expected: &str) -> anyhow::Result<()> {
    Command::cargo_bin(BIN)?
        .args(args)
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    Ok(())
}

#[test]
fn dies_not_enough_args() -> anyhow::Result<()> {
    dies(
        &[CSV],
        "the following required arguments were not provided:\n  \
        <--fields <FIELDS>|--bytes <BYTES>|--chars <CHARS>>",
    )
}

#[test]
fn dies_bad_digit_field() -> anyhow::Result<()> {
    let bad = util::random_string();
    dies(&[CSV, "-f", &bad], &format!("Invalid index: \"{}\"", &bad))
}

#[test]
fn dies_bad_digit_bytes() -> anyhow::Result<()> {
    let bad = util::random_string();
    dies(&[CSV, "-b", &bad], &format!("Invalid index: \"{}\"", &bad))
}

#[test]
fn dies_bad_digit_chars() -> anyhow::Result<()> {
    let bad = util::random_string();
    dies(&[CSV, "-c", &bad], &format!("Invalid index: \"{}\"", &bad))
}

#[test]
fn dies_empty_delimiter() -> anyhow::Result<()> {
    dies(
        &[CSV, "-f", "1", "-d", ""],
        r#"--delimiter "" must be a single byte"#,
    )
}

#[test]
fn dies_bad_delimiter() -> anyhow::Result<()> {
    dies(
        &[CSV, "-f", "1", "-d", "&&"],
        r#"--delimiter "&&" must be a single byte"#,
    )
}

#[test]
fn dies_bad_range_fields() -> anyhow::Result<()> {
    dies(&[CSV, "-f", "0-1"], r#"Invalid index: "0""#)?;
    dies(&[CSV, "-f", "1-1"], r#"Invalid range: "1-1""#)
}

#[test]
fn dies_bad_range_bytes() -> anyhow::Result<()> {
    dies(&[CSV, "-b", "0-1"], r#"Invalid index: "0""#)?;
    dies(&[CSV, "-b", "1-1"], r#"Invalid range: "1-1""#)
}

#[test]
fn dies_bad_range_chars() -> anyhow::Result<()> {
    dies(&[CSV, "-c", "0-1"], r#"Invalid index: "0""#)?;
    dies(&[CSV, "-c", "1-1"], r#"Invalid range: "1-1""#)
}

fn run(args: &[&str], expected_file: &str) -> anyhow::Result<()> {
    let expected = fs::read_to_string(expected_file);
    let output = Command::cargo_bin(BIN)?.args(args).output().expect("fail");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout.trim(), expected?);

    Ok(())
}

#[test]
fn test_csv_fields() -> anyhow::Result<()> {
    run(
        &[CSV, "-f", "1-2", "-d", ","],
        "tests/expected/movies.csv.f1-2.out",
    )
}

#[test]
fn test_cdv_bytes() -> anyhow::Result<()> {
    run(
        &[CSV, "-b", "1-2", "-d", ","],
        "tests/expected/movies.csv.b1-2.out",
    )
}

#[test]
fn test_csv_chars() -> anyhow::Result<()> {
    run(
        &[CSV, "-c", "8", "-d", ","],
        "tests/expected/movies.csv.c8.out",
    )
}
