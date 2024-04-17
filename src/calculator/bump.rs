use std::fmt;

use crate::{version::PreReleaseType, ForceBump};

use super::{ConventionalCommits, Route};

/// Bump at which the next increment will be made
///
#[derive(Debug, PartialOrd, PartialEq, Eq, Ord, Clone, Default)]
pub enum Bump {
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

impl fmt::Display for Bump {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Bump::None => write!(f, "none"),
            Bump::Patch => write!(f, "patch"),
            Bump::Minor => write!(f, "minor"),
            Bump::Major => write!(f, "major"),
            Bump::Release => write!(f, "release"),
            Bump::Alpha => write!(f, "alpha"),
            Bump::Beta => write!(f, "beta"),
            Bump::Rc => write!(f, "rc"),
            Bump::First => write!(f, "1.0.0"),
            Bump::Custom(s) => write!(f, "{}", s),
        }
    }
}

impl From<ForceBump> for Bump {
    fn from(force_level: ForceBump) -> Self {
        match force_level {
            ForceBump::First => Bump::First,
            ForceBump::Major => Bump::Major,
            ForceBump::Minor => Bump::Minor,
            ForceBump::Patch => Bump::Patch,
            ForceBump::Alpha => Bump::Alpha,
            ForceBump::Beta => Bump::Beta,
            ForceBump::Rc => Bump::Rc,
            ForceBump::Release => Bump::Release,
        }
    }
}

impl From<PreReleaseType> for Bump {
    fn from(pre_release_type: PreReleaseType) -> Self {
        match pre_release_type {
            PreReleaseType::Alpha => Bump::Alpha,
            PreReleaseType::Beta => Bump::Beta,
            PreReleaseType::Rc => Bump::Rc,
            PreReleaseType::Custom => Bump::Custom(String::new()),
        }
    }
}

use std::ffi::OsString;

impl From<Bump> for OsString {
    fn from(bump: Bump) -> Self {
        match bump {
            Bump::None => OsString::from("None"),
            Bump::Patch => OsString::from("Patch"),
            Bump::Minor => OsString::from("Minor"),
            Bump::Major => OsString::from("Major"),
            Bump::Release => OsString::from("Release"),
            Bump::Alpha => OsString::from("Alpha"),
            Bump::Beta => OsString::from("Beta"),
            Bump::Rc => OsString::from("Rc"),
            Bump::First => OsString::from("First"),
            Bump::Custom(label) => label.into(),
        }
    }
}

use colored::Colorize;

impl Bump {
    pub(crate) fn calculate(route: &Route, conventional: &ConventionalCommits) -> Bump {
        log::debug!(
            "Calculating according to the `{:?}` route: ",
            route.to_string().blue()
        );
        let mut bump = Bump::None;
        // check the conventional commits. No conventional commits; no change.
        if conventional.commits.is_empty() {
            log::warn!("Returning early from calculate as no conventional commits found.");
            return bump;
        };

        log::debug!("Starting calculation with bump level of {bump:?}");
        match route {
            Route::Forced(forced_level) => {
                log::debug!("Forcing the bump level output to `{forced_level}`");
                bump = forced_level.clone().into();
                return bump;
            }
            Route::NonProd => {
                bump = if conventional.breaking {
                    // Breaking change found in commits
                    log::info!("Non production breaking change found.");
                    Bump::Minor
                } else if 0 < *conventional.counts.get("feat").unwrap_or(&0_u32) {
                    log::debug!(
                        "{} feature commit(s) found requiring increment of minor number",
                        conventional.counts.get("feat").unwrap_or(&0_u32).to_owned()
                    );
                    Bump::Minor
                } else {
                    log::debug!(
                        "{} conventional commit(s) found requiring increment of patch number",
                        &conventional.counts.values().sum::<u32>()
                    );
                    Bump::Patch
                };
            }
            Route::PreRelease(pre_type) => {
                log::debug!("Calculating the pre-release version change bump");
                bump = pre_type.clone().into();
            }
            Route::Prod => {
                log::debug!("Calculating the prod version change bump");
                bump = if conventional.breaking {
                    log::debug!("breaking change found");
                    Bump::Major
                } else if 0 < *conventional.counts.get("feat").unwrap_or(&0_u32) {
                    log::debug!(
                        "{} feature commit(s) found requiring increment of minor number",
                        conventional.counts.get("feat").unwrap_or(&0_u32)
                    );
                    Bump::Minor
                } else {
                    log::debug!(
                        "{} conventional commit(s) found requiring increment of patch number",
                        &conventional.counts.values().sum::<u32>()
                    );
                    Bump::Patch
                };
            }
        };
        bump
    }
}

#[cfg(test)]
mod test {
    use map_macro::hash_map;

    use crate::{
        calculator::{ConventionalCommits, Route},
        version::PreReleaseType,
        ForceBump, Hierarchy,
    };

    use super::Bump;
    use rstest::{fixture, rstest};

    #[rstest]
    #[case::none(Bump::None, "none")]
    #[case::patch(Bump::Patch, "patch")]
    #[case::minor(Bump::Minor, "minor")]
    #[case::major(Bump::Major, "major")]
    #[case::release(Bump::Release, "release")]
    #[case::alpha(Bump::Alpha, "alpha")]
    #[case::beta(Bump::Beta, "beta")]
    #[case::rc(Bump::Rc, "rc")]
    #[case::first(Bump::First, "1.0.0")]
    #[case::custom(Bump::Custom(String::from("alpha.1")), "alpha.1")]
    fn display_value(#[case] test: Bump, #[case] expected: &str) {
        assert_eq!(expected, test.to_string().as_str());
    }

    #[rstest]
    #[case::first(ForceBump::First, Bump::First)]
    #[case::major(ForceBump::Major, Bump::Major)]
    #[case::minor(ForceBump::Minor, Bump::Minor)]
    #[case::patch(ForceBump::Patch, Bump::Patch)]
    fn from_forcelevel(#[case] from: ForceBump, #[case] expected: Bump) {
        assert_eq!(expected, from.into());
    }

    #[fixture]
    fn other() -> ConventionalCommits {
        ConventionalCommits {
            commits: vec!["chore: Updated minium rust version references".to_string()],
            counts: hash_map! {"chore".to_string() => 1},
            breaking: false,
            top_type: Hierarchy::Other,
            ..Default::default()
        }
    }

    #[fixture]
    fn fix() -> ConventionalCommits {
        ConventionalCommits {
            commits: vec!["fix: spelling of output in description of set_env".to_string()],
            counts: hash_map! {"fix".to_string() => 1},
            breaking: false,
            top_type: Hierarchy::Fix,
            ..Default::default()
        }
    }

    #[fixture]
    fn feature() -> ConventionalCommits {
        ConventionalCommits {
            commits: vec!["feat: Regex implemented to extract version string".to_string()],
            counts: hash_map! {"feat".to_string() => 1},
            breaking: false,
            top_type: Hierarchy::Feature,
            ..Default::default()
        }
    }

    #[fixture]
    fn breaking() -> ConventionalCommits {
        ConventionalCommits {
            commits: vec!["feat: Regex implemented to extract version string".to_string()],
            counts: hash_map! {"feat".to_string() => 1},
            breaking: true,
            top_type: Hierarchy::Breaking,
            ..Default::default()
        }
    }

    #[rstest]
    fn test_calculate(
        #[values(
            Route::NonProd,
            Route::PreRelease(PreReleaseType::Alpha),
            Route::PreRelease(PreReleaseType::Beta),
            Route::PreRelease(PreReleaseType::Rc),
            Route::PreRelease(PreReleaseType::Custom),
            Route::Prod,
            Route::Forced(ForceBump::Major),
            Route::Forced(ForceBump::Major),
            Route::Forced(ForceBump::Minor),
            Route::Forced(ForceBump::Patch),
            Route::Forced(ForceBump::Alpha),
            Route::Forced(ForceBump::Beta),
            Route::Forced(ForceBump::Rc),
            Route::Forced(ForceBump::Release)
        )]
        route: Route,
        #[values(other(), fix(), feature(), breaking())] conventional: ConventionalCommits,
    ) {
        println!("Route: {route}");
        println!("Conventional: {conventional:?}");
        let test = Bump::calculate(&route, &conventional);

        let expected = match route {
            Route::NonProd => match conventional.top_type {
                crate::Hierarchy::Other => Bump::Patch,
                crate::Hierarchy::Fix => Bump::Patch,
                crate::Hierarchy::Feature => Bump::Minor,
                crate::Hierarchy::Breaking => Bump::Minor,
            },
            Route::PreRelease(pre_type) => match conventional.top_type {
                crate::Hierarchy::Other => pre_type.into(),
                crate::Hierarchy::Fix => pre_type.into(),
                crate::Hierarchy::Feature => pre_type.into(),
                crate::Hierarchy::Breaking => pre_type.into(),
            },
            Route::Prod => match conventional.top_type {
                crate::Hierarchy::Other => Bump::Patch,
                crate::Hierarchy::Fix => Bump::Patch,
                crate::Hierarchy::Feature => Bump::Minor,
                crate::Hierarchy::Breaking => Bump::Major,
            },
            Route::Forced(bump) => match conventional.top_type {
                crate::Hierarchy::Other => bump.into(),
                crate::Hierarchy::Fix => bump.into(),
                crate::Hierarchy::Feature => bump.into(),
                crate::Hierarchy::Breaking => bump.into(),
            },
        };

        assert_eq!(expected, test);
    }
}
