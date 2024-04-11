use std::fmt;

use clap::ValueEnum;

/// The options for choosing the level of a forced change
///
/// The enum is used by the force method to define the level
/// at which the forced change is made.
///
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, ValueEnum)]
pub enum ForceLevel {
    /// force change to major
    Major,
    /// force change to minor
    Minor,
    /// force change to patch
    Patch,
    /// Force update of first production release (1.0.0)
    First,
    /// Release current version
    Release,
    /// Alpha pre-release of current version
    Alpha,
    /// Beta pre-release of current version
    Beta,
    /// Rc pre-release of current version
    Rc,
}

impl fmt::Display for ForceLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ForceLevel::Major => write!(f, "major"),
            ForceLevel::Minor => write!(f, "minor"),
            ForceLevel::Patch => write!(f, "patch"),
            ForceLevel::First => write!(f, "1.0.0"),
            ForceLevel::Alpha => write!(f, "alpha"),
            ForceLevel::Beta => write!(f, "beta"),
            ForceLevel::Rc => write!(f, "rc"),
            ForceLevel::Release => write!(f, "release"),
        }
    }
}

#[cfg(test)]
mod test {

    use super::ForceLevel;
    use rstest::rstest;

    #[rstest]
    #[case::patch(ForceLevel::Patch, "patch")]
    #[case::minor(ForceLevel::Minor, "minor")]
    #[case::major(ForceLevel::Major, "major")]
    #[case::release(ForceLevel::Release, "release")]
    #[case::alpha(ForceLevel::Alpha, "alpha")]
    #[case::beta(ForceLevel::Beta, "beta")]
    #[case::rc(ForceLevel::Rc, "rc")]
    #[case::first(ForceLevel::First, "1.0.0")]
    fn display_value(#[case] test: ForceLevel, #[case] expected: &str) {
        assert_eq!(expected, test.to_string().as_str());
    }
}
