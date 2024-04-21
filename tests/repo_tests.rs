use core::panic;
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
#[case::non_production_commit("v0.1.0", "-n")]
#[case::production_commit("v1.1.0", "-n")]
#[trace]
fn test_repo_with_commit(
    #[case] current_version: &str,
    #[values(
        "fix", "chore", "ci", "revert", "docs", "style", "refactor", "perf", "test", "custom",
        "build", "feat", "breaking"
    )]
    mut commit_type: &str,
    #[case] arguments: &str,
) {
    // select expected result
    let expected = match current_version {
        "v0.1.0" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" => "patch\n0.1.1\n",
            "breaking" | "feat" => "minor\n0.2.0\n",
            _ => panic!("unexpected commit type"),
        },
        "v1.1.0" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" => "patch\n1.1.1\n",
            "feat" => "minor\n1.2.0\n",
            "breaking" => "major\n2.0.0\n",
            _ => panic!("unexpected commit type"),
        },
        _ => panic!("unexpected current version"),
    };

    // setup base state
    let (temp_dir, repo) = git_utils::create_test_git_directory(current_version);
    println!("temp_dir: {:?}", temp_dir);

    // setup the change conditions
    if commit_type == "breaking" {
        commit_type = "fix!";
    };
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
