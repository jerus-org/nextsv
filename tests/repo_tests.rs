use std::fs::File;
use std::path::{Path, PathBuf};
use std::{fs, process::Command};

use git2::{Commit, ObjectType, Oid, Repository, Signature};
use uuid::Uuid;

const GIT_TEMPLATE_DIR: &str = "tests/repo_template";

pub(crate) fn create_test_git_directory() -> (PathBuf, Repository) {
    let temp_dir_string = format!("tests/tmp/test-{}", Uuid::new_v4());
    let temp_dir = Path::new(&temp_dir_string);
    println!("Temporary directory: {:?}", temp_dir);
    let result = copy_dir::copy_dir(GIT_TEMPLATE_DIR, temp_dir);
    println!("copy_dir result: {:?}", result);

    let repo = Repository::init(temp_dir).expect("failed to initialise repo");
    let path_to_file = Path::new("first-file");

    let res = add_file_and_first_commit(&repo, path_to_file, "chore: initial commit");

    println!("add_and_commit result: {:?}", res);

    if let Ok(commit) = find_last_commit(&repo) {
        println!("commit: {:?}", commit);
        repo.tag_lightweight("v0.1.0", commit.as_object(), false)
            .unwrap();
    }

    (temp_dir.into(), repo)
}

pub(crate) fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit()
        .map_err(|_| git2::Error::from_str("Couldn't find commit"))
}

#[allow(dead_code)]
pub(crate) fn display_commit(commit: &Commit) {
    let timestamp = commit.time().seconds();
    let tm = chrono::DateTime::from_timestamp(timestamp, 0).unwrap();
    // let tm = time::at(time::Timespec::new(timestamp, 0));
    println!(
        "commit {}\nAuthor: {}\nDate:   {}\n\n    {}",
        commit.id(),
        commit.author(),
        tm.to_rfc2822(),
        commit.message().unwrap_or("no commit message")
    );
}

pub(crate) fn add_and_commit(
    repo: &Repository,
    path: &Path,
    message: &str,
) -> Result<Oid, git2::Error> {
    let mut index = repo.index()?;
    index.add_path(path)?;
    let oid = index.write_tree()?;
    let signature = Signature::now("tester", "tester@example.net")?;
    let parent_commit = find_last_commit(repo)?;
    let tree = repo.find_tree(oid)?;
    repo.commit(
        Some("HEAD"), //  point HEAD to our new commit
        &signature,   // author
        &signature,   // committer
        message,      // commit message
        &tree,        // tree
        &[&parent_commit],
    ) // parents
}

pub(crate) fn add_file_and_first_commit(
    repo: &Repository,
    path: &Path,
    message: &str,
) -> Result<Oid, git2::Error> {
    let mut index = repo.index()?;
    println!("adding the path: {}", path.display());
    index.add_path(path)?;
    let oid = index.write_tree()?;

    let signature = Signature::now("tester", "tester@example.net")?;
    let tree = repo.find_tree(oid)?;
    repo.commit(
        Some("HEAD"), //  point HEAD to our new commit
        &signature,   // author
        &signature,   // committer
        message,      // commit message
        &tree,        // tree
        &[],          // parents
    )
}

#[test]
fn test_repo_no_changes() {
    let expected = "none\n";

    let (temp_dir, _repo) = create_test_git_directory();
    println!("temp_dir: {:?}", temp_dir);

    let output = Command::new("cargo")
        .arg("run")
        .current_dir(&temp_dir)
        .output()
        .unwrap();

    let test_result = String::from_utf8(output.stdout).unwrap();

    println!("stdout: {}", test_result);
    println!("stderr: {}", String::from_utf8(output.stderr).unwrap());

    // let result = fs::remove_dir_all(temp_dir);
    // println!("remove_dir_all result: {:?}", result);
    assert_eq!(expected, test_result);
}

#[test]
fn test_repo_with_commit() {
    let expected = "patch\n0.1.1\n";

    let (temp_dir, repo) = create_test_git_directory();
    println!("temp_dir: {:?}", temp_dir);

    // create a file
    use std::io::prelude::*;

    let file_name = "test.txt";

    let file_path = temp_dir.join(file_name);
    let mut file = File::create(&file_path).unwrap();
    file.write_all(b"Hello, world!").unwrap();

    println!("added file: {}", file_path.display());

    let message = "fix: adding a fix for a patch bump";

    // create file path from the file name
    let file_name_path = Path::new(file_name);
    let result = add_and_commit(&repo, file_name_path, message);
    println!("commit result: {:?}", result);

    // let commit = git_utils::find_last_commit(&repo).unwrap();
    // git_utils::display_commit(&commit);

    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("-n")
        .arg("-vvv")
        .current_dir(&temp_dir)
        .output()
        .unwrap();

    let test_result = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    println!("stdout:\n-------\n{}", test_result);
    println!("stderr:\n-------\n{}", stderr);

    let result = fs::remove_dir_all(temp_dir);
    println!("remove_dir_all result: {:?}", result);
    assert_eq!(expected, test_result);
}
