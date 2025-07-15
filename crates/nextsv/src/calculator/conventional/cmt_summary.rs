use crate::Error;

#[derive(Debug)]
pub struct CmtSummary {
    pub title: String,
    pub emoji: Option<String>,
    pub type_: Option<String>,
    pub scope: Option<String>,
    pub breaking: bool,
    // pub section: Option<ChangeKind>,
}

impl CmtSummary {
    pub fn parse(title: &str) -> Result<Self, Error> {
        let re = regex::Regex::new(
            r"^(?P<emoji>.+\s)?(?P<type>[a-z]+)(?:\((?P<scope>.+)\))?(?P<breaking>!)?: (?P<description>.*)$$",
        )?;

        log::debug!("String to parse: `{title}`");

        let cmt_summary = if let Some(captures) = re.captures(title) {
            log::debug!("Captures: {captures:#?}");
            let emoji = captures.name("emoji").map(|m| m.as_str().to_string());
            let type_ = captures.name("type").map(|m| m.as_str().to_string());
            let scope = captures.name("scope").map(|m| m.as_str().to_string());
            let breaking = captures.name("breaking").is_some();
            let title = captures
                .name("description")
                .map(|m| m.as_str().to_string())
                .unwrap();

            Self {
                title,
                emoji,
                type_,
                scope,
                breaking,
            }
        } else {
            Self {
                title: title.to_string(),
                emoji: None,
                type_: None,
                scope: None,
                breaking: false,
            }
        };

        log::debug!("Parsed title: {cmt_summary:?}");

        Ok(cmt_summary)
    }

    pub fn type_string(&self) -> String {
        self.type_.clone().unwrap_or_default()
    }
}

impl std::fmt::Display for CmtSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}: {}",
            self.emoji.clone().unwrap_or_default(),
            self.type_.clone().unwrap_or_default(),
            self.scope
                .as_ref()
                .map_or("".to_string(), |s| format!("({s})")),
            if self.breaking { "!" } else { "" },
            self.title
        )
    }
}

//test module
#[cfg(test)]
mod tests {

    use super::*;
    use color_eyre::Result;
    use log::LevelFilter;
    use rstest::rstest;

    fn get_test_logger() {
        let mut builder = env_logger::Builder::new();
        builder.filter(None, LevelFilter::Debug);
        builder.format_timestamp_secs().format_module_path(false);
        let _ = builder.try_init();
    }

    #[test]
    fn test_cmt_summary_parse() {
        let cmt_summary = CmtSummary::parse("feat: add new feature").unwrap();

        assert_eq!(cmt_summary.title, "add new feature");
        assert_eq!(cmt_summary.type_, Some("feat".to_string()));
        assert_eq!(cmt_summary.scope, None);
        assert!(!cmt_summary.breaking);

        let cmt_summary = CmtSummary::parse("feat(core): add new feature").unwrap();
        assert_eq!(cmt_summary.title, "add new feature");
        assert_eq!(cmt_summary.type_, Some("feat".to_string()));
        assert_eq!(cmt_summary.scope, Some("core".to_string()));
        assert!(!cmt_summary.breaking);

        let cmt_summary = CmtSummary::parse("feat(core)!: add new feature").unwrap();
        assert_eq!(cmt_summary.title, "add new feature");
        assert_eq!(cmt_summary.type_, Some("feat".to_string()));
        assert_eq!(cmt_summary.scope, Some("core".to_string()));
        assert!(cmt_summary.breaking);
    }

    #[test]
    fn test_cmt_summary_parse_with_breaking_scope() {
        let cmt_summary = CmtSummary::parse("feat(core)!: add new feature").unwrap();
        assert_eq!(cmt_summary.title, "add new feature");
        assert_eq!(cmt_summary.type_, Some("feat".to_string()));
        assert_eq!(cmt_summary.scope, Some("core".to_string()));
        assert!(cmt_summary.breaking);
    }

    #[test]
    fn test_cmt_summary_parse_with_security_scope() {
        let cmt_summary = CmtSummary::parse("fix(security): fix security vulnerability").unwrap();
        assert_eq!(cmt_summary.title, "fix security vulnerability");
        assert_eq!(cmt_summary.type_, Some("fix".to_string()));
        assert_eq!(cmt_summary.scope, Some("security".to_string()));
        assert!(!cmt_summary.breaking);
    }

    #[test]
    fn test_cmt_summary_parse_with_deprecate_scope() {
        let cmt_summary = CmtSummary::parse("chore(deprecate): deprecate old feature").unwrap();
        assert_eq!(cmt_summary.title, "deprecate old feature");
        assert_eq!(cmt_summary.type_, Some("chore".to_string()));
        assert_eq!(cmt_summary.scope, Some("deprecate".to_string()));
        assert!(!cmt_summary.breaking);
    }

    #[test]
    fn test_cmt_summary_parse_without_scope() {
        let cmt_summary = CmtSummary::parse("docs: update documentation").unwrap();
        assert_eq!(cmt_summary.title, "update documentation");
        assert_eq!(cmt_summary.type_, Some("docs".to_string()));
        assert_eq!(cmt_summary.scope, None);
        assert!(!cmt_summary.breaking);
    }

    #[test]
    fn test_cmt_summary_parse_issue_172() {
        let cmt_summary = CmtSummary::parse(
            "chore(config.yml): update jerus-org/circleci-toolkit orb version to 0.4.0",
        )
        .unwrap();
        assert_eq!(
            cmt_summary.title,
            "update jerus-org/circleci-toolkit orb version to 0.4.0"
        );
        assert_eq!(cmt_summary.type_, Some("chore".to_string()));
        assert_eq!(cmt_summary.scope, Some("config.yml".to_string()));
        assert!(!cmt_summary.breaking);
    }

    #[rstest]
    #[case("feat: add new feature", "feat")]
    #[case("✨ feat: add new feature", "feat")]
    #[case("feat: add new feature", "feat")]
    #[case("feat: add new feature", "feat")]
    #[case("feat: add new feature", "feat")]
    #[case("✨ feat: add new feature", "feat")]
    #[case("fix: fix an existing feature", "fix")]
    #[case("🐛 fix: fix an existing feature", "fix")]
    #[case("style: fix typo and lint issues", "style")]
    #[case("💄 style: fix typo and lint issues", "style")]
    #[case("test: update tests", "test")]
    #[case("fix(security): Fix security vulnerability", "fix")]
    #[case("chore(deps): Update dependencies", "chore")]
    #[case("🔧 chore(deps): Update dependencies", "chore")]
    #[case("refactor(remove): Remove unused code", "refactor")]
    #[case("♻️ refactor(remove): Remove unused code", "refactor")]
    #[case("docs(deprecate): Deprecate old API", "docs")]
    #[case("📚 docs(deprecate): Deprecate old API", "docs")]
    #[case("ci(other-scope): Update CI configuration", "ci")]
    #[case("👷 ci(other-scope): Update CI configuration", "ci")]
    #[case("test!: Update test cases", "test")]
    #[case::issue_172(
        "chore(config.yml): update jerus-org/circleci-toolkit orb version to 0.4.0",
        "chore"
    )]
    #[case::with_emoji("✨ feat(ci): add optional flag for push failure handling", "feat")]
    fn test_calculate_kind_and_description(
        #[case] title: &str,
        #[case] expected_type: &str,
    ) -> Result<()> {
        get_test_logger();

        let cmt_summary = CmtSummary::parse(title).unwrap();
        assert_eq!(expected_type, &cmt_summary.type_.unwrap());

        Ok(())
    }
}
