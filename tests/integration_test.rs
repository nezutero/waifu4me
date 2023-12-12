use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_cli_with_valid_arguments() {
    let mut cmd = Command::cargo_bin("waifu4me").unwrap();
    cmd.arg("-t").arg("sfw").arg("-c").arg("waifu");

    cmd.assert().success();
}

#[test]
fn test_cli_with_invalid_arguments() {
    let mut cmd = Command::cargo_bin("waifu4me").unwrap();
    cmd.arg("-t")
        .arg("invalid_type")
        .arg("-c")
        .arg("invalid_category");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error"));
}
