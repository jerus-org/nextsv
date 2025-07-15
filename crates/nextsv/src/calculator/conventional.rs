//! Represents a vector of conventional commits
//!

mod cmt_summary;
use cmt_summary::CmtSummary;

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
        log::debug!("Searching for the tag: `{reference}`");
        let tag_commit = match repo.find_reference(reference) {
            Ok(reference) => match reference.peel_to_commit() {
                Ok(commit) => commit,
                Err(e) => {
                    log::error!("Error finding the tag commit: {e:?}");
                    return Err(Error::Git2(e));
                }
            },
            Err(e) => {
                log::error!("Error finding the tag reference: {e:?}");
                return Err(Error::Git2(e));
            }
        };
        let tag_tree = match tag_commit.tree() {
            Ok(tree) => tree,
            Err(e) => {
                log::error!("Error finding the tag tree: {e:?}");
                return Err(Error::Git2(e));
            }
        };
        log::debug!("tag tree found: {tag_tree:?}");

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
            log::debug!("commit found: `{summary}`");

            if summary.starts_with("Merge") {
                log::debug!("Exiting loop as Merge commit found");
                continue;
            }

            let files = cmt.files();
            log::debug!("files found: `{files:#?}`");

            if let Some(subdir) = &subdir {
                log::debug!("subdir: `{subdir}`");
                let dup_files = files.clone();
                let root_files: Vec<_> = dup_files
                    .iter()
                    .filter(|file| !file.to_str().unwrap().contains("/"))
                    .collect();
                log::debug!("root files: `{root_files:#?}`");
                let mut qualified_files: Vec<_> = files
                    .iter()
                    .filter(|file| file.to_str().unwrap().contains(subdir))
                    .collect();
                log::debug!("qualified files: `{qualified_files:#?}`");
                log::info!("Checking for root directory files changed in addition to {subdir}");
                qualified_files.extend_from_slice(&root_files);

                if qualified_files.is_empty() {
                    log::debug!("Exiting loop because `{subdir}` not found");
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
        log::debug!("conventional commits found: {conventional_commits:#?}");

        Ok(conventional_commits)
    }

    pub(crate) fn push(&mut self, commit: &git2::Commit) -> &Self {
        if commit.summary().unwrap_or("No") != "No" {
            let summary = commit.summary().unwrap();
            self.update_from_summary(summary);
        }
        self.commits
            .push(commit.summary().unwrap_or("NotConventional").to_string());
        self
    }

    fn update_from_summary(&mut self, summary: &str) -> &Self {
        let cmt_summary = CmtSummary::parse(summary).unwrap();
        let commit_type = cmt_summary.type_string();

        log::trace!(
            "Commit: ({}) {} {}",
            &commit_type,
            cmt_summary.title,
            Hierarchy::parse(&cmt_summary.type_.unwrap_or("".to_string()))
                .unwrap_or(Hierarchy::Other),
        );
        let counter = self.counts.entry(commit_type.clone()).or_insert(0);
        *counter += 1;

        if !self.breaking {
            log::trace!("Not broken yet!");
            if cmt_summary.breaking {
                log::trace!("Breaking change found!");
                self.breaking = cmt_summary.breaking;
                self.top_type = TopType::Breaking;
            } else if TopType::parse(&commit_type).unwrap() > self.top_type {
                self.top_type = TopType::parse(&commit_type).unwrap();
                log::trace!("New top type found {}!", self.top_type);
            };
        };
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
    log::info!("Running release for package: {rel_package}");

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

    use log::LevelFilter;
    use rstest::rstest;

    use crate::{calculator::TopType, Hierarchy};

    fn get_test_logger() {
        let mut builder = env_logger::Builder::new();
        builder.filter(None, LevelFilter::Trace);
        builder.format_timestamp_secs().format_module_path(false);
        let _ = builder.try_init();
    }

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

    #[rstest]
    #[case::feat_other_feat("feat: add new feature", TopType::Other, TopType::Feature)]
    #[case::emoji_feat_other_feat("✨ feat: add new feature", TopType::Other, TopType::Feature)]
    #[case::fix_other_fix("fix: fix an existing feature", TopType::Other, TopType::Fix)]
    #[case::emoji_fix_other_fix("🐛 fix: fix an existing feature", TopType::Other, TopType::Fix)]
    #[case::style_other_other("style: fix typo and lint issues", TopType::Other, TopType::Other)]
    #[case::emoji_style_other_other(
        "💄 style: fix typo and lint issues",
        TopType::Other,
        TopType::Other
    )]
    #[case::test_other_other("test: update tests", TopType::Other, TopType::Other)]
    #[case::sec_other_fix(
        "fix(security): Fix security vulnerability",
        TopType::Other,
        TopType::Fix
    )]
    #[case::chore_other_other("chore(deps): Update dependencies", TopType::Other, TopType::Other)]
    #[case::emoji_chore_other_other(
        "🔧 chore(deps): Update dependencies",
        TopType::Other,
        TopType::Other
    )]
    #[case::refactor_other_other(
        "refactor(remove): Remove unused code",
        TopType::Other,
        TopType::Other
    )]
    #[case::emoji_refactor_other_other(
        "♻️ refactor(remove): Remove unused code",
        TopType::Other,
        TopType::Other
    )]
    #[case::docs_other_other("docs(deprecate): Deprecate old API", TopType::Other, TopType::Other)]
    #[case::emoji_docs_other_other(
        "📚 docs(deprecate): Deprecate old API",
        TopType::Other,
        TopType::Other
    )]
    #[case::ci_other_other(
        "ci(other-scope): Update CI configuration",
        TopType::Other,
        TopType::Other
    )]
    #[case::emoji_ci_other_other(
        "👷 ci(other-scope): Update CI configuration",
        TopType::Other,
        TopType::Other
    )]
    #[case::test_other_breaking("test!: Update test cases", TopType::Other, TopType::Breaking)]
    #[case::issue_172_other_other(
        "chore(config.yml): update jerus-org/circleci-toolkit orb version to 0.4.0",
        TopType::Other,
        TopType::Other
    )]
    #[case::with_emoji_feat_other_other(
        "✨ feat(ci): add optional flag for push failure handling",
        TopType::Other,
        TopType::Feature
    )]
    #[case::feat_fix_feat("feat: add new feature", TopType::Fix, TopType::Feature)]
    #[case::emoji_feat_fix_feat("✨ feat: add new feature", TopType::Fix, TopType::Feature)]
    #[case::fix_fix_fix("fix: fix an existing feature", TopType::Fix, TopType::Fix)]
    #[case::emoji_fix_fix_fix("🐛 fix: fix an existing feature", TopType::Fix, TopType::Fix)]
    #[case::style_fix_fix("style: fix typo and lint issues", TopType::Fix, TopType::Fix)]
    #[case::emoji_style_fix_fix("💄 style: fix typo and lint issues", TopType::Fix, TopType::Fix)]
    #[case::test_fix_fix("test: update tests", TopType::Fix, TopType::Fix)]
    #[case::security_fix_fix(
        "fix(security): Fix security vulnerability",
        TopType::Fix,
        TopType::Fix
    )]
    #[case::chore_fix_fix("chore(deps): Update dependencies", TopType::Fix, TopType::Fix)]
    #[case::emoji_chore_fix_fix("🔧 chore(deps): Update dependencies", TopType::Fix, TopType::Fix)]
    #[case::refactor_fix_fix("refactor(remove): Remove unused code", TopType::Fix, TopType::Fix)]
    #[case::emoji_refactor_fix_fix(
        "♻️ refactor(remove): Remove unused code",
        TopType::Fix,
        TopType::Fix
    )]
    #[case::docs_fix_fix("docs(deprecate): Deprecate old API", TopType::Fix, TopType::Fix)]
    #[case::emoji_docs_fix_fix("📚 docs(deprecate): Deprecate old API", TopType::Fix, TopType::Fix)]
    #[case::ci_fix_fix("ci(other-scope): Update CI configuration", TopType::Fix, TopType::Fix)]
    #[case::emoji_ci_fix_fix(
        "👷 ci(other-scope): Update CI configuration",
        TopType::Fix,
        TopType::Fix
    )]
    #[case::test_fix_breaking("test!: Update test cases", TopType::Fix, TopType::Breaking)]
    #[case::issue_172_chore_fix_fix(
        "chore(config.yml): update jerus-org/circleci-toolkit orb version to 0.4.0",
        TopType::Fix,
        TopType::Fix
    )]
    #[case::with_emoji_emoji_feat_fix_feat(
        "✨ feat(ci): add optional flag for push failure handling",
        TopType::Fix,
        TopType::Feature
    )]
    #[case::feat_feat_feat("feat: add new feature", TopType::Feature, TopType::Feature)]
    #[case::emoji_feat_feat_feat("✨ feat: add new feature", TopType::Feature, TopType::Feature)]
    #[case::fix_feat_feat("fix: fix an existing feature", TopType::Feature, TopType::Feature)]
    #[case::emoji_fix_feat_feat(
        "🐛 fix: fix an existing feature",
        TopType::Feature,
        TopType::Feature
    )]
    #[case::style_feat_feat("style: fix typo and lint issues", TopType::Feature, TopType::Feature)]
    #[case::emoji_style_feat_feat(
        "💄 style: fix typo and lint issues",
        TopType::Feature,
        TopType::Feature
    )]
    #[case::test_feat_feat("test: update tests", TopType::Feature, TopType::Feature)]
    #[case::security_feat_feat(
        "fix(security): Fix security vulnerability",
        TopType::Feature,
        TopType::Feature
    )]
    #[case::chore_feat_feat("chore(deps): Update dependencies", TopType::Feature, TopType::Feature)]
    #[case::emoji_chore_feat_feat(
        "🔧 chore(deps): Update dependencies",
        TopType::Feature,
        TopType::Feature
    )]
    #[case::refactor_feat_feat(
        "refactor(remove): Remove unused code",
        TopType::Feature,
        TopType::Feature
    )]
    #[case::refactor_feat_feat(
        "♻️ refactor(remove): Remove unused code",
        TopType::Feature,
        TopType::Feature
    )]
    #[case::docs_feat_feat(
        "docs(deprecate): Deprecate old API",
        TopType::Feature,
        TopType::Feature
    )]
    #[case::emoji_docs_feat_feat(
        "📚 docs(deprecate): Deprecate old API",
        TopType::Feature,
        TopType::Feature
    )]
    #[case::ci_feat_feat(
        "ci(other-scope): Update CI configuration",
        TopType::Feature,
        TopType::Feature
    )]
    #[case::emoji_ci_feat_feat(
        "👷 ci(other-scope): Update CI configuration",
        TopType::Feature,
        TopType::Feature
    )]
    #[case::test_feat_breaking("test!: Update test cases", TopType::Feature, TopType::Breaking)]
    #[case::issue_172_chore_feat_feat(
        "chore(config.yml): update jerus-org/circleci-toolkit orb version to 0.4.0",
        TopType::Feature,
        TopType::Feature
    )]
    #[case::with_emoji_emoji_feat_feat_feat(
        "✨ feat(ci): add optional flag for push failure handling",
        TopType::Feature,
        TopType::Feature
    )]
    #[case::feat_breaking_breaking("feat: add new feature", TopType::Breaking, TopType::Breaking)]
    #[case::emoji_feat_breaking_breaking(
        "✨ feat: add new feature",
        TopType::Breaking,
        TopType::Breaking
    )]
    #[case::fix_breaking_breaking(
        "fix: fix an existing feature",
        TopType::Breaking,
        TopType::Breaking
    )]
    #[case::emoji_fix_breaking_breaking(
        "🐛 fix: fix an existing feature",
        TopType::Breaking,
        TopType::Breaking
    )]
    #[case::style_breaking_breaking(
        "style: fix typo and lint issues",
        TopType::Breaking,
        TopType::Breaking
    )]
    #[case::emoji_style_breaking_breaking(
        "💄 style: fix typo and lint issues",
        TopType::Breaking,
        TopType::Breaking
    )]
    #[case::test_breaking_breaking("test: update tests", TopType::Breaking, TopType::Breaking)]
    #[case::security_breaking_breaking(
        "fix(security): Fix security vulnerability",
        TopType::Breaking,
        TopType::Breaking
    )]
    #[case::chore_breaking_breaking(
        "chore(deps): Update dependencies",
        TopType::Breaking,
        TopType::Breaking
    )]
    #[case::emoji_chore_breaking_breaking(
        "🔧 chore(deps): Update dependencies",
        TopType::Breaking,
        TopType::Breaking
    )]
    #[case::refactor_breaking_breaking(
        "refactor(remove): Remove unused code",
        TopType::Breaking,
        TopType::Breaking
    )]
    #[case::emoji_refactor_breaking_breaking(
        "♻️ refactor(remove): Remove unused code",
        TopType::Breaking,
        TopType::Breaking
    )]
    #[case::docs_breaking_breaking(
        "docs(deprecate): Deprecate old API",
        TopType::Breaking,
        TopType::Breaking
    )]
    #[case::emoji_docs_breaking_breaking(
        "📚 docs(deprecate): Deprecate old API",
        TopType::Breaking,
        TopType::Breaking
    )]
    #[case::ci_breaking_breaking(
        "ci(other-scope): Update CI configuration",
        TopType::Breaking,
        TopType::Breaking
    )]
    #[case::emoji_ci_breaking_breaking(
        "👷 ci(other-scope): Update CI configuration",
        TopType::Breaking,
        TopType::Breaking
    )]
    #[case::test_breaking_breaking(
        "test!: Update test cases",
        TopType::Breaking,
        TopType::Breaking
    )]
    #[case::issue_172_chore_breaking_breaking(
        "chore(config.yml): update jerus-org/circleci-toolkit orb version to 0.4.0",
        TopType::Breaking,
        TopType::Breaking
    )]
    #[case::with_emoji_feat_breaking_breaking(
        "✨ feat(ci): add optional flag for push failure handling",
        TopType::Breaking,
        TopType::Breaking
    )]
    fn test_calculate_kind_and_description(
        #[case] title: &str,
        #[case] base_top_type: TopType,
        #[case] expected_top_type: TopType,
    ) {
        get_test_logger();

        let mut con_commits = super::ConventionalCommits::new();
        if TopType::Breaking == base_top_type {
            con_commits.breaking = true;
        }
        con_commits.top_type = base_top_type;
        con_commits.update_from_summary(title);

        println!("Conventional commits: {con_commits:#?}");

        println!("Expected top type: {expected_top_type:#?}");

        assert_eq!(expected_top_type, con_commits.top_type);
    }
}
