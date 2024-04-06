use std::{cmp::Ordering, fmt};

use regex::Regex;

use crate::Error;

use super::Semantic;

/// The VersionTag data structure represents a git tag containing a
/// semantic version number.
///
#[derive(Debug, Default, Clone)]
pub struct VersionTag {
    pub(crate) refs: String,
    pub(crate) tag_prefix: String,
    pub(crate) version_prefix: String,
    pub(crate) semantic_version: Semantic,
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
            version_number_valid(tag, version_prefix)?;
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

fn version_number_valid(tag: &str, version_prefix: &str) -> Result<(), Error> {
    log::debug!("Validating the tag {tag} with version identified by {version_prefix}");
    let re = Regex::new(version_prefix).unwrap();
    let m_res = re.find(tag);

    // the tag string must start with the version_prefix
    let Some(m) = m_res else {
        return Err(Error::NotVersionTag(
            version_prefix.to_string(),
            tag.to_string(),
        ));
    };

    log::debug!("The found: {m:?}");
    let (_prefix, version) = tag.split_at(m.end());
    log::debug!("The version string is: {version}");
    // Remove any build data from the end
    if let Some((version, _build)) = version.rsplit_once('+') {
        log::debug!("The version after build is stripped is: {version}");
    };
    // Remove any build data from the end
    if let Some((version, _pre_release)) = version.rsplit_once('-') {
        log::debug!("The version after pre release is stripped is: {version}");
    };

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

#[cfg(test)]
mod tests {

    use rstest::rstest;

    use crate::semantic::{test_utils::gen_current_version, PreRelease};

    use super::*;

    fn version_tag_example_one() -> VersionTag {
        let pre_release = PreRelease::new("alpha.4");
        gen_current_version("v", 2, 9, 17, Some(pre_release), Some("2000".to_string()))
    }

    fn version_tag_example_two() -> VersionTag {
        let pre_release = PreRelease::new("beta.1");
        gen_current_version("v", 1, 17, 3, Some(pre_release), Some("2000".to_string()))
    }

    #[test]
    fn test_partial_eq() {
        let tag1 = version_tag_example_one();
        let tag2 = version_tag_example_one();
        assert_eq!(tag1, tag2);
    }

    #[test]
    fn test_eq() {
        let tag1 = version_tag_example_one();
        let tag2 = version_tag_example_one();
        assert!(tag1 == tag2);
    }

    #[test]
    fn test_partial_ord() {
        let tag1 = version_tag_example_one();
        let tag2 = version_tag_example_two();
        assert!(tag1 > tag2);
    }

    #[test]
    fn test_ord() {
        let tag1 = version_tag_example_one();
        let tag2 = version_tag_example_two();
        assert_eq!(tag1.cmp(&tag2), std::cmp::Ordering::Greater);
    }

    #[rstest]
    #[case::non_prod("v", 0, 7, 9, "", "", "refs/tags/v0.7.9")]
    #[case::first_alpha("v", 1, 0, 0, "alpha.1", "", "refs/tags/v1.0.0-alpha.1")]
    #[case::alpha_with_build("v", 1, 0, 0, "alpha.2", "10", "refs/tags/v1.0.0-alpha.2+10")]
    #[case::beta_with_build("v", 1, 0, 0, "beta.1", "30", "refs/tags/v1.0.0-beta.1+30")]
    #[case::release_candidate("v", 1, 0, 0, "rc.1", "40", "refs/tags/v1.0.0-rc.1+40")]
    #[case::first_version("v", 1, 0, 0, "", "", "refs/tags/v1.0.0")]
    #[case::patched_first_version("v", 1, 0, 1, "", "", "refs/tags/v1.0.1")]
    #[case::minor_update_first_version("v", 1, 1, 0, "", "", "refs/tags/v1.1.0")]
    #[case::custom_pre_release(
        "v",
        2,
        0,
        0,
        "pre.1",
        "circle.1",
        "refs/tags/v2.0.0-pre.1+circle.1"
    )]
    #[case::alphanumeric_build(
        "v",
        2,
        0,
        0,
        "pre.2",
        "circle.14",
        "refs/tags/v2.0.0-pre.2+circle.14"
    )]
    fn display_value(
        #[case] prefix: &str,
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
        let test_version =
            gen_current_version(prefix, major, minor, patch, pre_release, build_meta_data);
        assert_eq!(expected, test_version.to_string().as_str());
    }

    #[rstest]
    #[case::simple_version("refs/tags/v0.7.9", "v", true)]
    #[case::pre_release_version("refs/tags/ver1.0.0-alpha.1", "ver", true)]
    #[case::alpha_with_build("refs/tags/1.0.0-alpha.2+10", "", true)]
    #[case::invalid_version_prefix("refs/tags/ver1.0.0-beta.1+30", "v", true)]
    #[case::invalid_version_number("refs/tags/v1.a.0-rc.1+40", "v", false)]
    #[case::first_version("refs/tags/v1.0.0", "v", true)]
    #[case::patched_first_version("refs/tags/v1.0.1", "v", true)]
    #[case::minor_update_first_version("refs/tags/v1.1.0", "v", true)]
    #[case::custom_pre_release("refs/tags/v2.0.0-pre.1+circle.1", "v", true)]
    #[case::alphanumeric_build("refs/tags/v2.0.0-pre.2+circle.14", "v", true)]
    fn parse_value(#[case] input: &str, #[case] version_prefix: &str, #[case] expected: bool) {
        use log::LevelFilter;
        use log4rs_test_utils::test_logging;

        test_logging::init_logging_once_for(vec![], LevelFilter::Debug, None);

        let result = VersionTag::parse(input, version_prefix);
        assert_eq!(expected, result.is_ok());
    }

    #[rstest]
    #[case::simple_version("refs/tags/v0.7.9", "v", true)]
    #[case::pre_release_version("refs/tags/ver1.0.0-alpha.1", "ver", true)]
    #[case::alpha_with_build("refs/tags/1.0.0-alpha.2+10", "", true)]
    #[case::invalid_version_prefix("refs/tags/ver1.0.0-beta.1+30", "v", true)]
    #[case::invalid_version_number("refs/tags/v1.a.0-rc.1+40", "v", false)]
    #[case::first_version("refs/tags/v1.0.0", "v", true)]
    #[case::patched_first_version("refs/tags/v1.0.1", "v", true)]
    #[case::minor_update_first_version("refs/tags/v1.1.0", "v", true)]
    #[case::custom_pre_release("refs/tags/v2.0.0-pre.1+circle.1", "v", true)]
    #[case::alphanumeric_build("refs/tags/v2.0.0-pre.2+circle.14", "v", true)]
    fn tag_validation(#[case] input: &str, #[case] version_prefix: &str, #[case] expected: bool) {
        use log::LevelFilter;
        use log4rs_test_utils::test_logging;

        test_logging::init_logging_once_for(vec![], LevelFilter::Debug, None);

        let result = VersionTag::parse(input, version_prefix);

        assert_eq!(expected, result.is_ok());
    }
}
