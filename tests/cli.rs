use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const BIN: &'static str = "echor";

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin(BIN)?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin(BIN)?;
    cmd
        .arg("-n")
        .arg("hello")
        .assert().success();
    Ok(())
}

#[test]
fn hello_1() -> TestResult {
    let outfile = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(outfile)?;
    let mut cmd = Command::cargo_bin(BIN)?;
    cmd
        .arg("Hello there")
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn hello_2() -> TestResult {
    let outfile = "tests/expected/hello2.txt";
    let expected = fs::read_to_string(outfile)?;
    let mut cmd = Command::cargo_bin(BIN)?;
    cmd
        .args(&["Hello", "there"])
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}
