use crate::VersionTag;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub(crate) enum NextVersion {
    #[default]
    None,
    Updated(VersionTag),
}

impl NextVersion {
    pub(crate) fn version_number(&self) -> String {
        match self {
            NextVersion::Updated(version) => version.semantic_version.to_string(),
            NextVersion::None => String::from("0.0.0"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::NextVersion;

    use crate::test_utils::gen_current_version;

    use rstest::rstest;

    #[rstest]
    #[case::none(0, 0, 0, "", "0.0.0")]
    #[case::non_production(0, 7, 10, "", "0.7.10")]
    #[case::production(1, 0, 2, "", "1.0.2")]
    #[case::pre_release(1, 1, 0, "beta.2", "1.1.0-beta.2")]
    fn version_number(
        #[case] major: u32,
        #[case] minor: u32,
        #[case] patch: u32,
        #[case] pre_release: &str,
        #[case] expected: &str,
    ) {
        use crate::version::PreRelease;

        let pre_release = if pre_release.is_empty() {
            None
        } else {
            Some(PreRelease::new(pre_release))
        };

        let next_version = gen_current_version("v", major, minor, patch, pre_release, None);
        println!("next_version: {next_version:?}");
        let test = NextVersion::Updated(next_version);

        assert_eq!(expected, test.version_number());
    }
}
