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

use std::{cmp::Ordering, fmt};

use crate::Error;
use regex::Regex;

/// Level at which the next increment will be made
///
#[derive(Debug, PartialOrd, PartialEq, Eq, Ord, Clone, Default)]
pub enum Level {
    /// When no update has been detected the level is set to none
    #[default]
    None,
    /// Update will be made at the patch level
    Patch,
    /// Update will be made at the private level
    Minor,
    /// Update will be made at the major level
    Major,
    /// Update is a release removing any pre-release suffixes (for future use)
    Release,
    /// Update is to an alpha pre-release suffix (for future use)
    Alpha,
    /// Update is to an beta pre-release suffix (for future use)
    Beta,
    /// Update is to an rc pre-release suffix (for future use)
    Rc,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Level::None => write!(f, "none"),
            Level::Patch => write!(f, "patch"),
            Level::Minor => write!(f, "minor"),
            Level::Major => write!(f, "major"),
            Level::Release => write!(f, "release"),
            Level::Alpha => write!(f, "alpha"),
            Level::Beta => write!(f, "beta"),
            Level::Rc => write!(f, "rc"),
        }
    }
}

macro_rules! some_or_none_string {
    ($i:ident) => {
        if !$i.is_empty() {
            Some($i.to_string())
        } else {
            None
        }
    };
}

/// The VersionTag data structure represents a git tag containing a
/// semantic version number.
///
#[derive(Debug, Default, Clone)]
pub struct VersionTag {
    refs: String,
    tag_prefix: String,
    version_prefix: String,
    semantic_version: Semantic,
}

impl PartialEq for VersionTag {
    fn eq(&self, other: &Self) -> bool {
        self.semantic_version == other.semantic_version
    }
}

impl Eq for VersionTag {}

impl PartialOrd for VersionTag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VersionTag {
    fn cmp(&self, other: &Self) -> Ordering {
        self.semantic_version.cmp(&other.semantic_version)
    }
}

impl fmt::Display for VersionTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.refs, self.tag_prefix, self.version_prefix, self.semantic_version
        )
    }
}

impl VersionTag {
    // Create a new struct specifying each of the semantic version components.
    fn new(
        refs: String,
        tag_prefix: String,
        version_prefix: String,
        semantic_version: Semantic,
    ) -> Self {
        VersionTag {
            refs,
            tag_prefix,
            version_prefix,
            semantic_version,
        }
    }
    /// Parse a tag and return a struct
    /// String format expect: <version_prefix>x.y.z
    ///
    /// # Fields
    ///
    /// tag - the tag proposed as a semantic version tag
    /// version_prefix - any string before the semantic version number
    ///
    /// # Example
    ///
    /// Parse a tag into a semantic version number where "v" is used to identify
    /// tags representing semantic version numbers.
    ///
    /// ```rust
    /// # fn main() -> Result<(), nextsv::Error> {
    /// use nextsv::VersionTag;
    ///
    /// let tag = "refs/tags/v0.2.3";
    /// let semantic_version = VersionTag::parse(tag, "v")?;
    ///
    /// assert_eq!(0, semantic_version.version().major());
    /// assert_eq!(2, semantic_version.version().minor());
    /// assert_eq!(3, semantic_version.version().patch());
    ///
    /// # Ok(())
    /// # }
    /// ```
    /// to identify tags with semantic version numbers
    /// the tag name can be parsed
    pub fn parse(tag: &str, version_prefix: &str) -> Result<Self, Error> {
        let re_tag = format!(
            r"(?<refs>refs\/tags\/)(?<tag_prefix>.*)(?<version_prefix>{})(?<major>0|[1-9]\d*)\.(?<minor>0|[1-9]\d*)\.(?<patch>0|[1-9]\d*)(?:-(?<pre_release>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?<build_meta_data>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$",
            version_prefix
        );

        let re = Regex::new(&re_tag).unwrap();

        log::trace!("Parsing git tag `{tag}` into VersionTag");
        let caps_res = re.captures(tag);
        log::trace!("Regex captures result: {:?}", caps_res);
        let Some(caps) = caps_res else {
            tag_validation(tag, version_prefix)?;
            panic!("Tag validation failed");
        };

        let semantic_version = Semantic::new(
            caps.name("major").unwrap().as_str(),
            caps.name("minor").unwrap().as_str(),
            caps.name("patch").unwrap().as_str(),
            caps.name("pre_release").map_or("", |m| m.as_str()),
            caps.name("build_meta_data").map_or("", |m| m.as_str()),
        );

        Ok(VersionTag::new(
            caps.name("refs").map_or("", |m| m.as_str()).to_string(),
            caps.name("tag_prefix")
                .map_or("", |m| m.as_str())
                .to_string(),
            version_prefix.to_string(),
            semantic_version,
        ))
    }

    /// Provide a reference to the semantic version
    ///
    pub fn version(&self) -> &Semantic {
        &self.semantic_version
    }

    /// Provide a mutable reference to the semantic version
    ///
    pub fn version_mut(&mut self) -> &mut Semantic {
        &mut self.semantic_version
    }
}

fn tag_validation(tag: &str, version_prefix: &str) -> Result<(), Error> {
    log::debug!(
        "Error found: validating the tag {tag} with version identified by {version_prefix}"
    );
    let re = Regex::new(version_prefix).unwrap();
    let m_res = re.find(tag);

    // the tag string must start with the version_prefix
    let Some(m) = m_res else {
        return Err(Error::NotVersionTag(
            version_prefix.to_string(),
            tag.to_string(),
        ));
    };

    let (_prefix, version) = tag.split_at(m.end());
    log::debug!("The version string is: {version}");
    let components: Vec<&str> = version.split('.').collect();

    log::debug!("The components of the version string are {components:#?}");

    let mut count_numbers = 0;
    let mut numbers = vec![];

    for item in components {
        count_numbers += 1;
        if count_numbers > 3 {
            return Err(Error::TooManyComponents(count_numbers));
        }
        numbers.push(match item.parse::<usize>() {
            Ok(n) => n,
            Err(_) => return Err(Error::MustBeNumber(item.to_string())),
        });
    }

    if count_numbers < 3 {
        return Err(Error::TooFewComponents(count_numbers));
    };

    Ok(())
}

/// The Semantic data structure represents a semantic version number.
///
/// TODO: Implement support for pre-release and build
///
#[derive(Debug, Default, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct Semantic {
    major: u32,
    minor: u32,
    patch: u32,
    pre_release: Option<String>,
    build_meta_data: Option<String>,
}

impl fmt::Display for Semantic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut version = format!("{}.{}.{}", self.major, self.minor, self.patch);
        if let Some(pre) = &self.pre_release {
            version = version + "-" + &pre
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

        Semantic {
            major,
            minor,
            patch,
            pre_release: some_or_none_string!(pre_release),
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

    /// Increment the minor component of the version number by 1
    ///
    pub fn increment_minor(&mut self) -> &mut Self {
        self.minor += 1;
        self.patch = 0;
        self
    }

    /// Increment the major component of the version number by 1
    ///
    pub fn increment_major(&mut self) -> &mut Self {
        self.major += 1;
        self.minor = 0;
        self.patch = 0;
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
        let updated_version = version.increment_minor();

        assert_eq!("0.1.0", &updated_version.to_string());
    }

    #[test]
    fn bump_major_version_number_by_one() {
        let mut version = Semantic::default();
        let updated_version = version.increment_major();

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
        assert_eq!("Beta.3", vt.version().pre_release.as_ref().unwrap());
        assert_eq!("20876.675", vt.version().build_meta_data.as_ref().unwrap());
    }
}
