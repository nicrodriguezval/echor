use assert_cmd::Command;
use predicates::prelude::*;

const BIN: &'static str = "echor";

#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin(BIN).unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin(BIN).unwrap();
    cmd
        .arg("-n")
        .arg("hello")
        .assert().success();
}