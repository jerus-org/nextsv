use std::path::PathBuf;

use git2::{Commit as GitCommit, Repository};

#[derive(Clone)]
pub struct Commit<'a> {
    git_commit: GitCommit<'a>,
    repo: &'a Repository,
}

impl<'a> Commit<'a> {
    pub(crate) fn new(git_commit: GitCommit<'a>, repo: &'a Repository) -> Self {
        Self { git_commit, repo }
    }

    // pub(crate) fn commit(&self) -> &GitCommit<'a> {
    //     &self.git_commit
    // }

    pub(crate) fn message(&self) -> String {
        self.git_commit.summary().unwrap_or_default().to_string()
    }

    // pub(crate) fn hash(&self) -> String {
    //     self.git_commit.id().to_string()
    // }

    // pub(crate) fn author(&self) -> String {
    //     self.git_commit
    //         .author()
    //         .name()
    //         .unwrap_or_default()
    //         .to_string()
    // }

    // pub(crate) fn date(&self) -> String {
    //     self.git_commit.time().seconds().to_string()
    // }

    pub(crate) fn files(&self) -> Vec<PathBuf> {
        let mut diff_files = vec![];

        let a = if self.git_commit.parents().len() == 1 {
            let parent = self.git_commit.parent(0).unwrap();
            Some(parent.tree().unwrap())
        } else {
            None
        };
        let b = self.git_commit.tree().unwrap();
        let diff = self
            .repo
            .diff_tree_to_tree(a.as_ref(), Some(&b), None)
            .unwrap();
        let ds = diff.deltas();
        for d in ds {
            let file_name = d.new_file().path().unwrap().to_owned();
            // .unwrap().to_str().unwrap().to_owned()
            if !file_name.starts_with("master") {
                diff_files.push(file_name);
            }
        }

        diff_files
    }
}
