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

use std::cmp::Ordering;
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
#[derive(Debug, Default, Clone)]
pub(crate) struct Semantic {
    pub(crate) major: u32,
    pub(crate) minor: u32,
    pub(crate) patch: u32,
    pub(crate) pre_release: Option<PreRelease>,
    pub(crate) build_meta_data: Option<String>,
}

/// SemVer §11: Version precedence compares major, minor, patch numerically,
/// then pre-release identifiers. A pre-release version has lower precedence
/// than the associated normal version.
///
/// SemVer §10: Build metadata MUST be ignored when determining version
/// precedence.
impl Ord for Semantic {
    fn cmp(&self, other: &Self) -> Ordering {
        self.major
            .cmp(&other.major)
            .then(self.minor.cmp(&other.minor))
            .then(self.patch.cmp(&other.patch))
            .then(match (&self.pre_release, &other.pre_release) {
                (Some(a), Some(b)) => a.cmp(b),
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => Ordering::Equal,
            })
    }
}

impl PartialOrd for Semantic {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Semantic {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Semantic {}

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

    // SemVer Spec Section 11: Precedence rules
    //
    // Precedence MUST be calculated by separating the version into major,
    // minor, patch and pre-release identifiers in that order (Build
    // metadata does not figure into precedence).
    //
    // Precedence is determined by the first difference when comparing each
    // of these identifiers from left to right as follows: Major, minor,
    // and patch versions are always compared numerically.

    /// SemVer §11: Major version takes highest precedence
    #[rstest]
    #[case::major_gt(Semantic::new("2", "0", "0", "", ""), Semantic::new("1", "0", "0", "", ""))]
    #[case::major_gt_despite_minor(Semantic::new("2", "0", "0", "", ""), Semantic::new("1", "9", "0", "", ""))]
    #[case::major_gt_despite_patch(Semantic::new("2", "0", "0", "", ""), Semantic::new("1", "0", "9", "", ""))]
    fn major_version_precedence(#[case] higher: Semantic, #[case] lower: Semantic) {
        assert!(higher > lower, "{higher} should be > {lower}");
        assert!(lower < higher, "{lower} should be < {higher}");
    }

    /// SemVer §11: Minor version precedence when major is equal
    #[rstest]
    #[case::minor_gt(Semantic::new("1", "1", "0", "", ""), Semantic::new("1", "0", "0", "", ""))]
    #[case::minor_gt_despite_patch(Semantic::new("1", "2", "0", "", ""), Semantic::new("1", "1", "9", "", ""))]
    fn minor_version_precedence(#[case] higher: Semantic, #[case] lower: Semantic) {
        assert!(higher > lower, "{higher} should be > {lower}");
        assert!(lower < higher, "{lower} should be < {higher}");
    }

    /// SemVer §11: Patch version precedence when major and minor are equal
    #[rstest]
    #[case::patch_gt(Semantic::new("1", "0", "1", "", ""), Semantic::new("1", "0", "0", "", ""))]
    #[case::patch_ordering(Semantic::new("0", "1", "3", "", ""), Semantic::new("0", "1", "2", "", ""))]
    fn patch_version_precedence(#[case] higher: Semantic, #[case] lower: Semantic) {
        assert!(higher > lower, "{higher} should be > {lower}");
        assert!(lower < higher, "{lower} should be < {higher}");
    }

    /// SemVer §11: Equal versions
    #[rstest]
    #[case::zeros(Semantic::new("0", "0", "0", "", ""), Semantic::new("0", "0", "0", "", ""))]
    #[case::ones(Semantic::new("1", "1", "1", "", ""), Semantic::new("1", "1", "1", "", ""))]
    #[case::with_pre(Semantic::new("1", "0", "0", "alpha.1", ""), Semantic::new("1", "0", "0", "alpha.1", ""))]
    fn equal_versions(#[case] a: Semantic, #[case] b: Semantic) {
        assert_eq!(a.cmp(&b), std::cmp::Ordering::Equal, "{a} should equal {b}");
        assert_eq!(a, b, "{a} should == {b}");
    }

    // SemVer §9: A pre-release version indicates that the version is
    // unstable and might not satisfy the intended compatibility
    // requirements as denoted by its associated normal version.
    //
    // SemVer §11: When major, minor, and patch are equal, a pre-release
    // version has lower precedence than a normal version.
    // Example: 1.0.0-alpha < 1.0.0

    /// SemVer §11: Pre-release version has lower precedence than the
    /// associated normal version
    #[rstest]
    #[case::alpha_lt_release(
        Semantic::new("1", "0", "0", "alpha", ""),
        Semantic::new("1", "0", "0", "", "")
    )]
    #[case::beta_lt_release(
        Semantic::new("1", "0", "0", "beta.1", ""),
        Semantic::new("1", "0", "0", "", "")
    )]
    #[case::rc_lt_release(
        Semantic::new("1", "0", "0", "rc.1", ""),
        Semantic::new("1", "0", "0", "", "")
    )]
    #[case::pre_release_non_prod(
        Semantic::new("0", "1", "0", "alpha.1", ""),
        Semantic::new("0", "1", "0", "", "")
    )]
    fn pre_release_lower_than_release(#[case] pre: Semantic, #[case] release: Semantic) {
        assert!(pre < release, "{pre} should be < {release}");
        assert!(release > pre, "{release} should be > {pre}");
        assert_ne!(pre, release);
    }

    // SemVer §11: Precedence for two pre-release versions with the same
    // major, minor, and patch version MUST be determined by comparing
    // each dot separated identifier from left to right until a
    // difference is found.

    /// SemVer §11: Pre-release identifiers compared left to right
    #[rstest]
    #[case::alpha_lt_beta(
        Semantic::new("1", "0", "0", "alpha", ""),
        Semantic::new("1", "0", "0", "beta", "")
    )]
    #[case::alpha_1_lt_alpha_2(
        Semantic::new("1", "0", "0", "alpha.1", ""),
        Semantic::new("1", "0", "0", "alpha.2", "")
    )]
    #[case::beta_2_lt_beta_11(
        Semantic::new("1", "0", "0", "beta.2", ""),
        Semantic::new("1", "0", "0", "beta.11", "")
    )]
    #[case::beta_lt_rc(
        Semantic::new("1", "0", "0", "beta.1", ""),
        Semantic::new("1", "0", "0", "rc.1", "")
    )]
    fn pre_release_ordering(#[case] lower: Semantic, #[case] higher: Semantic) {
        assert!(lower < higher, "{lower} should be < {higher}");
        assert!(higher > lower, "{higher} should be > {lower}");
    }

    // SemVer §10: Build metadata MUST be ignored when determining
    // version precedence. Thus two versions that differ only in the
    // build metadata, have equal precedence.

    /// SemVer §10: Build metadata does not affect precedence
    #[rstest]
    #[case::different_builds(
        Semantic::new("1", "0", "0", "", "build.1"),
        Semantic::new("1", "0", "0", "", "build.2")
    )]
    #[case::build_vs_no_build(
        Semantic::new("1", "0", "0", "", "20130313144700"),
        Semantic::new("1", "0", "0", "", "")
    )]
    #[case::pre_release_different_builds(
        Semantic::new("1", "0", "0", "alpha.1", "001"),
        Semantic::new("1", "0", "0", "alpha.1", "exp.sha.5114f85")
    )]
    fn build_metadata_ignored_for_precedence(#[case] a: Semantic, #[case] b: Semantic) {
        assert_eq!(
            a.cmp(&b),
            std::cmp::Ordering::Equal,
            "{a} and {b} should have equal precedence"
        );
        assert_eq!(a, b, "{a} should == {b} (build metadata ignored)");
    }

    // SemVer §11 example:
    // 1.0.0-alpha < 1.0.0-alpha.1 < 1.0.0-alpha.beta < 1.0.0-beta
    //   < 1.0.0-beta.2 < 1.0.0-beta.11 < 1.0.0-rc.1 < 1.0.0

    /// SemVer §11: Full precedence example from the specification
    #[test]
    fn semver_spec_full_precedence_example() {
        let versions = [
            Semantic::new("1", "0", "0", "alpha", ""),
            Semantic::new("1", "0", "0", "alpha.1", ""),
            Semantic::new("1", "0", "0", "alpha.beta", ""),
            Semantic::new("1", "0", "0", "beta", ""),
            Semantic::new("1", "0", "0", "beta.2", ""),
            Semantic::new("1", "0", "0", "beta.11", ""),
            Semantic::new("1", "0", "0", "rc.1", ""),
            Semantic::new("1", "0", "0", "", ""),
        ];

        // Each version should be less than the next
        for i in 0..versions.len() - 1 {
            assert!(
                versions[i] < versions[i + 1],
                "{} should be < {}",
                versions[i],
                versions[i + 1]
            );
        }
    }

    /// SemVer §11: Sorting a shuffled list produces the correct order
    #[test]
    fn sort_produces_semver_order() {
        let mut versions = [
            Semantic::new("1", "0", "0", "", ""),
            Semantic::new("1", "0", "0", "beta.11", ""),
            Semantic::new("1", "0", "0", "alpha", ""),
            Semantic::new("1", "0", "0", "rc.1", ""),
            Semantic::new("1", "0", "0", "beta", ""),
            Semantic::new("1", "0", "0", "alpha.1", ""),
            Semantic::new("1", "0", "0", "beta.2", ""),
            Semantic::new("1", "0", "0", "alpha.beta", ""),
        ];

        versions.sort();

        let expected = [
            "1.0.0-alpha",
            "1.0.0-alpha.1",
            "1.0.0-alpha.beta",
            "1.0.0-beta",
            "1.0.0-beta.2",
            "1.0.0-beta.11",
            "1.0.0-rc.1",
            "1.0.0",
        ];

        let sorted_strings: Vec<String> = versions.iter().map(|v| v.to_string()).collect();
        assert_eq!(sorted_strings, expected);
    }

    /// SemVer: Sorting mixed versions with different major/minor/patch
    #[test]
    fn sort_mixed_versions() {
        let mut versions = [
            Semantic::new("2", "0", "0", "", ""),
            Semantic::new("1", "0", "0", "alpha", ""),
            Semantic::new("1", "0", "0", "", ""),
            Semantic::new("0", "1", "0", "", ""),
            Semantic::new("1", "1", "0", "", ""),
            Semantic::new("1", "0", "0", "rc.1", ""),
            Semantic::new("0", "0", "1", "", ""),
            Semantic::new("1", "0", "1", "", ""),
        ];

        versions.sort();

        let expected = [
            "0.0.1", "0.1.0", "1.0.0-alpha", "1.0.0-rc.1", "1.0.0", "1.0.1", "1.1.0", "2.0.0",
        ];

        let sorted_strings: Vec<String> = versions.iter().map(|v| v.to_string()).collect();
        assert_eq!(sorted_strings, expected);
    }

    /// SemVer §10: Build metadata does not change sort order
    #[test]
    fn sort_ignores_build_metadata() {
        let mut versions = [
            Semantic::new("1", "0", "0", "", "build.2"),
            Semantic::new("0", "9", "0", "", ""),
            Semantic::new("1", "0", "0", "", "build.1"),
            Semantic::new("1", "0", "1", "", ""),
        ];

        versions.sort();

        // 0.9.0 first, then two 1.0.0 (equal precedence regardless of
        // build), then 1.0.1
        assert_eq!(versions[0].to_string(), "0.9.0");
        assert!(versions[1].major == 1 && versions[1].minor == 0 && versions[1].patch == 0);
        assert!(versions[2].major == 1 && versions[2].minor == 0 && versions[2].patch == 0);
        assert_eq!(versions[3].to_string(), "1.0.1");
    }
}
