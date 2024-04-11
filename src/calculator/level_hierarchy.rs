use std::{cmp, fmt};

use clap::ValueEnum;
use colored::Colorize;

use crate::Error;

/// LevelHierarchy maps the types identified by git_conventional to a hierarchy of levels
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
pub enum LevelHierarchy {
    /// enforce requirements for all types
    Other = 1,
    /// enforce requirements for fix, feature and breaking
    Fix = 2,
    /// enforce requirements for features and breaking
    Feature = 3,
    /// enforce requirements for breaking only
    Breaking = 4,
}

impl Ord for LevelHierarchy {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match (self, other) {
            (LevelHierarchy::Breaking, LevelHierarchy::Breaking)
            | (LevelHierarchy::Feature, LevelHierarchy::Feature)
            | (LevelHierarchy::Fix, LevelHierarchy::Fix)
            | (LevelHierarchy::Other, LevelHierarchy::Other) => cmp::Ordering::Equal,
            (LevelHierarchy::Other, _) => cmp::Ordering::Less,
            (LevelHierarchy::Breaking, _) => cmp::Ordering::Greater,
            (LevelHierarchy::Fix, LevelHierarchy::Other) => cmp::Ordering::Greater,
            (LevelHierarchy::Fix, LevelHierarchy::Feature)
            | (LevelHierarchy::Fix, LevelHierarchy::Breaking) => cmp::Ordering::Less,
            (LevelHierarchy::Feature, LevelHierarchy::Other)
            | (LevelHierarchy::Feature, LevelHierarchy::Fix) => cmp::Ordering::Greater,
            (LevelHierarchy::Feature, LevelHierarchy::Breaking) => cmp::Ordering::Less,
        }
    }
}

impl PartialOrd for LevelHierarchy {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl LevelHierarchy {
    /// Parse a string into a TypeHierarchy mapping the types or "breaking"
    ///
    pub fn parse(s: &str) -> Result<LevelHierarchy, Error> {
        Ok(match s.to_lowercase().as_str() {
            "feat" => LevelHierarchy::Feature,
            "fix" => LevelHierarchy::Fix,
            "revert" => LevelHierarchy::Fix,
            "docs" => LevelHierarchy::Other,
            "style" => LevelHierarchy::Other,
            "refactor" => LevelHierarchy::Other,
            "perf" => LevelHierarchy::Other,
            "test" => LevelHierarchy::Other,
            "chore" => LevelHierarchy::Other,
            "breaking" => LevelHierarchy::Breaking,
            "build" => LevelHierarchy::Other,
            "ci" => LevelHierarchy::Other,
            _ => return Err(Error::NotTypeHierachyName(s.to_string())),
        })
    }
}

impl fmt::Display for LevelHierarchy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LevelHierarchy::Breaking => write!(f, "{}", "[Major]".red()),
            LevelHierarchy::Feature => write!(f, "{}", "[Minor]".yellow()),
            LevelHierarchy::Fix => write!(f, "{}", "[Patch]".green()),
            LevelHierarchy::Other => write!(f, "{}", "[Patch]".white()),
        }
    }
}
