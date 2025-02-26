use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn test_valid_input() {
    let mut cmd = Command::cargo_bin("team_work_lambert_w_function").unwrap();
    cmd.arg("0.05")
        .assert()
        .success()
        .stdout(contains("W(0.05) = "));
}
