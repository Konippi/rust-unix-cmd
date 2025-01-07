use std::fs;

use assert_cmd::Command;

const BIN: &str = "head";
const EMPTY: &str = "tests/inputs/empty.txt";
const ONELINE: &str = "tests/inputs/oneline.txt";
const UTF8: &str = "tests/inputs/utf8.txt";
const MULTILINE: &str = "tests/inputs/multiline.txt";

mod util;

#[test]
fn skip_bad_files() -> anyhow::Result<()> {
    let bad_file = util::gen_bad_file();
    let expected = format!("Failed to open {}", bad_file);

    Command::cargo_bin(BIN)?
        .arg(&bad_file)
        .assert()
        .success()
        .stderr(predicates::str::is_match(expected)?);

    Ok(())
}

#[test]
fn head_empty_file() -> anyhow::Result<()> {
    let expected = fs::read_to_string(EMPTY)?;

    Command::cargo_bin(BIN)?
        .arg(EMPTY)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn head_oneline_file() -> anyhow::Result<()> {
    let expected = fs::read_to_string(ONELINE)?;

    Command::cargo_bin(BIN)?
        .arg(ONELINE)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn head_utf8_file() -> anyhow::Result<()> {
    let expected = fs::read_to_string(UTF8)?;

    Command::cargo_bin(BIN)?
        .arg(UTF8)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn head_multiline_file() -> anyhow::Result<()> {
    let content = fs::read_to_string(MULTILINE)?;
    let default_lines = 10;
    let expected = util::ensure_trailing_newline(
        content
            .lines()
            .take(default_lines)
            .collect::<Vec<_>>()
            .join("\n"),
    );

    Command::cargo_bin(BIN)?
        .arg(MULTILINE)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn head_multiline_file_with_lines() -> anyhow::Result<()> {
    let content = fs::read_to_string(MULTILINE)?;
    let lines = 5;
    let expected =
        util::ensure_trailing_newline(content.lines().take(lines).collect::<Vec<_>>().join("\n"));

    Command::cargo_bin(BIN)?
        .arg(MULTILINE)
        .arg("-n")
        .arg(lines.to_string())
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn head_multiline_file_with_bytes() -> anyhow::Result<()> {
    let content = fs::read(MULTILINE)?;
    let bytes = 10;
    let expected = String::from_utf8_lossy(&content[..bytes.min(content.len())]).to_string();

    Command::cargo_bin(BIN)?
        .arg(MULTILINE)
        .arg("-c")
        .arg(bytes.to_string())
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}
