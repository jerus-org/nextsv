use git2::{Commit, ObjectType, Oid, Repository, Signature};
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};
use uuid::Uuid;

const GIT_TEMPLATE_DIR: &str = "tests/repo_template";

/// Create a temporary directory and copy the git template directory into it.
/// Initialise a git repository in the temporary directory and create a commit.
///         The commit is tagged with the current version.
/// Return the temporary directory and the repository.
pub fn create_test_git_directory(current_version: &str) -> (PathBuf, Repository) {
    let temp_dir_string = format!("tests/tmp/test-{}", Uuid::new_v4());
    let temp_dir = Path::new(&temp_dir_string);
    println!("Temporary directory: {temp_dir:?}");
    let result = copy_dir::copy_dir(GIT_TEMPLATE_DIR, temp_dir);
    println!("copy_dir result: {result:?}");

    let repo = Repository::init(temp_dir).expect("failed to initialise repo");
    let path_to_file = Path::new("first-file");

    let res = add_file_and_first_commit(&repo, path_to_file, "chore: initial commit");

    println!("add_and_commit result: {res:?}");

    if let Ok(commit) = find_last_commit(&repo) {
        println!("commit: {commit:?}");
        repo.tag_lightweight(current_version, commit.as_object(), false)
            .unwrap();
    }

    (temp_dir.into(), repo)
}

/// Add a file to the index and commit the change.
/// Return the commit id.  If there is an error, return the error.
/// This function is used to create the initial commit in the repository.
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

/// Find the last commit in the repository.
/// Return the commit.  If there is an error, return the error.
pub fn find_last_commit(repo: &'_ Repository) -> Result<Commit<'_>, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit()
        .map_err(|_| git2::Error::from_str("Couldn't find commit"))
}

/// Display the commit.
/// Print the commit id, author, date and message.
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

/// Add a file to the index and commit the change.
/// Return the commit id.  If there is an error, return the error.
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

pub fn create_file_and_commit(
    repo: &Repository,
    temp_dir: PathBuf,
    message: &str,
    filename: Option<&str>,
) -> Result<Oid, git2::Error> {
    let file_name: &str = match filename {
        Some(f) => f,
        None => "test.txt",
    };

    let file_path = temp_dir.join(file_name);
    let mut file = File::create(&file_path).unwrap();
    file.write_all(b"Hello, world!").unwrap();

    println!("added file: {}", file_path.display());

    // create file path from the file name
    let file_name_path = Path::new(file_name);
    let result = add_and_commit(repo, file_name_path, message);

    let commit = find_last_commit(repo).unwrap();
    display_commit(&commit);

    result
}

#[allow(dead_code)]
pub fn update_file_and_commit(
    repo: &Repository,
    temp_dir: PathBuf,
    message: &str,
    filename: Option<&str>,
) -> Result<Oid, git2::Error> {
    let file_name: &str = match filename {
        Some(f) => f,
        None => "test.txt",
    };

    let file_path = temp_dir.join(file_name);
    // Open file_path and append text to the end of the file
    let mut file = OpenOptions::new().append(true).open(&file_path).unwrap();
    file.write_all(b"Hello, world!").unwrap();

    println!("updated file: {}", file_path.display());

    // create file path from the file name
    let file_name_path = Path::new(file_name);
    let result = add_and_commit(repo, file_name_path, message);

    let commit = find_last_commit(repo).unwrap();
    display_commit(&commit);

    result
}
