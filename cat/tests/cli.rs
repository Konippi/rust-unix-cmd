use std::fs;

use assert_cmd::Command;

const BIN: &str = "cat";
const EMPTY: &str = "tests/inputs/empty.txt";
const ONELINE: &str = "tests/inputs/oneline.txt";
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
fn cat_empty_file() -> anyhow::Result<()> {
    let expected = fs::read_to_string(EMPTY)?;

    Command::cargo_bin(BIN)?
        .arg(EMPTY)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn cat_oneline_file() -> anyhow::Result<()> {
    let content = fs::read_to_string(ONELINE)?;
    let expected = util::ensure_trailing_newline(content);

    Command::cargo_bin(BIN)?
        .arg(ONELINE)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn cat_multiline_file() -> anyhow::Result<()> {
    let content = fs::read_to_string(MULTILINE)?;
    let expected = util::ensure_trailing_newline(content);

    Command::cargo_bin(BIN)?
        .arg(MULTILINE)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn cat_with_number_lines() -> anyhow::Result<()> {
    let content = fs::read_to_string(MULTILINE)?
        .lines()
        .enumerate()
        .map(|(i, line)| format!("{:>6}\t{}", i + 1, line))
        .collect::<Vec<_>>()
        .join("\n");
    let expected = util::ensure_trailing_newline(content);

    Command::cargo_bin(BIN)?
        .arg(MULTILINE)
        .arg("-n")
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn cat_with_number_nonblank_lines() -> anyhow::Result<()> {
    let content = fs::read_to_string(MULTILINE)?;
    let mut line_number = 1;
    let expected = util::ensure_trailing_newline(
        content
            .lines()
            .map(|line| {
                if line.is_empty() {
                    line.to_string()
                } else {
                    let numbered_line = format!("{:>6}\t{}", line_number, line);
                    line_number += 1;
                    numbered_line
                }
            })
            .collect::<Vec<_>>()
            .join("\n"),
    );

    Command::cargo_bin(BIN)?
        .arg(MULTILINE)
        .arg("-b")
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}
