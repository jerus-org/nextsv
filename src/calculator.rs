//! A semantic tag
//!
//! ## Example
//!
//!
//! ## Panics
//!
//!

use crate::{ConventionalCommits, Error, Level, Semantic, TypeHierarchy};
use git2::Repository;
use log::debug;
use regex::Regex;

use std::{collections::HashSet, ffi::OsString, fmt};
/// Struct the store the result of the calculation (the "answer" :) )
///
#[derive(Debug)]
pub struct Answer {
    /// the semantic level bump calcuated based on conventional commits
    pub bump_level: Level,
    /// the next version number calculated by applying the bump level to the
    /// current version number
    pub version_number: Semantic,
    /// the change level calculated during the review of conventional commits
    pub change_level: Option<TypeHierarchy>,
}

impl Answer {
    /// Create a calculation
    ///
    pub fn new(
        bump_level: Level,
        version_number: Semantic,
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
pub fn latest(version_prefix: &str) -> Result<Semantic, Error> {
    let repo = Repository::open(".")?;
    log::debug!("repo opened to find latest");

    let re_version = format!(r"({}\d+\.\d+\.\d+)", version_prefix);

    println!("the version regex is: {}", re_version);

    let re = Regex::new(&re_version).unwrap();

    let mut versions = vec![];
    repo.tag_foreach(|_id, tag| {
        if let Ok(tag) = String::from_utf8(tag.to_owned()) {
            // println!("Found tag called {name:?}");
            // let caps = re.captures(&name);
            // println!("regex captured {:#?}", caps);
            if let Some(version) = re.captures(&tag)
            // name.strip_prefix("refs/tags/")
            {
                println!("Captured version: {:#?}", version);
                let version = Semantic::parse(&tag, version_prefix).unwrap();

                versions.push(version);
            }
        }
        true
    })?;

    println!("versions: {versions:#?}");

    versions.sort();
    log::debug!("versions sorted");

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
#[derive(Debug)]
pub enum ForceLevel {
    /// force change to the major component of semver
    Major,
    /// force change to the minor component of semver
    Minor,
    /// force change to the patch component of semver
    Patch,
}

impl fmt::Display for ForceLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ForceLevel::Major => write!(f, "major"),
            ForceLevel::Minor => write!(f, "minor"),
            ForceLevel::Patch => write!(f, "patch"),
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
    current_version: Semantic,
    conventional: Option<ConventionalCommits>,
    files: Option<HashSet<OsString>>,
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
        })
    }

    /// Report the current_version
    ///
    pub fn name(&self) -> Semantic {
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
    pub fn force(&mut self, level: ForceLevel) -> Self {
        let mut conventional_commits = ConventionalCommits::new();
        log::debug!("forcing a change to {}", level);
        match level {
            ForceLevel::Major => {
                conventional_commits.set_breaking(true);
            }
            ForceLevel::Minor => {
                conventional_commits.increment_counts(git_conventional::Type::FEAT);
            }
            ForceLevel::Patch => {
                conventional_commits.increment_counts(git_conventional::Type::FIX);
            }
        }

        self.conventional = Some(conventional_commits);
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
        let glob = &self.current_version.tag();
        println!("the glob for revwalk is {glob}");
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
        log::debug!("Files found: {:#?}", &files);
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

        let final_bump = if self.current_version.major() == 0 {
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
        };
        let next_version = next_version_calculator(self.current_version.clone(), &final_bump);

        Answer::new(final_bump, next_version, None)
    }

    /// Report version 1.0.0 and update level major
    ///
    /// ## Error
    ///
    /// Report error if major version number is greater than 0
    pub fn promote_first(&mut self) -> Result<Answer, Error> {
        if 0 < self.current_version.major() {
            Err(Error::MajorAlreadyUsed(
                self.current_version.major().to_string(),
            ))
        } else {
            Ok(self.force(ForceLevel::Major).next_version())
        }
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

fn next_version_calculator(mut version: Semantic, bump: &Level) -> Semantic {
    match *bump {
        Level::Major => version.increment_major().clone(),
        Level::Minor => version.increment_minor().clone(),
        Level::Patch => version.increment_patch().clone(),
        _ => version,
    }
}
