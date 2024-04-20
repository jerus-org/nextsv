use std::{fs, process::Command};

use rstest::rstest;
use snapbox::cmd::cargo_bin;

mod git_utils;

#[test]
fn test_repo_no_changes() {
    let expected = "none\n";

    let (temp_dir, _repo) = git_utils::create_test_git_directory("v0.1.0");
    println!("temp_dir: {:?}", temp_dir);

    let output = Command::new(cargo_bin!("nextsv"))
        // .arg("run")
        .current_dir(&temp_dir)
        .output()
        .unwrap();

    let test_result = String::from_utf8(output.stdout).unwrap();

    println!("stdout: {}", test_result);
    println!("stderr: {}", String::from_utf8(output.stderr).unwrap());

    let result = fs::remove_dir_all(temp_dir);
    println!("remove_dir_all result: {:?}", result);
    assert_eq!(expected, test_result);
}

#[rstest]
#[case::fix_commit_non_production("v0.1.0", "fix", "-n -vvv", "patch\n0.1.1\n")]
#[case::chore_commit_non_production("v0.1.0", "chore", "-n", "patch\n0.1.1\n")]
#[case::ci_commit_non_production("v0.1.0", "ci", "-n", "patch\n0.1.1\n")]
#[case::revert_commit_non_production("v0.1.0", "revert", "-n", "patch\n0.1.1\n")]
#[case::docs_commit_non_production("v0.1.0", "docs", "-n", "patch\n0.1.1\n")]
#[case::style_commit_non_production("v0.1.0", "style", "-n", "patch\n0.1.1\n")]
#[case::refactor_commit_non_production("v0.1.0", "refactor", "-n", "patch\n0.1.1\n")]
#[case::perf_commit_non_production("v0.1.0", "perf", "-n", "patch\n0.1.1\n")]
#[case::test_commit_non_production("v0.1.0", "test", "-n", "patch\n0.1.1\n")]
#[case::build_commit_non_production("v0.1.0", "build", "-n", "patch\n0.1.1\n")]
#[case::feat_commit_non_production("v0.1.0", "feat", "-n", "minor\n0.2.0\n")]
#[case::breaking_commit_non_production("v0.1.0", "fix!", "-n", "minor\n0.2.0\n")]
#[case::custom_commit_non_production("v0.1.0", "custom", "-n", "patch\n0.1.1\n")]
#[case::fix_commit_production("v1.1.0", "fix", "-n -vvv", "patch\n1.1.1\n")]
#[case::chore_commit_non_production("v1.1.0", "chore", "-n", "patch\n1.1.1\n")]
#[case::ci_commit_production("v1.1.0", "ci", "-n", "patch\n1.1.1\n")]
#[case::revert_commit_production("v1.1.0", "revert", "-n", "patch\n1.1.1\n")]
#[case::docs_commit_production("v1.1.0", "docs", "-n", "patch\n1.1.1\n")]
#[case::style_commit_production("v1.1.0", "style", "-n", "patch\n1.1.1\n")]
#[case::refactor_commit_production("v1.1.0", "refactor", "-n", "patch\n1.1.1\n")]
#[case::perf_commit_production("v1.1.0", "perf", "-n", "patch\n1.1.1\n")]
#[case::test_commit_production("v1.1.0", "test", "-n", "patch\n1.1.1\n")]
#[case::build_commit_production("v1.1.0", "build", "-n", "patch\n1.1.1\n")]
#[case::feat_commit_production("v1.1.0", "feat", "-n", "minor\n1.2.0\n")]
#[case::breaking_commit_production("v1.1.0", "fix!", "-n", "major\n2.0.0\n")]
#[case::custom_commit_production("v1.1.0", "custom", "-n", "patch\n1.1.1\n")]
#[trace]
fn test_repo_with_commit(
    #[case] current_version: &str,
    #[case] commit_type: &str,
    #[case] arguments: &str,
    #[case] expected: &str,
) {
    // setup base state
    let (temp_dir, repo) = git_utils::create_test_git_directory(current_version);
    println!("temp_dir: {:?}", temp_dir);

    // setup the change conditions
    let message = format!("{}: {}", commit_type, "test commit");
    println!("message: {:?}", message);
    let result = git_utils::create_file_and_commit(&repo, temp_dir.clone(), &message);
    println!("commit result: {:?}", result);

    // execute the test
    let test_result = git_utils::execute_test(arguments, &temp_dir);

    // tidy up the test environment
    let result = fs::remove_dir_all(temp_dir);
    println!("remove_dir_all result: {:?}", result);

    // assess the result
    assert_eq!(expected, test_result);
}
