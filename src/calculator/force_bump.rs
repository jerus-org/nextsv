use std::fmt;

use clap::ValueEnum;

/// The options for choosing the level of a forced change
///
/// The enum is used by the force method to define the level
/// at which the forced change is made.
///
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, ValueEnum)]
pub enum ForceBump {
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

impl fmt::Display for ForceBump {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ForceBump::Major => write!(f, "major"),
            ForceBump::Minor => write!(f, "minor"),
            ForceBump::Patch => write!(f, "patch"),
            ForceBump::First => write!(f, "1.0.0"),
            ForceBump::Alpha => write!(f, "alpha"),
            ForceBump::Beta => write!(f, "beta"),
            ForceBump::Rc => write!(f, "rc"),
            ForceBump::Release => write!(f, "release"),
        }
    }
}

#[cfg(test)]
mod test {

    use super::ForceBump;
    use rstest::rstest;

    #[rstest]
    #[case::patch(ForceBump::Patch, "patch")]
    #[case::minor(ForceBump::Minor, "minor")]
    #[case::major(ForceBump::Major, "major")]
    #[case::release(ForceBump::Release, "release")]
    #[case::alpha(ForceBump::Alpha, "alpha")]
    #[case::beta(ForceBump::Beta, "beta")]
    #[case::rc(ForceBump::Rc, "rc")]
    #[case::first(ForceBump::First, "1.0.0")]
    fn display_value(#[case] test: ForceBump, #[case] expected: &str) {
        assert_eq!(expected, test.to_string().as_str());
    }
}
