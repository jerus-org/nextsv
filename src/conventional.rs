//! Represents a vector of conventional commits
//!

use std::{cmp, collections::HashMap, fmt};

use clap::ValueEnum;
use colored::Colorize;

use crate::Error;

/// TypeHierarchy maps the types identified by git_conventional to a hierarchy of levels
///
/// The enum provides an ordered list to identify the highest level type found in a set
/// of conventional commits.
///
/// Types are mapped as follows:
/// - FEAT: Feature
/// - FIX: Fix
/// - REVERT: Fix
/// - DOCS: Other
/// - STYLE: Other
/// - REFACTOR: Other
/// - PERF: Other
/// - TEST: Other
/// - CHORE: Other
///
/// If a breaking change is found it sets breaking hierarchy.
///
#[derive(Debug, PartialEq, Eq, Clone, ValueEnum)]
pub enum TypeHierarchy {
    /// enforce requirements for all types
    Other = 1,
    /// enforce requirements for fix, feature and breaking
    Fix = 2,
    /// enforce requirements for features and breaking
    Feature = 3,
    /// enforce requirements for breaking only
    Breaking = 4,
}

impl Ord for TypeHierarchy {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match (self, other) {
            (TypeHierarchy::Breaking, TypeHierarchy::Breaking)
            | (TypeHierarchy::Feature, TypeHierarchy::Feature)
            | (TypeHierarchy::Fix, TypeHierarchy::Fix)
            | (TypeHierarchy::Other, TypeHierarchy::Other) => cmp::Ordering::Equal,
            (TypeHierarchy::Other, _) => cmp::Ordering::Less,
            (TypeHierarchy::Breaking, _) => cmp::Ordering::Greater,
            (TypeHierarchy::Fix, TypeHierarchy::Other) => cmp::Ordering::Greater,
            (TypeHierarchy::Fix, TypeHierarchy::Feature)
            | (TypeHierarchy::Fix, TypeHierarchy::Breaking) => cmp::Ordering::Less,
            (TypeHierarchy::Feature, TypeHierarchy::Other)
            | (TypeHierarchy::Feature, TypeHierarchy::Fix) => cmp::Ordering::Greater,
            (TypeHierarchy::Feature, TypeHierarchy::Breaking) => cmp::Ordering::Less,
        }
    }
}

impl PartialOrd for TypeHierarchy {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl TypeHierarchy {
    /// Parse a string into a TypeHierarchy mapping the types or "breaking"
    ///
    pub fn parse(s: &str) -> Result<TypeHierarchy, Error> {
        Ok(match s.to_lowercase().as_str() {
            "feat" => TypeHierarchy::Feature,
            "fix" => TypeHierarchy::Fix,
            "revert" => TypeHierarchy::Fix,
            "docs" => TypeHierarchy::Other,
            "style" => TypeHierarchy::Other,
            "refactor" => TypeHierarchy::Other,
            "perf" => TypeHierarchy::Other,
            "test" => TypeHierarchy::Other,
            "chore" => TypeHierarchy::Other,
            "breaking" => TypeHierarchy::Breaking,
            "build" => TypeHierarchy::Other,
            "ci" => TypeHierarchy::Other,
            _ => return Err(Error::NotTypeHierachyName(s.to_string())),
        })
    }
}

impl fmt::Display for TypeHierarchy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeHierarchy::Breaking => write!(f, "{}", "[Major]".red()),
            TypeHierarchy::Feature => write!(f, "{}", "[Minor]".yellow()),
            TypeHierarchy::Fix => write!(f, "{}", "[Patch]".green()),
            TypeHierarchy::Other => write!(f, "{}", "[Patch]".white()),
        }
    }
}
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct ConventionalCommits {
    pub(crate) commits: Vec<String>,
    pub(crate) counts: HashMap<String, u32>,
    pub(crate) breaking: bool,
    pub(crate) top_type: Option<TypeHierarchy>,
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
                    TypeHierarchy::parse(&conventional.type_().to_string())
                        .unwrap_or(TypeHierarchy::Other),
                );
                self.increment_counts(conventional.type_());

                if !self.breaking {
                    if conventional.breaking() {
                        self.breaking = conventional.breaking();
                        self.set_top_type(TypeHierarchy::Breaking);
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

    pub fn counts(&self) -> HashMap<String, u32> {
        self.counts.clone()
    }

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

    fn set_top_type(&mut self, top_type: TypeHierarchy) -> &mut Self {
        self.top_type = Some(top_type);
        self
    }

    fn set_top_type_if_higher(&mut self, type_: &str) -> &mut Self {
        log::trace!("Testing if {type_:?} is higher than {:?}", self.top_type);
        let th = TypeHierarchy::parse(type_);
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
    pub fn top_type(&self) -> Option<TypeHierarchy> {
        self.top_type.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::TypeHierarchy;

    use super::ConventionalCommits;

    #[test]
    fn type_hierarchy_ordering() {
        let tests = [
            (
                TypeHierarchy::Breaking,
                TypeHierarchy::Breaking,
                Ordering::Equal,
            ),
            (
                TypeHierarchy::Feature,
                TypeHierarchy::Feature,
                Ordering::Equal,
            ),
            (TypeHierarchy::Fix, TypeHierarchy::Fix, Ordering::Equal),
            (TypeHierarchy::Other, TypeHierarchy::Other, Ordering::Equal),
            (
                TypeHierarchy::Breaking,
                TypeHierarchy::Feature,
                Ordering::Greater,
            ),
            (
                TypeHierarchy::Breaking,
                TypeHierarchy::Fix,
                Ordering::Greater,
            ),
            (
                TypeHierarchy::Breaking,
                TypeHierarchy::Other,
                Ordering::Greater,
            ),
            (
                TypeHierarchy::Feature,
                TypeHierarchy::Breaking,
                Ordering::Less,
            ),
            (
                TypeHierarchy::Feature,
                TypeHierarchy::Fix,
                Ordering::Greater,
            ),
            (
                TypeHierarchy::Feature,
                TypeHierarchy::Other,
                Ordering::Greater,
            ),
            (TypeHierarchy::Fix, TypeHierarchy::Breaking, Ordering::Less),
            (TypeHierarchy::Fix, TypeHierarchy::Feature, Ordering::Less),
            (TypeHierarchy::Fix, TypeHierarchy::Other, Ordering::Greater),
            (
                TypeHierarchy::Other,
                TypeHierarchy::Breaking,
                Ordering::Less,
            ),
            (TypeHierarchy::Other, TypeHierarchy::Feature, Ordering::Less),
            (TypeHierarchy::Other, TypeHierarchy::Fix, Ordering::Less),
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

        let test_level = TypeHierarchy::Breaking;

        let tests = ["other", "fix", "feat", "breaking"];
        const ARRAY_REPEAT_VALUE: crate::conventional::TypeHierarchy = TypeHierarchy::Breaking;
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

        let test_level = TypeHierarchy::Feature;

        let tests = ["other", "fix", "feat", "breaking"];
        let expected = [
            TypeHierarchy::Feature,
            TypeHierarchy::Feature,
            TypeHierarchy::Feature,
            TypeHierarchy::Breaking,
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

        let test_level = TypeHierarchy::Fix;

        let tests = ["other", "fix", "feat", "breaking"];
        let expected = [
            TypeHierarchy::Fix,
            TypeHierarchy::Fix,
            TypeHierarchy::Feature,
            TypeHierarchy::Breaking,
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

        let test_level = TypeHierarchy::Other;

        let tests = ["other", "fix", "feat", "breaking"];
        let expected = [
            TypeHierarchy::Other,
            TypeHierarchy::Fix,
            TypeHierarchy::Feature,
            TypeHierarchy::Breaking,
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
