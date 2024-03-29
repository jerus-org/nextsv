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
    /// Update is to a generic pre-release suffix (for future use)
    PreRelease,
    /// Update is a release removing any pre-release suffixes (for future use)
    Release,
    /// Update is to an alpha pre-release suffix (for future use)
    Alpha,
    /// Update is to a beta pre-release suffix (for future use)
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
            Level::PreRelease => write!(f, "pre-release"),
            Level::Alpha => write!(f, "alpha"),
            Level::Beta => write!(f, "beta"),
            Level::Rc => write!(f, "rc"),
        }
    }
}

const PRE_RELEASE_PREFIX: &str = "-";

/// The semantic data structure for pre-releases
///
#[derive(Debug, Default, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct SemanticPreRelease {
    suffix: String,
    id: usize,
}

impl fmt::Display for SemanticPreRelease {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "-{}.{}", self.suffix, self.id)
    }
}

impl SemanticPreRelease {
    // Create a new struct specifying each of the semantic version components.
    fn new(suffix: String, id: usize) -> Self {
        SemanticPreRelease { suffix, id }
    }

    /// ```rust
    /// # fn main() -> Result<(), nextsv::Error> {
    /// use nextsv::SemanticPreRelease;
    ///
    /// let semantic_version = SemanticPreRelease::parse("rc.0")?;
    ///
    /// assert_eq!("rc", semantic_version.suffix());
    /// assert_eq!(0, semantic_version.id());
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn parse(pre_release: &str) -> Result<Self, Error> {
        let components: Vec<&str> = pre_release.split('.').collect();

        if components.len() < 2 {
            return Err(Error::InvalidPreReleaseFormat(pre_release.to_string()));
        }

        let suffix = components[0].to_string();
        let id: usize = components[1].to_string().parse().unwrap();

        Ok(SemanticPreRelease::new(suffix, id))
    }

    /// Increment pre-release version
    ///
    pub fn increment(&mut self) -> &mut Self {
        self.id += 1;
        self
    }

    /// get suffix
    ///
    pub fn suffix(&self) -> String {
        self.suffix.clone()
    }

    /// get id
    ///
    pub fn id(&self) -> usize {
        self.id
    }
}
/// The Semantic data structure represents a semantic version number.
///
/// TODO: Implement support for build
///
#[derive(Debug, Default, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct Semantic {
    version_prefix: String,
    major: usize,
    minor: usize,
    patch: usize,
    pre_release: Option<SemanticPreRelease>,
}

impl fmt::Display for Semantic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let pre_release = self.pre_release()
        let mut pre_release_tag: String = "".to_string();
        if let Some(pre_release_some) = self.pre_release() {
            pre_release_tag = format!("{}", pre_release_some)
        }
        write!(
            f,
            "{}{}.{}.{}{}",
            self.version_prefix, self.major, self.minor, self.patch, pre_release_tag
        )
    }
}

impl Semantic {
    // Create a new struct specifying each of the semantic version components.
    fn new(
        version_prefix: String,
        major: usize,
        minor: usize,
        patch: usize,
        pre_release: Option<SemanticPreRelease>,
    ) -> Self {
        Semantic {
            version_prefix,
            major,
            minor,
            patch,
            pre_release,
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
    /// use nextsv::Semantic;
    ///
    /// let tag = "v0.2.3";
    /// let semantic_version = Semantic::parse(tag, "v")?;
    ///
    /// assert_eq!(0, semantic_version.major());
    /// assert_eq!(2, semantic_version.minor());
    /// assert_eq!(3, semantic_version.patch());
    /// assert_eq!(tag, semantic_version.to_string());
    ///
    /// # Ok(())
    /// # }
    /// ```
    /// to identify tags with semantic version numbers
    /// the tag name can be parsed
    pub fn parse(tag: &str, version_prefix: &str) -> Result<Self, Error> {
        // the tag string must start with the version_prefix
        if !tag.starts_with(version_prefix) {
            return Err(Error::NotVersionTag(
                version_prefix.to_string(),
                tag.to_string(),
            ));
        }

        let version = tag.trim_start_matches(version_prefix);
        let semver_parts: Vec<&str> = version.split(PRE_RELEASE_PREFIX).collect();
        let components: Vec<&str> = semver_parts[0].split('.').collect();

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
        }

        let mut pre_release = None;
        if semver_parts.len() > 1 {
            match SemanticPreRelease::parse(semver_parts[1]) {
                Ok(t) => pre_release = Some(t),
                Err(e) => return Err(e),
            }
            // pre_release = SemanticPreRelease::parse(semver_parts[1])
        }

        Ok(Semantic::new(
            version_prefix.to_string(),
            numbers[0],
            numbers[1],
            numbers[2],
            pre_release.clone(),
        ))
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

    /// Increment the number of the pre-release
    ///
    pub fn increment_pre_release(&mut self) -> &mut Self {
        self.pre_release.as_mut().unwrap().increment();
        self
    }

    /// Create initial pre-release
    ///
    pub fn first_pre_release(&mut self, suffix: &str) -> &mut Self {
        self.pre_release = Some(SemanticPreRelease::new(suffix.to_string(), 0));
        self
    }

    /// Unset pre-release, for promoting to actual release
    ///
    pub fn unset_pre_release(&mut self) -> &mut Self {
        self.pre_release = None;
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
    pub fn major(&self) -> usize {
        self.major
    }
    /// Report the minor version number
    pub fn minor(&self) -> usize {
        self.minor
    }

    /// Report the patch version number
    ///
    pub fn patch(&self) -> usize {
        self.patch
    }

    /// Report the pre-release
    ///
    pub fn pre_release(&self) -> Option<SemanticPreRelease> {
        self.pre_release.clone()
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
    fn set_pre_release_version_number() {
        let mut version = Semantic::default();
        let updated_version = version.first_pre_release("rc");

        assert_eq!("0.0.0-rc.0", &updated_version.to_string());
    }

    #[test]
    fn bump_pre_release_version_number_by_one() {
        let mut version = Semantic::default();
        version.first_pre_release("rc");

        let updated_version = version.increment_pre_release();

        assert_eq!("0.0.0-rc.1", &updated_version.to_string());
    }

    #[test]
    fn parse_valid_pre_release_version_tag_to_new_semantic_struct() {
        let tag = "v0.3.90-rc.4";
        let version_prefix = "v";
        let semantic = Semantic::parse(tag, version_prefix);

        claims::assert_ok!(&semantic);
        let semantic = match semantic {
            Ok(s) => s.to_string(),
            Err(e) => e.to_string(),
        };
        assert_eq!(tag, semantic);
    }

    #[test]
    fn parse_valid_version_tag_to_new_semantic_struct() {
        let tag = "v0.3.90";
        let version_prefix = "v";
        let semantic = Semantic::parse(tag, version_prefix);

        claims::assert_ok!(&semantic);
        let semantic = match semantic {
            Ok(s) => s.to_string(),
            Err(e) => e.to_string(),
        };
        assert_eq!(tag, semantic);
    }

    #[test]
    fn parse_long_valid_version_tag_to_new_semantic_struct() {
        let tag = "Release Version 0.3.90";
        let version_prefix = "Release Version ";
        let semantic = Semantic::parse(tag, version_prefix);

        claims::assert_ok!(&semantic);
        let semantic = match semantic {
            Ok(s) => s.to_string(),
            Err(e) => e.to_string(),
        };
        assert_eq!(tag, semantic);
    }

    #[test]
    fn parse_error_failed_not_version_tag() {
        let tag = "0.3.90";
        let version_prefix = "v";
        let semantic = Semantic::parse(tag, version_prefix);

        claims::assert_err!(&semantic);
        let semantic = match semantic {
            Ok(s) => s.to_string(),
            Err(e) => e.to_string(),
        };
        assert_eq!(
            r#"Version tags must start with "v" but tag is 0.3.90"#,
            semantic
        );
    }

    #[test]
    fn parse_error_too_many_components() {
        let tag = "v0.3.90.8";
        let version_prefix = "v";
        let semantic = Semantic::parse(tag, version_prefix);

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
        let tag = "v0.3";
        let version_prefix = "v";
        let semantic = Semantic::parse(tag, version_prefix);

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
    fn parse_error_invalid_pre_release() {
        let tag = "v0.3.90-8";
        let version_prefix = "v";
        let semantic = Semantic::parse(tag, version_prefix);

        claims::assert_err!(&semantic);
        let semantic = match semantic {
            Ok(s) => s.to_string(),
            Err(e) => e.to_string(),
        };
        assert_eq!("Invalid PreRelease format: 8", semantic);
    }
    // #[error("Version must be a number")]
    // MustBeNumber,
}
