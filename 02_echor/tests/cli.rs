use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("USAGE"));
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello").assert().success();
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn hello1() {
    run(&["Hello there"], "tests/expected/hello1.txt");
}

#[test]
fn hello1n() {
  run(&["Hello there", "-n"], "tests/expected/hello1n.txt");
}

#[test]
fn hello2() {
    run(&["Hello there"], "tests/expected/hello2.txt");
}

#[test]
fn hello2n() {
  run(&["Hello", "there", "-n"], "tests/expected/hello2n.txt");
}

