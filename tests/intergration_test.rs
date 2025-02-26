use assert_cmd::Command;
use predicates::str::contains;
use Team_Work_Lambert_w_function::lambert_w::lambert_function;

#[test]
fn test_valid_input() {
    let mut cmd = Command::cargo_bin("lambert_function").unwrap();
    cmd.arg("0.05")
        .assert()
        .success()
        .stdout(contains("W(0.05)"));
}

#[test]
fn test_out_of_domain_input() {
    let mut cmd = Command::cargo_bin("lambert_function").unwrap();
    cmd.arg("-1.0")
        .assert()
        .failure();
}

#[test]
fn test_lambert_function() {
    let result = lambert_function(0.05);
    assert!(result.is_ok(), "Function should return a valid result");
}