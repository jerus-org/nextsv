//! Represents a vector of conventional commits
//!

use std::{collections::HashMap, fmt};

use clap::ValueEnum;
use colored::Colorize;
use log::debug;

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
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, ValueEnum)]
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
    commits: Vec<String>,
    counts: HashMap<String, u32>,
    breaking: bool,
    top_type: Option<TypeHierarchy>,
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
                debug!(
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
                        self.set_top_type_if_higher("breaking");
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

    /// Set the breaking flag value
    ///
    pub fn set_breaking(&mut self, flag: bool) -> &mut Self {
        self.breaking = flag;
        self
    }

    fn set_top_type_if_higher(&mut self, type_: &str) -> &mut Self {
        let th = TypeHierarchy::parse(type_);
        if let Ok(th) = th {
            if let Some(current_top_type) = &self.top_type {
                if th > *current_top_type {
                    self.top_type = Some(th)
                }
            }
        }

        self
    }

    /// top_type_discriminant
    ///
    /// Returns the discriminant of a TypeHierarchy if the Option is Some
    /// and returns 0 if the Option is None.  
    ///
    fn top_type_discriminant(&self) -> u32 {
        if self.top_type.is_none() {
            0_u32
        } else {
            self.top_type().unwrap() as u32
        }
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
    use super::ConventionalCommits;

    #[test]
    fn top_discrimant_returns_0_for_none() {
        let value_under_test = ConventionalCommits::new();
        let expected = 0_u32;

        assert_eq!(expected, value_under_test.top_type_discriminant());
    }

    #[test]
    fn top_discrimant_returns_other() {
        let mut value_under_test = ConventionalCommits::new();
        value_under_test.top_type = Some(crate::TypeHierarchy::Other);
        let expected = 1_u32;

        assert_eq!(expected, value_under_test.top_type_discriminant());
    }

    #[test]
    fn top_discrimant_returns_fix() {
        let mut value_under_test = ConventionalCommits::new();
        value_under_test.top_type = Some(crate::TypeHierarchy::Fix);
        let expected = 2_u32;

        assert_eq!(expected, value_under_test.top_type_discriminant());
    }

    #[test]
    fn top_discrimant_returns_feature() {
        let mut value_under_test = ConventionalCommits::new();
        value_under_test.top_type = Some(crate::TypeHierarchy::Feature);
        let expected = 3_u32;

        assert_eq!(expected, value_under_test.top_type_discriminant());
    }

    #[test]
    fn top_discrimant_returns_breaking() {
        let mut value_under_test = ConventionalCommits::new();
        value_under_test.top_type = Some(crate::TypeHierarchy::Breaking);
        let expected = 4_u32;

        assert_eq!(expected, value_under_test.top_type_discriminant());
    }
}
