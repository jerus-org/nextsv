use std::{
    collections::{HashMap, HashSet},
    ffi::OsString,
    fmt,
    str::FromStr,
};

use crate::calculator::TopType;
use crate::version::Semantic;
use crate::{
    calculator::ConventionalCommits,
    version::{PreRelease, VersionTag},
};

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

// use crate::calculator::
#[allow(dead_code)]
pub(crate) fn gen_conventional_commits() -> ConventionalCommits {
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

    let files = gen_files();

    ConventionalCommits {
        commits: vec![
            "fix: spelling of output in description of set_env".to_string(),
            "Merge branch 'main' of github.com:jerusdp/nextsv into fix/version-level-assessment"
                .to_string(),
            "test: Ensure all current tests are passing".to_string(),
            "refactor: implemented VersionTag".to_string(),
            "feat: Regex implemented to extract version string".to_string(),
            "chore: Updated minium rust version references".to_string(),
            "ci: Updated Minimum rust version to 1.74".to_string(),
            "docs: Updated tests in docs.".to_string(),
        ],
        counts,
        changed_files: files.clone(),
        all_files: files,
        breaking: false,
        top_type: TopType::Feature,
    }
}

pub(crate) fn gen_conventional_commit(
    commit_type: ConventionalType,
    breaking: bool,
) -> ConventionalCommits {
    let mut counts = HashMap::new();
    counts.insert(format!("{commit_type}"), 1);

    let commits = vec![format!(
        "{}{} commit for testing purposes only",
        if breaking { "!" } else { "" },
        breaking
    )];

    let files = gen_files();

    let top_type = TopType::parse(&commit_type.to_string()).unwrap();

    ConventionalCommits {
        commits,
        counts,
        breaking,
        changed_files: files.clone(),
        all_files: files,
        top_type,
    }
}

pub(crate) fn gen_files() -> HashSet<OsString> {
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

    files
}

#[allow(dead_code)]
pub(crate) fn gen_current_version(
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
