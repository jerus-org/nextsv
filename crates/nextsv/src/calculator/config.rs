use std::{collections::HashSet, ffi::OsString};

use crate::{Calculator, Error, ForceBump, Hierarchy};

/// Captures the user configuration set for the bump and version number
/// calculation
///
/// The configuration is set following the builder pattern and final build
/// command creates a [`Calculator`] struct.
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct CalculatorConfig {
    /// Required: version number prefix
    pub(crate) prefix: String,
    /// Optional: subdir filter for subordinate packages
    pub(crate) subdir: Option<String>,
    /// Optional: package to calculate from workspace
    pub(crate) package: Option<String>,
    /// Optional: Force the calculation to return the specified bump level
    pub(crate) force: Option<ForceBump>,
    /// Optional: Force the first version to be calculated as 1.0.0
    pub force_first_version: bool,
    /// Report the level bump for the version change [default: true]
    pub(crate) report_bump: bool,
    /// Report the calculated next version number  [default: false]
    pub(crate) report_number: bool,
    /// Optional: Files that must be updated before making the release
    pub(crate) files: HashSet<OsString>,
    /// Level at which file updates should be enforced [default: feature]
    pub(crate) enforce: Hierarchy,
    /// Threshold level at which release should proceed [default: Other]
    /// Returns Level::None if the threshold is not met.
    pub(crate) threshold: Hierarchy,
}

impl CalculatorConfig {
    /// Initialise a new calculator config by providing the version prefix.
    ///
    /// The version prefix identifies the start of the version string in the tag.
    pub fn new() -> CalculatorConfig {
        CalculatorConfig {
            report_bump: true,
            ..Default::default()
        }
    }

    /// Set the version prefix.
    ///
    /// The version prefix identifies the start of the version string in the tag.
    ///
    /// # Example
    ///
    /// Where you have a version tag v0.7.9 the configuration should be set as follows:
    ///
    /// ```no_run
    /// # fn main() -> Result<(),nextsv::Error> {
    /// # use nextsv::CalculatorConfig;
    ///     let calculator = CalculatorConfig::new()
    ///         .set_prefix("v")
    ///         .build()?;
    ///
    ///     calculator.report();
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_prefix(mut self, version_prefix: &str) -> Self {
        self.prefix = version_prefix.to_string();
        self
    }

    /// Set the optional subdir for commit analysis.
    ///
    /// The subdir identifies a string by which the commits will be filtered.
    /// The objective of the filtering is to limit the calculation to commits related
    /// to one crate within a workspace.
    ///
    /// # Example
    ///
    /// Where you have a crate called crate2 in a workspace and wish to calculate the
    /// version for that crate only.
    ///
    /// Note:
    ///
    /// A version prefix should be used to identify the relevant crate version tag.
    ///
    /// ```no_run
    /// # fn main() -> Result<(),nextsv::Error> {
    /// # use nextsv::CalculatorConfig;
    ///     let calculator = CalculatorConfig::new()
    ///         .set_prefix("crate2-v")
    ///         .set_subdir(Some("crate2"))
    ///         .build()?;
    ///
    ///     calculator.report();
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_subdir(mut self, subdir: Option<&str>) -> Self {
        self.subdir = subdir.map(|subdir| subdir.to_string());
        self
    }

    /// Set the optional package for workspace subset.
    ///
    /// The package identifies a subset of a workspace to consider when calculating the next version.
    ///
    /// # Example
    ///
    /// Where you have a crate called crate2 in a workspace and wish to calculate the
    /// version for that crate only.
    ///
    /// Note:
    ///
    /// A version prefix should be used to identify the relevant crate version tag.
    ///
    /// ```no_run
    /// # fn main() -> Result<(),nextsv::Error> {
    /// # use nextsv::CalculatorConfig;
    ///     let calculator = CalculatorConfig::new()
    ///         .set_prefix("crate2-v")
    ///         .set_package(Some("crate2"))
    ///         .build()?;
    ///
    ///     calculator.report();
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_package(mut self, subdir: Option<&str>) -> Self {
        self.package = subdir.map(|package| package.to_string());
        self
    }

    /// Set the flag indicating if the bump should be reported by the [`Calculator::report`] method.
    /// - `true` indicates that the value should be reported
    /// - `false` indicates that the value should not be reported
    pub fn set_bump_report(mut self, bump_report: bool) -> Self {
        self.report_bump = bump_report;
        self
    }

    /// Set the flag indicating if the calculated version number should be reported by the [`Calculator::report`] method.
    /// - `true` indicates that the value should be reported
    /// - `false` indicates that the value should not be reported
    pub fn set_version_report(mut self, report_number: bool) -> Self {
        self.report_number = report_number;
        self
    }

    /// Force the bump result ignoring the bump that would be indicated by an analysis of conventional commits.
    ///
    /// The forced bump result will be reported and the next version will be calculated based on the forced bump level.
    ///
    /// # Example
    ///
    /// Publish the first production release.
    ///
    /// ```no_run
    /// # fn main() -> Result<(),nextsv::Error> {
    /// # use nextsv::{CalculatorConfig, ForceBump};
    ///     let calculator = CalculatorConfig::new()
    ///         .set_prefix("v")
    ///         .set_force_bump(ForceBump::First)
    ///         .build()?;
    ///
    ///     calculator.report();
    /// # Ok(())
    /// # }
    /// ```
    /// Produces the output:
    ///
    /// ```console
    /// v1.0.0
    /// ```
    pub fn set_force_bump(mut self, force_level: ForceBump) -> Self {
        self.force = Some(force_level);
        self
    }

    /// Add a list of files that should be updated if the calculated level of the
    /// conventional commits analysed meets or exceeds the enforcement level set
    /// by [`CalculatorConfig::set_required_enforcement`].
    ///
    /// # Examples
    ///
    /// ## Using `feature` (default enforcement level)
    ///
    /// Require that the `README.md` and `CHANGELOG.md` are updated for the calculation
    /// to report successfully if the calculated bump is `feature` or higher.
    ///
    /// ```no_run
    /// # fn main() -> Result<(),nextsv::Error> {
    /// # use std::ffi::OsString;
    /// # use nextsv::CalculatorConfig;
    ///     let required_files = vec![
    ///         OsString::from("README.md"),
    ///         OsString::from("CHANGELOG.md"),
    ///         ];
    ///
    ///     let calculator = CalculatorConfig::new()
    ///         .set_prefix("v")
    ///         .add_required_files(required_files)
    ///         .build()?;
    ///
    ///     calculator.report();
    /// # Ok(())
    /// # }
    /// ```
    /// The bump `minor` confirms that the checked files have been updated by the
    /// conventional commits submitted as part of proposed release.
    ///
    /// ```console
    /// minor
    /// ```
    ///
    /// In the event that one or more of the files was not found but the highest commit reached
    /// the `feature` threshold the response would be `none`.
    ///
    /// ```console
    /// none
    /// ```
    ///
    /// ## Setting `breaking` as the enforcement level
    ///
    /// Require that the `README.md` and `CHANGELOG.md` are updated for the calculation
    /// to report successfully if the calculated bump is `breaking` or higher.
    ///
    /// See
    ///
    /// ```no_run
    /// # fn main() -> Result<(),nextsv::Error> {
    /// # use std::ffi::OsString;
    /// # use nextsv::{CalculatorConfig, Hierarchy};
    /// // Current version is 1.2.0
    ///     let required_files = vec![
    ///         OsString::from("README.md"),
    ///         OsString::from("CHANGELOG.md"),
    ///         ];
    ///
    ///     let calculator = CalculatorConfig::new()
    ///         .set_prefix("v")
    ///         .add_required_files(required_files)
    ///         .set_required_enforcement(Hierarchy::Breaking)
    ///         .build()?;
    ///
    ///     calculator.report();
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// The bump `minor` indicates that the highest conventional commit in the hierarchy
    /// is a `feat` commit. The commits may or may not includes changes to the required files.
    ///
    /// ```console
    /// minor
    /// ```
    ///
    /// The bump `major` confirms that the checked files have been updated by the
    /// conventional commits submitted as part of proposed release.
    ///
    /// ```console
    /// major
    /// ```
    ///
    /// In the event that one or more of the files was not found but the highest commit reached
    /// the `breaking` threshold the response would be `none`.
    ///
    /// ```console
    /// none
    /// ```
    ///
    /// Files that may be listed would include README.md and CHANGELOG.md.
    ///
    pub fn add_required_files(mut self, files: Vec<OsString>) -> Self {
        if !files.is_empty() {
            for file in files {
                self.files.insert(file);
            }
        }

        self
    }

    /// Sets the enforcement for the files submitted in [`CalculatorConfig::add_required_files`] according to the [`Hierarchy`] enum.
    ///
    /// # Example
    ///
    /// See the second example in [`CalculatorConfig::add_required_files`] for the use of `set_required_enforcement`
    pub fn set_required_enforcement(mut self, enforce: Hierarchy) -> Self {
        self.enforce = enforce;
        self
    }

    /// Set a threshold that must be met before the calculated bump is reported.
    ///
    /// The threshold is set based on the [`Hierarchy`] enum.
    ///
    /// # Example
    ///
    /// Set a threshold of `Hierarchy::Feature` for reporting of the change bump.
    ///
    /// ```no_run
    /// # fn main() -> Result<(),nextsv::Error> {
    /// # use nextsv::{CalculatorConfig, Hierarchy};
    ///     let calculator = CalculatorConfig::new()
    ///         .set_prefix("v")
    ///         .set_reporting_threshold(Hierarchy::Feature)
    ///         .build()?;
    ///
    ///     calculator.report();
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// The bump `minor` indicates that the highest conventional commit in the hierarchy
    /// is a `feat` commit.
    ///
    /// ```console
    /// minor
    /// ```
    ///
    ///  If the threshold is not met bump is `none`
    ///
    /// ```console
    /// none
    /// ```
    pub fn set_reporting_threshold(mut self, threshold: Hierarchy) -> Self {
        self.threshold = threshold;
        self
    }

    /// Set the flag to force the first version to be calculated as 1.0.0 to true. This is
    /// useful in combination with a pre-release to create a pre-release for the for the
    /// first production release.
    pub fn set_first_version(mut self) -> Self {
        self.force_first_version = true;
        self
    }

    /// Executes the calculator with the `CalculatorConfig` returning a completed
    /// [`Calculator`] or an [`Error`].
    ///
    pub fn build(self) -> Result<Calculator, Error> {
        log::debug!("Config at build: {self:?}");
        Calculator::execute(self)
    }
}

#[cfg(test)]
mod test {
    use std::ffi::OsString;

    use map_macro::hash_set;
    use rstest::{fixture, rstest};
    use std::collections::HashSet;

    use super::CalculatorConfig;
    use crate::{ForceBump, Hierarchy};

    fn default_calculator_config() -> CalculatorConfig {
        CalculatorConfig {
            prefix: String::from(""),
            subdir: None,
            package: None,
            force: None,
            force_first_version: false,
            report_bump: true,
            report_number: false,
            files: hash_set![],
            enforce: Hierarchy::Other,
            threshold: Hierarchy::Other,
        }
    }

    #[test]
    fn test_default_calculator_config() {
        let expected = default_calculator_config();
        let test = CalculatorConfig::new();

        assert_eq!(expected, test);
    }

    #[rstest]
    #[case::alpha(ForceBump::Alpha, Some(ForceBump::Alpha))]
    #[case::beta(ForceBump::Beta, Some(ForceBump::Beta))]
    #[case::first(ForceBump::First, Some(ForceBump::First))]
    #[case::major(ForceBump::Major, Some(ForceBump::Major))]
    #[case::minor(ForceBump::Minor, Some(ForceBump::Minor))]
    #[case::patch(ForceBump::Patch, Some(ForceBump::Patch))]
    #[case::rc(ForceBump::Rc, Some(ForceBump::Rc))]
    #[case::release(ForceBump::Release, Some(ForceBump::Release))]
    fn test_set_force_to(#[case] force: ForceBump, #[case] expected_force: Option<ForceBump>) {
        let test = CalculatorConfig::new().set_prefix("v").clone();
        let test = test.set_force_bump(force);

        let expected = CalculatorConfig {
            prefix: String::from("v"),
            report_bump: true,
            force: expected_force,
            ..Default::default()
        };

        assert_eq!(expected, test);
    }

    #[fixture]
    fn default_config() -> CalculatorConfig {
        CalculatorConfig {
            prefix: String::from("v"),
            report_bump: true,
            ..Default::default()
        }
    }

    #[fixture]
    fn readme_config() -> CalculatorConfig {
        let files = hash_set![OsString::from("README.md")];

        CalculatorConfig {
            prefix: String::from("v"),
            report_bump: true,
            files,
            ..Default::default()
        }
    }

    #[fixture]
    fn readme_and_changes_input() -> Vec<OsString> {
        vec![OsString::from("CHANGES.md"), OsString::from("README.md")]
    }

    #[fixture]
    fn readme_file_input() -> Vec<OsString> {
        vec![OsString::from("README.md")]
    }

    #[fixture]
    fn changes_file_input() -> Vec<OsString> {
        vec![OsString::from("CHANGES.md")]
    }

    #[fixture]
    fn multiple_files_not_readme_input() -> Vec<OsString> {
        vec![OsString::from("CHANGELOG.md"), OsString::from("CHANGES.md")]
    }

    #[fixture]
    fn multiple_files_including_readme_input() -> Vec<OsString> {
        vec![
            OsString::from("CHANGELOG.md"),
            OsString::from("CHANGES.md"),
            OsString::from("README.md"),
        ]
    }

    #[fixture]
    fn readme_and_changes_expected() -> HashSet<OsString> {
        hash_set![OsString::from("CHANGES.md"), OsString::from("README.md")]
    }

    #[fixture]
    fn readme_file_expected() -> HashSet<OsString> {
        hash_set![OsString::from("README.md")]
    }

    #[fixture]
    fn changes_file_expected() -> HashSet<OsString> {
        hash_set![OsString::from("CHANGES.md")]
    }

    #[fixture]
    fn multiple_files_not_readme_expected() -> HashSet<OsString> {
        hash_set![OsString::from("CHANGELOG.md"), OsString::from("CHANGES.md")]
    }

    #[fixture]
    fn multiple_files_including_readme_expected() -> HashSet<OsString> {
        hash_set![
            OsString::from("CHANGELOG.md"),
            OsString::from("CHANGES.md"),
            OsString::from("README.md"),
        ]
    }

    #[rstest]
    #[case::add_readme_file_to_empty_list(
        default_config(),
        readme_file_input(),
        readme_file_expected()
    )]
    #[case::add_changes_file_to_list_with_readme(
        readme_config(),
        changes_file_input(),
        readme_and_changes_expected()
    )]
    #[case::add_multiple_files_to_empty_list(
        default_config(),
        multiple_files_including_readme_input(),
        multiple_files_including_readme_expected()
    )]
    #[case::add_readme_file_to_list_with_readme(
        readme_config(),
        readme_file_input(),
        readme_file_expected()
    )]
    #[case::add_multiple_files_to_list_with_readme(
        readme_config(),
        multiple_files_not_readme_input(),
        multiple_files_including_readme_expected()
    )]
    #[case::add_multiple_files_including_readme_to_list_with_readme(
        readme_config(),
        multiple_files_including_readme_input(),
        multiple_files_including_readme_expected()
    )]
    fn test_add_required_files(
        #[case] initial_config: CalculatorConfig,
        #[case] additional_files: Vec<OsString>,
        #[case] expected_files: HashSet<OsString>,
    ) {
        let test = initial_config.add_required_files(additional_files);

        let expected = CalculatorConfig {
            prefix: String::from("v"),
            report_bump: true,
            files: expected_files,
            ..Default::default()
        };

        assert_eq!(expected, test);
    }

    #[rstest]
    #[case::breaking(Hierarchy::Breaking, Hierarchy::Breaking)]
    #[case::feature(Hierarchy::Feature, Hierarchy::Feature)]
    #[case::fix(Hierarchy::Fix, Hierarchy::Fix)]
    #[case::other(Hierarchy::Other, Hierarchy::Other)]
    fn test_set_enforcement_level_to(
        #[case] enforcement_level: Hierarchy,
        #[case] expected_level: Hierarchy,
    ) {
        let test = CalculatorConfig::new().set_prefix("v").clone();
        let test = test.set_required_enforcement(enforcement_level);

        let expected = CalculatorConfig {
            prefix: String::from("v"),
            report_bump: true,
            enforce: expected_level,
            ..Default::default()
        };

        assert_eq!(expected, test);
    }

    #[rstest]
    #[case::breaking(Hierarchy::Breaking, Hierarchy::Breaking)]
    #[case::feature(Hierarchy::Feature, Hierarchy::Feature)]
    #[case::fix(Hierarchy::Fix, Hierarchy::Fix)]
    #[case::other(Hierarchy::Other, Hierarchy::Other)]
    fn test_set_threshold_to(#[case] threshold: Hierarchy, #[case] expected_threshold: Hierarchy) {
        let test = CalculatorConfig::new().set_prefix("v").clone();
        let test = test.set_reporting_threshold(threshold);

        let expected = CalculatorConfig {
            prefix: String::from("v"),
            report_bump: true,
            threshold: expected_threshold,
            ..Default::default()
        };

        assert_eq!(expected, test);
    }
}
