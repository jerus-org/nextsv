use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(crate) enum PreReleaseType {
    Alpha,
    Beta,
    Rc,
    Custom,
}

impl fmt::Display for PreReleaseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PreReleaseType::Alpha => write!(f, "Alpha"),
            PreReleaseType::Beta => write!(f, "Beta"),
            PreReleaseType::Rc => write!(f, "RC"),
            PreReleaseType::Custom => write!(f, "Custom"),
        }
    }
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
        log::debug!("PreRelease::new({})", pre_release);
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

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::version::PreRelease;

    use super::PreReleaseType;

    #[rstest]
    #[case::alpha("alpha", PreReleaseType::Alpha, "alpha")]
    #[case::beta("beta.1", PreReleaseType::Beta, "beta.1")]
    #[case::release_candidate("rc.1", PreReleaseType::Rc, "rc.1")]
    #[case::custom("pre.2", PreReleaseType::Custom, "pre.2")]
    fn display_value(
        #[case] pre_release: &str,
        #[case] expected_type: PreReleaseType,
        #[case] expected: &str,
    ) {
        let test_version = PreRelease::new(pre_release);
        assert_eq!(test_version.pre_type, expected_type);
        assert_eq!(expected, test_version.to_string().as_str());
    }
}
