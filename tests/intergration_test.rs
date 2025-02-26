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

// #[test]
// fn test_out_of_domain_input() {
//     let mut cmd = Command::cargo_bin("team_work_lambert_w_function").unwrap();
//     cmd.arg("-10.0")
//         .assert()
//         .failure()
//         .stderr(contains("value should be more than -1/e")); // Update to match the actual error message
// }

// #[test]
// fn test_lambert_function() {
//     let result = lambert_function(0.05);
//     assert!(result.is_ok(), "Function should return a valid result");
// }