//! Represents a vector of conventional commits
//!

use std::{
    collections::{HashMap, HashSet},
    ffi::OsString,
};

use git2::Repository;

use crate::Error;

use super::Hierarchy;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub(crate) struct ConventionalCommits {
    pub(crate) commits: Vec<String>,
    pub(crate) counts: HashMap<String, u32>,
    pub(crate) breaking: bool,
    pub(crate) top_type: Hierarchy,
    pub(crate) files: HashSet<OsString>,
}

impl ConventionalCommits {
    pub(crate) fn new() -> ConventionalCommits {
        ConventionalCommits::default()
    }

    pub(crate) fn walk_back_commits_to_tag_reference(
        repo: &Repository,
        reference: &str,
    ) -> Result<Self, Error> {
        log::debug!("repo opened to find conventional commits");
        let mut revwalk = repo.revwalk()?;
        revwalk.set_sorting(git2::Sort::NONE)?;
        revwalk.push_head()?;
        log::debug!("starting the walk from the HEAD");
        log::debug!("the reference to walk back to is: `{reference}`");
        revwalk.hide_ref(reference)?;

        macro_rules! filter_try {
            ($e:expr) => {
                match $e {
                    Ok(t) => t,
                    Err(e) => return Some(Err(e)),
                }
            };
        }

        #[allow(clippy::unnecessary_filter_map)]
        let revwalk = revwalk.filter_map(|id| {
            let id = filter_try!(id);
            let commit = repo.find_commit(id);
            let commit = filter_try!(commit);
            Some(Ok(commit))
        });

        let mut conventional_commits = ConventionalCommits::new();

        // Walk back through the commits
        let mut files = HashSet::new();
        for commit in revwalk.flatten() {
            // Get the summary for the conventional commits vec
            log::trace!("commit found: `{}`", &commit.summary().unwrap_or_default());
            conventional_commits.push(&commit);
            // Get the files for the files vec
            let tree = commit.tree()?;
            let diff = repo.diff_tree_to_workdir(Some(&tree), None).unwrap();

            diff.print(git2::DiffFormat::NameOnly, |delta, _hunk, _line| {
                let file = delta.new_file().path().unwrap().file_name().unwrap();
                log::trace!("file found: {:?}", file);
                files.insert(file.to_os_string());
                true
            })
            .unwrap();
        }

        conventional_commits.files = files;

        Ok(conventional_commits)
    }

    pub(crate) fn push(&mut self, commit: &git2::Commit) -> &Self {
        if commit.summary().take().unwrap_or("No") != "No" {
            if let Ok(conventional) = git_conventional::Commit::parse(
                commit.summary().take().unwrap_or("NotConventional"),
            ) {
                log::trace!(
                    "Commit: ({}) {} {}",
                    conventional.type_(),
                    conventional.description(),
                    Hierarchy::parse(&conventional.type_().to_string()).unwrap_or(Hierarchy::Other),
                );
                let counter = self
                    .counts
                    .entry(conventional.type_().to_string())
                    .or_insert(0);
                *counter += 1;

                if !self.breaking {
                    if conventional.breaking() {
                        self.breaking = conventional.breaking();
                        self.top_type = Hierarchy::Breaking;
                    } else if Hierarchy::parse(conventional.type_().as_str()).unwrap()
                        > self.top_type
                    {
                        self.top_type = Hierarchy::parse(conventional.type_().as_str()).unwrap();
                    };
                }
            }
            self.commits.push(
                commit
                    .summary()
                    .take()
                    .unwrap_or("NotConventional")
                    .to_string(),
            );
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::Hierarchy;

    #[test]
    fn type_hierarchy_ordering() {
        let tests = [
            (Hierarchy::Breaking, Hierarchy::Breaking, Ordering::Equal),
            (Hierarchy::Feature, Hierarchy::Feature, Ordering::Equal),
            (Hierarchy::Fix, Hierarchy::Fix, Ordering::Equal),
            (Hierarchy::Other, Hierarchy::Other, Ordering::Equal),
            (Hierarchy::Breaking, Hierarchy::Feature, Ordering::Greater),
            (Hierarchy::Breaking, Hierarchy::Fix, Ordering::Greater),
            (Hierarchy::Breaking, Hierarchy::Other, Ordering::Greater),
            (Hierarchy::Feature, Hierarchy::Breaking, Ordering::Less),
            (Hierarchy::Feature, Hierarchy::Fix, Ordering::Greater),
            (Hierarchy::Feature, Hierarchy::Other, Ordering::Greater),
            (Hierarchy::Fix, Hierarchy::Breaking, Ordering::Less),
            (Hierarchy::Fix, Hierarchy::Feature, Ordering::Less),
            (Hierarchy::Fix, Hierarchy::Other, Ordering::Greater),
            (Hierarchy::Other, Hierarchy::Breaking, Ordering::Less),
            (Hierarchy::Other, Hierarchy::Feature, Ordering::Less),
            (Hierarchy::Other, Hierarchy::Fix, Ordering::Less),
        ];

        for test in tests {
            println!("Test case: {test:#?}");
            let lhs = test.0;
            assert_eq!(test.2, lhs.cmp(&test.1));
        }
    }
}
