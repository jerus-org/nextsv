use crate::version::Semantic;

use super::bump::Bump;
#[allow(unused_imports)]
use super::CalculatorConfig; // import used by documentation
use clap::ValueEnum;
use std::{cmp, fmt};

/// This enum is used by [`CalculatorConfig::set_force_bump`] to override the bump
/// that would be calculated from conventional commits.
///
/// ## Variants
///
/// Variants are valid depending on the current version type.
///
/// |  Variant  | Non Prodution |  Pre Release  |  Production   |              Description                  |
/// | ----------|---------------| --------------|---------------|-------------------------------------------|
/// | `Major`   |       X       |               |       X       | Bump the major component                  |
/// | `Minor`   |       X       |               |       X       | Bump the minor component                  |
/// | `Patch`   |       X       |               |       X       | Bump the patch component                  |
/// | `First`   |       X       |               |               | Set the first production version (1.0.0)  |
/// | `Release` |               |       X       |               | Remove the pre-release                    |
/// | `Rc`      |       X       |       X       |       X       | Bump or create rc pre-release             |
/// | `Beta`    |       X       |       X       |       X       | Bump or create beta pre-release           |
/// | `Alpha`   |       X       |       X       |       X       | Bump or create alpha pre-release          |
///
///  Where it is not valid the bump is forced to [`None`].
#[derive(Debug, PartialEq, Eq, Clone, ValueEnum)]
pub enum ForceBump {
    /// Bump the major version component.
    Major,
    /// Bump the major version component.
    Minor,
    /// Bump the major version component.
    Patch,
    /// Release first production (1.0.0) version.
    First,
    /// Remove the pre-release version component if present.
    Release,
    /// Bump or create the rc pre-release version component.
    Rc,
    /// Bump or create the beta pre-release version component.
    Beta,
    /// Bump or create the alpha pre-release version component.
    Alpha,
}

use crate::version::VersionType;

impl ForceBump {
    /// Returns the bump type that should be used to calculate the next version.
    pub(crate) fn to_bump(&self, version_number: &Semantic) -> Bump {
        log::debug!(
            "ForceBump::to_bump({}) with version `{}`",
            self,
            version_number
        );
        match version_number.version_type() {
            VersionType::NonProduction => match self {
                ForceBump::Major => Bump::Major,
                ForceBump::Minor => Bump::Minor,
                ForceBump::Patch => Bump::Patch,
                ForceBump::First => Bump::First,
                ForceBump::Release => Bump::None,
                ForceBump::Rc => Bump::Rc,
                ForceBump::Beta => Bump::Beta,
                ForceBump::Alpha => Bump::Alpha,
            },
            VersionType::PreRelease => match self {
                ForceBump::Major => Bump::None,
                ForceBump::Minor => Bump::None,
                ForceBump::Patch => Bump::None,
                ForceBump::First => Bump::None,
                ForceBump::Release => Bump::Release,
                ForceBump::Rc => {
                    if version_number.is_pre_release("rc") {
                        Bump::Rc
                    } else {
                        Bump::None
                    }
                }
                ForceBump::Beta => {
                    if version_number.is_pre_release("beta") {
                        Bump::Beta
                    } else {
                        Bump::None
                    }
                }
                ForceBump::Alpha => {
                    if version_number.is_pre_release("alpha") {
                        Bump::Alpha
                    } else {
                        Bump::None
                    }
                }
            },
            VersionType::Production => match self {
                ForceBump::Major => Bump::Major,
                ForceBump::Minor => Bump::Minor,
                ForceBump::Patch => Bump::Patch,
                ForceBump::First => Bump::None,
                ForceBump::Release => Bump::None,
                ForceBump::Rc => Bump::Rc,
                ForceBump::Beta => Bump::Beta,
                ForceBump::Alpha => Bump::Alpha,
            },
        }
    }
}

impl fmt::Display for ForceBump {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ForceBump::Major => write!(f, "major"),
            ForceBump::Minor => write!(f, "minor"),
            ForceBump::Patch => write!(f, "patch"),
            ForceBump::First => write!(f, "1.0.0"),
            ForceBump::Alpha => write!(f, "alpha"),
            ForceBump::Beta => write!(f, "beta"),
            ForceBump::Rc => write!(f, "rc"),
            ForceBump::Release => write!(f, "release"),
        }
    }
}

impl Ord for ForceBump {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match (self, other) {
            (ForceBump::Major, ForceBump::Major)
            | (ForceBump::Minor, ForceBump::Minor)
            | (ForceBump::Patch, ForceBump::Patch)
            | (ForceBump::First, ForceBump::First)
            | (ForceBump::Release, ForceBump::Release)
            | (ForceBump::Rc, ForceBump::Rc)
            | (ForceBump::Beta, ForceBump::Beta)
            | (ForceBump::Alpha, ForceBump::Alpha) => cmp::Ordering::Equal,
            (ForceBump::Major, _) => cmp::Ordering::Greater,
            (ForceBump::Alpha, _) => cmp::Ordering::Less,
            (ForceBump::Minor, ForceBump::Major) => cmp::Ordering::Less,
            (ForceBump::Minor, ForceBump::Patch)
            | (ForceBump::Minor, ForceBump::First)
            | (ForceBump::Minor, ForceBump::Release)
            | (ForceBump::Minor, ForceBump::Rc)
            | (ForceBump::Minor, ForceBump::Beta)
            | (ForceBump::Minor, ForceBump::Alpha) => cmp::Ordering::Greater,
            (ForceBump::Patch, ForceBump::Major) | (ForceBump::Patch, ForceBump::Minor) => {
                cmp::Ordering::Less
            }
            (ForceBump::Patch, ForceBump::First)
            | (ForceBump::Patch, ForceBump::Release)
            | (ForceBump::Patch, ForceBump::Rc)
            | (ForceBump::Patch, ForceBump::Beta)
            | (ForceBump::Patch, ForceBump::Alpha) => cmp::Ordering::Greater,
            (ForceBump::First, ForceBump::Major)
            | (ForceBump::First, ForceBump::Minor)
            | (ForceBump::First, ForceBump::Patch) => cmp::Ordering::Less,
            (ForceBump::First, ForceBump::Release)
            | (ForceBump::First, ForceBump::Rc)
            | (ForceBump::First, ForceBump::Beta)
            | (ForceBump::First, ForceBump::Alpha) => cmp::Ordering::Greater,
            (ForceBump::Release, ForceBump::Major)
            | (ForceBump::Release, ForceBump::Minor)
            | (ForceBump::Release, ForceBump::Patch)
            | (ForceBump::Release, ForceBump::First) => cmp::Ordering::Less,
            (ForceBump::Release, ForceBump::Rc)
            | (ForceBump::Release, ForceBump::Beta)
            | (ForceBump::Release, ForceBump::Alpha) => cmp::Ordering::Greater,
            (ForceBump::Rc, ForceBump::Major)
            | (ForceBump::Rc, ForceBump::Minor)
            | (ForceBump::Rc, ForceBump::Patch)
            | (ForceBump::Rc, ForceBump::First)
            | (ForceBump::Rc, ForceBump::Release) => cmp::Ordering::Less,
            (ForceBump::Rc, ForceBump::Beta) | (ForceBump::Rc, ForceBump::Alpha) => {
                cmp::Ordering::Greater
            }
            (ForceBump::Beta, ForceBump::Major)
            | (ForceBump::Beta, ForceBump::Minor)
            | (ForceBump::Beta, ForceBump::Patch)
            | (ForceBump::Beta, ForceBump::First)
            | (ForceBump::Beta, ForceBump::Release) => cmp::Ordering::Less,
            (ForceBump::Beta, ForceBump::Rc) => cmp::Ordering::Less,
            (ForceBump::Beta, ForceBump::Alpha) => cmp::Ordering::Greater,
        }
    }
}

impl PartialOrd for ForceBump {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {

    use std::cmp;

    use super::ForceBump;
    use rstest::rstest;

    #[rstest]
    #[case::patch(ForceBump::Patch, "patch")]
    #[case::minor(ForceBump::Minor, "minor")]
    #[case::major(ForceBump::Major, "major")]
    #[case::release(ForceBump::Release, "release")]
    #[case::alpha(ForceBump::Alpha, "alpha")]
    #[case::beta(ForceBump::Beta, "beta")]
    #[case::rc(ForceBump::Rc, "rc")]
    #[case::first(ForceBump::First, "1.0.0")]
    fn display_value(#[case] test: ForceBump, #[case] expected: &str) {
        assert_eq!(expected, test.to_string().as_str());
    }

    #[rstest]
    #[case::major(ForceBump::Major, ForceBump::Major, cmp::Ordering::Equal)]
    #[case::minor(ForceBump::Minor, ForceBump::Minor, cmp::Ordering::Equal)]
    #[case::patch(ForceBump::Patch, ForceBump::Patch, cmp::Ordering::Equal)]
    #[case::first(ForceBump::First, ForceBump::First, cmp::Ordering::Equal)]
    #[case::release(ForceBump::Release, ForceBump::Release, cmp::Ordering::Equal)]
    #[case::rc(ForceBump::Rc, ForceBump::Rc, cmp::Ordering::Equal)]
    #[case::beta(ForceBump::Beta, ForceBump::Beta, cmp::Ordering::Equal)]
    #[case::alpha(ForceBump::Alpha, ForceBump::Alpha, cmp::Ordering::Equal)]
    #[case::major_minor(ForceBump::Major, ForceBump::Minor, cmp::Ordering::Greater)]
    #[case::minor_major(ForceBump::Minor, ForceBump::Major, cmp::Ordering::Less)]
    #[case::minor_patch(ForceBump::Minor, ForceBump::Patch, cmp::Ordering::Greater)]
    #[case::minor_first(ForceBump::Minor, ForceBump::First, cmp::Ordering::Greater)]
    #[case::minor_release(ForceBump::Minor, ForceBump::Release, cmp::Ordering::Greater)]
    #[case::minor_rc(ForceBump::Minor, ForceBump::Rc, cmp::Ordering::Greater)]
    #[case::minor_beta(ForceBump::Minor, ForceBump::Beta, cmp::Ordering::Greater)]
    #[case::minor_alpha(ForceBump::Minor, ForceBump::Alpha, cmp::Ordering::Greater)]
    #[case::patch_major(ForceBump::Patch, ForceBump::Major, cmp::Ordering::Less)]
    #[case::patch_minor(ForceBump::Patch, ForceBump::Minor, cmp::Ordering::Less)]
    #[case::patch_first(ForceBump::Patch, ForceBump::First, cmp::Ordering::Greater)]
    #[case::patch_release(ForceBump::Patch, ForceBump::Release, cmp::Ordering::Greater)]
    #[case::patch_rc(ForceBump::Patch, ForceBump::Rc, cmp::Ordering::Greater)]
    #[case::patch_beta(ForceBump::Patch, ForceBump::Beta, cmp::Ordering::Greater)]
    #[case::patch_alpha(ForceBump::Patch, ForceBump::Alpha, cmp::Ordering::Greater)]
    #[case::first_major(ForceBump::First, ForceBump::Major, cmp::Ordering::Less)]
    #[case::first_minor(ForceBump::First, ForceBump::Minor, cmp::Ordering::Less)]
    #[case::first_patch(ForceBump::First, ForceBump::Patch, cmp::Ordering::Less)]
    #[case::first_release(ForceBump::First, ForceBump::Release, cmp::Ordering::Greater)]
    #[case::first_rc(ForceBump::First, ForceBump::Rc, cmp::Ordering::Greater)]
    #[case::first_beta(ForceBump::First, ForceBump::Beta, cmp::Ordering::Greater)]
    #[case::first_alpha(ForceBump::First, ForceBump::Alpha, cmp::Ordering::Greater)]
    #[case::release_major(ForceBump::Release, ForceBump::Major, cmp::Ordering::Less)]
    #[case::release_minor(ForceBump::Release, ForceBump::Minor, cmp::Ordering::Less)]
    #[case::release_patch(ForceBump::Release, ForceBump::Patch, cmp::Ordering::Less)]
    #[case::release_first(ForceBump::Release, ForceBump::First, cmp::Ordering::Less)]
    #[case::release_rc(ForceBump::Release, ForceBump::Rc, cmp::Ordering::Greater)]
    #[case::release_beta(ForceBump::Release, ForceBump::Beta, cmp::Ordering::Greater)]
    #[case::release_alpha(ForceBump::Release, ForceBump::Alpha, cmp::Ordering::Greater)]
    #[case::rc_major(ForceBump::Rc, ForceBump::Major, cmp::Ordering::Less)]
    #[case::rc_minor(ForceBump::Rc, ForceBump::Minor, cmp::Ordering::Less)]
    #[case::rc_patch(ForceBump::Rc, ForceBump::Patch, cmp::Ordering::Less)]
    #[case::rc_first(ForceBump::Rc, ForceBump::First, cmp::Ordering::Less)]
    #[case::rc_release(ForceBump::Rc, ForceBump::Release, cmp::Ordering::Less)]
    #[case::rc_beta(ForceBump::Rc, ForceBump::Beta, cmp::Ordering::Greater)]
    #[case::rc_alpha(ForceBump::Rc, ForceBump::Alpha, cmp::Ordering::Greater)]
    #[case::beta_major(ForceBump::Beta, ForceBump::Major, cmp::Ordering::Less)]
    #[case::beta_minor(ForceBump::Beta, ForceBump::Minor, cmp::Ordering::Less)]
    #[case::beta_patch(ForceBump::Beta, ForceBump::Patch, cmp::Ordering::Less)]
    #[case::beta_first(ForceBump::Beta, ForceBump::First, cmp::Ordering::Less)]
    #[case::beta_release(ForceBump::Beta, ForceBump::Release, cmp::Ordering::Less)]
    #[case::beta_rc(ForceBump::Beta, ForceBump::Rc, cmp::Ordering::Less)]
    #[case::beta_alpha(ForceBump::Beta, ForceBump::Alpha, cmp::Ordering::Greater)]
    #[case::alpha_major(ForceBump::Alpha, ForceBump::Major, cmp::Ordering::Less)]
    #[case::alpha_minor(ForceBump::Alpha, ForceBump::Minor, cmp::Ordering::Less)]
    #[case::alpha_patch(ForceBump::Alpha, ForceBump::Patch, cmp::Ordering::Less)]
    #[case::alpha_first(ForceBump::Alpha, ForceBump::First, cmp::Ordering::Less)]
    #[case::alpha_release(ForceBump::Alpha, ForceBump::Release, cmp::Ordering::Less)]
    #[case::alpha_rc(ForceBump::Alpha, ForceBump::Rc, cmp::Ordering::Less)]
    #[case::alpha_beta(ForceBump::Alpha, ForceBump::Beta, cmp::Ordering::Less)]
    fn test_cmp(#[case] a: ForceBump, #[case] b: ForceBump, #[case] expected: cmp::Ordering) {
        assert_eq!(expected, a.cmp(&b));
    }
}
