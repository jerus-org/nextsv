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
#[derive(Debug, PartialEq, Eq, Clone, ValueEnum, Default)]
pub enum LevelHierarchy {
    /// enforce requirements for all types
    #[default]
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

#[cfg(test)]
mod test {
    use super::LevelHierarchy;

    use rstest::rstest;

    fn level_hierarchy_example_lowest() -> LevelHierarchy {
        LevelHierarchy::Other
    }
    fn level_hierarchy_example_highest() -> LevelHierarchy {
        LevelHierarchy::Breaking
    }
    fn level_hierarchy_example_mid() -> LevelHierarchy {
        LevelHierarchy::Feature
    }

    #[test]
    fn test_partial_eq() {
        let one = level_hierarchy_example_highest();
        let two = level_hierarchy_example_highest();
        assert_eq!(one, two);
    }

    #[test]
    fn test_eq() {
        let one = level_hierarchy_example_lowest();
        let two = level_hierarchy_example_lowest();
        assert!(one == two);
    }

    #[test]
    fn test_partial_ord() {
        let one = level_hierarchy_example_highest();
        let two = level_hierarchy_example_mid();
        assert!(one > two);
    }

    #[test]
    fn test_ord() {
        let one = level_hierarchy_example_lowest();
        let two = level_hierarchy_example_mid();
        assert_eq!(two.cmp(&one), std::cmp::Ordering::Greater);
    }

    #[rstest]
    #[case::breaking(LevelHierarchy::Breaking, "[Major]")]
    #[case::non_production(LevelHierarchy::Feature, "[Minor]")]
    #[case::production(LevelHierarchy::Fix, "[Patch]")]
    #[case::release(LevelHierarchy::Other, "[Patch]")]
    fn display_value(#[case] test: LevelHierarchy, #[case] expected: &str) {
        assert_eq!(expected, test.to_string().as_str());
    }

    #[rstest]
    #[case::feature("feat", LevelHierarchy::Feature)]
    #[case::fix("fix", LevelHierarchy::Fix)]
    #[case::revert("revert", LevelHierarchy::Fix)]
    #[case::docs("docs", LevelHierarchy::Other)]
    #[case::style("style", LevelHierarchy::Other)]
    #[case::refactor("refactor", LevelHierarchy::Other)]
    #[case::perf("perf", LevelHierarchy::Other)]
    #[case::test("test", LevelHierarchy::Other)]
    #[case::chore("chore", LevelHierarchy::Other)]
    #[case::breaking("breaking", LevelHierarchy::Breaking)]
    #[case::build("build", LevelHierarchy::Other)]
    #[case::ci("ci", LevelHierarchy::Other)]
    fn parse_conventional_commit_label_to_level_hierarchy(
        #[case] label: &str,
        #[case] expected: LevelHierarchy,
    ) {
        let test_level = LevelHierarchy::parse(label).unwrap();

        assert_eq!(expected, test_level);
    }
}
