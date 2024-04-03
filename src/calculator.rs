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

use crate::{ConventionalCommits, Error, Level, TypeHierarchy, VersionTag};
use git2::Repository;
use log::debug;
use regex::Regex;

use std::{
    collections::HashSet,
    ffi::OsString,
    fmt::{self},
};
/// Struct the store the result of the calculation (the "answer" :) )
///
#[derive(Debug)]
pub struct Answer {
    /// the semantic level bump calcuated based on conventional commits
    pub bump_level: Level,
    /// the next version number calculated by applying the bump level to the
    /// current version number
    pub version_number: VersionTag,
    /// the change level calculated during the review of conventional commits
    pub change_level: Option<TypeHierarchy>,
}

impl Answer {
    /// Create a calculation
    ///
    pub fn new(
        bump_level: Level,
        version_number: VersionTag,
        change_level: Option<TypeHierarchy>,
    ) -> Answer {
        Answer {
            bump_level,
            version_number,
            change_level,
        }
    }
    /// Unwrap the change_level
    ///
    /// ## Error Handling
    ///
    /// If the option is None the lowest level TypeHierarchy will be returned
    ///
    pub fn change_level(&self) -> TypeHierarchy {
        self.change_level.clone().unwrap_or(TypeHierarchy::Other)
    }
}

/// The latest semantic version tag (vx.y.z)
///
pub fn latest(version_prefix: &str) -> Result<VersionTag, Error> {
    let repo = Repository::open(".")?;
    log::debug!("repo opened to find latest");

    let re_version = format!(r"({}\d+\.\d+\.\d+)", version_prefix);

    log::debug!("The version regex is: {}", re_version);

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

    macro_rules! log_items {
        ($versions:ident,$prefix_version:ident) => {
            log::trace!(
                "Tags with semantic version numbers prefixed with `{}`",
                version_prefix
            );
            for ver in &versions {
                log::trace!("\t{}", ver);
            }
        };
    }

    log_items!(versions, prefix_version);
    versions.sort();
    log::debug!("Version tags have been sorted");
    log_items!(versions, prefix_version);

    match versions.last().cloned() {
        Some(v) => {
            log::trace!("latest version found is {:#?}", &v);
            Ok(v)
        }
        None => Err(Error::NoVersionTag),
    }
}

/// The options for choosing the level of a forced change
///
/// The enum is used by the force method to define the level
/// at which the forced change is made.
///
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
pub enum ForceLevel {
    /// force change to the major component of semver
    Major,
    /// force change to the minor component of semver
    Minor,
    /// force change to the patch component of semver
    Patch,
    /// Force update of major version number from 0 to 1
    First,
}

impl fmt::Display for ForceLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ForceLevel::Major => write!(f, "major"),
            ForceLevel::Minor => write!(f, "minor"),
            ForceLevel::Patch => write!(f, "patch"),
            ForceLevel::First => write!(f, "first"),
        }
    }
}

/// VersionCalculator
///
/// Builds up data about the current version to calculate the next version
/// number and change level
///
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VersionCalculator {
    current_version: VersionTag,
    conventional: Option<ConventionalCommits>,
    files: Option<HashSet<OsString>>,
    force_level: Option<ForceLevel>,
}

impl VersionCalculator {
    /// Create a new VersionCalculator struct
    ///
    /// ## Parameters
    ///
    ///  - version_prefix - identifies version tags
    ///
    pub fn new(version_prefix: &str) -> Result<VersionCalculator, Error> {
        let current_version = latest(version_prefix)?;
        Ok(VersionCalculator {
            current_version,
            conventional: None,
            files: None,
            force_level: None,
        })
    }

    /// Report the current_version
    ///
    pub fn name(&self) -> VersionTag {
        self.current_version.clone()
    }

    /// Report top level
    ///
    pub fn top_level(&self) -> Option<TypeHierarchy> {
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
    pub fn count_commits_by_type(&self, commit_type: &str) -> u32 {
        match self.conventional.clone() {
            Some(conventional) => conventional
                .counts()
                .get(commit_type)
                .unwrap_or(&0_u32)
                .to_owned(),
            None => 0_u32,
        }
    }

    /// Report the status of the breaking flag in the conventional commits
    ///
    /// ## Error Handling
    ///
    /// If the conventional is None returns false
    ///
    pub fn breaking(&self) -> bool {
        match self.conventional.clone() {
            Some(conventional) => conventional.breaking(),
            None => false,
        }
    }

    /// Force update next_version to return a specific result
    ///
    /// Options are defined in `ForceLevel`
    ///
    pub fn set_force(&mut self, level: Option<ForceLevel>) -> Self {
        self.force_level = level;
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
    pub fn walk_commits(mut self) -> Result<Self, Error> {
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

        Ok(self)
    }

    /// Calculate the next version and report the version number
    /// and level at which the change is made.
    pub fn next_version(&mut self) -> Answer {
        // check the conventional commits. No conventional commits; no change.
        #[cfg(let_else)]
        let Some(conventional) = self.conventional.clone() else {
            return Answer::new(Level::None, self.current_version.clone(), None);
        };
        #[cfg(not(let_else))]
        let conventional = match self.conventional.clone() {
            Some(c) => c,
            None => return Answer::new(Level::None, self.current_version.clone(), None),
        };

        let final_bump = match &self.force_level {
            None => {
                let bump = if conventional.breaking() {
                    // Breaking change found in commits
                    log::debug!("breaking change found");
                    Level::Major
                } else if 0 < conventional.commits_by_type("feat") {
                    log::debug!(
                        "{} feature commit(s) found requiring increment of minor number",
                        &conventional.commits_by_type("feat")
                    );
                    Level::Minor
                } else if 0 < conventional.commits_all_types() {
                    log::debug!(
                        "{} conventional commit(s) found requiring increment of patch number",
                        &conventional.commits_all_types()
                    );
                    Level::Patch
                } else {
                    Level::None
                };

                if self.current_version.version().major() == 0 {
                    log::info!("Not yet at a stable version");
                    match bump {
                        Level::Major => {
                            let new_bump = Level::Minor;
                            debug!("Shifting right from {} to {}", bump, new_bump);
                            new_bump
                        }
                        Level::Minor => {
                            let new_bump = Level::Patch;
                            debug!("Shifting right from {} to {}", bump, new_bump);
                            new_bump
                        }
                        _ => bump,
                    }
                } else {
                    bump
                }
            }
            Some(forced_level) => forced_level.clone().into(),
        };

        let next_version = next_version_calculator(self.current_version.clone(), &final_bump);

        Answer::new(final_bump, next_version, None)
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
        level: TypeHierarchy,
    ) -> Result<(), Error> {
        // How to use level to ensure that the rule is only applied
        // when required levels of commits are included

        if self
            .conventional
            .as_ref()
            .unwrap()
            .top_type()
            .unwrap_or(TypeHierarchy::Other)
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
}

fn next_version_calculator(mut version: VersionTag, bump: &Level) -> VersionTag {
    let next_version = match *bump {
        Level::Major => {
            version.version_mut().increment_major();
            version
        }
        Level::Minor => {
            version.version_mut().increment_minor();
            version
        }
        Level::Patch => {
            version.version_mut().increment_patch();
            version
        }
        Level::First => {
            version.version_mut().major = 1;
            version.version_mut().minor = 0;
            version.version_mut().patch = 0;
            version
        }
        _ => version,
    };
    log::debug!("Next version is: {next_version}");

    next_version
}

#[cfg(test)]
mod test {
    use std::collections::{HashMap, HashSet};
    use std::ffi::OsString;

    use crate::ForceLevel;
    use crate::TypeHierarchy::Feature;
    use crate::{semantic::Semantic, ConventionalCommits, VersionCalculator, VersionTag};

    fn gen_current_version(
        version_prefix: &str,
        major: u32,
        minor: u32,
        patch: u32,
        pre_release: Option<String>,
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

    #[test]
    fn promote_to_version_one() {
        let current_version = gen_current_version("v", 0, 7, 9, None, None);

        let conventional = gen_conventional_commits();

        let files = gen_files();

        let force_level = Some(ForceLevel::First);

        let mut this_version = VersionCalculator {
            current_version,
            conventional,
            files,
            force_level,
        };

        let new_version = this_version.next_version();
        println!("The promoted version string is: {:?}", new_version);
        let version_string = new_version.version_number.version().to_string();
        println!("The promoted version string is: {:?}", version_string);

        assert_eq!("1.0.0", new_version.bump_level.to_string().as_str());

        let version_number = format!(
            "{}.{}.{}",
            new_version.version_number.semantic_version.major,
            new_version.version_number.semantic_version.minor,
            new_version.version_number.semantic_version.patch
        );

        assert_eq!("1.0.0", version_number)
    }
}
