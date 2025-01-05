use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("echo").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn run() {
    let mut cmd = Command::cargo_bin("echo").unwrap();
    cmd.arg("-t")
        .arg("hello")
        .arg("world")
        .assert()
        .stdout("hello world\n");
}
