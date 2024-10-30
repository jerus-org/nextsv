use crate::version::Semantic;

use super::{top_type::TopType, ConventionalCommits};
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub(crate) enum ChangeBump {
    Major,
    Minor,
    Patch,
    #[default]
    None,
}

impl ChangeBump {
    pub(crate) fn calculate(version: &Semantic, conventional: &ConventionalCommits) -> ChangeBump {
        let mut change_bump = match conventional.top_type {
            TopType::Breaking => ChangeBump::Major,
            TopType::Feature => ChangeBump::Minor,
            TopType::Fix => ChangeBump::Patch,
            TopType::Other => ChangeBump::Patch,
            TopType::None => ChangeBump::None,
        };

        if version.major == 0 && ChangeBump::Major == change_bump {
            change_bump = ChangeBump::Minor;
        };

        change_bump
    }
}

impl Display for ChangeBump {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangeBump::Major => write!(f, "major"),
            ChangeBump::Minor => write!(f, "minor"),
            ChangeBump::Patch => write!(f, "patch"),
            ChangeBump::None => write!(f, "none"),
        }
    }
}
