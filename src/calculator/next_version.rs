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
            _ => String::from("0.0.0"),
        }
    }
}
