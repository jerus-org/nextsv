//! A semantic tag
//!
//! ## Example
//!
//!
//! ## Panics
//!
//!
//!
//!

mod bump;
mod config;
mod conventional;
mod force_level;
mod hierarchy;
mod next_version;
mod route;

use self::bump::Bump;
pub use self::config::CalculatorConfig;

pub use self::force_level::ForceBump;
pub(crate) use self::route::Route;
pub(crate) use self::{conventional::ConventionalCommits, next_version::NextVersion};
pub use hierarchy::Hierarchy;

use crate::{Error, VersionTag};
use git2::Repository;

/// VersionCalculator
///
/// Builds up data about the current version to calculate the next version
/// number and change level
///
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Calculator {
    config: CalculatorConfig,
    current_version: VersionTag,
    route: Route,
    conventional: ConventionalCommits,
    bump: Bump,
    next_version: NextVersion,
    // change_level: Option<Hierarchy>,
}

impl Calculator {
    /// Initialize the version calculator
    ///
    pub(crate) fn init(config: CalculatorConfig) -> Result<Self, Error> {
        let repo = Repository::open(".")?;

        let current_version = VersionTag::find_in_repo(&repo, config.prefix.as_str())?;
        let route = if let Some(ref force_level) = config.force {
            Route::Forced(force_level.clone())
        } else {
            Route::calculate(&current_version.semantic_version)
        };

        let conventional = ConventionalCommits::walk_back_commits_to_tag_reference(
            &repo,
            current_version.to_string().as_str(),
        )?;
        // Check the threshold and exit early if it has not been met.
        if config.threshold > conventional.top_type {
            log::info!(
                "The highest level change `{}` does not exceed the threshold `{}`",
                conventional.top_type,
                config.threshold
            );
            let bump = Bump::None;
            let next_version = NextVersion::None;
            let mut new_config = config;
            new_config.report_bump = true;
            new_config.report_number = false;
            return Ok(Calculator {
                config: new_config,
                current_version,
                route,
                conventional,
                bump,
                next_version,
            });
        }

        let bump = Bump::calculate(&route, &conventional);
        let (next_version, bump) = NextVersion::calculate(&current_version, bump);

        Ok(Calculator {
            config,
            current_version,
            route,
            conventional,
            bump,
            next_version,
        })
    }

    /// Report the bump level
    ///
    pub fn bump(&self) -> Bump {
        self.bump.clone()
    }

    /// ### Report the results of the calculation
    ///
    /// Depending on the config settings for bump and version number report either or both.
    /// If both are reported they are reported on two lines.
    ///
    pub fn report(&self) -> String {
        match (self.config.report_bump, self.config.report_number) {
            (true, true) => format!("{}\n{}", self.bump, self.next_version.version_number()),
            (false, true) => self.next_version.version_number(),
            (true, false) => self.bump().to_string(),
            (false, false) => String::from(""),
        }
    }

    /// ### Report the next version number
    ///
    pub fn next_version_number(&self) -> String {
        if let NextVersion::Updated(version) = &self.next_version {
            version.semantic_version.to_string()
        } else {
            String::from("")
        }
    }
}

#[cfg(test)]
mod test {
    use log::LevelFilter;
    use log4rs_test_utils::test_logging;
    use rstest::rstest;

    use crate::calculator::{bump::Bump, NextVersion, Route};
    use crate::test_utils;
    use crate::test_utils::*;
    use crate::version::PreRelease;
    use crate::ForceBump;

    #[rstest]
    #[case::feature("feat", "minor", "0.8.0")]
    #[case::fix("fix", "patch", "0.7.10")]
    #[case::docs("docs", "patch", "0.7.10")]
    #[case::style("style", "patch", "0.7.10")]
    #[case::refactor("refactor", "patch", "0.7.10")]
    #[case::perf("perf", "patch", "0.7.10")]
    #[case::test("test", "patch", "0.7.10")]
    #[case::build("build", "patch", "0.7.10")]
    #[case::chore("chore", "patch", "0.7.10")]
    #[case::ci("ci", "patch", "0.7.10")]
    #[case::revert("revert", "patch", "0.7.10")]
    fn bump_result_for_nonprod_current_version_and_nonbreaking(
        #[case] commit: ConventionalType,
        #[case] expected_bump: &str,
        #[case] expected_version: &str,
    ) {
        let current_version = test_utils::gen_current_version("v", 0, 7, 9, None, None);
        let conventional = test_utils::gen_conventional_commit(commit, false);

        let bump = Bump::calculate(&Route::NonProd, &conventional);
        let (next_version, bump) = NextVersion::calculate(&current_version, bump);

        assert_eq!(expected_bump, bump.to_string().as_str());
        assert_eq!(expected_version, next_version.version_number());
    }

    #[rstest]
    #[case::feature("feat")]
    #[case::fix("fix")]
    #[case::docs("docs")]
    #[case::style("style")]
    #[case::refactor("refactor")]
    #[case::perf("perf")]
    #[case::test("test")]
    #[case::build("build")]
    #[case::chore("chore")]
    #[case::ci("ci")]
    #[case::revert("revert")]
    // #[trace]
    fn bump_result_for_nonprod_current_version_and_breaking(#[case] commit: ConventionalType) {
        let current_version = test_utils::gen_current_version("v", 0, 7, 9, None, None);
        let conventional = test_utils::gen_conventional_commit(commit, true);

        let bump = Bump::calculate(&Route::NonProd, &conventional);
        let (next_version, bump) = NextVersion::calculate(&current_version, bump);

        assert_eq!("minor", bump.to_string().as_str());
        assert_eq!("0.8.0", next_version.version_number());
    }

    #[rstest]
    #[case::feat("feat", "minor", "1.8.0")]
    #[case::fix("fix", "patch", "1.7.10")]
    #[case::docs("docs", "patch", "1.7.10")]
    #[case::style("style", "patch", "1.7.10")]
    #[case::refactor("refactor", "patch", "1.7.10")]
    #[case::perf("perf", "patch", "1.7.10")]
    #[case::test("test", "patch", "1.7.10")]
    #[case::build("build", "patch", "1.7.10")]
    #[case::chore("chore", "patch", "1.7.10")]
    #[case::ci("ci", "patch", "1.7.10")]
    #[case::revert("revert", "patch", "1.7.10")]
    fn bump_result_for_prod_current_version_and_nonbreaking(
        #[case] commit: ConventionalType,
        #[case] expected_bump: &str,
        #[case] expected_version: &str,
    ) {
        let current_version = test_utils::gen_current_version("v", 1, 7, 9, None, None);
        let conventional = test_utils::gen_conventional_commit(commit, false);

        let bump = Bump::calculate(&Route::Prod, &conventional);
        let (next_version, bump) = NextVersion::calculate(&current_version, bump);

        assert_eq!(expected_bump, bump.to_string().as_str());
        assert_eq!(expected_version, next_version.version_number());
    }

    #[rstest]
    // #[trace]
    fn bump_result_for_nonprod_current_version_and_nonbreaking_with_prerelease(
        #[values(
            "feat", "fix", "docs", "style", "refactor", "perf", "test", "build", "chore", "ci",
            "revert"
        )]
        commit: ConventionalType,
    ) {
        test_logging::init_logging_once_for(vec![], LevelFilter::Debug, None);
        let current_version =
            test_utils::gen_current_version("v", 0, 7, 9, Some(PreRelease::new("alpha.1")), None);
        let conventional = test_utils::gen_conventional_commit(commit, false);

        let route = Route::calculate(&current_version.semantic_version);

        let bump = Bump::calculate(&route, &conventional);
        let (next_version, bump) = NextVersion::calculate(&current_version, bump);

        assert_eq!("alpha", bump.to_string().as_str());
        assert_eq!("0.7.9-alpha.2", next_version.version_number());
    }

    #[rstest]
    // #[trace]
    fn bump_result_for_prod_current_version_and_breaking(
        #[values(
            "feat", "fix", "docs", "style", "refactor", "perf", "test", "build", "chore", "ci",
            "revert"
        )]
        commit: ConventionalType,
    ) {
        test_logging::init_logging_once_for(vec![], LevelFilter::Debug, None);
        let current_version = test_utils::gen_current_version("v", 1, 7, 9, None, None);
        let conventional = test_utils::gen_conventional_commit(commit, true);

        let bump = Bump::calculate(&Route::Prod, &conventional);
        let (next_version, bump) = NextVersion::calculate(&current_version, bump);

        assert_eq!("major", bump.to_string().as_str());
        assert_eq!("2.0.0", next_version.version_number());
    }

    #[test]
    fn promote_to_version_one() {
        test_logging::init_logging_once_for(vec![], LevelFilter::Debug, None);
        let current_version = test_utils::gen_current_version("v", 0, 7, 9, None, None);
        let conventional = test_utils::gen_conventional_commits();

        let bump = Bump::calculate(&Route::Forced(ForceBump::First), &conventional);
        let (next_version, bump) = NextVersion::calculate(&current_version, bump);

        assert_eq!("1.0.0", bump.to_string().as_str());
        assert_eq!("1.0.0", next_version.version_number());
    }
}
