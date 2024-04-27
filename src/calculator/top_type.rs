use std::{
    cmp,
    fmt::{self, Display},
};

use crate::{Error, Hierarchy};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub(crate) enum TopType {
    Breaking,
    Feature,
    Fix,
    Other,
    #[default]
    None,
}

impl TopType {
    #[allow(missing_docs)]
    pub fn parse(s: &str) -> Result<Self, Error> {
        Ok(match s.to_lowercase().as_str() {
            "breaking" => Self::Breaking,
            "feat" => Self::Feature,
            "fix" => Self::Fix,
            "revert" => Self::Fix,
            _ => Self::Other,
        })
    }
}

impl Display for TopType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Breaking => write!(f, "[Major]"),
            Self::Feature => write!(f, "[Minor]"),
            Self::Fix => write!(f, "[Patch]"),
            Self::Other => write!(f, "[Patch]"),
            Self::None => write!(f, "[None]"),
        }
    }
}

impl From<&Hierarchy> for TopType {
    fn from(h: &Hierarchy) -> Self {
        match h {
            Hierarchy::Breaking => Self::Breaking,
            Hierarchy::Feature => Self::Feature,
            Hierarchy::Fix => Self::Fix,
            Hierarchy::Other => Self::Fix,
        }
    }
}

impl Ord for TopType {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match (self, other) {
            (TopType::Breaking, TopType::Breaking)
            | (TopType::Feature, TopType::Feature)
            | (TopType::Fix, TopType::Fix)
            | (TopType::Other, TopType::Other)
            | (TopType::None, TopType::None) => cmp::Ordering::Equal,
            (TopType::None, _) => cmp::Ordering::Less,
            (TopType::Breaking, _) => cmp::Ordering::Greater,
            (TopType::Other, TopType::None) => cmp::Ordering::Greater,
            (TopType::Other, TopType::Fix)
            | (TopType::Other, TopType::Feature)
            | (TopType::Other, TopType::Breaking) => cmp::Ordering::Less,
            (TopType::Fix, TopType::None) | (TopType::Fix, TopType::Other) => {
                cmp::Ordering::Greater
            }
            (TopType::Fix, TopType::Feature) | (TopType::Fix, TopType::Breaking) => {
                cmp::Ordering::Less
            }
            (TopType::Feature, TopType::None)
            | (TopType::Feature, TopType::Other)
            | (TopType::Feature, TopType::Fix) => cmp::Ordering::Greater,
            (TopType::Feature, TopType::Breaking) => cmp::Ordering::Less,
        }
    }
}

impl PartialOrd for TopType {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl AsRef<TopType> for TopType {
    fn as_ref(&self) -> &TopType {
        self
    }
}
