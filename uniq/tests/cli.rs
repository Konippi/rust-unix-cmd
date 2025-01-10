use std::fs;

use assert_cmd::Command;
use tempfile::NamedTempFile;

const BIN: &str = "uniq";
const EMPTY: Test = Test {
    input: "tests/inputs/empty.txt",
    output: "tests/expected/empty.txt.out",
    output_count: "tests/expected/empty.txt.c.out",
};
const DUPLICATE: Test = Test {
    input: "tests/inputs/duplicate.txt",
    output: "tests/expected/duplicate.txt.out",
    output_count: "tests/expected/duplicate.txt.c.out",
};

mod util;

struct Test {
    input: &'static str,
    output: &'static str,
    output_count: &'static str,
}

#[test]
fn skip_bad_files() -> anyhow::Result<()> {
    let bad_file = util::gen_bad_file();
    let expected = format!("Failed to open {}", bad_file);

    Command::cargo_bin(BIN)?
        .arg(&bad_file)
        .assert()
        .failure()
        .stderr(predicates::str::is_match(expected)?);

    Ok(())
}

#[test]
fn test_empty() -> anyhow::Result<()> {
    run(&EMPTY)
}

#[test]
fn test_empty_count() -> anyhow::Result<()> {
    run_count(&EMPTY)
}

#[test]
fn test_empty_stdin() -> anyhow::Result<()> {
    run_stdin(&EMPTY)
}

#[test]
fn test_empty_stdin_count() -> anyhow::Result<()> {
    run_stdin_count(&EMPTY)
}

#[test]
fn test_empty_output_file() -> anyhow::Result<()> {
    run_output_file(&EMPTY)
}

#[test]
fn test_empty_output_file_count() -> anyhow::Result<()> {
    run_outfile_count(&EMPTY)
}

#[test]
fn test_duplicate() -> anyhow::Result<()> {
    run(&DUPLICATE)
}

#[test]
fn test_duplicate_count() -> anyhow::Result<()> {
    run_count(&DUPLICATE)
}

#[test]
fn test_duplicate_stdin() -> anyhow::Result<()> {
    run_stdin(&DUPLICATE)
}

#[test]
fn test_duplicate_stdin_count() -> anyhow::Result<()> {
    run_stdin_count(&DUPLICATE)
}

#[test]
fn test_duplicate_output_file() -> anyhow::Result<()> {
    run_output_file(&DUPLICATE)
}

#[test]
fn test_duplicate_output_file_count() -> anyhow::Result<()> {
    run_outfile_count(&DUPLICATE)
}

fn run(test: &Test) -> anyhow::Result<()> {
    let expected = fs::read_to_string(test.output)?;
    let output = Command::cargo_bin(BIN)?.arg(test.input).output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(expected, stdout);

    Ok(())
}

fn run_count(test: &Test) -> anyhow::Result<()> {
    let expected = fs::read_to_string(test.output_count)?;
    let output = Command::cargo_bin(BIN)?
        .args(&["-c", test.input])
        .output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(expected, stdout);

    Ok(())
}

fn run_stdin(test: &Test) -> anyhow::Result<()> {
    let input = fs::read_to_string(test.input)?;
    let expected = fs::read_to_string(test.output)?;
    let output = Command::cargo_bin(BIN)?.write_stdin(input).output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(expected, stdout);

    Ok(())
}

fn run_stdin_count(test: &Test) -> anyhow::Result<()> {
    let input = fs::read_to_string(test.input)?;
    let expected = fs::read_to_string(test.output_count)?;
    let output = Command::cargo_bin(BIN)?
        .arg("-c")
        .write_stdin(input)
        .output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(expected, stdout);

    Ok(())
}

fn run_output_file(test: &Test) -> anyhow::Result<()> {
    let expected = fs::read_to_string(test.output)?;
    let outfile = NamedTempFile::new()?;
    let outpath = &outfile.path().to_str().unwrap();

    Command::cargo_bin(BIN)?
        .args([test.input, outpath])
        .assert()
        .success()
        .stdout("");
    let contents = fs::read_to_string(outpath)?;
    assert_eq!(&expected, &contents);

    Ok(())
}

fn run_outfile_count(test: &Test) -> anyhow::Result<()> {
    let outfile = NamedTempFile::new()?;
    let outpath = &outfile.path().to_str().unwrap();

    Command::cargo_bin(BIN)?
        .args([test.input, outpath, "-c"])
        .assert()
        .success()
        .stdout("");

    let expected = fs::read_to_string(test.output_count)?;
    let contents = fs::read_to_string(outpath)?;
    assert_eq!(&expected, &contents);

    Ok(())
}
