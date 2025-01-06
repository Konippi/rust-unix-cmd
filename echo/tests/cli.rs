use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn dies_no_args() -> anyhow::Result<()> {
    Command::cargo_bin("echo")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));

    Ok(())
}

#[test]
fn run() -> anyhow::Result<()> {
    Command::cargo_bin("echo")?
        .arg("-t")
        .arg("hello")
        .arg("world")
        .assert()
        .stdout("hello world\n");

    Ok(())
}
