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
pub struct ConventionalCommits {
    pub(crate) commits: Vec<String>,
    pub(crate) counts: HashMap<String, u32>,
    pub(crate) breaking: bool,
    pub(crate) top_type: Hierarchy,
    pub(crate) files: HashSet<OsString>,
}

impl ConventionalCommits {
    pub fn new() -> ConventionalCommits {
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

    pub fn push(&mut self, commit: &git2::Commit) -> &Self {
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
                self.increment_counts(conventional.type_());

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

    pub fn increment_counts(&mut self, commit_type: git_conventional::Type) {
        let counter = self.counts.entry(commit_type.to_string()).or_insert(0);
        *counter += 1;
    }

    // pub fn counts(&self) -> HashMap<String, u32> {
    //     self.counts.clone()
    // }

    pub fn commits_by_type(&self, commit_type: &str) -> u32 {
        self.counts.get(commit_type).unwrap_or(&0_u32).to_owned()
    }

    pub fn commits_all_types(&self) -> u32 {
        self.counts.values().sum()
    }

    pub fn breaking(&self) -> bool {
        self.breaking
    }

    /// Set the value of the top type to a valid TypeHierarchy value
    ///

    fn set_top_type(&mut self, top_type: Hierarchy) -> &mut Self {
        self.top_type = top_type;
        self
    }

    fn update_top_type_if_higher(&mut self, type_: &str) -> &mut Self {
        log::trace!("Testing if {type_:?} is higher than {:?}", self.top_type);
        let new_type_level = Hierarchy::parse(type_).unwrap_or_default();
        if self.top_type < new_type_level {
            self.top_type = new_type_level;
        };
        self
    }

    /// top_type  
    ///
    /// Returns the top type.
    ///
    pub fn top_type(&self) -> Hierarchy {
        self.top_type.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::Hierarchy;

    use super::ConventionalCommits;

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

    #[test]
    fn set_top_type_test_currently_breaking() {
        let mut value_under_test = ConventionalCommits::new();

        let test_level = Hierarchy::Breaking;

        let tests = ["other", "fix", "feat", "breaking"];
        const ARRAY_REPEAT_VALUE: Hierarchy = Hierarchy::Breaking;
        let expected = [ARRAY_REPEAT_VALUE; 4];

        let test_result_pairs = tests.iter().zip(expected);

        for pair in test_result_pairs {
            println!("Testing pair: {pair:?}");
            value_under_test.set_top_type(test_level.clone());
            assert_eq!(test_level.clone(), value_under_test.top_type());
            value_under_test.update_top_type_if_higher(pair.0);
            assert_eq!(pair.1, value_under_test.top_type());
        }
    }

    #[test]
    fn set_top_type_test_currently_feature() {
        let mut value_under_test = ConventionalCommits::new();

        let test_level = Hierarchy::Feature;

        let tests = ["other", "fix", "feat", "breaking"];
        let expected = [
            Hierarchy::Feature,
            Hierarchy::Feature,
            Hierarchy::Feature,
            Hierarchy::Breaking,
        ];

        let test_result_pairs = tests.iter().zip(expected);

        for pair in test_result_pairs {
            println!("Testing pair: {pair:?}");
            value_under_test.set_top_type(test_level.clone());
            assert_eq!(test_level.clone(), value_under_test.top_type());
            value_under_test.update_top_type_if_higher(pair.0);
            assert_eq!(pair.1, value_under_test.top_type());
        }
    }

    #[test]
    fn set_top_type_test_currently_fix() {
        let mut value_under_test = ConventionalCommits::new();

        let test_level = Hierarchy::Fix;

        let tests = ["other", "fix", "feat", "breaking"];
        let expected = [
            Hierarchy::Fix,
            Hierarchy::Fix,
            Hierarchy::Feature,
            Hierarchy::Breaking,
        ];

        let test_result_pairs = tests.iter().zip(expected);

        for pair in test_result_pairs {
            println!("Testing pair: {pair:?}");
            value_under_test.set_top_type(test_level.clone());
            assert_eq!(test_level.clone(), value_under_test.top_type());
            value_under_test.update_top_type_if_higher(pair.0);
            assert_eq!(pair.1, value_under_test.top_type());
        }
    }

    #[test]
    fn set_top_type_test_currently_other() {
        let mut value_under_test = ConventionalCommits::new();

        let test_level = Hierarchy::Other;

        let tests = ["other", "fix", "feat", "breaking"];
        let expected = [
            Hierarchy::Other,
            Hierarchy::Fix,
            Hierarchy::Feature,
            Hierarchy::Breaking,
        ];

        let test_result_pairs = tests.iter().zip(expected);

        for pair in test_result_pairs {
            println!("Testing pair: {pair:?}");
            value_under_test.set_top_type(test_level.clone());
            assert_eq!(test_level.clone(), value_under_test.top_type());
            value_under_test.update_top_type_if_higher(pair.0);
            assert_eq!(pair.1, value_under_test.top_type());
        }
    }
}
