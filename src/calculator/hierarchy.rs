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
pub enum Hierarchy {
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

impl Ord for Hierarchy {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match (self, other) {
            (Hierarchy::Breaking, Hierarchy::Breaking)
            | (Hierarchy::Feature, Hierarchy::Feature)
            | (Hierarchy::Fix, Hierarchy::Fix)
            | (Hierarchy::Other, Hierarchy::Other) => cmp::Ordering::Equal,
            (Hierarchy::Other, _) => cmp::Ordering::Less,
            (Hierarchy::Breaking, _) => cmp::Ordering::Greater,
            (Hierarchy::Fix, Hierarchy::Other) => cmp::Ordering::Greater,
            (Hierarchy::Fix, Hierarchy::Feature) | (Hierarchy::Fix, Hierarchy::Breaking) => {
                cmp::Ordering::Less
            }
            (Hierarchy::Feature, Hierarchy::Other) | (Hierarchy::Feature, Hierarchy::Fix) => {
                cmp::Ordering::Greater
            }
            (Hierarchy::Feature, Hierarchy::Breaking) => cmp::Ordering::Less,
        }
    }
}

impl PartialOrd for Hierarchy {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hierarchy {
    /// Parse a string into a TypeHierarchy mapping the types or "breaking"
    ///
    pub fn parse(s: &str) -> Result<Hierarchy, Error> {
        Ok(match s.to_lowercase().as_str() {
            "feat" => Hierarchy::Feature,
            "fix" => Hierarchy::Fix,
            "revert" => Hierarchy::Fix,
            "docs" => Hierarchy::Other,
            "style" => Hierarchy::Other,
            "refactor" => Hierarchy::Other,
            "perf" => Hierarchy::Other,
            "test" => Hierarchy::Other,
            "chore" => Hierarchy::Other,
            "breaking" => Hierarchy::Breaking,
            "build" => Hierarchy::Other,
            "ci" => Hierarchy::Other,
            _ => return Err(Error::NotTypeHierachyName(s.to_string())),
        })
    }
}

impl fmt::Display for Hierarchy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Hierarchy::Breaking => write!(f, "{}", "[Major]".red()),
            Hierarchy::Feature => write!(f, "{}", "[Minor]".yellow()),
            Hierarchy::Fix => write!(f, "{}", "[Patch]".green()),
            Hierarchy::Other => write!(f, "{}", "[Patch]".white()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Hierarchy;

    use colored::Colorize;
    use rstest::rstest;

    fn level_hierarchy_example_lowest() -> Hierarchy {
        Hierarchy::Other
    }
    fn level_hierarchy_example_highest() -> Hierarchy {
        Hierarchy::Breaking
    }
    fn level_hierarchy_example_mid() -> Hierarchy {
        Hierarchy::Feature
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
    #[case::breaking(Hierarchy::Breaking, format!("{}","[Major]".red()))]
    #[case::non_production(Hierarchy::Feature, format!("{}","[Minor]".yellow()))]
    #[case::production(Hierarchy::Fix, format!("{}","[Patch]".green()))]
    #[case::release(Hierarchy::Other, format!("{}","[Patch]".white()))]
    fn display_value(#[case] test: Hierarchy, #[case] expected: String) {
        assert_eq!(expected, test.to_string());
    }

    #[rstest]
    #[case::feature("feat", Hierarchy::Feature)]
    #[case::fix("fix", Hierarchy::Fix)]
    #[case::revert("revert", Hierarchy::Fix)]
    #[case::docs("docs", Hierarchy::Other)]
    #[case::style("style", Hierarchy::Other)]
    #[case::refactor("refactor", Hierarchy::Other)]
    #[case::perf("perf", Hierarchy::Other)]
    #[case::test("test", Hierarchy::Other)]
    #[case::chore("chore", Hierarchy::Other)]
    #[case::breaking("breaking", Hierarchy::Breaking)]
    #[case::build("build", Hierarchy::Other)]
    #[case::ci("ci", Hierarchy::Other)]
    fn parse_conventional_commit_label_to_level_hierarchy(
        #[case] label: &str,
        #[case] expected: Hierarchy,
    ) {
        let test_level = Hierarchy::parse(label).unwrap();

        assert_eq!(expected, test_level);
    }
}
