use std::ffi::OsString;

use crate::{Error, ForceLevel, Level, LevelHierarchy, VersionCalculator};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct CalculatorConfig {
    /// Required: version number prefix
    prefix: String,
    /// Optional: Force the calculation to return the specified bump level
    force: Option<ForceLevel>,
    /// Report the level bump for the version change [default: true]
    report_level: bool,
    /// Report the calculated next version number  [default: false]
    report_number: bool,
    /// Optional: Files that must be updated before making the release
    files: Vec<OsString>,
    /// Level at which file updates should be enforced [default: feature]
    enforce_level: LevelHierarchy,
    /// Optional: Threshold level at which release should proceed
    /// Returns Level::None if the threshold is not met.
    threshold: Option<Level>,
}

impl CalculatorConfig {
    fn new(version_prefix: &str) -> CalculatorConfig {
        CalculatorConfig {
            prefix: version_prefix.to_string(),
            report_level: true,
            ..Default::default()
        }
    }

    fn set_force_level(&mut self, force_level: ForceLevel) -> &mut Self {
        self.force = Some(force_level);
        self
    }

    fn add_required_files(&mut self, file_name: &mut Vec<OsString>) -> Result<&mut Self, Error> {
        if file_name.is_empty() {
            return Ok(self);
        }
        let _new_length: isize = (self.files.len() + file_name.len()).try_into()?;
        self.files.append(file_name);
        self.files.sort();
        self.files.dedup();

        Ok(self)
    }

    fn set_file_requirement_enforcement_level(
        &mut self,
        level_hierarchy: LevelHierarchy,
    ) -> &mut Self {
        self.enforce_level = level_hierarchy;
        self
    }

    fn set_threshold(&mut self, threshold: Level) -> &mut Self {
        if threshold == Level::None {
            self.threshold = None;
        } else {
            self.threshold = Some(threshold);
        }
        self
    }

    fn build_calculator(self) -> VersionCalculator {
        VersionCalculator::init(self)
    }
}

#[cfg(test)]
mod test {
    use std::ffi::OsString;

    use rstest::{fixture, rstest};

    use super::CalculatorConfig;
    use crate::{ForceLevel, Level, LevelHierarchy};

    fn default_calculator_config() -> CalculatorConfig {
        CalculatorConfig {
            prefix: String::from(""),
            force: None,
            report_level: true,
            report_number: false,
            files: vec![],
            enforce_level: LevelHierarchy::Other,
            threshold: None,
        }
    }

    #[test]
    fn test_default_calculator_config() {
        let expected = default_calculator_config();
        let test = CalculatorConfig::new("");

        assert_eq!(expected, test);
    }

    #[rstest]
    #[case::alpha(ForceLevel::Alpha, Some(ForceLevel::Alpha))]
    #[case::beta(ForceLevel::Beta, Some(ForceLevel::Beta))]
    #[case::first(ForceLevel::First, Some(ForceLevel::First))]
    #[case::major(ForceLevel::Major, Some(ForceLevel::Major))]
    #[case::minor(ForceLevel::Minor, Some(ForceLevel::Minor))]
    #[case::patch(ForceLevel::Patch, Some(ForceLevel::Patch))]
    #[case::rc(ForceLevel::Rc, Some(ForceLevel::Rc))]
    #[case::release(ForceLevel::Release, Some(ForceLevel::Release))]
    fn test_set_force_to(#[case] force: ForceLevel, #[case] expected_force: Option<ForceLevel>) {
        let mut test = CalculatorConfig::new("v");
        let test = test.set_force_level(force);

        let expected = CalculatorConfig {
            prefix: String::from("v"),
            report_level: true,
            force: expected_force,
            ..Default::default()
        };

        assert_eq!(expected, *test);
    }

    #[fixture]
    fn default_config() -> CalculatorConfig {
        CalculatorConfig {
            prefix: String::from("v"),
            report_level: true,
            ..Default::default()
        }
    }

    #[fixture]
    fn readme_config() -> CalculatorConfig {
        let files = vec![OsString::from("README.md")];

        CalculatorConfig {
            prefix: String::from("v"),
            report_level: true,
            files,
            ..Default::default()
        }
    }

    #[fixture]
    fn readme_and_changes() -> Vec<OsString> {
        vec![OsString::from("CHANGES.md"), OsString::from("README.md")]
    }

    #[fixture]
    fn readme_file() -> Vec<OsString> {
        vec![OsString::from("README.md")]
    }

    #[fixture]
    fn changes_file() -> Vec<OsString> {
        vec![OsString::from("CHANGES.md")]
    }

    #[fixture]
    fn multiple_files_not_readme() -> Vec<OsString> {
        vec![OsString::from("CHANGELOG.md"), OsString::from("CHANGES.md")]
    }

    #[fixture]
    fn multiple_files_including_readme() -> Vec<OsString> {
        vec![
            OsString::from("CHANGELOG.md"),
            OsString::from("CHANGES.md"),
            OsString::from("README.md"),
        ]
    }

    #[rstest]
    #[case::add_readme_file_to_empty_list(default_config(), readme_file(), readme_file())]
    #[case::add_changes_file_to_list_with_readme(
        readme_config(),
        changes_file(),
        readme_and_changes()
    )]
    #[case::add_multiple_files_to_empty_list(
        default_config(),
        multiple_files_including_readme(),
        multiple_files_including_readme()
    )]
    #[case::add_readme_file_to_list_with_readme(readme_config(), readme_file(), readme_file())]
    #[case::add_multiple_files_to_list_with_readme(
        readme_config(),
        multiple_files_not_readme(),
        multiple_files_including_readme()
    )]
    #[case::add_multiple_files_including_readme_to_list_with_readme(
        readme_config(),
        multiple_files_including_readme(),
        multiple_files_including_readme()
    )]
    fn test_add_required_files(
        #[case] mut initial_config: CalculatorConfig,
        #[case] mut additional_files: Vec<OsString>,
        #[case] expected_files: Vec<OsString>,
    ) {
        let test = initial_config
            .add_required_files(&mut additional_files)
            .unwrap();

        let expected = CalculatorConfig {
            prefix: String::from("v"),
            report_level: true,
            files: expected_files,
            ..Default::default()
        };

        assert_eq!(expected, *test);
    }

    #[rstest]
    #[case::breaking(LevelHierarchy::Breaking, LevelHierarchy::Breaking)]
    #[case::feature(LevelHierarchy::Feature, LevelHierarchy::Feature)]
    #[case::fix(LevelHierarchy::Fix, LevelHierarchy::Fix)]
    #[case::other(LevelHierarchy::Other, LevelHierarchy::Other)]
    fn test_set_enforcement_level_to(
        #[case] enforcement_level: LevelHierarchy,
        #[case] expected_level: LevelHierarchy,
    ) {
        let mut test = CalculatorConfig::new("v");
        let test = test.set_file_requirement_enforcement_level(enforcement_level);

        let expected = CalculatorConfig {
            prefix: String::from("v"),
            report_level: true,
            enforce_level: expected_level,
            ..Default::default()
        };

        assert_eq!(expected, *test);
    }

    #[rstest]
    #[case::alpha(Level::Alpha, Some(Level::Alpha))]
    #[case::beta(Level::Beta, Some(Level::Beta))]
    #[case::first(Level::First, Some(Level::First))]
    #[case::major(Level::Major, Some(Level::Major))]
    #[case::minor(Level::Minor, Some(Level::Minor))]
    #[case::none(Level::None, None)]
    #[case::patch(Level::Patch, Some(Level::Patch))]
    #[case::rc(Level::Rc, Some(Level::Rc))]
    #[case::release(Level::Release, Some(Level::Release))]
    fn test_set_threshold_to(#[case] threshold: Level, #[case] expected_level: Option<Level>) {
        let mut test = CalculatorConfig::new("v");
        let test = test.set_threshold(threshold);

        let expected = CalculatorConfig {
            prefix: String::from("v"),
            report_level: true,
            threshold: expected_level,
            ..Default::default()
        };

        assert_eq!(expected, *test);
    }
}
