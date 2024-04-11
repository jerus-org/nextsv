use std::fmt;

use super::super::semantic::Semantic;
use crate::ForceLevel;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub(crate) enum CalcRoute {
    NonProd,
    PreRelease,
    #[default]
    Prod,
    Forced(ForceLevel),
}

impl CalcRoute {
    pub(crate) fn new(version: &Semantic) -> CalcRoute {
        if version.pre_release.is_some() {
            return CalcRoute::PreRelease;
        };
        if 0 == version.major {
            return CalcRoute::NonProd;
        };
        CalcRoute::Prod
    }
}

impl fmt::Display for CalcRoute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcRoute::PreRelease => write!(f, "pre release"),
            CalcRoute::NonProd => write!(f, "non production"),
            CalcRoute::Prod => write!(f, "production"),
            CalcRoute::Forced(level) => write!(f, "Force to `{}`", level),
        }
    }
}

#[cfg(test)]
mod test {

    use crate::{semantic::Semantic, ForceLevel};

    use super::CalcRoute;
    use rstest::rstest;

    #[rstest]
    #[case::pre_release(CalcRoute::PreRelease, "pre release")]
    #[case::non_production(CalcRoute::NonProd, "non production")]
    #[case::production(CalcRoute::Prod, "production")]
    #[case::release(CalcRoute::Forced(ForceLevel::Minor), "Force to `minor`")]
    fn display_value(#[case] test: CalcRoute, #[case] expected: &str) {
        assert_eq!(expected, test.to_string().as_str());
    }

    #[rstest]
    #[case::non_production("0", "7", "9", "", CalcRoute::NonProd)]
    #[case::pre_release("1", "0", "0", "alpha.1", CalcRoute::PreRelease)]
    #[case::production("1", "0", "5", "", CalcRoute::Prod)]
    fn calculate_route(
        #[case] major: &str,
        #[case] minor: &str,
        #[case] patch: &str,
        #[case] pre_release: &str,
        #[case] expected: CalcRoute,
    ) {
        let version = Semantic::new(major, minor, patch, pre_release, "");

        let test = CalcRoute::new(&version);

        assert_eq!(expected, test);
    }
}
