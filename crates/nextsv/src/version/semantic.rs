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

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum VersionType {
    NonProduction,
    PreRelease,
    Production,
}

/// The Semantic data structure represents a semantic version number.
///
/// TODO: Implement support for pre-release and build
///
#[derive(Debug, Default, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub(crate) struct Semantic {
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
            version = version + "+" + build
        }
        write!(f, "{version}")
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

    pub(crate) fn increment_pre_release(&mut self) -> &mut Self {
        if let Some(mut pre_release) = self.pre_release.clone() {
            let new_count = if let Some(mut c) = pre_release.counter {
                c += 1;
                c
            } else {
                1
            };
            pre_release.counter = Some(new_count);

            self.pre_release = Some(pre_release);
        };
        self
    }

    /// Returns the type of version based on the major version number.
    pub(crate) fn version_type(&self) -> VersionType {
        if self.pre_release.is_some() {
            return VersionType::PreRelease;
        }
        if self.major == 0 {
            VersionType::NonProduction
        } else {
            VersionType::Production
        }
    }

    pub(crate) fn is_production_version(&self) -> bool {
        self.major != 0
    }
}

#[cfg(test)]
mod tests {

    use log::LevelFilter;
    use rstest::rstest;

    use super::*;

    fn get_test_logger() {
        let mut builder = env_logger::Builder::new();
        builder.filter(None, LevelFilter::Debug);
        builder.format_timestamp_secs().format_module_path(false);
        let _ = builder.try_init();
    }

    #[test]
    fn bump_patch_version_number_by_one() {
        let version = Semantic::default();
        let mut updated_version = version;
        updated_version.patch += 1;

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
    #[case::simple_non_prod(0, 7, 9, "", "", "1.0.0")]
    #[case::non_prod_alpha(0, 2, 200, "alpha.1", "", "1.0.0")]
    #[case::non_prod_beta_with_build(0, 24, 2, "beta.1", "30", "1.0.0")]
    #[case::non_prod_release_candidate(0, 78, 3, "rc.1", "40", "1.0.0")]
    #[case::already_first_version(1, 0, 0, "", "", "1.0.0")]
    #[case::patched_first_version(1, 0, 1, "", "", "1.0.1")]
    #[case::minor_update_first_version(1, 1, 0, "", "", "1.1.0")]
    #[case::non_prod_custom_pre_release(0, 23, 1, "pre.1", "circle.1", "1.0.0")]
    #[case::post_first_version_alphanumeric_build(
        2,
        0,
        0,
        "pre.2",
        "circle.14",
        "2.0.0-pre.2+circle.14"
    )]
    fn set_first_production_version_number(
        #[case] major: u32,
        #[case] minor: u32,
        #[case] patch: u32,
        #[case] pre_release: &str,
        #[case] build_meta_data: &str,
        #[case] expected: &str,
    ) {
        get_test_logger();

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

        if test_version.major == 0 {
            test_version.major = 1;
            test_version.minor = 0;
            test_version.patch = 0;
            test_version.pre_release = None;
            test_version.build_meta_data = None;
        };
        assert_eq!(expected, test_version.to_string().as_str());
    }
}
