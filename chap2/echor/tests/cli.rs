use assert_cmd::Command;
use predicates::prelude::predicate;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello").assert().success();
}

#[test]
fn one_arg() {
    let outfile = "tests/expected/one_arg.txt";
    let expected = fs::read_to_string(outfile).unwrap();
    let mut cmd = Command::cargo_bin("echor").unwrap();

    cmd.arg("Hello there").assert().success().stdout(expected);
}

#[test]
fn dies_no_args_with_test_result() -> TestResult {
    Command::cargo_bin("echor")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));

    Ok(())
}

#[test]
fn one_arg_with_test_result() -> TestResult {
    let outfile = "tests/expected/one_arg.txt";
    let expected = fs::read_to_string(outfile)?;
    let mut cmd = Command::cargo_bin("echor")?;

    cmd.arg("Hello there").assert().success().stdout(expected);
    Ok(())
}

#[test]
fn two_args() -> TestResult {
    let expected = fs::read_to_string("tests/expected/two_args.txt")?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(vec!["Hello", "there"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn one_arg_with_n() -> TestResult {
    run(&["Hello  there", "-n"], "tests/expected/one_arg_with_n.txt")
}

#[test]
fn two_args_with_n() -> TestResult {
    run(&["-n", "Hello", "there"], "tests/expected/two_args_with_n.txt")
}

fn run(args: &[&str], expeced_file: &str) -> TestResult {
    let expected = fs::read_to_string(expeced_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}