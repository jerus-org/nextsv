use std::fmt;

use super::super::version::Semantic;
use crate::{version::PreReleaseType, ForceBump};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub(crate) enum Route {
    NonProd,
    PreRelease(PreReleaseType),
    #[default]
    Prod,
    Forced(ForceBump),
}

impl Route {
    pub(crate) fn new(version: &Semantic) -> Route {
        if let Some(pre_release) = &version.pre_release {
            return Route::PreRelease(pre_release.pre_type.clone());
        };
        if 0 == version.major {
            return Route::NonProd;
        };
        Route::Prod
    }
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Route::PreRelease(pre_type) => write!(f, "{} pre release", pre_type),
            Route::NonProd => write!(f, "non production"),
            Route::Prod => write!(f, "production"),
            Route::Forced(level) => write!(f, "Force to `{}`", level),
        }
    }
}

#[cfg(test)]
mod test {

    use crate::{
        version::{PreReleaseType, Semantic},
        ForceBump,
    };

    use super::Route;
    use rstest::rstest;

    #[rstest]
    #[case::pre_release(Route::PreRelease(PreReleaseType::Alpha), "pre release")]
    #[case::non_production(Route::NonProd, "non production")]
    #[case::production(Route::Prod, "production")]
    #[case::release(Route::Forced(ForceBump::Minor), "Force to `minor`")]
    fn display_value(#[case] test: Route, #[case] expected: &str) {
        assert_eq!(expected, test.to_string().as_str());
    }

    #[rstest]
    #[case::non_production("0", "7", "9", "", Route::NonProd)]
    #[case::pre_release("1", "0", "0", "alpha.1", Route::PreRelease(PreReleaseType::Alpha))]
    #[case::production("1", "0", "5", "", Route::Prod)]
    fn calculate_route(
        #[case] major: &str,
        #[case] minor: &str,
        #[case] patch: &str,
        #[case] pre_release: &str,
        #[case] expected: Route,
    ) {
        let version = Semantic::new(major, minor, patch, pre_release, "");

        let test = Route::new(&version);

        assert_eq!(expected, test);
    }
}
