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

use super::PreRelease;

macro_rules! some_or_none_string {
    ($i:ident) => {
        if !$i.is_empty() {
            Some($i.to_string())
        } else {
            None
        }
    };
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
    pub(crate) fn new(
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
    /// Unless the version number is already a production version
    /// number sets the version number to 1.0.0. Any pre release
    /// or build meta data fields will be removed.
    ///
    ///
    pub fn first_production(&mut self) -> Result<(), Error> {
        log::debug!("Current version number: {self}");
        if 0 < self.major {
            return Err(Error::MajorAlreadyUsed(self.major.to_string()));
        } else {
            log::debug!("Making changes");
            self.major = 1;
            self.minor = 0;
            self.patch = 0;
            self.pre_release = None;
            self.build_meta_data = None;
        }
        log::debug!("Revised version number: {self}");
        Ok(())
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

    use rstest::rstest;

    use super::*;

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
        version.major += 1;
        version.minor += 0;
        version.patch = 0;

        assert_eq!("1.0.0", &version.to_string());
    }

    #[rstest]
    #[case::non_prod(0, 7, 9, "", "", "0.7.9")]
    #[case::first_alpha(1, 0, 0, "alpha.1", "", "1.0.0-alpha.1")]
    #[case::alpha_with_build(1, 0, 0, "alpha.2", "10", "1.0.0-alpha.2+10")]
    #[case::beta_with_build(1, 0, 0, "beta.1", "30", "1.0.0-beta.1+30")]
    #[case::release_candidate(1, 0, 0, "rc.1", "40", "1.0.0-rc.1+40")]
    #[case::first_version(1, 0, 0, "", "", "1.0.0")]
    #[case::patched_first_version(1, 0, 1, "", "", "1.0.1")]
    #[case::minor_update_first_version(1, 1, 0, "", "", "1.1.0")]
    #[case::custom_pre_release(2, 0, 0, "pre.1", "circle.1", "2.0.0-pre.1+circle.1")]
    #[case::alphanumeric_build(2, 0, 0, "pre.2", "circle.14", "2.0.0-pre.2+circle.14")]
    fn display_value(
        #[case] major: u32,
        #[case] minor: u32,
        #[case] patch: u32,
        #[case] pre_release: &str,
        #[case] build_meta_data: &str,
        #[case] expected: &str,
    ) {
        let pre_release = if !pre_release.is_empty() {
            Some(PreRelease::new(pre_release))
        } else {
            None
        };
        let build_meta_data = if build_meta_data.is_empty() {
            None
        } else {
            Some(build_meta_data.to_string())
        };
        let test_version = Semantic {
            major,
            minor,
            patch,
            pre_release,
            build_meta_data,
        };
        assert_eq!(expected, test_version.to_string().as_str());
    }

    #[rstest]
    #[case::simple_non_prod(0, 7, 9, "", "", true, "1.0.0")]
    #[case::non_prod_alpha(0, 2, 200, "alpha.1", "", true, "1.0.0")]
    #[case::non_prod_beta_with_build(0, 24, 2, "beta.1", "30", true, "1.0.0")]
    #[case::non_prod_release_candidate(0, 78, 3, "rc.1", "40", true, "1.0.0")]
    #[case::already_first_version(1, 0, 0, "", "", false, "1.0.0")]
    #[case::patched_first_version(1, 0, 1, "", "", false, "1.0.1")]
    #[case::minor_update_first_version(1, 1, 0, "", "", false, "1.1.0")]
    #[case::non_prod_custom_pre_release(0, 23, 1, "pre.1", "circle.1", true, "1.0.0")]
    #[case::post_first_version_alphanumeric_build(
        2,
        0,
        0,
        "pre.2",
        "circle.14",
        false,
        "2.0.0-pre.2+circle.14"
    )]
    fn set_first_production_version_number(
        #[case] major: u32,
        #[case] minor: u32,
        #[case] patch: u32,
        #[case] pre_release: &str,
        #[case] build_meta_data: &str,
        #[case] expected_ok: bool,
        #[case] expected: &str,
    ) {
        use log::LevelFilter;
        use log4rs_test_utils::test_logging;

        test_logging::init_logging_once_for(vec![], LevelFilter::Debug, None);

        let pre_release = if !pre_release.is_empty() {
            Some(PreRelease::new(pre_release))
        } else {
            None
        };
        let build_meta_data = if build_meta_data.is_empty() {
            None
        } else {
            Some(build_meta_data.to_string())
        };
        let mut test_version = Semantic {
            major,
            minor,
            patch,
            pre_release,
            build_meta_data,
        };

        let result = test_version.first_production();
        assert_eq!(expected_ok, result.is_ok());

        println!("{result:?}");
        // if result.is_ok() {
        //     test_version = result.unwrap();
        // }

        println!("expecting {expected} and got {test_version}");
        assert_eq!(expected, test_version.to_string().as_str());
    }
}
