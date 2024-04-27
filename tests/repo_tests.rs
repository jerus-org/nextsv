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
#[case::non_production_pre_release_alpha_commit("v0.1.0-alpha.2", "-n")]
#[case::production_pre_release_alpha_commit("v1.1.0-alpha.3", "-n")]
#[case::non_production_pre_release_beta_commit("v0.1.0-beta.4", "-n")]
#[case::production_pre_release_beta_commit("v1.1.0-beta.5", "-n")]
#[case::non_production_pre_release_rc_commit("v0.1.0-rc.6", "-n")]
#[case::production_pre_release_rc_commit("v1.1.0-rc.7", "-n")]
#[case::non_production_pre_release_pre_commit("v0.1.0-pre.8", "-n -vvvv")]
#[case::production_pre_release_pre_commit("v1.1.0-pre.9", "-n -vvvv")]
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
        "v0.1.0-alpha.2" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "breaking" | "feat" => "alpha\n0.1.0-alpha.3\n",
            _ => panic!("unexpected commit type"),
        },
        "v1.1.0-alpha.3" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "breaking" | "feat" => "alpha\n1.1.0-alpha.4\n",
            _ => panic!("unexpected commit type"),
        },
        "v0.1.0-beta.4" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "breaking" | "feat" => "beta\n0.1.0-beta.5\n",
            _ => panic!("unexpected commit type"),
        },
        "v1.1.0-beta.5" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "breaking" | "feat" => "beta\n1.1.0-beta.6\n",
            _ => panic!("unexpected commit type"),
        },
        "v0.1.0-rc.6" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "breaking" | "feat" => "rc\n0.1.0-rc.7\n",
            _ => panic!("unexpected commit type"),
        },
        "v1.1.0-rc.7" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "breaking" | "feat" => "rc\n1.1.0-rc.8\n",
            _ => panic!("unexpected commit type"),
        },
        "v0.1.0-pre.8" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "breaking" | "feat" => "0.1.0-pre.9\n0.1.0-pre.9\n",
            _ => panic!("unexpected commit type"),
        },
        "v1.1.0-pre.9" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "breaking" | "feat" => "1.1.0-pre.10\n1.1.0-pre.10\n",
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
    let result = git_utils::create_file_and_commit(&repo, temp_dir.clone(), &message, None);
    println!("commit result: {:?}", result);

    // execute the test
    let test_result = git_utils::execute_test(arguments, &temp_dir);

    // tidy up the test environment
    let result = fs::remove_dir_all(temp_dir);
    println!("remove_dir_all result: {:?}", result);

    // assess the result
    assert_eq!(expected, test_result);
}

#[rstest]
#[case::non_production_commit("v0.1.0", "-n -vvv -f ")]
#[case::production_commit("v1.1.0", "-n -f")]
#[case::non_production_pre_release_alpha_commit("v0.1.0-alpha.2", "-n -vvvv -f")]
#[case::production_pre_release_alpha_commit("v1.1.0-alpha.3", "-n -vvvv -f")]
#[case::non_production_pre_release_beta_commit("v0.1.0-beta.4", "-n -f")]
#[case::production_pre_release_beta_commit("v1.1.0-beta.5", "-n -f")]
#[case::non_production_pre_release_rc_commit("v0.1.0-rc.6", "-n -f")]
#[case::production_pre_release_rc_commit("v1.1.0-rc.7", "-n -vvv -f")]
#[case::non_production_pre_release_pre_commit("v0.1.0-pre.8", "-n -f")]
#[case::production_pre_release_pre_commit("v1.1.0-pre.9", "-n -f")]
#[trace]
fn test_repo_with_commit_and_force_bump(
    #[case] current_version: &str,
    #[values(
        "fix", "chore", "ci", "revert", "docs", "style", "refactor", "perf", "test", "custom",
        "build", "feat", "breaking"
    )]
    mut commit_type: &str,
    #[case] arguments: &str,
    #[values("major", "minor", "patch", "first", "release", "rc", "beta", "alpha")]
    force_bump: &str,
) {
    // select expected result
    let expected = match current_version {
        "v0.1.0" => match force_bump {
            "major" => "major\n1.0.0\n",
            "minor" => "minor\n0.2.0\n",
            "patch" => "patch\n0.1.1\n",
            "first" => "1.0.0\n1.0.0\n",
            "release" => "none\n",
            "rc" => match commit_type {
                "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf"
                | "test" | "custom" | "build" => "rc\n0.1.1-rc.1\n",
                "feat" | "breaking" => "rc\n0.2.0-rc.1\n",
                _ => panic!("unexpected commit type"),
            },
            "beta" => match commit_type {
                "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf"
                | "test" | "custom" | "build" => "beta\n0.1.1-beta.1\n",
                "feat" | "breaking" => "beta\n0.2.0-beta.1\n",
                _ => panic!("unexpected commit type"),
            },
            "alpha" => match commit_type {
                "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf"
                | "test" | "custom" | "build" => "alpha\n0.1.1-alpha.1\n",
                "feat" | "breaking" => "alpha\n0.2.0-alpha.1\n",
                _ => panic!("unexpected commit type"),
            },
            _ => panic!("unexpected force_bump"),
        },
        "v1.1.0" => match force_bump {
            "major" => "major\n2.0.0\n",
            "minor" => "minor\n1.2.0\n",
            "patch" => "patch\n1.1.1\n",
            "first" => "none\n",
            "release" => "none\n",
            "rc" => match commit_type {
                "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf"
                | "test" | "custom" | "build" => "rc\n1.1.1-rc.1\n",
                "feat" => "rc\n1.2.0-rc.1\n",
                "breaking" => "rc\n2.0.0-rc.1\n",
                _ => panic!("unexpected commit type"),
            },
            "beta" => match commit_type {
                "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf"
                | "test" | "custom" | "build" => "beta\n1.1.1-beta.1\n",
                "feat" => "beta\n1.2.0-beta.1\n",
                "breaking" => "beta\n2.0.0-beta.1\n",
                _ => panic!("unexpected commit type"),
            },
            "alpha" => match commit_type {
                "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf"
                | "test" | "custom" | "build" => "alpha\n1.1.1-alpha.1\n",
                "feat" => "alpha\n1.2.0-alpha.1\n",
                "breaking" => "alpha\n2.0.0-alpha.1\n",
                _ => panic!("unexpected commit type"),
            },
            _ => panic!("unexpected force_bump"),
        },
        "v0.1.0-alpha.2" => match force_bump {
            "major" => "none\n",
            "minor" => "none\n",
            "patch" => "none\n",
            "first" => "1.0.0\n1.0.0\n",
            "release" => "release\n0.1.0\n",
            "rc" => "rc\n0.1.0-rc.1\n",
            "beta" => "beta\n0.1.0-beta.1\n",
            "alpha" => "alpha\n0.1.0-alpha.3\n",
            _ => panic!("unexpected force_bump"),
        },
        "v1.1.0-alpha.3" => match force_bump {
            "major" => "none\n",
            "minor" => "none\n",
            "patch" => "none\n",
            "first" => "none\n",
            "release" => "release\n1.1.0\n",
            "rc" => "rc\n1.1.0-rc.1\n",
            "beta" => "beta\n1.1.0-beta.1\n",
            "alpha" => "alpha\n1.1.0-alpha.4\n",
            _ => panic!("unexpected force_bump"),
        },
        "v0.1.0-beta.4" => match force_bump {
            "major" => "none\n",
            "minor" => "none\n",
            "patch" => "none\n",
            "first" => "1.0.0\n1.0.0\n",
            "release" => "release\n0.1.0\n",
            "rc" => "rc\n0.1.0-rc.1\n",
            "beta" => "beta\n0.1.0-beta.5\n",
            "alpha" => "alpha\n0.1.0-alpha.1\n",
            _ => panic!("unexpected force_bump"),
        },
        "v1.1.0-beta.5" => match force_bump {
            "major" => "none\n",
            "minor" => "none\n",
            "patch" => "none\n",
            "first" => "none\n",
            "release" => "release\n1.1.0\n",
            "rc" => "rc\n1.1.0-rc.1\n",
            "beta" => "beta\n1.1.0-beta.6\n",
            "alpha" => "alpha\n1.1.0-alpha.1\n",
            _ => panic!("unexpected force_bump"),
        },
        "v0.1.0-rc.6" => match force_bump {
            "major" => "none\n",
            "minor" => "none\n",
            "patch" => "none\n",
            "first" => "1.0.0\n1.0.0\n",
            "release" => "release\n0.1.0\n",
            "rc" => "rc\n0.1.0-rc.7\n",
            "beta" => "beta\n0.1.0-beta.1\n",
            "alpha" => "alpha\n0.1.0-alpha.1\n",
            _ => panic!("unexpected force_bump"),
        },
        "v1.1.0-rc.7" => match force_bump {
            "major" => "none\n",
            "minor" => "none\n",
            "patch" => "none\n",
            "first" => "none\n",
            "release" => "release\n1.1.0\n",
            "rc" => "rc\n1.1.0-rc.8\n",
            "beta" => "beta\n1.1.0-beta.1\n",
            "alpha" => "alpha\n1.1.0-alpha.1\n",
            _ => panic!("unexpected force_bump"),
        },
        "v0.1.0-pre.8" => match force_bump {
            "major" => "none\n",
            "minor" => "none\n",
            "patch" => "none\n",
            "first" => "1.0.0\n1.0.0\n",
            "release" => "release\n0.1.0\n",
            "rc" => "rc\n0.1.0-rc.1\n",
            "beta" => "beta\n0.1.0-beta.1\n",
            "alpha" => "alpha\n0.1.0-alpha.1\n",
            _ => panic!("unexpected force_bump"),
        },
        "v1.1.0-pre.9" => match force_bump {
            "major" => "none\n",
            "minor" => "none\n",
            "patch" => "none\n",
            "first" => "none\n",
            "release" => "release\n1.1.0\n",
            "rc" => "rc\n1.1.0-rc.1\n",
            "beta" => "beta\n1.1.0-beta.1\n",
            "alpha" => "alpha\n1.1.0-alpha.1\n",
            _ => panic!("unexpected force_bump"),
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
    let result = git_utils::create_file_and_commit(&repo, temp_dir.clone(), &message, None);
    println!("commit result: {:?}", result);

    // execute the test
    let mut arguments = arguments.to_string();
    arguments.push(' ');
    arguments.push_str(force_bump);
    let test_result = git_utils::execute_test(&arguments, &temp_dir);

    // tidy up the test environment
    let result = fs::remove_dir_all(temp_dir);
    println!("remove_dir_all result: {:?}", result);

    // assess the result
    assert_eq!(expected, test_result);
}

#[rstest]
#[case::non_production_commit("v0.1.0", "-n -vvv -c ")]
#[case::production_commit("v1.1.0", "-n -c")]
#[case::non_production_pre_release_alpha_commit("v0.1.0-alpha.2", "-n -vvvv -c")]
#[case::production_pre_release_alpha_commit("v1.1.0-alpha.3", "-n -vvvv -c")]
#[case::non_production_pre_release_beta_commit("v0.1.0-beta.4", "-n -c")]
#[case::production_pre_release_beta_commit("v1.1.0-beta.5", "-n -c")]
#[case::non_production_pre_release_rc_commit("v0.1.0-rc.6", "-n -c")]
#[case::production_pre_release_rc_commit("v1.1.0-rc.7", "-n -c")]
#[case::non_production_pre_release_pre_commit("v0.1.0-pre.8", "-n -c")]
#[case::production_pre_release_pre_commit("v1.1.0-pre.9", "-n -c")]
#[trace]
fn test_repo_with_commit_and_check(
    #[case] current_version: &str,
    #[values(
        "fix", "chore", "ci", "revert", "docs", "style", "refactor", "perf", "test", "custom",
        "build", "feat", "breaking"
    )]
    mut commit_type: &str,
    #[case] arguments: &str,
    #[values("other", "fix", "feature", "breaking")] check: &str,
) {
    // select expected result
    let expected = match current_version {
        "v0.1.0" => match commit_type {
            "chore" | "ci" | "docs" | "style" | "refactor" | "perf" | "test" | "custom"
            | "build" => match check {
                "other" => "patch\n0.1.1\n",
                "fix" => "none\n",
                "feature" => "none\n",
                "breaking" => "none\n",
                _ => panic!("unexpected check"),
            },
            "fix" | "revert" => match check {
                "other" => "patch\n0.1.1\n",
                "fix" => "patch\n0.1.1\n",
                "feature" => "none\n",
                "breaking" => "none\n",
                _ => panic!("unexpected check"),
            },
            "feat" => match check {
                "other" => "minor\n0.2.0\n",
                "fix" => "minor\n0.2.0\n",
                "feature" => "minor\n0.2.0\n",
                "breaking" => "none\n",
                _ => panic!("unexpected check"),
            },
            "breaking" => match check {
                "other" => "minor\n0.2.0\n",
                "fix" => "minor\n0.2.0\n",
                "feature" => "minor\n0.2.0\n",
                "breaking" => "minor\n0.2.0\n",
                _ => panic!("unexpected check"),
            },
            _ => panic!("unexpected commit type"),
        },
        "v1.1.0" => match commit_type {
            "chore" | "ci" | "docs" | "style" | "refactor" | "perf" | "test" | "custom"
            | "build" => match check {
                "other" => "patch\n1.1.1\n",
                "fix" => "none\n",
                "feature" => "none\n",
                "breaking" => "none\n",
                _ => panic!("unexpected check"),
            },
            "fix" | "revert" => match check {
                "other" => "patch\n1.1.1\n",
                "fix" => "patch\n1.1.1\n",
                "feature" => "none\n",
                "breaking" => "none\n",
                _ => panic!("unexpected check"),
            },
            "feat" => match check {
                "other" => "minor\n1.2.0\n",
                "fix" => "minor\n1.2.0\n",
                "feature" => "minor\n1.2.0\n",
                "breaking" => "none\n",
                _ => panic!("unexpected check"),
            },
            "breaking" => match check {
                "other" => "major\n2.0.0\n",
                "fix" => "major\n2.0.0\n",
                "feature" => "major\n2.0.0\n",
                "breaking" => "major\n2.0.0\n",
                _ => panic!("unexpected check"),
            },
            _ => panic!("unexpected commit type"),
        },
        "v0.1.0-alpha.2" => match commit_type {
            "chore" | "ci" | "docs" | "style" | "refactor" | "perf" | "test" | "custom"
            | "build" => match check {
                "other" => "alpha\n0.1.0-alpha.3\n",
                "fix" => "none\n",
                "feature" => "none\n",
                "breaking" => "none\n",
                _ => panic!("unexpected check"),
            },
            "fix" | "revert" => match check {
                "other" => "alpha\n0.1.0-alpha.3\n",
                "fix" => "alpha\n0.1.0-alpha.3\n",
                "feature" => "none\n",
                "breaking" => "none\n",
                _ => panic!("unexpected check"),
            },
            "feat" => match check {
                "other" => "alpha\n0.1.0-alpha.3\n",
                "fix" => "alpha\n0.1.0-alpha.3\n",
                "feature" => "alpha\n0.1.0-alpha.3\n",
                "breaking" => "none\n",
                _ => panic!("unexpected check"),
            },
            "breaking" => match check {
                "other" => "alpha\n0.1.0-alpha.3\n",
                "fix" => "alpha\n0.1.0-alpha.3\n",
                "feature" => "alpha\n0.1.0-alpha.3\n",
                "breaking" => "alpha\n0.1.0-alpha.3\n",
                _ => panic!("unexpected check"),
            },
            _ => panic!("unexpected commit type"),
        },
        "v1.1.0-alpha.3" => match commit_type {
            "chore" | "ci" | "docs" | "style" | "refactor" | "perf" | "test" | "custom"
            | "build" => match check {
                "other" => "alpha\n1.1.0-alpha.4\n",
                "fix" => "none\n",
                "feature" => "none\n",
                "breaking" => "none\n",
                _ => panic!("unexpected check"),
            },
            "fix" | "revert" => match check {
                "other" => "alpha\n1.1.0-alpha.4\n",
                "fix" => "alpha\n1.1.0-alpha.4\n",
                "feature" => "none\n",
                "breaking" => "none\n",
                _ => panic!("unexpected check"),
            },
            "feat" => match check {
                "other" => "alpha\n1.1.0-alpha.4\n",
                "fix" => "alpha\n1.1.0-alpha.4\n",
                "feature" => "alpha\n1.1.0-alpha.4\n",
                "breaking" => "none\n",
                _ => panic!("unexpected check"),
            },
            "breaking" => match check {
                "other" => "alpha\n1.1.0-alpha.4\n",
                "fix" => "alpha\n1.1.0-alpha.4\n",
                "feature" => "alpha\n1.1.0-alpha.4\n",
                "breaking" => "alpha\n1.1.0-alpha.4\n",
                _ => panic!("unexpected check"),
            },
            _ => panic!("unexpected commit type"),
        },
        "v0.1.0-beta.4" => match commit_type {
            "chore" | "ci" | "docs" | "style" | "refactor" | "perf" | "test" | "custom"
            | "build" => match check {
                "other" => "beta\n0.1.0-beta.5\n",
                "fix" => "none\n",
                "feature" => "none\n",
                "breaking" => "none\n",
                _ => panic!("unexpected check"),
            },
            "fix" | "revert" => match check {
                "other" => "beta\n0.1.0-beta.5\n",
                "fix" => "beta\n0.1.0-beta.5\n",
                "feature" => "none\n",
                "breaking" => "none\n",
                _ => panic!("unexpected check"),
            },
            "feat" => match check {
                "other" => "beta\n0.1.0-beta.5\n",
                "fix" => "beta\n0.1.0-beta.5\n",
                "feature" => "beta\n0.1.0-beta.5\n",
                "breaking" => "none\n",
                _ => panic!("unexpected check"),
            },
            "breaking" => match check {
                "other" => "beta\n0.1.0-beta.5\n",
                "fix" => "beta\n0.1.0-beta.5\n",
                "feature" => "beta\n0.1.0-beta.5\n",
                "breaking" => "beta\n0.1.0-beta.5\n",
                _ => panic!("unexpected check"),
            },
            _ => panic!("unexpected commit type"),
        },
        "v1.1.0-beta.5" => match commit_type {
            "chore" | "ci" | "docs" | "style" | "refactor" | "perf" | "test" | "custom"
            | "build" => match check {
                "other" => "beta\n1.1.0-beta.6\n",
                "fix" => "none\n",
                "feature" => "none\n",
                "breaking" => "none\n",
                _ => panic!("unexpected check"),
            },
            "fix" | "revert" => match check {
                "other" => "beta\n1.1.0-beta.6\n",
                "fix" => "beta\n1.1.0-beta.6\n",
                "feature" => "none\n",
                "breaking" => "none\n",
                _ => panic!("unexpected check"),
            },
            "feat" => match check {
                "other" => "beta\n1.1.0-beta.6\n",
                "fix" => "beta\n1.1.0-beta.6\n",
                "feature" => "beta\n1.1.0-beta.6\n",
                "breaking" => "none\n",
                _ => panic!("unexpected check"),
            },
            "breaking" => match check {
                "other" => "beta\n1.1.0-beta.6\n",
                "fix" => "beta\n1.1.0-beta.6\n",
                "feature" => "beta\n1.1.0-beta.6\n",
                "breaking" => "beta\n1.1.0-beta.6\n",
                _ => panic!("unexpected check"),
            },
            _ => panic!("unexpected commit type"),
        },
        "v0.1.0-rc.6" => match commit_type {
            "chore" | "ci" | "docs" | "style" | "refactor" | "perf" | "test" | "custom"
            | "build" => match check {
                "other" => "rc\n0.1.0-rc.7\n",
                "fix" => "none\n",
                "feature" => "none\n",
                "breaking" => "none\n",
                _ => panic!("unexpected check"),
            },
            "fix" | "revert" => match check {
                "other" => "rc\n0.1.0-rc.7\n",
                "fix" => "rc\n0.1.0-rc.7\n",
                "feature" => "none\n",
                "breaking" => "none\n",
                _ => panic!("unexpected check"),
            },
            "feat" => match check {
                "other" => "rc\n0.1.0-rc.7\n",
                "fix" => "rc\n0.1.0-rc.7\n",
                "feature" => "rc\n0.1.0-rc.7\n",
                "breaking" => "none\n",
                _ => panic!("unexpected check"),
            },
            "breaking" => match check {
                "other" => "rc\n0.1.0-rc.7\n",
                "fix" => "rc\n0.1.0-rc.7\n",
                "feature" => "rc\n0.1.0-rc.7\n",
                "breaking" => "rc\n0.1.0-rc.7\n",
                _ => panic!("unexpected check"),
            },
            _ => panic!("unexpected commit type"),
        },
        "v1.1.0-rc.7" => match commit_type {
            "chore" | "ci" | "docs" | "style" | "refactor" | "perf" | "test" | "custom"
            | "build" => match check {
                "other" => "rc\n1.1.0-rc.8\n",
                "fix" => "none\n",
                "feature" => "none\n",
                "breaking" => "none\n",
                _ => panic!("unexpected check"),
            },
            "fix" | "revert" => match check {
                "other" => "rc\n1.1.0-rc.8\n",
                "fix" => "rc\n1.1.0-rc.8\n",
                "feature" => "none\n",
                "breaking" => "none\n",
                _ => panic!("unexpected check"),
            },
            "feat" => match check {
                "other" => "rc\n1.1.0-rc.8\n",
                "fix" => "rc\n1.1.0-rc.8\n",
                "feature" => "rc\n1.1.0-rc.8\n",
                "breaking" => "none\n",
                _ => panic!("unexpected check"),
            },
            "breaking" => match check {
                "other" => "rc\n1.1.0-rc.8\n",
                "fix" => "rc\n1.1.0-rc.8\n",
                "feature" => "rc\n1.1.0-rc.8\n",
                "breaking" => "rc\n1.1.0-rc.8\n",
                _ => panic!("unexpected check"),
            },
            _ => panic!("unexpected commit type"),
        },
        "v0.1.0-pre.8" => match commit_type {
            "chore" | "ci" | "docs" | "style" | "refactor" | "perf" | "test" | "custom"
            | "build" => match check {
                "other" => "0.1.0-pre.9\n0.1.0-pre.9\n",
                "fix" => "none\n",
                "feature" => "none\n",
                "breaking" => "none\n",
                _ => panic!("unexpected check"),
            },
            "fix" | "revert" => match check {
                "other" => "0.1.0-pre.9\n0.1.0-pre.9\n",
                "fix" => "0.1.0-pre.9\n0.1.0-pre.9\n",
                "feature" => "none\n",
                "breaking" => "none\n",
                _ => panic!("unexpected check"),
            },
            "feat" => match check {
                "other" => "0.1.0-pre.9\n0.1.0-pre.9\n",
                "fix" => "0.1.0-pre.9\n0.1.0-pre.9\n",
                "feature" => "0.1.0-pre.9\n0.1.0-pre.9\n",
                "breaking" => "none\n",
                _ => panic!("unexpected check"),
            },
            "breaking" => match check {
                "other" => "0.1.0-pre.9\n0.1.0-pre.9\n",
                "fix" => "0.1.0-pre.9\n0.1.0-pre.9\n",
                "feature" => "0.1.0-pre.9\n0.1.0-pre.9\n",
                "breaking" => "0.1.0-pre.9\n0.1.0-pre.9\n",
                _ => panic!("unexpected check"),
            },
            _ => panic!("unexpected commit type"),
        },
        "v1.1.0-pre.9" => match commit_type {
            "chore" | "ci" | "docs" | "style" | "refactor" | "perf" | "test" | "custom"
            | "build" => match check {
                "other" => "1.1.0-pre.10\n1.1.0-pre.10\n",
                "fix" => "none\n",
                "feature" => "none\n",
                "breaking" => "none\n",
                _ => panic!("unexpected check"),
            },
            "fix" | "revert" => match check {
                "other" => "1.1.0-pre.10\n1.1.0-pre.10\n",
                "fix" => "1.1.0-pre.10\n1.1.0-pre.10\n",
                "feature" => "none\n",
                "breaking" => "none\n",
                _ => panic!("unexpected check"),
            },
            "feat" => match check {
                "other" => "1.1.0-pre.10\n1.1.0-pre.10\n",
                "fix" => "1.1.0-pre.10\n1.1.0-pre.10\n",
                "feature" => "1.1.0-pre.10\n1.1.0-pre.10\n",
                "breaking" => "none\n",
                _ => panic!("unexpected check"),
            },
            "breaking" => match check {
                "other" => "1.1.0-pre.10\n1.1.0-pre.10\n",
                "fix" => "1.1.0-pre.10\n1.1.0-pre.10\n",
                "feature" => "1.1.0-pre.10\n1.1.0-pre.10\n",
                "breaking" => "1.1.0-pre.10\n1.1.0-pre.10\n",
                _ => panic!("unexpected check"),
            },
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
    let result = git_utils::create_file_and_commit(&repo, temp_dir.clone(), &message, None);
    println!("commit result: {:?}", result);

    // execute the test
    let mut arguments = arguments.to_string();
    arguments.push(' ');
    arguments.push_str(check);
    let test_result = git_utils::execute_test(&arguments, &temp_dir);

    // tidy up the test environment
    let result = fs::remove_dir_all(temp_dir);
    println!("remove_dir_all result: {:?}", result);

    // assess the result
    assert_eq!(expected, test_result);
}

#[rstest]
#[case::non_production_commit("v0.1.0", "-n -vvvv -r")]
#[case::production_commit("v1.1.0", "-n -r")]
#[case::non_production_pre_release_alpha_commit("v0.1.0-alpha.2", "-n -r")]
#[case::production_pre_release_alpha_commit("v1.1.0-alpha.3", "-n -r")]
#[case::non_production_pre_release_beta_commit("v0.1.0-beta.4", "-n -vvv -r")]
#[case::production_pre_release_beta_commit("v1.1.0-beta.5", "-n -r")]
#[case::non_production_pre_release_rc_commit("v0.1.0-rc.6", "-n -r")]
#[case::production_pre_release_rc_commit("v1.1.0-rc.7", "-n -r")]
#[case::non_production_pre_release_pre_commit("v0.1.0-pre.8", "-n -r")]
#[case::production_pre_release_pre_commit("v1.1.0-pre.9", "-n -r")]
#[trace]
fn test_repo_with_commit_and_enforce_test_file(
    #[case] current_version: &str,
    #[values(
        "fix", "chore", "ci", "revert", "docs", "style", "refactor", "perf", "test", "custom",
        "build", "feat", "breaking"
    )]
    mut commit_type: &str,
    #[case] arguments: &str,
    #[values("test.txt", "missing.txt", "first-file -r test.txt")] file: &str,
    #[values("other", "fix", "feature", "breaking")] check: &str,
) {
    // select expected result
    let expected = match current_version {
        "v0.1.0" => match file {
            "test.txt" | "missing.txt" => match commit_type {
                "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf"
                | "test" | "custom" | "build" => "patch\n0.1.1\n",
                "feat" | "breaking" => "minor\n0.2.0\n",
                _ => panic!("unexpected commit type"),
            },
            "first-file -r test.txt" => match commit_type {
                "chore" | "ci" | "docs" | "style" | "refactor" | "perf" | "test" | "custom"
                | "build" => match check {
                    "other" => "none\n",
                    "fix" | "feature" | "breaking" => "patch\n0.1.1\n",
                    _ => panic!("unexpected check"),
                },
                "fix" | "revert" => match check {
                    "other" | "fix" => "none\n",
                    "feature" | "breaking" => "patch\n0.1.1\n",
                    _ => panic!("unexpected check"),
                },
                "feat" => match check {
                    "other" | "fix" | "feature" => "none\n",
                    "breaking" => "minor\n0.2.0\n",
                    _ => panic!("unexpected check"),
                },
                "breaking" => match check {
                    "other" | "fix" | "feature" | "breaking" => "none\n",
                    _ => panic!("unexpected check"),
                },
                _ => panic!("unexpected commit type"),
            },
            _ => panic!("unexpected file"),
        },
        "v1.1.0" => match file {
            "test.txt" | "missing.txt" => match commit_type {
                "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf"
                | "test" | "custom" | "build" => "patch\n1.1.1\n",
                "feat" => "minor\n1.2.0\n",
                "breaking" => "major\n2.0.0\n",
                _ => panic!("unexpected commit type"),
            },
            "first-file -r test.txt" => match commit_type {
                "chore" | "ci" | "docs" | "style" | "refactor" | "perf" | "test" | "custom"
                | "build" => match check {
                    "other" => "none\n",
                    "fix" | "feature" | "breaking" => "patch\n1.1.1\n",
                    _ => panic!("unexpected check"),
                },
                "fix" | "revert" => match check {
                    "other" | "fix" => "none\n",
                    "feature" | "breaking" => "patch\n1.1.1\n",
                    _ => panic!("unexpected check"),
                },
                "feat" => match check {
                    "other" | "fix" | "feature" => "none\n",
                    "breaking" => "minor\n1.2.0\n",
                    _ => panic!("unexpected check"),
                },
                "breaking" => match check {
                    "other" | "fix" | "feature" | "breaking" => "none\n",
                    _ => panic!("unexpected check"),
                },
                _ => panic!("unexpected commit type"),
            },
            _ => panic!("unexpected file"),
        },
        "v0.1.0-alpha.2" => match file {
            "test.txt" | "missing.txt" => match commit_type {
                "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf"
                | "test" | "custom" | "build" | "feat" | "breaking" => "alpha\n0.1.0-alpha.3\n",
                _ => panic!("unexpected commit type"),
            },
            "first-file -r test.txt" => match commit_type {
                "chore" | "ci" | "docs" | "style" | "refactor" | "perf" | "test" | "custom"
                | "build" => match check {
                    "other" => "none\n",
                    "fix" | "feature" | "breaking" => "alpha\n0.1.0-alpha.3\n",
                    _ => panic!("unexpected check"),
                },
                "fix" | "revert" => match check {
                    "other" | "fix" => "none\n",
                    "feature" | "breaking" => "alpha\n0.1.0-alpha.3\n",
                    _ => panic!("unexpected check"),
                },
                "feat" => match check {
                    "other" | "fix" | "feature" => "none\n",
                    "breaking" => "alpha\n0.1.0-alpha.3\n",
                    _ => panic!("unexpected check"),
                },
                "breaking" => match check {
                    "other" | "fix" | "feature" | "breaking" => "none\n",
                    _ => panic!("unexpected check"),
                },
                _ => panic!("unexpected commit type"),
            },
            _ => panic!("unexpected file"),
        },
        "v1.1.0-alpha.3" => match file {
            "test.txt" | "missing.txt" => match commit_type {
                "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf"
                | "test" | "custom" | "build" | "feat" | "breaking" => "alpha\n1.1.0-alpha.4\n",
                _ => panic!("unexpected commit type"),
            },
            "first-file -r test.txt" => match commit_type {
                "chore" | "ci" | "docs" | "style" | "refactor" | "perf" | "test" | "custom"
                | "build" => match check {
                    "other" => "none\n",
                    "fix" | "feature" | "breaking" => "alpha\n1.1.0-alpha.4\n",
                    _ => panic!("unexpected check"),
                },
                "fix" | "revert" => match check {
                    "other" | "fix" => "none\n",
                    "feature" | "breaking" => "alpha\n1.1.0-alpha.4\n",
                    _ => panic!("unexpected check"),
                },
                "feat" => match check {
                    "other" | "fix" | "feature" => "none\n",
                    "breaking" => "alpha\n1.1.0-alpha.4\n",
                    _ => panic!("unexpected check"),
                },
                "breaking" => match check {
                    "other" | "fix" | "feature" | "breaking" => "none\n",
                    _ => panic!("unexpected check"),
                },
                _ => panic!("unexpected commit type"),
            },
            _ => panic!("unexpected file"),
        },
        "v0.1.0-beta.4" => match file {
            "test.txt" | "missing.txt" => match commit_type {
                "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf"
                | "test" | "custom" | "build" | "feat" | "breaking" => "beta\n0.1.0-beta.5\n",
                _ => panic!("unexpected commit type"),
            },
            "first-file -r test.txt" => match commit_type {
                "chore" | "ci" | "docs" | "style" | "refactor" | "perf" | "test" | "custom"
                | "build" => match check {
                    "other" => "none\n",
                    "fix" | "feature" | "breaking" => "beta\n0.1.0-beta.5\n",
                    _ => panic!("unexpected check"),
                },
                "fix" | "revert" => match check {
                    "other" | "fix" => "none\n",
                    "feature" | "breaking" => "beta\n0.1.0-beta.5\n",
                    _ => panic!("unexpected check"),
                },
                "feat" => match check {
                    "other" | "fix" | "feature" => "none\n",
                    "breaking" => "beta\n0.1.0-beta.5\n",
                    _ => panic!("unexpected check"),
                },
                "breaking" => match check {
                    "other" | "fix" | "feature" | "breaking" => "none\n",
                    _ => panic!("unexpected check"),
                },
                _ => panic!("unexpected commit type"),
            },
            _ => panic!("unexpected file"),
        },
        "v1.1.0-beta.5" => match file {
            "test.txt" | "missing.txt" => match commit_type {
                "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf"
                | "test" | "custom" | "build" | "feat" | "breaking" => "beta\n1.1.0-beta.6\n",
                _ => panic!("unexpected commit type"),
            },
            "first-file -r test.txt" => match commit_type {
                "chore" | "ci" | "docs" | "style" | "refactor" | "perf" | "test" | "custom"
                | "build" => match check {
                    "other" => "none\n",
                    "fix" | "feature" | "breaking" => "beta\n1.1.0-beta.6\n",
                    _ => panic!("unexpected check"),
                },
                "fix" | "revert" => match check {
                    "other" | "fix" => "none\n",
                    "feature" | "breaking" => "beta\n1.1.0-beta.6\n",
                    _ => panic!("unexpected check"),
                },
                "feat" => match check {
                    "other" | "fix" | "feature" => "none\n",
                    "breaking" => "beta\n1.1.0-beta.6\n",
                    _ => panic!("unexpected check"),
                },
                "breaking" => match check {
                    "other" | "fix" | "feature" | "breaking" => "none\n",
                    _ => panic!("unexpected check"),
                },
                _ => panic!("unexpected commit type"),
            },
            _ => panic!("unexpected file"),
        },
        "v0.1.0-rc.6" => match file {
            "test.txt" | "missing.txt" => match commit_type {
                "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf"
                | "test" | "custom" | "build" | "feat" | "breaking" => "rc\n0.1.0-rc.7\n",
                _ => panic!("unexpected commit type"),
            },
            "first-file -r test.txt" => match commit_type {
                "chore" | "ci" | "docs" | "style" | "refactor" | "perf" | "test" | "custom"
                | "build" => match check {
                    "other" => "none\n",
                    "fix" | "feature" | "breaking" => "rc\n0.1.0-rc.7\n",
                    _ => panic!("unexpected check"),
                },
                "fix" | "revert" => match check {
                    "other" | "fix" => "none\n",
                    "feature" | "breaking" => "rc\n0.1.0-rc.7\n",
                    _ => panic!("unexpected check"),
                },
                "feat" => match check {
                    "other" | "fix" | "feature" => "none\n",
                    "breaking" => "rc\n0.1.0-rc.7\n",
                    _ => panic!("unexpected check"),
                },
                "breaking" => match check {
                    "other" | "fix" | "feature" | "breaking" => "none\n",
                    _ => panic!("unexpected check"),
                },
                _ => panic!("unexpected commit type"),
            },
            _ => panic!("unexpected file"),
        },
        "v1.1.0-rc.7" => match file {
            "test.txt" | "missing.txt" => match commit_type {
                "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf"
                | "test" | "custom" | "build" | "feat" | "breaking" => "rc\n1.1.0-rc.8\n",
                _ => panic!("unexpected commit type"),
            },
            "first-file -r test.txt" => match commit_type {
                "chore" | "ci" | "docs" | "style" | "refactor" | "perf" | "test" | "custom"
                | "build" => match check {
                    "other" => "none\n",
                    "fix" | "feature" | "breaking" => "rc\n1.1.0-rc.8\n",
                    _ => panic!("unexpected check"),
                },
                "fix" | "revert" => match check {
                    "other" | "fix" => "none\n",
                    "feature" | "breaking" => "rc\n1.1.0-rc.8\n",
                    _ => panic!("unexpected check"),
                },
                "feat" => match check {
                    "other" | "fix" | "feature" => "none\n",
                    "breaking" => "rc\n1.1.0-rc.8\n",
                    _ => panic!("unexpected check"),
                },
                "breaking" => match check {
                    "other" | "fix" | "feature" | "breaking" => "none\n",
                    _ => panic!("unexpected check"),
                },
                _ => panic!("unexpected commit type"),
            },
            _ => panic!("unexpected file"),
        },
        "v0.1.0-pre.8" => match file {
            "test.txt" | "missing.txt" => match commit_type {
                "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf"
                | "test" | "custom" | "build" | "feat" | "breaking" => "0.1.0-pre.9\n0.1.0-pre.9\n",
                _ => panic!("unexpected commit type"),
            },
            "first-file -r test.txt" => match commit_type {
                "chore" | "ci" | "docs" | "style" | "refactor" | "perf" | "test" | "custom"
                | "build" => match check {
                    "other" => "none\n",
                    "fix" | "feature" | "breaking" => "0.1.0-pre.9\n0.1.0-pre.9\n",
                    _ => panic!("unexpected check"),
                },
                "fix" | "revert" => match check {
                    "other" | "fix" => "none\n",
                    "feature" | "breaking" => "0.1.0-pre.9\n0.1.0-pre.9\n",
                    _ => panic!("unexpected check"),
                },
                "feat" => match check {
                    "other" | "fix" | "feature" => "none\n",
                    "breaking" => "0.1.0-pre.9\n0.1.0-pre.9\n",
                    _ => panic!("unexpected check"),
                },
                "breaking" => match check {
                    "other" | "fix" | "feature" | "breaking" => "none\n",
                    _ => panic!("unexpected check"),
                },
                _ => panic!("unexpected commit type"),
            },
            _ => panic!("unexpected file"),
        },
        "v1.1.0-pre.9" => match file {
            "test.txt" | "missing.txt" => match commit_type {
                "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf"
                | "test" | "custom" | "build" | "feat" | "breaking" => {
                    "1.1.0-pre.10\n1.1.0-pre.10\n"
                }
                _ => panic!("unexpected commit type"),
            },
            "first-file -r test.txt" => match commit_type {
                "chore" | "ci" | "docs" | "style" | "refactor" | "perf" | "test" | "custom"
                | "build" => match check {
                    "other" => "none\n",
                    "fix" | "feature" | "breaking" => "1.1.0-pre.10\n1.1.0-pre.10\n",
                    _ => panic!("unexpected check"),
                },
                "fix" | "revert" => match check {
                    "other" | "fix" => "none\n",
                    "feature" | "breaking" => "1.1.0-pre.10\n1.1.0-pre.10\n",
                    _ => panic!("unexpected check"),
                },
                "feat" => match check {
                    "other" | "fix" | "feature" => "none\n",
                    "breaking" => "1.1.0-pre.10\n1.1.0-pre.10\n",
                    _ => panic!("unexpected check"),
                },
                "breaking" => match check {
                    "other" | "fix" | "feature" | "breaking" => "none\n",
                    _ => panic!("unexpected check"),
                },
                _ => panic!("unexpected commit type"),
            },
            _ => panic!("unexpected file"),
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
    let result = git_utils::create_file_and_commit(&repo, temp_dir.clone(), &message, None);
    println!("commit result: {:?}", result);

    // execute the test
    let mut arguments = arguments.to_string();
    arguments.push(' ');
    arguments.push_str(file);
    arguments.push_str(" -e ");
    arguments.push_str(check);
    let test_result = git_utils::execute_test(&arguments, &temp_dir);

    // tidy up the test environment
    let result = fs::remove_dir_all(temp_dir);
    println!("remove_dir_all result: {:?}", result);

    // assess the result
    assert_eq!(expected, test_result);
}

#[rstest]
#[case::non_production_commit("0.1.0", "-n -vvvv -p")]
#[case::production_commit("1.1.0", "-n -vvv -p")]
#[case::non_production_pre_release_alpha_commit("0.1.0-alpha.2", "-n -p")]
#[case::production_pre_release_alpha_commit("1.1.0-alpha.3", "-n -p")]
#[case::non_production_pre_release_beta_commit("0.1.0-beta.4", "-n -p")]
#[case::production_pre_release_beta_commit("1.1.0-beta.5", "-n -p")]
#[case::non_production_pre_release_rc_commit("0.1.0-rc.6", "-n -p")]
#[case::production_pre_release_rc_commit("1.1.0-rc.7", "-n -p")]
#[case::non_production_pre_release_pre_commit("0.1.0-pre.8", "-n -p")]
#[case::production_pre_release_pre_commit("1.1.0-pre.9", "-n -p")]
#[trace]
fn test_repo_custom_version_prefix_with_commit(
    #[case] current_version: &str,
    #[values(
        "fix", "chore", "ci", "revert", "docs", "style", "refactor", "perf", "test", "custom",
        "build", "feat", "breaking"
    )]
    mut commit_type: &str,
    #[case] arguments: &str,
    #[values("ver", "version_", "rel", "v")] prefix: &str,
) {
    // select expected result
    let expected = match current_version {
        "0.1.0" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" => "patch\n0.1.1\n",
            "feat" | "breaking" => "minor\n0.2.0\n",
            _ => panic!("unexpected commit type"),
        },
        "1.1.0" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" => "patch\n1.1.1\n",
            "feat" => "minor\n1.2.0\n",
            "breaking" => "major\n2.0.0\n",
            _ => panic!("unexpected commit type"),
        },
        "0.1.0-alpha.2" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "feat" | "breaking" => "alpha\n0.1.0-alpha.3\n",
            _ => panic!("unexpected commit type"),
        },
        "1.1.0-alpha.3" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "feat" | "breaking" => "alpha\n1.1.0-alpha.4\n",
            _ => panic!("unexpected commit type"),
        },
        "0.1.0-beta.4" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "feat" | "breaking" => "beta\n0.1.0-beta.5\n",
            _ => panic!("unexpected commit type"),
        },
        "1.1.0-beta.5" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "feat" | "breaking" => "beta\n1.1.0-beta.6\n",
            _ => panic!("unexpected commit type"),
        },
        "0.1.0-rc.6" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "feat" | "breaking" => "rc\n0.1.0-rc.7\n",
            _ => panic!("unexpected commit type"),
        },
        "1.1.0-rc.7" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "feat" | "breaking" => "rc\n1.1.0-rc.8\n",
            _ => panic!("unexpected commit type"),
        },
        "0.1.0-pre.8" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "feat" | "breaking" => "0.1.0-pre.9\n0.1.0-pre.9\n",
            _ => panic!("unexpected commit type"),
        },
        "1.1.0-pre.9" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "feat" | "breaking" => "1.1.0-pre.10\n1.1.0-pre.10\n",
            _ => panic!("unexpected commit type"),
        },
        _ => panic!("unexpected current version"),
    };

    // setup base state
    let version = format!("{}{}", prefix, current_version);
    let (temp_dir, repo) = git_utils::create_test_git_directory(&version);
    println!("temp_dir: {:?}", temp_dir);

    // setup the change conditions
    if commit_type == "breaking" {
        commit_type = "fix!";
    };
    let message = format!("{}: {}", commit_type, "test commit");
    println!("message: {:?}", message);
    let result = git_utils::create_file_and_commit(&repo, temp_dir.clone(), &message, None);
    println!("commit result: {:?}", result);

    // execute the test
    let mut arguments = arguments.to_string();
    arguments.push(' ');
    arguments.push_str(prefix);
    let test_result = git_utils::execute_test(&arguments, &temp_dir);

    // tidy up the test environment
    let result = fs::remove_dir_all(temp_dir);
    println!("remove_dir_all result: {:?}", result);

    // assess the result
    assert_eq!(expected, test_result);
}

#[rstest]
#[case::non_production_commit("v0.1.0", "-vvvv")]
#[case::production_commit("v1.1.0", "-vvv")]
#[case::non_production_pre_release_alpha_commit("v0.1.0-alpha.2", "")]
#[case::production_pre_release_alpha_commit("v1.1.0-alpha.3", "")]
#[case::non_production_pre_release_beta_commit("v0.1.0-beta.4", "")]
#[case::production_pre_release_beta_commit("v1.1.0-beta.5", "")]
#[case::non_production_pre_release_rc_commit("v0.1.0-rc.6", "")]
#[case::production_pre_release_rc_commit("v1.1.0-rc.7", "")]
#[case::non_production_pre_release_pre_commit("v0.1.0-pre.8", "")]
#[case::production_pre_release_pre_commit("v1.1.0-pre.9", "")]
#[trace]
fn test_repo_bump_only_with_commit(
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
            | "custom" | "build" => "patch\n",
            "feat" | "breaking" => "minor\n",
            _ => panic!("unexpected commit type"),
        },
        "v1.1.0" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" => "patch\n",
            "feat" => "minor\n",
            "breaking" => "major\n",
            _ => panic!("unexpected commit type"),
        },
        "v0.1.0-alpha.2" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "feat" | "breaking" => "alpha\n",
            _ => panic!("unexpected commit type"),
        },
        "v1.1.0-alpha.3" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "feat" | "breaking" => "alpha\n",
            _ => panic!("unexpected commit type"),
        },
        "v0.1.0-beta.4" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "feat" | "breaking" => "beta\n",
            _ => panic!("unexpected commit type"),
        },
        "v1.1.0-beta.5" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "feat" | "breaking" => "beta\n",
            _ => panic!("unexpected commit type"),
        },
        "v0.1.0-rc.6" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "feat" | "breaking" => "rc\n",
            _ => panic!("unexpected commit type"),
        },
        "v1.1.0-rc.7" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "feat" | "breaking" => "rc\n",
            _ => panic!("unexpected commit type"),
        },
        "v0.1.0-pre.8" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "feat" | "breaking" => "0.1.0-pre.9\n",
            _ => panic!("unexpected commit type"),
        },
        "v1.1.0-pre.9" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "feat" | "breaking" => "1.1.0-pre.10\n",
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
    let result = git_utils::create_file_and_commit(&repo, temp_dir.clone(), &message, None);
    println!("commit result: {:?}", result);

    // execute the test
    let test_result = git_utils::execute_test(arguments, &temp_dir);

    // tidy up the test environment
    let result = fs::remove_dir_all(temp_dir);
    println!("remove_dir_all result: {:?}", result);

    // assess the result
    assert_eq!(expected, test_result);
}

#[rstest]
#[case::non_production_commit("v0.1.0", "-n -vvvv -b")]
#[case::production_commit("v1.1.0", "-n -vvv -b")]
#[case::non_production_pre_release_alpha_commit("v0.1.0-alpha.2", "-n -b")]
#[case::production_pre_release_alpha_commit("v1.1.0-alpha.3", "-n -b")]
#[case::non_production_pre_release_beta_commit("v0.1.0-beta.4", "-n -b")]
#[case::production_pre_release_beta_commit("v1.1.0-beta.5", "-n -b")]
#[case::non_production_pre_release_rc_commit("v0.1.0-rc.6", "-n -b")]
#[case::production_pre_release_rc_commit("v1.1.0-rc.7", "-n -b")]
#[case::non_production_pre_release_pre_commit("v0.1.0-pre.8", "-n -b")]
#[case::production_pre_release_pre_commit("v1.1.0-pre.9", "-n -b")]
#[trace]
fn test_repo_number_only_with_commit(
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
            | "custom" | "build" => "0.1.1\n",
            "feat" | "breaking" => "0.2.0\n",
            _ => panic!("unexpected commit type"),
        },
        "v1.1.0" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" => "1.1.1\n",
            "feat" => "1.2.0\n",
            "breaking" => "2.0.0\n",
            _ => panic!("unexpected commit type"),
        },
        "v0.1.0-alpha.2" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "feat" | "breaking" => "0.1.0-alpha.3\n",
            _ => panic!("unexpected commit type"),
        },
        "v1.1.0-alpha.3" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "feat" | "breaking" => "1.1.0-alpha.4\n",
            _ => panic!("unexpected commit type"),
        },
        "v0.1.0-beta.4" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "feat" | "breaking" => "0.1.0-beta.5\n",
            _ => panic!("unexpected commit type"),
        },
        "v1.1.0-beta.5" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "feat" | "breaking" => "1.1.0-beta.6\n",
            _ => panic!("unexpected commit type"),
        },
        "v0.1.0-rc.6" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "feat" | "breaking" => "0.1.0-rc.7\n",
            _ => panic!("unexpected commit type"),
        },
        "v1.1.0-rc.7" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "feat" | "breaking" => "1.1.0-rc.8\n",
            _ => panic!("unexpected commit type"),
        },
        "v0.1.0-pre.8" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "feat" | "breaking" => "0.1.0-pre.9\n",
            _ => panic!("unexpected commit type"),
        },
        "v1.1.0-pre.9" => match commit_type {
            "fix" | "chore" | "ci" | "revert" | "docs" | "style" | "refactor" | "perf" | "test"
            | "custom" | "build" | "feat" | "breaking" => "1.1.0-pre.10\n",
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
    let result = git_utils::create_file_and_commit(&repo, temp_dir.clone(), &message, None);
    println!("commit result: {:?}", result);

    // execute the test
    let test_result = git_utils::execute_test(arguments, &temp_dir);

    // tidy up the test environment
    let result = fs::remove_dir_all(temp_dir);
    println!("remove_dir_all result: {:?}", result);

    // assess the result
    assert_eq!(expected, test_result);
}
