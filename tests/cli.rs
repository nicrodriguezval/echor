use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

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

#[test]
fn hello_1() {
    let outfile = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(outfile).unwrap();
    let mut cmd = Command::cargo_bin(BIN).unwrap();
    cmd
        .arg("Hello there")
        .assert()
        .success()
        .stdout(expected);
}