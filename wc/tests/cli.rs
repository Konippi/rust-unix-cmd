use assert_cmd::Command;

const BIN: &str = "wc";
const EMPTY: &str = "tests/inputs/empty.txt";
const MULTILINE: &str = "tests/inputs/multiline.txt";
const NUMBER: &str = "tests/inputs/number.txt";

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
fn wc_empty_file() -> anyhow::Result<()> {
    let expected = format!("       0       0       0 {}\n", EMPTY);

    Command::cargo_bin(BIN)?
        .arg(EMPTY)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn wc_multiline_file() -> anyhow::Result<()> {
    let expected = format!("       2      52     103 {}\n", MULTILINE);

    Command::cargo_bin(BIN)?
        .arg(MULTILINE)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn wc_multiple_files() -> anyhow::Result<()> {
    let expected = format!(
        "       2      52     103 {}
       2      52     103 total
       1      10      20 {}
       3      62     123 total\n",
        MULTILINE, NUMBER
    );

    Command::cargo_bin(BIN)?
        .arg(MULTILINE)
        .arg(NUMBER)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn wc_number_lines() -> anyhow::Result<()> {
    let expected = format!("      52 {}\n", MULTILINE);

    Command::cargo_bin(BIN)?
        .arg(MULTILINE)
        .arg("-w")
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn wc_number_words() -> anyhow::Result<()> {
    let expected = format!("      52 {}\n", MULTILINE);

    Command::cargo_bin(BIN)?
        .arg(MULTILINE)
        .arg("-w")
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn wc_number_bytes() -> anyhow::Result<()> {
    let expected = format!("     103 {}\n", MULTILINE);

    Command::cargo_bin(BIN)?
        .arg(MULTILINE)
        .arg("-c")
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn wc_number_chars() -> anyhow::Result<()> {
    let expected = format!("     103 {}\n", MULTILINE);

    Command::cargo_bin(BIN)?
        .arg(MULTILINE)
        .arg("-m")
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}
