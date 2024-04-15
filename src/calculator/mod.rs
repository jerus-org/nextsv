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
use std::{collections::HashSet, ffi::OsString};

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

use crate::{version::PreReleaseType, Error, VersionTag};
use colored::Colorize;
use git2::Repository;
use log::warn;
use regex::Regex;

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
        let route = Route::new(&current_version.semantic_version);
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
            return Ok(Calculator {
                config,
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

    pub fn report(&self) -> Result<String, Error> {
        Ok(if self.conventional.top_type >= self.config.threshold {
            match (self.config.report_bump, self.config.report_number) {
                (true, true) => format!("{}\n{}", self.bump, self.next_version.version_number()),
                (false, true) => self.next_version.version_number(),
                (true, false) => self.bump().to_string(),
                (false, false) => String::from(""),
            }
        } else {
            log::info!("the minimum level is not met");
            return Err(Error::MinimumChangeLevelNotMet);
        })
    }

    /// Report the change level
    ///
    pub fn next_version_number(&self) -> String {
        if let NextVersion::Updated(version) = &self.next_version {
            version.semantic_version.to_string()
        } else {
            String::from("")
        }
    }

    /// Create a new VersionCalculator struct
    ///
    /// ## Parameters
    ///
    ///  - version_prefix - identifies version tags
    ///
    fn new(version_prefix: &str) -> Result<Calculator, Error> {
        let repo = Repository::open(".")?;
        log::debug!("Repo opened to find latest version tag.");

        // Setup regex to test the tag for a version number: major.minor,patch
        let re_version = format!(r"({}\d+\.\d+\.\d+)", version_prefix);
        log::debug!("Regex to search for version tags is: `{}`.", re_version);
        let re = match Regex::new(&re_version) {
            Ok(r) => r,
            Err(e) => return Err(Error::CorruptVersionRegex(e)),
        };

        let mut versions = vec![];
        repo.tag_foreach(|_id, tag| {
            if let Ok(tag) = String::from_utf8(tag.to_owned()) {
                log::trace!("Is git tag `{tag}` a version tag?");
                if let Some(version) = re.captures(&tag) {
                    log::trace!("Captured version: {:?}", version);
                    let version = VersionTag::parse(&tag, version_prefix).unwrap();
                    versions.push(version);
                }
            }
            true
        })?;

        // trace_items(versions.clone(), version_prefix);
        log::trace!("Original last version: {:?}", versions.last());
        versions.sort();
        log::debug!("Version tags have been sorted");
        // trace_items(versions.clone(), version_prefix);

        let current_version = match versions.last().cloned() {
            Some(v) => {
                log::trace!("latest version found is {:?}", &v);
                v
            }
            None => return Err(Error::NoVersionTag),
        };

        let route = Route::new(&current_version.semantic_version);

        Ok(Calculator {
            current_version,
            route,
            ..Default::default()
        })
    }

    /// Report the current_version
    ///
    // pub fn name(&self) -> VersionTag {
    //     self.current_version.clone()
    // }

    /// Report top level
    ///
    pub fn top_level(&self) -> Hierarchy {
        self.conventional.clone().top_type()
    }

    /// The count of commits of a type in the conventional commits field
    ///
    /// ## Parameters
    ///
    /// - commit_type - identifies the type of commit e.g. "feat"
    ///
    /// ## Error handling
    ///
    /// If there are no conventional commits it returns 0.
    /// If conventional is None returns 0.
    ///
    // pub fn count_commits_by_type(&self, commit_type: &str) -> u32 {
    //     match self.conventional.clone() {
    //         Some(conventional) => conventional
    //             .counts()
    //             .get(commit_type)
    //             .unwrap_or(&0_u32)
    //             .to_owned(),
    //         None => 0_u32,
    //     }
    // }

    /// Report the status of the breaking flag in the conventional commits
    ///
    /// ## Error Handling
    ///
    /// If the conventional is None returns false
    ///
    // pub fn breaking(&self) -> bool {
    //     match self.conventional.clone() {
    //         Some(conventional) => conventional.breaking(),
    //         None => false,
    //     }
    // }

    /// Force update next_version to return a specific result
    ///
    /// Options are defined in `ForceLevel`
    ///
    pub fn set_force(&mut self, level: Option<ForceBump>) -> Self {
        if let Some(level) = level {
            self.route = Route::Forced(level)
        }
        self.clone()
    }

    /// Get the conventional commits created since the tag was created
    ///
    /// Uses `git2` to open the repository and walk back to the
    /// latest version tag collecting the conventional commits.
    ///
    /// ## Error Handling
    ///
    /// Errors from 'git2' are returned.
    ///
    pub fn walk_commits(&mut self) -> Result<(), Error> {
        let repo = git2::Repository::open(".")?;
        log::debug!("repo opened to find conventional commits");
        let mut revwalk = repo.revwalk()?;
        revwalk.set_sorting(git2::Sort::NONE)?;
        revwalk.push_head()?;
        log::debug!("starting the walk from the HEAD");
        let glob = &self.current_version.to_string();
        log::debug!(
            "the glob for revwalk is {glob} based on current version of {:?}",
            self.current_version
        );
        revwalk.hide_ref(glob)?;
        log::debug!("hide commits from {}", &self.current_version);

        macro_rules! filter_try {
            ($e:expr) => {
                match $e {
                    Ok(t) => t,
                    Err(e) => return Some(Err(e)),
                }
            };
        }

        #[allow(clippy::unnecessary_filter_map)]
        let revwalk = revwalk.filter_map(|id| {
            let id = filter_try!(id);
            let commit = repo.find_commit(id);
            let commit = filter_try!(commit);
            Some(Ok(commit))
        });

        let mut conventional_commits = ConventionalCommits::new();

        // Walk back through the commits
        let mut files = HashSet::new();
        for commit in revwalk.flatten() {
            // Get the summary for the conventional commits vec
            log::trace!("commit found: {}", &commit.summary().unwrap_or_default());
            conventional_commits.push(&commit);
            // Get the files for the files vec
            let tree = commit.tree()?;
            let diff = repo.diff_tree_to_workdir(Some(&tree), None).unwrap();

            diff.print(git2::DiffFormat::NameOnly, |delta, _hunk, _line| {
                let file = delta.new_file().path().unwrap().file_name().unwrap();
                log::trace!("file found: {:?}", file);
                files.insert(file.to_os_string());
                true
            })
            .unwrap();
        }

        self.conventional = conventional_commits;

        Ok(())
    }

    /// Calculate the next version and report the version number
    /// and level at which the change is made.
    pub fn calculate(&mut self) {
        log::debug!(
            "Calculating according to the `{:?}` route: ",
            &self.route.to_string().blue()
        );
        // check the conventional commits. No conventional commits; no change.
        #[cfg(let_else)]
        if self.conventional.commits.is_empty() {
            self.bump = Bump::None;
            self.next_version = NextVersion::Updated(self.current_version.clone());
            warn!("Returning early from calculate as no conventional commits found.");
            return;
        };

        let mut bump = Bump::None;
        log::debug!("Starting calculation with bump level of {bump:?}");
        match &self.route {
            Route::Forced(forced_level) => {
                log::debug!("Forcing the bump level output to `{forced_level}`");
                self.bump = forced_level.clone().into();
                self.calculate_next_version();
                return;
            }
            Route::NonProd => {
                bump = if self.conventional.breaking() {
                    // Breaking change found in commits
                    log::debug!("breaking change found");
                    Bump::Minor
                } else if 0 < self.conventional.commits_by_type("feat") {
                    log::debug!(
                        "{} feature commit(s) found requiring increment of minor number",
                        &self.conventional.commits_by_type("feat")
                    );
                    Bump::Minor
                } else {
                    log::debug!(
                        "{} conventional commit(s) found requiring increment of patch number",
                        &self.conventional.commits_all_types()
                    );
                    Bump::Patch
                };

                log::debug!("Calculting the non-prod version change bump");
            }
            Route::PreRelease(pre_type) => {
                bump = match pre_type {
                    PreReleaseType::Alpha => Bump::Alpha,
                    PreReleaseType::Beta => Bump::Beta,
                    PreReleaseType::Rc => Bump::Rc,
                    PreReleaseType::Custom => Bump::Custom(String::new()),
                };
                log::debug!("Calculting the pre-release version change bump");
            }
            Route::Prod => {
                log::debug!("Calculting the prod version change bump");
                bump = if self.conventional.breaking() {
                    log::debug!("breaking change found");
                    Bump::Major
                } else if 0 < self.conventional.commits_by_type("feat") {
                    log::debug!(
                        "{} feature commit(s) found requiring increment of minor number",
                        &self.conventional.commits_by_type("feat")
                    );
                    Bump::Minor
                } else {
                    log::debug!(
                        "{} conventional commit(s) found requiring increment of patch number",
                        &self.conventional.commits_all_types()
                    );
                    Bump::Patch
                };
            }
        };
        self.bump = bump;
        self.calculate_next_version();
    }

    /// Check for required files
    ///
    /// ## Parameters
    ///
    /// - files - a list of the required files or None
    ///
    /// ## Error
    ///
    /// Report error if one of the files are not found.
    /// Exits on the first failure.
    pub fn has_required(
        &self,
        files_required: Vec<OsString>,
        level: Hierarchy,
    ) -> Result<(), Error> {
        // How to use level to ensure that the rule is only applied
        // when required levels of commits are included

        if self.conventional.top_type >= level {
            let files = self.conventional.files.clone();
            if !files.is_empty() {
                let mut missing_files = vec![];

                for file in files_required {
                    if !files.contains(&file) {
                        missing_files.push(file.clone());
                    }
                }

                if !missing_files.is_empty() {
                    return Err(Error::MissingRequiredFile(missing_files));
                }
            } else {
                return Err(Error::NoFilesListed);
            }
        }

        Ok(())
    }

    fn calculate_next_version(&mut self) {
        let mut next_version = self.current_version.clone();
        log::debug!(
            "Starting version: `{}`; bump level `{}`",
            next_version,
            self.bump
        );

        // let mut new_bump = bump.clone();
        let next_version = match &self.bump {
            Bump::Major => {
                next_version.version_mut().major += 1;
                next_version.version_mut().minor = 0;
                next_version.version_mut().patch = 0;
                next_version
            }
            Bump::Minor => {
                next_version.version_mut().minor += 1;
                next_version.version_mut().patch = 0;
                next_version
            }
            Bump::Patch => {
                next_version.version_mut().patch += 1;
                next_version
            }
            Bump::First => {
                next_version.version_mut().major = 1;
                next_version.version_mut().minor = 0;
                next_version.version_mut().patch = 0;
                next_version
            }
            Bump::Alpha | Bump::Beta | Bump::Rc => {
                next_version.version_mut().increment_pre_release();
                next_version
            }
            Bump::Custom(_s) => {
                next_version.version_mut().increment_pre_release();
                self.bump = Bump::Custom(next_version.to_string());
                next_version
            }
            _ => next_version,
        };
        log::debug!("Next version is: {next_version}");

        self.next_version = NextVersion::Updated(next_version);
    }
}

#[cfg(test)]
mod test {
    use log::LevelFilter;
    use log4rs_test_utils::test_logging;
    use rstest::rstest;

    use super::Calculator;
    use crate::calculator::Route;
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
        #[case] expected_level: &str,
        #[case] expected_version: &str,
    ) {
        let current_version = test_utils::gen_current_version("v", 0, 7, 9, None, None);
        let conventional = test_utils::gen_conventional_commit(commit, false);
        let files = test_utils::gen_files();

        let mut calculator = Calculator {
            current_version,
            conventional,
            route: Route::NonProd,
            ..Default::default()
        };

        calculator.calculate();

        assert_eq!(expected_level, calculator.bump.to_string().as_str());
        assert_eq!(expected_version, calculator.next_version_number())
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
        let files = test_utils::gen_files();

        let mut calculator = Calculator {
            current_version,
            conventional,
            route: Route::NonProd,
            ..Default::default()
        };

        calculator.calculate();

        assert_eq!("minor", calculator.bump.to_string().as_str());
        assert_eq!("0.8.0", calculator.next_version_number());
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
        let files = test_utils::gen_files();

        let mut calculator = Calculator {
            current_version,
            conventional,
            route: Route::Prod,
            ..Default::default()
        };

        calculator.calculate();

        assert_eq!(expected_bump, calculator.bump.to_string());
        assert_eq!(expected_version, calculator.next_version_number())
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

        let route = Route::new(&current_version.semantic_version);

        let mut calculator = Calculator {
            current_version,
            conventional,
            route,
            ..Default::default()
        };

        calculator.calculate();

        println!("Version: {:?}", calculator);

        assert_eq!("alpha", calculator.bump.to_string().as_str());
        assert_eq!("0.7.9-alpha.2", calculator.next_version_number())
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

        let mut calculator = Calculator {
            current_version,
            conventional,
            route: Route::Prod,
            ..Default::default()
        };

        calculator.calculate();

        assert_eq!("major", calculator.bump.to_string().as_str());
        assert_eq!("2.0.0", calculator.next_version_number())
    }

    #[test]
    fn promote_to_version_one() {
        test_logging::init_logging_once_for(vec![], LevelFilter::Debug, None);
        let current_version = test_utils::gen_current_version("v", 0, 7, 9, None, None);
        let conventional = test_utils::gen_conventional_commits();

        let mut calculator = Calculator {
            current_version,
            conventional,
            route: Route::Forced(ForceBump::First),
            ..Default::default()
        };

        calculator.calculate();

        assert_eq!("1.0.0", calculator.bump.to_string().as_str());
        assert_eq!("1.0.0", calculator.next_version_number())
    }
}
