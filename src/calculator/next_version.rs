use crate::VersionTag;

use super::bump::Bump;

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

    pub(crate) fn calculate(current_version: &VersionTag, mut bump: Bump) -> (Self, Bump) {
        let mut next_version = current_version.clone();
        log::debug!(
            "Starting version: `{}`; bump level `{}`",
            next_version,
            bump
        );

        let next_version = match bump {
            Bump::Major => {
                next_version.version_mut().major += 1;
                next_version.version_mut().minor = 0;
                next_version.version_mut().patch = 0;
                next_version
            }
            Bump::Minor => {
                next_version.version_mut().minor += 1;
                next_version.version_mut().patch = 0;
                next_version
            }
            Bump::Patch => {
                next_version.version_mut().patch += 1;
                next_version
            }
            Bump::First => {
                next_version.version_mut().major = 1;
                next_version.version_mut().minor = 0;
                next_version.version_mut().patch = 0;
                next_version
            }
            Bump::Alpha | Bump::Beta | Bump::Rc => {
                next_version.version_mut().increment_pre_release();
                next_version
            }
            Bump::Custom(_s) => {
                next_version.version_mut().increment_pre_release();
                bump = Bump::Custom(next_version.to_string());
                next_version
            }
            _ => next_version,
        };
        log::debug!("Next version is: {next_version}");

        (NextVersion::Updated(next_version), bump)
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
