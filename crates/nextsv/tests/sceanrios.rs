use std::{path::PathBuf, process::Command};

use git2::Repository;
use snapbox::cmd::cargo_bin;

use test_utils::git_utils;

fn execute_test(arguments: &str, temp_dir: &PathBuf) -> String {
    let cmd = cargo_bin!("nextsv");
    println!("cmd: {cmd:?}");

    let test_args: Vec<&str> = arguments.split_ascii_whitespace().collect();
    println!("test_args: {test_args:?}");

    let output = Command::new(cmd)
        .args(test_args)
        .current_dir(temp_dir)
        .output()
        .unwrap();

    println!("Exit code: {}", output.status.code().unwrap());
    let test_result = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    println!("stdout:\n-------\n{test_result}");
    println!("stderr:\n-------\n{stderr}");
    test_result
}

#[test]
fn test_initial_to_production() {
    // Scenario: Initial to Production
    //  - Create a new git repository with an initial version number.
    //  - Add a feature commit
    //  - Add a fix commit
    //  - Add a new feature commit
    //  - Add a breaking change commit
    //  - Promote to production release

    let version_prefix = "v";
    let arguments = "-n -vvv calculate";

    let initial_version = format!("{version_prefix}0.1.0");

    // Create the initial directory with the initial version number.
    let (temp_dir, repo) = git_utils::create_test_git_directory(&initial_version);

    // Add a file and create a feature commit
    let feature = "feature1.txt";
    let message = "feat: add feature1";
    let expected = ("patch".to_string(), "0.1.1".to_string());

    add_feature(
        &repo,
        &temp_dir,
        feature,
        message,
        expected,
        arguments,
        version_prefix,
    );

    // Update the file and create a fix commit

    let feature = "feature1.txt";
    let message = "fix: fix to feature1";
    let expected = ("patch".to_string(), "0.1.2".to_string());

    update_feature(
        &repo,
        &temp_dir,
        feature,
        message,
        expected,
        arguments,
        version_prefix,
    );

    // Add a new feature
    let feature = "feature2.txt";
    let message = "feat: add feature1";
    let expected = ("patch".to_string(), "0.1.3".to_string());

    add_feature(
        &repo,
        &temp_dir,
        feature,
        message,
        expected,
        arguments,
        version_prefix,
    );

    // Add a breaking change

    let feature = "feature2.txt";
    let message = "fix!: breaking fix to feature2";
    let expected = ("minor".to_string(), "0.2.0".to_string());

    update_feature(
        &repo,
        &temp_dir,
        feature,
        message,
        expected,
        arguments,
        version_prefix,
    );

    // Promote to production release
    let test_result_str = execute_test("-n force first", &temp_dir);
    let test_result = test_result_split(&test_result_str);
    println!("test_result: {test_result_str:?}");

    expected_result("1.0.0", "1.0.0", &test_result);

    // tidy up the test environment
    let result = std::fs::remove_dir_all(temp_dir);
    println!("remove_dir_all result: {result:?}");
}

#[test]
fn test_initial_to_production_with_pre_releases() {
    // Scenario: Initial to Production

    let version_prefix = "v";
    // let arguments = "-n -vvv";
    let arguments = "-n calculate";

    let initial_version = format!("{version_prefix}0.1.0");

    // Create the initial directory with the initial version number.
    let (temp_dir, repo) = git_utils::create_test_git_directory(&initial_version);

    // Add a file and create a feature commit
    let feature = "feature1.txt";
    let message = "feat: add feature1";
    let expected = ("patch".to_string(), "0.1.1".to_string());

    add_feature(
        &repo,
        &temp_dir,
        feature,
        message,
        expected,
        arguments,
        version_prefix,
    );

    // Update the file and create a fix commit

    let message = "fix: fix to feature1";
    let expected = ("patch".to_string(), "0.1.2".to_string());

    update_feature(
        &repo,
        &temp_dir,
        feature,
        message,
        expected,
        arguments,
        version_prefix,
    );

    // Update a new feature
    let message = "feat: update feature1";
    let expected = ("patch".to_string(), "0.1.3".to_string());

    update_feature(
        &repo,
        &temp_dir,
        feature,
        message,
        expected,
        arguments,
        version_prefix,
    );

    // Update feature and make alpha pre-release
    let feature = "feature1.txt";
    let message = "fix!: breaking fix to feature1";
    let expected = ("alpha".to_string(), "0.2.0-alpha.1".to_string());

    update_feature(
        &repo,
        &temp_dir,
        feature,
        message,
        expected,
        "-n force alpha",
        version_prefix,
    );

    // Update feature and apply a fix
    let feature = "feature1.txt";
    let message = "fix: breaking fix to feature1";
    let expected = ("alpha".to_string(), "0.2.0-alpha.2".to_string());

    update_feature(
        &repo,
        &temp_dir,
        feature,
        message,
        expected,
        arguments,
        version_prefix,
    );

    // Update feature and make beta pre-release
    let feature = "feature1.txt";
    let message = "fix: fix to feature1";
    let expected = ("beta".to_string(), "0.2.0-beta.1".to_string());

    update_feature(
        &repo,
        &temp_dir,
        feature,
        message,
        expected,
        "-n force beta",
        version_prefix,
    );

    // Update feature and apply a fix
    let feature = "feature1.txt";
    let message = "fix: breaking fix to feature1";
    let expected = ("beta".to_string(), "0.2.0-beta.2".to_string());

    update_feature(
        &repo,
        &temp_dir,
        feature,
        message,
        expected,
        arguments,
        version_prefix,
    );

    // Update feature and make rc pre-release
    let feature = "feature1.txt";
    let message = "fix: fix to feature1";
    let expected = ("rc".to_string(), "0.2.0-rc.1".to_string());

    update_feature(
        &repo,
        &temp_dir,
        feature,
        message,
        expected,
        "-n force rc",
        version_prefix,
    );

    // Update feature and apply a fix
    let feature = "feature1.txt";
    let message = "fix: breaking fix to feature1";
    let expected = ("rc".to_string(), "0.2.0-rc.2".to_string());

    update_feature(
        &repo,
        &temp_dir,
        feature,
        message,
        expected,
        arguments,
        version_prefix,
    );

    // Promote to production release
    let test_result_str = execute_test("-n -vvv force first", &temp_dir);
    let test_result = test_result_split(&test_result_str);
    println!("test_result: {test_result_str:?}");

    expected_result("1.0.0", "1.0.0", &test_result);

    // tidy up the test environment
    let result = std::fs::remove_dir_all(temp_dir);
    println!("remove_dir_all result: {result:?}");
}

#[test]
fn test_initial_to_production_with_production_pre_releases() {
    // Scenario: Initial to Production

    let version_prefix = "v";
    // let arguments = "-n calculate -vvv";
    let arguments = "-n calculate";

    let initial_version = format!("{version_prefix}0.1.0");

    // Create the initial directory with the initial version number.
    let (temp_dir, repo) = git_utils::create_test_git_directory(&initial_version);

    // Add a file and create a feature commit
    let feature = "feature1.txt";
    let message = "feat: add feature1";
    let expected = ("patch".to_string(), "0.1.1".to_string());

    add_feature(
        &repo,
        &temp_dir,
        feature,
        message,
        expected,
        arguments,
        version_prefix,
    );

    // Update the file and create a fix commit

    let message = "fix: fix to feature1";
    let expected = ("patch".to_string(), "0.1.2".to_string());

    update_feature(
        &repo,
        &temp_dir,
        feature,
        message,
        expected,
        arguments,
        version_prefix,
    );

    // Update a new feature
    let message = "feat: update feature1";
    let expected = ("patch".to_string(), "0.1.3".to_string());

    update_feature(
        &repo,
        &temp_dir,
        feature,
        message,
        expected,
        arguments,
        version_prefix,
    );

    // Promote to version 1.0.0
    let message = "feat: update feature1";
    let arguments = "-n -vvv force -f alpha ";
    let expected = ("1.0.0-alpha.1".to_string(), "1.0.0-alpha.1".to_string());

    update_feature(
        &repo,
        &temp_dir,
        feature,
        message,
        expected,
        arguments,
        version_prefix,
    );

    // // Make alpha pre-release
    // let test_result_str = execute_test("-n force alpha", &temp_dir);
    // let test_result = test_result_split(&test_result_str);
    // println!("test_result: {:?}", test_result_str);

    // expected_result("alpha", "1.0.0-alpha.1", &test_result);

    // add_tag(&repo, &test_result.1, version_prefix);

    // Update feature and apply a fix
    let feature = "feature1.txt";
    let message = "fix: breaking fix to feature1";
    let arguments = "-n -vvvv force alpha";
    let expected = ("alpha".to_string(), "1.0.0-alpha.2".to_string());

    update_feature(
        &repo,
        &temp_dir,
        feature,
        message,
        expected,
        arguments,
        version_prefix,
    );

    // Update feature and make beta pre-release
    let feature = "feature1.txt";
    let message = "fix: fix to feature1";
    let arguments = "-n force beta";
    let expected = ("beta".to_string(), "1.0.0-beta.1".to_string());

    update_feature(
        &repo,
        &temp_dir,
        feature,
        message,
        expected,
        arguments,
        version_prefix,
    );

    // Update feature and apply a fix
    let feature = "feature1.txt";
    let message = "fix: breaking fix to feature1";
    let arguments = "-n force beta";
    let expected = ("beta".to_string(), "1.0.0-beta.2".to_string());

    update_feature(
        &repo,
        &temp_dir,
        feature,
        message,
        expected,
        arguments,
        version_prefix,
    );

    // Update feature and make rc pre-release
    let feature = "feature1.txt";
    let message = "fix: fix to feature1";
    let arguments = "-n force rc";
    let expected = ("rc".to_string(), "1.0.0-rc.1".to_string());

    update_feature(
        &repo,
        &temp_dir,
        feature,
        message,
        expected,
        arguments,
        version_prefix,
    );

    // Update feature and apply a fix
    let feature = "feature1.txt";
    let message = "fix: breaking fix to feature1";
    let arguments = "-n force rc";
    let expected = ("rc".to_string(), "1.0.0-rc.2".to_string());

    update_feature(
        &repo,
        &temp_dir,
        feature,
        message,
        expected,
        arguments,
        version_prefix,
    );

    // Release production release
    let test_result_str = execute_test("-n -vvv force release", &temp_dir);
    let test_result = test_result_split(&test_result_str);
    println!("test_result: {test_result_str:?}");

    expected_result("release", "1.0.0", &test_result);

    // tidy up the test environment
    let result = std::fs::remove_dir_all(temp_dir);
    println!("remove_dir_all result: {result:?}");
}

// Will require a custom pre-release option to enable this feature
// // Update feature and make pre pre-release
// let feature = "feature1.txt";
// let message = "fix: fix to feature1";
// let arguments = "-n force pre";
// let expected = ("pre".to_string(), "1.0.0-pre.1".to_string());

// update_feature(
//     &repo,
//     &temp_dir,
//     feature,
//     message,
//     expected,
//     arguments,
//     version_prefix,
// );

// // Update feature and apply a fix
// let feature = "feature1.txt";
// let message = "fix: breaking fix to feature1";
// let arguments = "-n force pre";
// let expected = ("pre".to_string(), "1.0.0-pre.2".to_string());

// update_feature(
//     &repo,
//     &temp_dir,
//     feature,
//     message,
//     expected,
//     arguments,
//     version_prefix,
// );

fn expected_result(bump: &str, version: &str, result: &(String, String)) {
    assert_eq!(bump, result.0);
    assert_eq!(version, result.1);
}

fn test_result_split(result: &str) -> (String, String) {
    // split and return the bump and number
    let test_result: Vec<&str> = result.split('\n').collect();
    (test_result[0].to_string(), test_result[1].to_string())
}

fn add_feature(
    repo: &Repository,
    temp_dir: &PathBuf,
    feature: &str,
    message: &str,
    expected: (String, String),
    arguments: &str,
    version_prefix: &str,
) {
    let result = git_utils::create_file_and_commit(repo, temp_dir.clone(), message, Some(feature));
    println!("commit result: {result:?}");

    // execute the test
    let test_result_str = execute_test(arguments, temp_dir);
    let test_result = test_result_split(&test_result_str);
    println!("test_result: {test_result_str:?}");

    expected_result(&expected.0, &expected.1, &test_result);
    add_tag(repo, &test_result.1, version_prefix);
}

fn add_tag(repo: &Repository, version: &str, version_prefix: &str) {
    if let Ok(commit) = git_utils::find_last_commit(repo) {
        println!("commit: {commit:?}");
        let tag = format!("{version_prefix}{version}");
        repo.tag_lightweight(&tag, commit.as_object(), false)
            .unwrap();
        println!("tagged commit `{commit:?}` with `{tag}`");
    };
}

fn update_feature(
    repo: &Repository,
    temp_dir: &PathBuf,
    feature: &str,
    message: &str,
    expected: (String, String),
    arguments: &str,
    version_prefix: &str,
) {
    let result = git_utils::update_file_and_commit(repo, temp_dir.clone(), message, Some(feature));
    println!("commit result: {result:?}");

    // execute the test
    let test_result_str = execute_test(arguments, temp_dir);
    let test_result = test_result_split(&test_result_str);
    println!("test_result: {test_result_str:?}");

    expected_result(&expected.0, &expected.1, &test_result);

    add_tag(repo, &test_result.1, version_prefix);
}
