//! Represents a vector of conventional commits
//!

use std::collections::HashMap;

use super::LevelHierarchy;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct ConventionalCommits {
    pub(crate) commits: Vec<String>,
    pub(crate) counts: HashMap<String, u32>,
    pub(crate) breaking: bool,
    pub(crate) top_type: Option<LevelHierarchy>,
}

impl ConventionalCommits {
    pub fn new() -> ConventionalCommits {
        ConventionalCommits::default()
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
                    LevelHierarchy::parse(&conventional.type_().to_string())
                        .unwrap_or(LevelHierarchy::Other),
                );
                self.increment_counts(conventional.type_());

                if !self.breaking {
                    if conventional.breaking() {
                        self.breaking = conventional.breaking();
                        self.set_top_type(LevelHierarchy::Breaking);
                    } else {
                        self.set_top_type_if_higher(conventional.type_().as_str());
                    }
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

    fn set_top_type(&mut self, top_type: LevelHierarchy) -> &mut Self {
        self.top_type = Some(top_type);
        self
    }

    fn set_top_type_if_higher(&mut self, type_: &str) -> &mut Self {
        log::trace!("Testing if {type_:?} is higher than {:?}", self.top_type);
        let th = LevelHierarchy::parse(type_);
        log::trace!("Result of parse to TypeHierarchy: {th:?}");
        if let Ok(th) = th {
            if self.top_type.is_some() {
                if th > self.top_type().unwrap() {
                    self.top_type = Some(th);
                }
            } else {
                self.top_type = Some(th);
            }
        }

        self
    }

    /// top_type  
    ///
    /// Returns the top type.
    ///
    pub fn top_type(&self) -> Option<LevelHierarchy> {
        self.top_type.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::LevelHierarchy;

    use super::ConventionalCommits;

    #[test]
    fn type_hierarchy_ordering() {
        let tests = [
            (
                LevelHierarchy::Breaking,
                LevelHierarchy::Breaking,
                Ordering::Equal,
            ),
            (
                LevelHierarchy::Feature,
                LevelHierarchy::Feature,
                Ordering::Equal,
            ),
            (LevelHierarchy::Fix, LevelHierarchy::Fix, Ordering::Equal),
            (
                LevelHierarchy::Other,
                LevelHierarchy::Other,
                Ordering::Equal,
            ),
            (
                LevelHierarchy::Breaking,
                LevelHierarchy::Feature,
                Ordering::Greater,
            ),
            (
                LevelHierarchy::Breaking,
                LevelHierarchy::Fix,
                Ordering::Greater,
            ),
            (
                LevelHierarchy::Breaking,
                LevelHierarchy::Other,
                Ordering::Greater,
            ),
            (
                LevelHierarchy::Feature,
                LevelHierarchy::Breaking,
                Ordering::Less,
            ),
            (
                LevelHierarchy::Feature,
                LevelHierarchy::Fix,
                Ordering::Greater,
            ),
            (
                LevelHierarchy::Feature,
                LevelHierarchy::Other,
                Ordering::Greater,
            ),
            (
                LevelHierarchy::Fix,
                LevelHierarchy::Breaking,
                Ordering::Less,
            ),
            (LevelHierarchy::Fix, LevelHierarchy::Feature, Ordering::Less),
            (
                LevelHierarchy::Fix,
                LevelHierarchy::Other,
                Ordering::Greater,
            ),
            (
                LevelHierarchy::Other,
                LevelHierarchy::Breaking,
                Ordering::Less,
            ),
            (
                LevelHierarchy::Other,
                LevelHierarchy::Feature,
                Ordering::Less,
            ),
            (LevelHierarchy::Other, LevelHierarchy::Fix, Ordering::Less),
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

        let test_level = LevelHierarchy::Breaking;

        let tests = ["other", "fix", "feat", "breaking"];
        const ARRAY_REPEAT_VALUE: LevelHierarchy = LevelHierarchy::Breaking;
        let expected = [ARRAY_REPEAT_VALUE; 4];

        let test_result_pairs = tests.iter().zip(expected);

        for pair in test_result_pairs {
            println!("Testing pair: {pair:?}");
            value_under_test.set_top_type(test_level.clone());
            assert_eq!(Some(test_level.clone()), value_under_test.top_type());
            value_under_test.set_top_type_if_higher(pair.0);
            assert_eq!(Some(pair.1), value_under_test.top_type());
        }
    }

    #[test]
    fn set_top_type_test_currently_feature() {
        let mut value_under_test = ConventionalCommits::new();

        let test_level = LevelHierarchy::Feature;

        let tests = ["other", "fix", "feat", "breaking"];
        let expected = [
            LevelHierarchy::Feature,
            LevelHierarchy::Feature,
            LevelHierarchy::Feature,
            LevelHierarchy::Breaking,
        ];

        let test_result_pairs = tests.iter().zip(expected);

        for pair in test_result_pairs {
            println!("Testing pair: {pair:?}");
            value_under_test.set_top_type(test_level.clone());
            assert_eq!(Some(test_level.clone()), value_under_test.top_type());
            value_under_test.set_top_type_if_higher(pair.0);
            assert_eq!(Some(pair.1), value_under_test.top_type());
        }
    }

    #[test]
    fn set_top_type_test_currently_fix() {
        let mut value_under_test = ConventionalCommits::new();

        let test_level = LevelHierarchy::Fix;

        let tests = ["other", "fix", "feat", "breaking"];
        let expected = [
            LevelHierarchy::Fix,
            LevelHierarchy::Fix,
            LevelHierarchy::Feature,
            LevelHierarchy::Breaking,
        ];

        let test_result_pairs = tests.iter().zip(expected);

        for pair in test_result_pairs {
            println!("Testing pair: {pair:?}");
            value_under_test.set_top_type(test_level.clone());
            assert_eq!(Some(test_level.clone()), value_under_test.top_type());
            value_under_test.set_top_type_if_higher(pair.0);
            assert_eq!(Some(pair.1), value_under_test.top_type());
        }
    }

    #[test]
    fn set_top_type_test_currently_other() {
        let mut value_under_test = ConventionalCommits::new();

        let test_level = LevelHierarchy::Other;

        let tests = ["other", "fix", "feat", "breaking"];
        let expected = [
            LevelHierarchy::Other,
            LevelHierarchy::Fix,
            LevelHierarchy::Feature,
            LevelHierarchy::Breaking,
        ];

        let test_result_pairs = tests.iter().zip(expected);

        for pair in test_result_pairs {
            println!("Testing pair: {pair:?}");
            value_under_test.set_top_type(test_level.clone());
            assert_eq!(Some(test_level.clone()), value_under_test.top_type());
            value_under_test.set_top_type_if_higher(pair.0);
            assert_eq!(Some(pair.1), value_under_test.top_type());
        }
    }
}
