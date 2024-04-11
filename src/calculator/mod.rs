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

mod conventional;
mod force_level;
mod level_hierarchy;
mod next_version;
mod route;

pub use self::force_level::ForceLevel;
use self::route::CalcRoute;
use self::{conventional::ConventionalCommits, next_version::NextVersion};
pub use level_hierarchy::LevelHierarchy;

use crate::{version::PreReleaseType, Error, Level, VersionTag};
use colored::Colorize;
use git2::Repository;
use log::warn;
use regex::Regex;

// /// Struct the store the result of the calculation (the "answer" :) )
// ///
// #[derive(Debug)]
// pub struct Answer {
//     /// the semantic level bump calcuated based on conventional commits
//     pub bump_level: Level,
//     /// the next version number calculated by applying the bump level to the
//     /// current version number
//     pub version_number: VersionTag,
//     /// the change level calculated during the review of conventional commits
//     pub change_level: Option<LevelHierarchy>,
// }

// impl Answer {
//     /// Create a calculation
//     ///
//     pub fn new(
//         bump_level: Level,
//         version_number: VersionTag,
//         change_level: Option<LevelHierarchy>,
//     ) -> Answer {
//         Answer {
//             bump_level,
//             version_number,
//             change_level,
//         }
//     }
//     /// Unwrap the change_level
//     ///
//     /// ## Error Handling
//     ///
//     /// If the option is None the lowest level TypeHierarchy will be returned
//     ///
//     pub fn change_level(&self) -> LevelHierarchy {
//         self.change_level.clone().unwrap_or(LevelHierarchy::Other)
//     }
// }

// /// The latest semantic version tag (vx.y.z)
// ///
// pub fn latest(version_prefix: &str) -> Result<VersionTag, Error> {
//     fn trace_items(versions: Vec<VersionTag>, prefix: &str) {
//         log::trace!(
//             "Tags with semantic version numbers prefixed with `{}`",
//             prefix
//         );

//         versions.iter().map(|ver| log::trace!("\t{}", ver));
//     }

//     let repo = Repository::open(".")?;
//     log::debug!("repo opened to find latest");
//     let re_version = format!(r"({}\d+\.\d+\.\d+)", version_prefix);
//     log::debug!("Regex to search for version tags is: {}", re_version);

//     let re = match Regex::new(&re_version) {
//         Ok(r) => r,
//         Err(e) => return Err(Error::CorruptVersionRegex(e)),
//     };

//     let mut versions = vec![];
//     repo.tag_foreach(|_id, tag| {
//         if let Ok(tag) = String::from_utf8(tag.to_owned()) {
//             log::trace!("Is git tag `{tag}` a version tag?");
//             if let Some(version) = re.captures(&tag) {
//                 log::trace!("Captured version: {:?}", version);
//                 let version = VersionTag::parse(&tag, version_prefix).unwrap();

//                 versions.push(version);
//             }
//         }
//         true
//     })?;

//     trace_items(versions.clone(), version_prefix);
//     versions.sort();
//     log::debug!("Version tags have been sorted");
//     trace_items(versions.clone(), version_prefix);

//     match versions.last().cloned() {
//         Some(v) => {
//             log::trace!("latest version found is {:#?}", &v);
//             Ok(v)
//         }
//         None => Err(Error::NoVersionTag),
//     }
// }

/// VersionCalculator
///
/// Builds up data about the current version to calculate the next version
/// number and change level
///
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct VersionCalculator {
    version_prefix: String,
    route: CalcRoute,
    current_version: VersionTag,
    conventional: Option<ConventionalCommits>,
    files: Option<HashSet<OsString>>,
    /// the semantic level bump calcuated based on conventional commits
    bump_level: Level,
    /// the next version number calculated by applying `bump_level` to the
    /// current version number
    next_version: NextVersion,
    /// the change level calculated based on conventional commits
    change_level: Option<LevelHierarchy>,
}

impl VersionCalculator {
    /// Report the bump level
    ///
    pub fn bump_level(&self) -> Level {
        self.bump_level.clone()
    }

    /// Report the change level
    ///
    pub fn change_level(&self) -> Option<LevelHierarchy> {
        self.change_level.clone()
    }

    /// Report the change level
    ///
    pub fn next_version_number(&self) -> String {
        self.next_version.version_number()
    }

    /// Create a new VersionCalculator struct
    ///
    /// ## Parameters
    ///
    ///  - version_prefix - identifies version tags
    ///
    pub fn new(version_prefix: &str) -> Result<VersionCalculator, Error> {
        fn trace_items(versions: Vec<VersionTag>, prefix: &str) {
            log::trace!(
                "Tags with semantic version numbers prefixed with `{}`",
                prefix
            );
            for ver in &versions {
                log::trace!("\t{}", ver);
            }
        }

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

        trace_items(versions.clone(), version_prefix);
        log::trace!("Original last version: {:?}", versions.last());
        versions.sort();
        log::debug!("Version tags have been sorted");
        trace_items(versions.clone(), version_prefix);

        let current_version = match versions.last().cloned() {
            Some(v) => {
                log::trace!("latest version found is {:?}", &v);
                v
            }
            None => return Err(Error::NoVersionTag),
        };

        let route = CalcRoute::new(&current_version.semantic_version);

        Ok(VersionCalculator {
            version_prefix: version_prefix.to_string(),
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
    pub fn top_level(&self) -> Option<LevelHierarchy> {
        if self.conventional.is_none() {
            None
        } else {
            self.conventional.clone().unwrap().top_type()
        }
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
    pub fn set_force(&mut self, level: Option<ForceLevel>) -> Self {
        if let Some(level) = level {
            self.route = CalcRoute::Forced(level)
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

        self.conventional = Some(conventional_commits);
        log::debug!("Files found: {:?}", &files);
        self.files = Some(files);

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
        let Some(conventional) = self.conventional.clone() else {
            self.bump_level = Level::None;
            self.next_version = NextVersion::Updated(self.current_version.clone());
            self.change_level = None;
            warn!("Returning early from calculate as no conventional commits found.");
            return;
        };
        #[cfg(not(let_else))]
        let conventional = match self.conventional.clone() {
            Some(c) => c,
            None => {
                self.bump_level = Level::None;
                self.next_version = NextVersion::Updated(self.current_version.clone());
                self.change_level = None;
                warn!("Returning early from calculate as no conventional commits found.");
                return;
            }
        };

        let mut bump = Level::None;
        log::debug!("Starting calculation with bump level of {bump:?}");
        match &self.route {
            CalcRoute::Forced(forced_level) => {
                log::debug!("Forcing the bump level output to `{forced_level}`");
                self.bump_level = forced_level.clone().into();
                self.change_level = None;
                self.calculate_next_version();
                return;
            }
            CalcRoute::NonProd => {
                bump = if conventional.breaking() {
                    // Breaking change found in commits
                    log::debug!("breaking change found");
                    Level::Minor
                } else if 0 < conventional.commits_by_type("feat") {
                    log::debug!(
                        "{} feature commit(s) found requiring increment of minor number",
                        &conventional.commits_by_type("feat")
                    );
                    Level::Minor
                } else {
                    log::debug!(
                        "{} conventional commit(s) found requiring increment of patch number",
                        &conventional.commits_all_types()
                    );
                    Level::Patch
                };

                log::debug!("Calculting the non-prod version change bump");
            }
            CalcRoute::PreRelease => {
                bump = match self
                    .current_version
                    .semantic_version
                    .pre_release
                    .as_ref()
                    .unwrap()
                    .pre_type
                {
                    PreReleaseType::Alpha => Level::Alpha,
                    PreReleaseType::Beta => Level::Beta,
                    PreReleaseType::Rc => Level::Rc,
                    PreReleaseType::Custom => Level::Custom(String::new()),
                };
                log::debug!("Calculting the pre-release version change bump");
            }
            CalcRoute::Prod => {
                log::debug!("Calculting the prod version change bump");
                bump = if conventional.breaking() {
                    log::debug!("breaking change found");
                    Level::Major
                } else if 0 < conventional.commits_by_type("feat") {
                    log::debug!(
                        "{} feature commit(s) found requiring increment of minor number",
                        &conventional.commits_by_type("feat")
                    );
                    Level::Minor
                } else {
                    log::debug!(
                        "{} conventional commit(s) found requiring increment of patch number",
                        &conventional.commits_all_types()
                    );
                    Level::Patch
                };
            }
        };
        self.bump_level = bump;
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
        level: LevelHierarchy,
    ) -> Result<(), Error> {
        // How to use level to ensure that the rule is only applied
        // when required levels of commits are included

        if self
            .conventional
            .as_ref()
            .unwrap()
            .top_type()
            .unwrap_or(LevelHierarchy::Other)
            >= level
        {
            let files = self.files.clone();
            if let Some(files) = files {
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
            self.bump_level
        );

        // let mut new_bump = bump.clone();
        let next_version = match &self.bump_level {
            Level::Major => {
                next_version.version_mut().major += 1;
                next_version.version_mut().minor = 0;
                next_version.version_mut().patch = 0;
                next_version
            }
            Level::Minor => {
                next_version.version_mut().minor += 1;
                next_version.version_mut().patch = 0;
                next_version
            }
            Level::Patch => {
                next_version.version_mut().patch += 1;
                next_version
            }
            Level::First => {
                next_version.version_mut().major = 1;
                next_version.version_mut().minor = 0;
                next_version.version_mut().patch = 0;
                next_version
            }
            Level::Alpha | Level::Beta | Level::Rc => {
                next_version.version_mut().increment_pre_release();
                next_version
            }
            Level::Custom(_s) => {
                next_version.version_mut().increment_pre_release();
                self.bump_level = Level::Custom(next_version.to_string());
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
    use std::collections::{HashMap, HashSet};
    use std::ffi::OsString;
    use std::fmt;
    use std::str::FromStr;

    use log::LevelFilter;
    use log4rs_test_utils::test_logging;
    use rstest::rstest;

    use crate::calculator::CalcRoute;
    use crate::version::PreRelease;
    use crate::LevelHierarchy::Feature;
    use crate::{version::Semantic, VersionCalculator, VersionTag};
    use crate::{ForceLevel, LevelHierarchy};

    use super::conventional::ConventionalCommits;

    #[derive(Debug)]
    pub(crate) enum ConventionalType {
        Feat,
        Fix,
        Docs,
        Style,
        Refactor,
        Perf,
        Test,
        Build,
        Ci,
        Chore,
        Revert,
    }

    impl fmt::Display for ConventionalType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ConventionalType::Feat => write!(f, "feat"),
                ConventionalType::Fix => write!(f, "fix"),
                ConventionalType::Docs => write!(f, "docs"),
                ConventionalType::Style => write!(f, "style"),
                ConventionalType::Refactor => write!(f, "refactor"),
                ConventionalType::Perf => write!(f, "perf"),
                ConventionalType::Test => write!(f, "test"),
                ConventionalType::Build => write!(f, "build"),
                ConventionalType::Chore => write!(f, "chore"),
                ConventionalType::Ci => write!(f, "ci"),
                ConventionalType::Revert => write!(f, "revert"),
            }
        }
    }

    // Implement the FromStr trait for the Direction enum
    impl FromStr for ConventionalType {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.trim().to_lowercase().as_str() {
                "feat" => Ok(ConventionalType::Feat),
                "fix" => Ok(ConventionalType::Fix),
                "docs" => Ok(ConventionalType::Docs),
                "style" => Ok(ConventionalType::Style),
                "refactor" => Ok(ConventionalType::Refactor),
                "perf" => Ok(ConventionalType::Perf),
                "test" => Ok(ConventionalType::Test),
                "build" => Ok(ConventionalType::Build),
                "chore" => Ok(ConventionalType::Chore),
                "ci" => Ok(ConventionalType::Ci),
                "revert" => Ok(ConventionalType::Revert),
                _ => Err(()),
            }
        }
    }

    fn gen_current_version(
        version_prefix: &str,
        major: u32,
        minor: u32,
        patch: u32,
        pre_release: Option<PreRelease>,
        build_meta_data: Option<String>,
    ) -> VersionTag {
        VersionTag {
            refs: "refs/tags/".to_string(),
            tag_prefix: "".to_string(),
            version_prefix: version_prefix.to_string(),
            semantic_version: Semantic {
                major,
                minor,
                patch,
                pre_release,
                build_meta_data,
            },
        }
    }

    fn gen_conventional_commits() -> Option<ConventionalCommits> {
        let mut counts = HashMap::new();
        let values = [
            ("chore".to_string(), 1),
            ("docs".to_string(), 1),
            ("feat".to_string(), 1),
            ("refactor".to_string(), 1),
            ("ci".to_string(), 1),
            ("test".to_string(), 1),
            ("fix".to_string(), 1),
        ];

        for val in values {
            counts.insert(val.0, val.1);
        }

        Some(
            ConventionalCommits {
                commits: vec!(
                    "fix: spelling of output in description of set_env".to_string(),
                    "Merge branch 'main' of github.com:jerusdp/nextsv into fix/version-level-assessment".to_string(),
                    "test: Ensure all current tests are passing".to_string(),
                    "refactor: implemented VersionTag".to_string(),
                    "feat: Regex implemented to extract version string".to_string(),
                    "chore: Updated minium rust version references".to_string(),
                    "ci: Updated Minimum rust version to 1.74".to_string(),
                    "docs: Updated tests in docs.".to_string()
                ),
                counts,
                breaking: false,
                top_type: Some(
                    Feature,
                ),
            },
        )
    }

    fn gen_conventional_commit(
        commit_type: ConventionalType,
        breaking: bool,
    ) -> Option<ConventionalCommits> {
        let mut counts = HashMap::new();
        counts.insert(format!("{}", commit_type), 1);

        let commits = vec![format!(
            "{}{} commit for testing purposes only",
            if breaking { "!" } else { "" },
            breaking
        )];

        let top_type = Some(LevelHierarchy::parse(&commit_type.to_string()).unwrap());

        Some(ConventionalCommits {
            commits,
            counts,
            breaking,
            top_type,
        })
    }

    fn gen_files() -> Option<HashSet<OsString>> {
        let file_list = [
            "calculator.rs",
            "help.trycmd",
            "CHANGELOG.md",
            "Cargo.toml",
            "config.yml",
            "error.rs",
            "README.md",
        ];
        let mut files = HashSet::new();

        for file in file_list {
            files.insert(OsString::from(file));
        }

        Some(files)
    }

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
        let current_version = gen_current_version("v", 0, 7, 9, None, None);
        let conventional = gen_conventional_commit(commit, false);
        let files = gen_files();

        let mut calculator = VersionCalculator {
            current_version,
            conventional,
            files,
            route: CalcRoute::NonProd,
            ..Default::default()
        };

        calculator.calculate();

        assert_eq!(expected_level, calculator.bump_level.to_string().as_str());
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
        let current_version = gen_current_version("v", 0, 7, 9, None, None);
        let conventional = gen_conventional_commit(commit, true);
        let files = gen_files();

        let mut calculator = VersionCalculator {
            current_version,
            conventional,
            files,
            route: CalcRoute::NonProd,
            ..Default::default()
        };

        calculator.calculate();

        assert_eq!("minor", calculator.bump_level.to_string().as_str());
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
        let current_version = gen_current_version("v", 1, 7, 9, None, None);
        let conventional = gen_conventional_commit(commit, false);
        let files = gen_files();

        let mut calculator = VersionCalculator {
            current_version,
            conventional,
            files,
            route: CalcRoute::Prod,
            ..Default::default()
        };

        calculator.calculate();

        assert_eq!(expected_bump, calculator.bump_level.to_string());
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
            gen_current_version("v", 0, 7, 9, Some(PreRelease::new("alpha.1")), None);
        let conventional = gen_conventional_commit(commit, false);
        let files = gen_files();

        let route = CalcRoute::new(&current_version.semantic_version);

        let mut calculator = VersionCalculator {
            current_version,
            conventional,
            files,
            route,
            ..Default::default()
        };

        calculator.calculate();

        println!("Version: {:?}", calculator);

        assert_eq!("alpha", calculator.bump_level.to_string().as_str());
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
        let current_version = gen_current_version("v", 1, 7, 9, None, None);
        let conventional = gen_conventional_commit(commit, true);
        let files = gen_files();

        let mut calculator = VersionCalculator {
            current_version,
            conventional,
            files,
            route: CalcRoute::Prod,
            ..Default::default()
        };

        calculator.calculate();

        assert_eq!("major", calculator.bump_level.to_string().as_str());
        assert_eq!("2.0.0", calculator.next_version_number())
    }

    #[test]
    fn promote_to_version_one() {
        test_logging::init_logging_once_for(vec![], LevelFilter::Debug, None);
        let current_version = gen_current_version("v", 0, 7, 9, None, None);
        let conventional = gen_conventional_commits();
        let files = gen_files();

        let mut calculator = VersionCalculator {
            current_version,
            conventional,
            files,
            route: CalcRoute::Forced(ForceLevel::First),
            ..Default::default()
        };

        calculator.calculate();

        assert_eq!("1.0.0", calculator.bump_level.to_string().as_str());
        assert_eq!("1.0.0", calculator.next_version_number())
    }
}
