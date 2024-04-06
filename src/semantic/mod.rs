//! Semantic Versioning Struct
//!
//! Holds a semantic version number as defined by
//! the [Semantic Version Specification v 2.0.0](https://semver.org/spec/v2.0.0.html)
//!
//! ## Notes
//!
//! Initial implementation does not include support
//! for pre-release suffixes.
//!

use std::fmt;

use crate::Error;

mod level;
pub(crate) mod test_utils;
mod version_tag;

pub use level::Level;
pub use version_tag::VersionTag;

macro_rules! some_or_none_string {
    ($i:ident) => {
        if !$i.is_empty() {
            Some($i.to_string())
        } else {
            None
        }
    };
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(crate) enum PreReleaseType {
    Alpha,
    Beta,
    Rc,
    Custom,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(crate) struct PreRelease {
    pub(crate) label: String,
    pub(crate) counter: Option<u32>,
    pub(crate) pre_type: PreReleaseType,
}

impl fmt::Display for PreRelease {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(number) = self.counter {
            write!(f, "{}.{}", self.label, number)
        } else {
            write!(f, "{}", self.label)
        }
    }
}

impl PreRelease {
    pub(crate) fn new(pre_release: &str) -> PreRelease {
        let (label, counter) = if let Some((label, number)) = pre_release.rsplit_once('.') {
            match number.parse::<u32>() {
                Ok(n) => (label.to_string(), Some(n)),
                Err(_) => (format!("{}.{}", label, number), None),
            }
        } else {
            (pre_release.to_string(), None)
        };
        let mut pre_type = PreReleaseType::Custom;
        if label.to_ascii_lowercase() == "alpha" {
            pre_type = PreReleaseType::Alpha;
        }
        if label.to_ascii_lowercase() == "beta" {
            pre_type = PreReleaseType::Beta;
        }
        if label.to_ascii_lowercase() == "rc" {
            pre_type = PreReleaseType::Rc;
        }
        PreRelease {
            label,
            counter,
            pre_type,
        }
    }
}
/// The Semantic data structure represents a semantic version number.
///
/// TODO: Implement support for pre-release and build
///
#[derive(Debug, Default, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct Semantic {
    pub(crate) major: u32,
    pub(crate) minor: u32,
    pub(crate) patch: u32,
    pub(crate) pre_release: Option<PreRelease>,
    pub(crate) build_meta_data: Option<String>,
}

impl fmt::Display for Semantic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut version = format!("{}.{}.{}", self.major, self.minor, self.patch);
        if let Some(pre) = &self.pre_release {
            version = version + "-" + pre.to_string().as_str()
        };
        if let Some(build) = &self.build_meta_data {
            version = version + "+" + &build
        }
        write!(f, "{}", version)
    }
}

impl Semantic {
    // Create a new struct specifying each of the semantic version components.
    fn new(
        major: &str,
        minor: &str,
        patch: &str,
        pre_release: &str,
        build_meta_data: &str,
    ) -> Self {
        let major: u32 = major.parse().unwrap();
        let minor: u32 = minor.parse().unwrap();
        let patch: u32 = patch.parse().unwrap();

        let pre_release = if pre_release.is_empty() {
            None
        } else {
            Some(PreRelease::new(pre_release))
        };

        Semantic {
            major,
            minor,
            patch,
            pre_release,
            build_meta_data: some_or_none_string!(build_meta_data),
        }
    }

    /// Increment the version based on a breaking change
    /// When the major number is 0 increment the minor
    /// number else increment the major number
    ///
    pub fn breaking_increment(&mut self) -> &mut Self {
        if self.major == 0 {
            self.minor += 1;
            self.patch = 0;
        } else {
            self.major += 1;
            self.minor = 0;
            self.patch = 0;
        }
        self
    }

    /// Increment the patch component of the version number by 1
    ///
    pub fn increment_patch(&mut self) -> &mut Self {
        self.patch += 1;
        self
    }

    /// Increment the pre_release component of the version number by 1
    /// If no counter exists a counter will be added with an iniital
    /// count of 1.
    ///
    pub fn increment_pre_release(&mut self) -> &mut Self {
        let mut pre_release = self.pre_release.clone().unwrap();
        let new_count = if let Some(mut c) = pre_release.counter {
            c += 1;
            c
        } else {
            1
        };
        pre_release.counter = Some(new_count);

        self.pre_release = Some(pre_release);
        self
    }

    /// Set the first production release version
    ///
    pub fn first_production(&mut self) -> Result<&mut Self, Error> {
        if 0 < self.major {
            return Err(Error::MajorAlreadyUsed(self.major.to_string()));
        } else {
            self.major = 1;
            self.minor = 0;
            self.patch = 0;
        }
        Ok(self)
    }

    /// Report the major version number
    ///
    pub fn major(&self) -> u32 {
        self.major
    }
    /// Report the minor version number
    pub fn minor(&self) -> u32 {
        self.minor
    }

    /// Report the patch version number
    ///
    pub fn patch(&self) -> u32 {
        self.patch
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn display_semantic_version_number() {
        let version = Semantic::default();

        assert_eq!("0.0.0", &version.to_string());
    }

    #[test]
    fn bump_patch_version_number_by_one() {
        let mut version = Semantic::default();
        let updated_version = version.increment_patch();

        assert_eq!("0.0.1", &updated_version.to_string());
    }

    #[test]
    fn bump_minor_version_number_by_one() {
        let mut version = Semantic::default();
        version.minor += 1;
        version.patch = 0;

        assert_eq!("0.1.0", &version.to_string());
    }

    #[test]
    fn bump_major_version_number_by_one() {
        let mut version = Semantic::default();
        let updated_version = version.increment_pre_release();

        assert_eq!("1.0.0", &updated_version.to_string());
    }

    #[test]
    fn parse_valid_version_tag_to_new_semantic_struct() {
        let tag = "refs/tags/v0.3.90";
        let version_prefix = "v";
        let semantic = VersionTag::parse(tag, version_prefix);

        claims::assert_ok!(&semantic);
        let semantic = match semantic {
            Ok(s) => s.to_string(),
            Err(e) => e.to_string(),
        };
        assert_eq!(tag, semantic);
    }

    #[test]
    fn parse_valid_long_version_tag_to_new_semantic_struct() {
        let tag = "refs/tags/Release Version 0.3.90";
        let version_prefix = "Release Version ";
        let semantic = VersionTag::parse(tag, version_prefix);

        claims::assert_ok!(&semantic);
        let semantic = match semantic {
            Ok(s) => s.to_string(),
            Err(e) => e.to_string(),
        };
        assert_eq!(tag, semantic);
    }

    #[test]
    fn parse_error_failed_not_version_tag() {
        let tag = "ref/tags/0.3.90";
        let version_prefix = "v";
        let semantic = VersionTag::parse(tag, version_prefix);

        claims::assert_err!(&semantic);
        let semantic = match semantic {
            Ok(s) => s.to_string(),
            Err(e) => e.to_string(),
        };
        assert_eq!(
            r#"Version tags must start with "v" but tag is ref/tags/0.3.90"#,
            semantic
        );
    }

    #[test]
    fn parse_error_too_many_components() {
        let tag = "refs/tags/v0.3.90.8";
        let version_prefix = "v";
        let semantic = VersionTag::parse(tag, version_prefix);

        claims::assert_err!(&semantic);
        let semantic = match semantic {
            Ok(s) => s.to_string(),
            Err(e) => e.to_string(),
        };
        assert_eq!(
            "Version must have three components but at least 4 were found",
            semantic
        );
    }

    #[test]
    fn parse_error_not_enough_components() {
        let tag = "refs/tags/v0.3";
        let version_prefix = "v";
        let semantic = VersionTag::parse(tag, version_prefix);

        claims::assert_err!(&semantic);
        let semantic = match semantic {
            Ok(s) => s.to_string(),
            Err(e) => e.to_string(),
        };
        assert_eq!(
            "Version must have three components but only 2 found",
            semantic
        );
    }

    #[test]
    fn parse_error_version_must_be_a_number() {
        let tag = "refs/tags/v0.3.9a";
        let version_prefix = "v";
        let semantic = VersionTag::parse(tag, version_prefix);

        claims::assert_err!(&semantic);
        let semantic = match semantic {
            Ok(s) => s.to_string(),
            Err(e) => e.to_string(),
        };
        assert_eq!("Version must be a number but found 9a", semantic);
    }
    // #[error("Version must be a number")]
    // MustBeNumber,

    #[test]
    fn display_returns_the_tag_string() {
        let tag = "refs/tags/hcaptcha-v2.3.1-ALPHA+build";

        let mytag = VersionTag::parse(tag, "v").unwrap().to_string();

        assert_eq!(tag, mytag);
    }

    #[test]
    fn tag_broken_down_correctly() {
        let tag = "refs/tags/hcaptcha-v2.3.1-Beta.3+20876.675";

        let vt = VersionTag::parse(tag, "v").unwrap();

        assert_eq!("refs/tags/", vt.refs);
        assert_eq!("hcaptcha-", vt.tag_prefix);
        assert_eq!("v", vt.version_prefix);
        assert_eq!("2.3.1-Beta.3+20876.675", vt.version().to_string().as_str());
        assert_eq!(2, vt.version().major);
        assert_eq!(3, vt.version().minor);
        assert_eq!(1, vt.version().patch);
        assert_eq!(
            "Beta.3",
            vt.semantic_version
                .pre_release
                .unwrap()
                .to_string()
                .as_str()
        );
        assert_eq!(
            "20876.675",
            vt.semantic_version.build_meta_data.as_ref().unwrap()
        );
    }
}
