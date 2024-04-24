mod bump;
mod change_bump;
mod config;
mod conventional;
mod force_bump;
mod hierarchy;
mod next_version;
mod route;

use self::bump::Bump;
pub use self::config::CalculatorConfig;

pub use self::force_bump::ForceBump;
pub(crate) use self::route::Route;
pub(crate) use self::{conventional::ConventionalCommits, next_version::NextVersion};
use crate::version::VersionTag;
use crate::Error;
pub(crate) use change_bump::ChangeBump;
use git2::Repository;
pub use hierarchy::Hierarchy;

/// Used to calculate the bump and next version number.
///
/// Created by calling the [`CalculatorConfig::build`] method.
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Calculator {
    config: CalculatorConfig,
    current_version: VersionTag,
    conventional: ConventionalCommits,
    change_bump: Option<ChangeBump>,
    bump: Bump,
    next_version: NextVersion,
}

impl Calculator {
    /// Collect the data and apply the configuration to calculate the
    /// required outputs.
    ///
    /// This method is typically executed by [`CalculatorConfig::build`].
    pub(crate) fn execute(mut config: CalculatorConfig) -> Result<Self, Error> {
        let repo = Repository::open(".")?;

        let current_version = VersionTag::find_in_repo(&repo, config.prefix.as_str())?;

        let conventional = ConventionalCommits::walk_back_commits_to_tag_reference(
            &repo,
            current_version.to_string().as_str(),
        )?;

        log::debug!("File enforcement level `{}`", config.enforce);
        if conventional.top_type >= config.enforce {
            log::debug!("Enforcing the files: {:?}", config.files);
            if !config.files.is_subset(&conventional.files) {
                let mut missing_files = vec![];
                for file in config.files.clone() {
                    if !&conventional.files.contains(&file) {
                        missing_files.push(file)
                    }
                }
                log::error!("Missing required files: {:?}", missing_files);
                return Ok(Calculator::exit_none(
                    config.clone(),
                    current_version,
                    conventional,
                    None,
                ));
            } else {
                log::debug!("All required files are present");
            };
        }

        let change_bump = Some(ChangeBump::calculate(
            &current_version.semantic_version,
            &conventional,
        ));
        let route = Route::calculate(&current_version.semantic_version);
        let mut bump = Bump::calculate(&route, &conventional);
        log::debug!(
            "Calculated the bump `{}` based on the route `{}`",
            bump,
            route
        );

        // Check the force level and apply if required
        log::debug!("Force level: {:?}", config.force);
        if let Some(ref force_level) = config.force {
            log::trace!("Forcing bump level: `{}`", force_level);
            log::trace!("Current version: `{}`", current_version.semantic_version);
            bump = force_level.to_bump(&current_version.semantic_version);
        };

        // Check the threshold and exit early if it has not been met.
        if config.threshold > conventional.top_type {
            log::info!(
                "The highest level change `{}` does not exceed the threshold `{}`",
                conventional.top_type,
                config.threshold
            );

            return Ok(Calculator::exit_none(
                config,
                current_version,
                conventional,
                change_bump,
            ));
        }

        log::debug!(
            "Calculating the next version number with version `{}`, bump `{}` and change `{:?}`",
            current_version,
            bump,
            change_bump
        );
        let (next_version, bump) =
            NextVersion::calculate(&current_version, bump, change_bump.as_ref());

        if bump == Bump::None {
            config.report_number = false;
        }

        let calculated_result = Ok(Calculator {
            config,
            current_version,
            conventional,
            change_bump,
            bump,
            next_version,
        });

        log::trace!("Calculated result for reporting: {:?}", calculated_result);

        calculated_result
    }

    /// Output the bump level
    pub fn bump(&self) -> String {
        self.bump.clone().to_string()
    }

    /// Output the next version number
    pub fn next_version_number(&self) -> String {
        if let NextVersion::Updated(version) = &self.next_version {
            version.semantic_version.to_string()
        } else {
            String::from("")
        }
    }

    /// Output a string containing the bump and version number as required
    /// by the configuration.
    ///
    /// # Examples
    ///
    /// ## Non production with feature changes
    ///
    /// With a current version number of 0.7.9 and `feat` conventional commits
    /// and the following configuration requiring both bump and next version
    /// number to tbe reported:
    ///     
    /// ```no_run
    /// # use nextsv::CalculatorConfig;
    /// # fn main() -> Result<(), nextsv::Error> {
    ///     let calculator = CalculatorConfig::new()
    ///         .set_prefix("v")
    ///         .set_version_report(true)
    ///         .build()?;
    ///
    ///     calculator.report();
    /// # Ok(())
    /// # }
    /// ```
    /// The output is the following:
    ///
    /// ```console
    /// minor
    /// v0.8.0
    /// ```
    ///
    /// ## Production with breaking change
    ///
    /// With a current production version number and `breaking` conventional commits
    /// and the following configuration requiring only the bump to be reported:
    ///     
    /// ```no_run
    /// # use nextsv::CalculatorConfig;
    /// # fn main() -> Result<(), nextsv::Error> {
    ///     let calculator = CalculatorConfig::new()
    ///         .set_prefix("v")
    ///         .build()?;
    ///
    ///     calculator.report();
    /// # Ok(())
    /// # }
    /// ```
    /// The output is the following:
    ///
    /// ```console
    /// major
    /// ```
    ///
    /// ## Production with feature change reporting both
    ///
    /// With a current production version number(v1.7.9) and `feat`
    /// conventional commits and the following configuration requiring
    /// only the bump and next version number to tbe reported:
    ///
    /// ```no_run
    /// # use nextsv::CalculatorConfig;
    /// # fn main() -> Result<(), nextsv::Error> {
    ///     let calculator = CalculatorConfig::new()
    ///         .set_prefix("v")
    ///         .build()?;
    ///
    ///     calculator.report();
    /// # Ok(())
    /// # }
    /// ```
    /// The output is the following:
    ///
    /// ```console
    /// major
    /// v1.8.0
    /// ```
    ///
    /// ## Pre-release with fix change reporting both
    ///
    /// With a current pre-release version number(v1.0.0-beta.4) and `fix`
    /// conventional commits and the following configuration requiring only
    /// the bump and next version number to tbe reported:
    ///
    /// ```no_run
    /// # use nextsv::CalculatorConfig;
    /// # fn main() -> Result<(), nextsv::Error> {
    ///     let calculator = CalculatorConfig::new()
    ///         .set_prefix("v")
    ///         .build()?;
    ///
    ///     calculator.report();
    /// # Ok(())
    /// # }
    /// ```
    /// The output is the following:
    ///
    /// ```console
    /// beta
    /// v1.0.0-beta.5
    /// ```
    pub fn report(&self) -> String {
        log::debug!("Config for reporting: {:?}", self.config);
        match (self.config.report_bump, self.config.report_number) {
            (true, true) => format!("{}\n{}", self.bump, self.next_version.version_number()),
            (false, true) => self.next_version.version_number(),
            (true, false) => self.bump.to_string(),
            (false, false) => String::from(""),
        }
    }

    pub(crate) fn exit_none(
        mut config: CalculatorConfig,
        current_version: VersionTag,
        conventional: ConventionalCommits,
        change_bump: Option<ChangeBump>,
    ) -> Calculator {
        let bump = Bump::None;
        let next_version = NextVersion::None;
        config.report_bump = true;
        config.report_number = false;

        Calculator {
            config,
            current_version,
            conventional,
            change_bump,
            bump,
            next_version,
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
        let (next_version, bump) = NextVersion::calculate(&current_version, bump, None);

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
        let (next_version, bump) = NextVersion::calculate(&current_version, bump, None);

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
        let (next_version, bump) = NextVersion::calculate(&current_version, bump, None);

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
        let (next_version, bump) = NextVersion::calculate(&current_version, bump, None);

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
        let (next_version, bump) = NextVersion::calculate(&current_version, bump, None);

        assert_eq!("major", bump.to_string().as_str());
        assert_eq!("2.0.0", next_version.version_number());
    }

    #[test]
    fn promote_to_version_one() {
        test_logging::init_logging_once_for(vec![], LevelFilter::Debug, None);
        let current_version = test_utils::gen_current_version("v", 0, 7, 9, None, None);

        let force = ForceBump::First;
        let bump = force.to_bump(&current_version.semantic_version);
        let (next_version, bump) = NextVersion::calculate(&current_version, bump, None);

        assert_eq!("1.0.0", bump.to_string().as_str());
        assert_eq!("1.0.0", next_version.version_number());
    }
}
