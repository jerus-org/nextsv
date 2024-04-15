use std::ffi::OsString;

use crate::{Calculator, Error, ForceBump, Hierarchy};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct CalculatorConfig {
    /// Required: version number prefix
    pub(crate) prefix: String,
    /// Optional: Force the calculation to return the specified bump level
    pub(crate) force: Option<ForceBump>,
    /// Report the level bump for the version change [default: true]
    pub(crate) report_bump: bool,
    /// Report the calculated next version number  [default: false]
    pub(crate) report_number: bool,
    /// Optional: Files that must be updated before making the release
    pub(crate) files: Vec<OsString>,
    /// Level at which file updates should be enforced [default: feature]
    pub(crate) enforce_level: Hierarchy,
    /// Threshold level at which release should proceed [default: Other]
    /// Returns Level::None if the threshold is not met.
    pub(crate) threshold: Hierarchy,
}

impl CalculatorConfig {
    pub fn new(version_prefix: &str) -> CalculatorConfig {
        CalculatorConfig {
            prefix: version_prefix.to_string(),
            report_bump: true,
            ..Default::default()
        }
    }

    pub fn set_print_level(&mut self, report_level: bool) -> &mut Self {
        self.report_bump = report_level;
        self
    }

    pub fn set_print_version_number(&mut self, report_number: bool) -> &mut Self {
        self.report_number = report_number;
        self
    }

    pub fn set_force_level(&mut self, force_level: ForceBump) -> &mut Self {
        self.force = Some(force_level);
        self
    }

    pub fn add_required_files(&mut self, file_name: &mut Vec<OsString>) -> &mut Self {
        if file_name.is_empty() {
            return self;
        }
        let _new_length: isize = (self.files.len() + file_name.len())
            .try_into()
            .expect("sum of array lenghts exceeds maximum array size");
        self.files.append(file_name);
        self.files.sort();
        self.files.dedup();

        self
    }

    pub fn set_file_requirement_enforcement_level(
        &mut self,
        level_hierarchy: Hierarchy,
    ) -> &mut Self {
        self.enforce_level = level_hierarchy;
        self
    }

    pub fn set_threshold(&mut self, threshold: Hierarchy) -> &mut Self {
        self.threshold = threshold;
        self
    }

    pub fn build_calculator(self) -> Result<Calculator, Error> {
        Calculator::init(self)
    }
}

#[cfg(test)]
mod test {
    use std::ffi::OsString;

    use rstest::{fixture, rstest};

    use super::CalculatorConfig;
    use crate::{ForceBump, Hierarchy};

    fn default_calculator_config() -> CalculatorConfig {
        CalculatorConfig {
            prefix: String::from(""),
            force: None,
            report_bump: true,
            report_number: false,
            files: vec![],
            enforce_level: Hierarchy::Other,
            threshold: Hierarchy::Other,
        }
    }

    #[test]
    fn test_default_calculator_config() {
        let expected = default_calculator_config();
        let test = CalculatorConfig::new("");

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
        let mut test = CalculatorConfig::new("v");
        let test = test.set_force_level(force);

        let expected = CalculatorConfig {
            prefix: String::from("v"),
            report_bump: true,
            force: expected_force,
            ..Default::default()
        };

        assert_eq!(expected, *test);
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
        let files = vec![OsString::from("README.md")];

        CalculatorConfig {
            prefix: String::from("v"),
            report_bump: true,
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
        let test = initial_config.add_required_files(&mut additional_files);

        let expected = CalculatorConfig {
            prefix: String::from("v"),
            report_bump: true,
            files: expected_files,
            ..Default::default()
        };

        assert_eq!(expected, *test);
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
        let mut test = CalculatorConfig::new("v");
        let test = test.set_file_requirement_enforcement_level(enforcement_level);

        let expected = CalculatorConfig {
            prefix: String::from("v"),
            report_bump: true,
            enforce_level: expected_level,
            ..Default::default()
        };

        assert_eq!(expected, *test);
    }

    #[rstest]
    #[case::breaking(Hierarchy::Breaking, Hierarchy::Breaking)]
    #[case::feature(Hierarchy::Feature, Hierarchy::Feature)]
    #[case::fix(Hierarchy::Fix, Hierarchy::Fix)]
    #[case::other(Hierarchy::Other, Hierarchy::Other)]
    fn test_set_threshold_to(#[case] threshold: Hierarchy, #[case] expected_threshold: Hierarchy) {
        let mut test = CalculatorConfig::new("v");
        let test = test.set_threshold(threshold);

        let expected = CalculatorConfig {
            prefix: String::from("v"),
            report_bump: true,
            threshold: expected_threshold,
            ..Default::default()
        };

        assert_eq!(expected, *test);
    }
}
