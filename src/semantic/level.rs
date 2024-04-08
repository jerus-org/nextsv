use std::fmt;

use crate::ForceLevel;

/// Level at which the next increment will be made
///
#[derive(Debug, PartialOrd, PartialEq, Eq, Ord, Clone, Default)]
pub enum Level {
    /// When no update has been detected the level is set to none
    #[default]
    None,
    /// Update will be made at the patch level
    Patch,
    /// Update will be made at the private level
    Minor,
    /// Update will be made at the major level
    Major,
    /// Update is a release removing any pre-release suffixes (for future use)
    Release,
    /// Update is to an alpha pre-release suffix (for future use)
    Alpha,
    /// Update is to an beta pre-release suffix (for future use)
    Beta,
    /// Update is to an rc pre-release suffix (for future use)
    Rc,
    /// Update is to version 1.0.0
    First,
    /// Custom for update to a custom pre-release label
    Custom(String),
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Level::None => write!(f, "none"),
            Level::Patch => write!(f, "patch"),
            Level::Minor => write!(f, "minor"),
            Level::Major => write!(f, "major"),
            Level::Release => write!(f, "release"),
            Level::Alpha => write!(f, "alpha"),
            Level::Beta => write!(f, "beta"),
            Level::Rc => write!(f, "rc"),
            Level::First => write!(f, "1.0.0"),
            Level::Custom(s) => write!(f, "{}", s),
        }
    }
}

impl From<ForceLevel> for Level {
    fn from(force_level: ForceLevel) -> Self {
        match force_level {
            ForceLevel::First => Level::First,
            ForceLevel::Major => Level::Major,
            ForceLevel::Minor => Level::Minor,
            ForceLevel::Patch => Level::Patch,
            ForceLevel::Alpha => Level::Alpha,
            ForceLevel::Beta => Level::Beta,
            ForceLevel::Rc => Level::Rc,
            ForceLevel::Release => Level::Release,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::ForceLevel;

    use super::Level;
    use rstest::rstest;

    #[rstest]
    #[case::none(Level::None, "none")]
    #[case::patch(Level::Patch, "patch")]
    #[case::minor(Level::Minor, "minor")]
    #[case::major(Level::Major, "major")]
    #[case::release(Level::Release, "release")]
    #[case::alpha(Level::Alpha, "alpha")]
    #[case::beta(Level::Beta, "beta")]
    #[case::rc(Level::Rc, "rc")]
    #[case::first(Level::First, "1.0.0")]
    #[case::custom(Level::Custom(String::from("alpha.1")), "alpha.1")]
    fn display_value(#[case] test: Level, #[case] expected: &str) {
        assert_eq!(expected, test.to_string().as_str());
    }

    #[rstest]
    #[case::first(ForceLevel::First, Level::First)]
    #[case::major(ForceLevel::Major, Level::Major)]
    #[case::minor(ForceLevel::Minor, Level::Minor)]
    #[case::patch(ForceLevel::Patch, Level::Patch)]
    fn from_forcelevel(#[case] from: ForceLevel, #[case] expected: Level) {
        assert_eq!(expected, from.into());
    }
}
