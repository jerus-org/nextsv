use crate::{version::Semantic, Hierarchy};

use super::ConventionalCommits;

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub(crate) enum ChangeBump {
    Major,
    Minor,
    #[default]
    Patch,
}

impl ChangeBump {
    pub(crate) fn calculate(version: &Semantic, conventional: &ConventionalCommits) -> ChangeBump {
        let mut change_bump = match conventional.top_type {
            Hierarchy::Breaking => ChangeBump::Major,
            Hierarchy::Feature => ChangeBump::Minor,
            Hierarchy::Fix => ChangeBump::Patch,
            Hierarchy::Other => ChangeBump::Patch,
        };

        if version.major == 0 && ChangeBump::Major == change_bump {
            change_bump = ChangeBump::Minor;
        };

        change_bump
    }
}
