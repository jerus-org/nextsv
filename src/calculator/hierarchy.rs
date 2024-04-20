use std::{cmp, fmt};

use clap::ValueEnum;
use colored::Colorize;

use crate::Error;

/// The `Hierarchy` enum provides a hieracchy for commit types that maps to the levels
/// of Major, Minor and Patch levels of semver. The top level is for breaking
/// commits and would be mapped to a Major version change for a new
/// production release.
///
/// | Hierarchy | Conventional Commit Type | Prod Semver | Non-Prod Semver |
/// |-----------|--------------------------|-------------|-----------------|
/// | Breaking  | breaking                 | Major       | Minor           |
/// | Feature   | feat                     | Minor       | Minor           |
/// | Fix       | fix, revert              | Patch       | Patch           |
/// | Other     | docs, style, refactor, perf, test, chore, build, ci, etc. | Patch       | Patch           |
///
/// The hierachy is listed in order of importance.
#[derive(Debug, PartialEq, Eq, Clone, ValueEnum, Default)]
pub enum Hierarchy {
    /// Other variant represents other changes.
    #[default]
    Other,
    /// Fix variant represents fixes.
    Fix,
    /// Feature variant represents new features.
    Feature,
    /// Breaking variant represents breaking changes.
    Breaking,
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
    #[allow(missing_docs)]
    pub fn parse(s: &str) -> Result<Hierarchy, Error> {
        Ok(match s.to_lowercase().as_str() {
            "breaking" => Hierarchy::Breaking,
            "feat" => Hierarchy::Feature,
            "fix" => Hierarchy::Fix,
            "revert" => Hierarchy::Fix,
            _ => Hierarchy::Other,
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
