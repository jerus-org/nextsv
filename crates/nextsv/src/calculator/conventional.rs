//! Represents a vector of conventional commits
//!

use std::{
    collections::{HashMap, HashSet},
    ffi::OsString,
    path::Path,
};

use git2::{Repository, TreeWalkMode, TreeWalkResult};

use super::commit::Commit;

use crate::{Error, Workspace};

use super::{Hierarchy, TopType};

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub(crate) struct ConventionalCommits {
    pub(crate) commits: Vec<String>,
    pub(crate) counts: HashMap<String, u32>,
    pub(crate) breaking: bool,
    pub(crate) top_type: TopType,
    pub(crate) changed_files: HashSet<OsString>,
    pub(crate) all_files: HashSet<OsString>,
}

impl ConventionalCommits {
    pub(crate) fn new() -> ConventionalCommits {
        ConventionalCommits::default()
    }

    pub(crate) fn walk_back_commits_to_tag_reference(
        repo: &Repository,
        reference: &str,
        subdir: Option<&str>,
        package: Option<&str>,
    ) -> Result<Self, Error> {
        let subdir = get_subdir_for_package(package, subdir);

        log::debug!("repo opened to find conventional commits");
        log::debug!("Searching for the tag: `{}`", reference);
        let tag_commit = match repo.find_reference(reference) {
            Ok(reference) => match reference.peel_to_commit() {
                Ok(commit) => commit,
                Err(e) => {
                    log::error!("Error finding the tag commit: {:?}", e);
                    return Err(Error::Git2(e));
                }
            },
            Err(e) => {
                log::error!("Error finding the tag reference: {:?}", e);
                return Err(Error::Git2(e));
            }
        };
        let tag_tree = match tag_commit.tree() {
            Ok(tree) => tree,
            Err(e) => {
                log::error!("Error finding the tag tree: {:?}", e);
                return Err(Error::Git2(e));
            }
        };
        log::debug!("tag tree found: {:?}", tag_tree);

        let mut revwalk = repo.revwalk()?;
        revwalk.set_sorting(git2::Sort::NONE)?;
        revwalk.push_head()?;
        log::debug!("starting the walk from the HEAD");
        log::debug!("the reference to walk back to is: `{reference}`");
        revwalk.hide_ref(reference)?;

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
        let mut file_names = HashSet::new();

        let mut tree_flag = true;
        // Walk back through the commits to collect the commit summary and identify conventional commits
        for commit in revwalk.flatten() {
            let cmt = Commit::new(commit.clone(), repo);
            let summary = cmt.message();
            log::debug!("commit found: `{}`", summary);

            if summary.starts_with("Merge") {
                log::debug!("Exiting loop as Merge commit found");
                continue;
            }

            let files = cmt.files();
            log::debug!("files found: `{:#?}`", files);

            if let Some(subdir) = &subdir {
                log::debug!("subdir: `{}`", subdir);
                let qualified_files: Vec<_> = files
                    .iter()
                    .filter(|file| file.to_str().unwrap().contains(subdir))
                    .collect();
                if qualified_files.is_empty() {
                    log::debug!("Exiting loop because `{}` not found", subdir);
                    continue;
                }
            }

            conventional_commits.push(&commit);

            for path in files {
                if let Some(os_string) = path.file_name() {
                    file_names.insert(OsString::from(os_string));
                }
            }

            if tree_flag {
                let tree = commit.tree().unwrap();
                let mut all_files = HashSet::new();
                tree.walk(TreeWalkMode::PreOrder, |_, entry| {
                    if let Some(os_string) = entry.name() {
                        all_files.insert(OsString::from(os_string));
                    }
                    TreeWalkResult::Ok
                })?;
                conventional_commits.all_files = all_files;
                // let mut diff_options = DiffOptions::new();
                // let diff =
                //     repo.diff_tree_to_tree(Some(&tag_tree), Some(&tree), Some(&mut diff_options))?;
                // let mut files = HashSet::new();
                // diff.print(git2::DiffFormat::NameOnly, |delta, _hunk, _line| {
                //     let file = delta.new_file().path().unwrap().file_name().unwrap();
                //     log::debug!("file found: {:?}", file);
                //     files.insert(file.to_os_string());
                //     true
                // })?;
                tree_flag = false;
            }
        }
        conventional_commits.changed_files = file_names;
        log::debug!("conventional commits found: {:#?}", conventional_commits);

        Ok(conventional_commits)
    }

    pub(crate) fn push(&mut self, commit: &git2::Commit) -> &Self {
        if commit.summary().unwrap_or("No") != "No" {
            if let Ok(conventional) = git_conventional::Commit::parse(
                commit.summary().unwrap_or("NotConventional"),
            ) {
                log::trace!(
                    "Commit: ({}) {} {}",
                    conventional.type_(),
                    conventional.description(),
                    Hierarchy::parse(&conventional.type_().to_string()).unwrap_or(Hierarchy::Other),
                );
                let counter = self
                    .counts
                    .entry(conventional.type_().to_string())
                    .or_insert(0);
                *counter += 1;

                if !self.breaking {
                    log::trace!("Not broken yet!");
                    if conventional.breaking() {
                        log::trace!("Breaking change found!");
                        self.breaking = conventional.breaking();
                        self.top_type = TopType::Breaking;
                    } else if TopType::parse(conventional.type_().as_str()).unwrap() > self.top_type
                    {
                        self.top_type = TopType::parse(conventional.type_().as_str()).unwrap();
                        log::trace!("New top type found {}!", self.top_type);
                    };
                }
            }
            self.commits.push(
                commit
                    .summary()
                    .unwrap_or("NotConventional")
                    .to_string(),
            );
        }
        self
    }
}

fn get_subdir_for_package(package: Option<&str>, subdir: Option<&str>) -> Option<String> {
    if package.is_none() {
        if let Some(subdir) = subdir {
            let s = subdir.to_string();
            return Some(s);
        } else {
            return None;
        }
    };

    let rel_package = package.unwrap();
    log::info!("Running release for package: {}", rel_package);

    let path = Path::new("./Cargo.toml");
    let workspace = Workspace::new(path).unwrap();

    let packages = workspace.packages();

    if let Some(packages) = packages {
        for package in packages {
            log::debug!("Found workspace package: {}", package.name);
            if package.name != rel_package {
                continue;
            }
            return Some(package.member);
        }
    };
    None
}
#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::Hierarchy;

    #[test]
    fn type_hierarchy_ordering() {
        let tests = [
            (Hierarchy::Breaking, Hierarchy::Breaking, Ordering::Equal),
            (Hierarchy::Feature, Hierarchy::Feature, Ordering::Equal),
            (Hierarchy::Fix, Hierarchy::Fix, Ordering::Equal),
            (Hierarchy::Other, Hierarchy::Other, Ordering::Equal),
            (Hierarchy::Breaking, Hierarchy::Feature, Ordering::Greater),
            (Hierarchy::Breaking, Hierarchy::Fix, Ordering::Greater),
            (Hierarchy::Breaking, Hierarchy::Other, Ordering::Greater),
            (Hierarchy::Feature, Hierarchy::Breaking, Ordering::Less),
            (Hierarchy::Feature, Hierarchy::Fix, Ordering::Greater),
            (Hierarchy::Feature, Hierarchy::Other, Ordering::Greater),
            (Hierarchy::Fix, Hierarchy::Breaking, Ordering::Less),
            (Hierarchy::Fix, Hierarchy::Feature, Ordering::Less),
            (Hierarchy::Fix, Hierarchy::Other, Ordering::Greater),
            (Hierarchy::Other, Hierarchy::Breaking, Ordering::Less),
            (Hierarchy::Other, Hierarchy::Feature, Ordering::Less),
            (Hierarchy::Other, Hierarchy::Fix, Ordering::Less),
        ];

        for test in tests {
            println!("Test case: {test:#?}");
            let lhs = test.0;
            assert_eq!(test.2, lhs.cmp(&test.1));
        }
    }
}
