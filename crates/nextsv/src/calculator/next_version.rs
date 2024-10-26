use crate::version::{PreRelease, PreReleaseType, VersionTag};

use super::{bump::Bump, ChangeBump};

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
            NextVersion::None => String::from("0.0.0"),
        }
    }

    pub(crate) fn calculate(
        current_version: &VersionTag,
        mut bump: Bump,
        change_bump: Option<&ChangeBump>,
    ) -> (Self, Bump) {
        let mut next_version = current_version.clone();
        log::debug!(
            "Starting version: `{}`; bump level `{}`",
            next_version,
            bump
        );

        let pre_release_flag = current_version.semantic_version.pre_release.is_some();

        let next_version = match bump {
            Bump::Major => {
                if !pre_release_flag {
                    next_version.version_mut().major += 1;
                    next_version.version_mut().minor = 0;
                    next_version.version_mut().patch = 0;
                } else {
                    next_version.version_mut().increment_pre_release();
                }
                next_version
            }
            Bump::Minor => {
                if !pre_release_flag {
                    next_version.version_mut().minor += 1;
                    next_version.version_mut().patch = 0;
                } else {
                    next_version.version_mut().increment_pre_release();
                }
                next_version
            }
            Bump::Patch => {
                if !pre_release_flag {
                    next_version.version_mut().patch += 1;
                } else {
                    next_version.version_mut().increment_pre_release();
                }
                next_version
            }
            Bump::First => {
                if next_version.version_mut().major == 0 {
                    next_version.version_mut().major = 1;
                    next_version.version_mut().minor = 0;
                    next_version.version_mut().patch = 0;
                    next_version.version_mut().pre_release = None;
                }
                next_version
            }
            Bump::Alpha => {
                log::trace!("Bumping to alpha");
                match next_version.semantic_version.pre_release.as_ref() {
                    Some(pre_release) => {
                        log::trace!("Already at pre-release: `{pre_release}`");
                        if pre_release.pre_type == PreReleaseType::Alpha {
                            log::trace!("Incrementing pre-release");
                            next_version.version_mut().increment_pre_release();
                        } else {
                            log::trace!("Setting pre-release to alpha.1");
                            next_version.version_mut().pre_release =
                                Some(PreRelease::new("alpha.1"));
                        }
                    }
                    None => {
                        log::trace!("No pre-release found, setting to alpha.1");
                        next_version.version_mut().pre_release = Some(PreRelease::new("alpha.1"));
                        log::trace!("Checking for change bump");
                        if let Some(change_bump) = change_bump {
                            log::info!("Making first pre-release on changes requring version bump: `{change_bump}`");
                            match change_bump {
                                ChangeBump::Major => {
                                    next_version.version_mut().major += 1;
                                    next_version.version_mut().minor = 0;
                                    next_version.version_mut().patch = 0;
                                }
                                ChangeBump::Minor => {
                                    next_version.version_mut().minor += 1;
                                    next_version.version_mut().patch = 0;
                                }
                                ChangeBump::Patch => {
                                    next_version.version_mut().patch += 1;
                                }
                                ChangeBump::None => {}
                            }
                        }
                    }
                }
                next_version
            }
            Bump::Beta => {
                match next_version.semantic_version.pre_release.as_ref() {
                    Some(pre_release) => {
                        if pre_release.pre_type == PreReleaseType::Beta {
                            next_version.version_mut().increment_pre_release();
                        } else {
                            next_version.version_mut().pre_release =
                                Some(PreRelease::new("beta.1"));
                        }
                    }
                    None => {
                        next_version.version_mut().pre_release = Some(PreRelease::new("beta.1"));
                        if let Some(change_bump) = change_bump {
                            match change_bump {
                                ChangeBump::Major => {
                                    next_version.version_mut().major += 1;
                                    next_version.version_mut().minor = 0;
                                    next_version.version_mut().patch = 0;
                                }
                                ChangeBump::Minor => {
                                    next_version.version_mut().minor += 1;
                                    next_version.version_mut().patch = 0;
                                }
                                ChangeBump::Patch => {
                                    next_version.version_mut().patch += 1;
                                }
                                ChangeBump::None => {}
                            }
                        }
                    }
                }
                next_version
            }
            Bump::Rc => {
                match next_version.semantic_version.pre_release.as_ref() {
                    Some(pre_release) => {
                        if pre_release.pre_type == PreReleaseType::Rc {
                            next_version.version_mut().increment_pre_release();
                        } else {
                            next_version.version_mut().pre_release = Some(PreRelease::new("rc.1"));
                        }
                    }
                    None => {
                        next_version.version_mut().pre_release = Some(PreRelease::new("rc.1"));
                        if let Some(change_bump) = change_bump {
                            match change_bump {
                                ChangeBump::Major => {
                                    next_version.version_mut().major += 1;
                                    next_version.version_mut().minor = 0;
                                    next_version.version_mut().patch = 0;
                                }
                                ChangeBump::Minor => {
                                    next_version.version_mut().minor += 1;
                                    next_version.version_mut().patch = 0;
                                }
                                ChangeBump::Patch => {
                                    next_version.version_mut().patch += 1;
                                }
                                ChangeBump::None => {}
                            }
                        }
                    }
                }
                next_version
            }
            Bump::Custom(s) => {
                if let Some(pre_release) = next_version.semantic_version.pre_release.as_ref() {
                    if pre_release.pre_type == PreReleaseType::Custom {
                        next_version.version_mut().increment_pre_release();
                    } else {
                        next_version.version_mut().pre_release =
                            Some(PreRelease::new(format!("{}.1", s).as_str()));
                    }
                }
                bump = Bump::Custom(next_version.semantic_version.to_string());
                next_version
            }
            Bump::Release => {
                next_version.version_mut().pre_release = None;
                next_version
            }
            _ => next_version,
        };
        log::debug!("Next version is: {next_version}");

        (NextVersion::Updated(next_version), bump)
    }
}

#[cfg(test)]
mod test {
    use super::NextVersion;

    use crate::calculator::bump::Bump;
    use crate::calculator::ChangeBump;
    use crate::test_utils::*;
    use crate::version::{PreRelease, VersionTag};
    use log::LevelFilter;
    use log4rs_test_utils::test_logging;

    use rstest::rstest;

    #[rstest]
    #[case::none(0, 0, 0, "", "0.0.0")]
    #[case::non_production(0, 7, 10, "", "0.7.10")]
    #[case::production(1, 0, 2, "", "1.0.2")]
    #[case::pre_release(1, 1, 0, "beta.2", "1.1.0-beta.2")]
    fn test_updated_version_number(
        #[case] major: u32,
        #[case] minor: u32,
        #[case] patch: u32,
        #[case] pre_release: &str,
        #[case] expected: &str,
    ) {
        let pre_release = if pre_release.is_empty() {
            None
        } else {
            Some(PreRelease::new(pre_release))
        };

        let next_version = gen_current_version("v", major, minor, patch, pre_release, None);
        println!("next_version: {next_version:?}");
        let test = NextVersion::Updated(next_version);

        assert_eq!(expected, test.version_number());
    }

    #[rstest]
    fn test_calculation_of_next_version(
        #[values(
            "refs/tags/v0.7.9",
            "refs/tags/v0.7.9-alpha.1",
            "refs/tags/v0.7.9-beta.1",
            "refs/tags/v0.7.9-rc.1",
            "refs/tags/v0.7.9-pre.1",
            "refs/tags/v1.7.9"
        )]
        tag: &str,
        #[values(Bump::None, Bump::Patch, Bump::Minor,
            Bump::Major, Bump::Alpha, Bump::Beta, Bump::Rc, Bump::Release, Bump::Custom("pre".to_string()), Bump::First) ]
        bump: Bump,
    ) {
        test_logging::init_logging_once_for(vec![], LevelFilter::Debug, None);

        let current_version = VersionTag::parse(tag, "v").unwrap();
        let mut change_bump = None;

        let expected = match tag {
            "refs/tags/v0.7.9" => match bump {
                Bump::None | Bump::Release | Bump::Custom(_) => {
                    NextVersion::Updated(VersionTag::parse("v0.7.9", "v").unwrap())
                }
                Bump::Rc => NextVersion::Updated(VersionTag::parse("v0.7.9-rc.1", "v").unwrap()),
                Bump::Beta => {
                    NextVersion::Updated(VersionTag::parse("v0.7.9-beta.1", "v").unwrap())
                }
                Bump::Alpha => {
                    NextVersion::Updated(VersionTag::parse("v0.7.9-alpha.1", "v").unwrap())
                }
                Bump::Patch => {
                    change_bump = Some(ChangeBump::Patch);
                    NextVersion::Updated(VersionTag::parse("v0.7.10", "v").unwrap())
                }
                Bump::Minor => {
                    change_bump = Some(ChangeBump::Minor);
                    NextVersion::Updated(VersionTag::parse("v0.8.0", "v").unwrap())
                }
                Bump::Major | Bump::First => {
                    change_bump = Some(ChangeBump::Major);
                    NextVersion::Updated(VersionTag::parse("v1.0.0", "v").unwrap())
                }
            },
            "refs/tags/v1.7.9" => match bump {
                Bump::None | Bump::Release | Bump::First | Bump::Custom(_) => {
                    NextVersion::Updated(VersionTag::parse("v1.7.9", "v").unwrap())
                }
                Bump::Rc => NextVersion::Updated(VersionTag::parse("v1.7.9-rc.1", "v").unwrap()),
                Bump::Beta => {
                    NextVersion::Updated(VersionTag::parse("v1.7.9-beta.1", "v").unwrap())
                }
                Bump::Alpha => {
                    NextVersion::Updated(VersionTag::parse("v1.7.9-alpha.1", "v").unwrap())
                }
                Bump::Patch => {
                    change_bump = Some(ChangeBump::Patch);
                    NextVersion::Updated(VersionTag::parse("v1.7.10", "v").unwrap())
                }
                Bump::Minor => {
                    change_bump = Some(ChangeBump::Patch);
                    NextVersion::Updated(VersionTag::parse("v1.8.0", "v").unwrap())
                }
                Bump::Major => {
                    change_bump = Some(ChangeBump::Patch);
                    NextVersion::Updated(VersionTag::parse("v2.0.0", "v").unwrap())
                }
            },
            "refs/tags/v0.7.9-alpha.1" => match bump {
                Bump::None => {
                    NextVersion::Updated(VersionTag::parse("v0.7.9-alpha.1", "v").unwrap())
                }
                Bump::Alpha | Bump::Patch | Bump::Minor | Bump::Major => {
                    NextVersion::Updated(VersionTag::parse("v0.7.9-alpha.2", "v").unwrap())
                }
                Bump::Beta => {
                    NextVersion::Updated(VersionTag::parse("v0.7.9-beta.1", "v").unwrap())
                }
                Bump::Rc => NextVersion::Updated(VersionTag::parse("v0.7.9-rc.1", "v").unwrap()),
                Bump::Custom(ref pre) => NextVersion::Updated(
                    VersionTag::parse(format!("v0.7.9-{pre}.1").as_str(), "v").unwrap(),
                ),
                Bump::Release => NextVersion::Updated(VersionTag::parse("v0.7.9", "v").unwrap()),
                Bump::First => NextVersion::Updated(VersionTag::parse("v1.0.0", "v").unwrap()),
            },
            "refs/tags/v0.7.9-beta.1" => match bump {
                Bump::None => {
                    NextVersion::Updated(VersionTag::parse("v0.7.9-beta.1", "v").unwrap())
                }
                Bump::Alpha => {
                    NextVersion::Updated(VersionTag::parse("v0.7.9-alpha.1", "v").unwrap())
                }
                Bump::Beta | Bump::Patch | Bump::Minor | Bump::Major => {
                    NextVersion::Updated(VersionTag::parse("v0.7.9-beta.2", "v").unwrap())
                }
                Bump::Rc => NextVersion::Updated(VersionTag::parse("v0.7.9-rc.1", "v").unwrap()),
                Bump::Custom(ref pre) => NextVersion::Updated(
                    VersionTag::parse(format!("v0.7.9-{pre}.1").as_str(), "v").unwrap(),
                ),
                Bump::Release => NextVersion::Updated(VersionTag::parse("v0.7.9", "v").unwrap()),
                Bump::First => NextVersion::Updated(VersionTag::parse("v1.0.0", "v").unwrap()),
            },
            "refs/tags/v0.7.9-rc.1" => match bump {
                Bump::None => NextVersion::Updated(VersionTag::parse("v0.7.9-rc.1", "v").unwrap()),
                Bump::Alpha => {
                    NextVersion::Updated(VersionTag::parse("v0.7.9-alpha.1", "v").unwrap())
                }
                Bump::Beta => {
                    NextVersion::Updated(VersionTag::parse("v0.7.9-beta.1", "v").unwrap())
                }
                Bump::Custom(ref pre) => NextVersion::Updated(
                    VersionTag::parse(format!("v0.7.9-{pre}.1").as_str(), "v").unwrap(),
                ),
                Bump::Rc | Bump::Patch | Bump::Minor | Bump::Major => {
                    NextVersion::Updated(VersionTag::parse("v0.7.9-rc.2", "v").unwrap())
                }
                Bump::Release => NextVersion::Updated(VersionTag::parse("v0.7.9", "v").unwrap()),
                Bump::First => NextVersion::Updated(VersionTag::parse("v1.0.0", "v").unwrap()),
            },
            "refs/tags/v0.7.9-pre.1" => match bump {
                Bump::None => NextVersion::Updated(VersionTag::parse("v0.7.9-pre.1", "v").unwrap()),
                Bump::Alpha => {
                    NextVersion::Updated(VersionTag::parse("v0.7.9-alpha.1", "v").unwrap())
                }
                Bump::Beta => {
                    NextVersion::Updated(VersionTag::parse("v0.7.9-beta.1", "v").unwrap())
                }
                Bump::Rc => NextVersion::Updated(VersionTag::parse("v0.7.9-rc.1", "v").unwrap()),
                Bump::Custom(_) | Bump::Patch | Bump::Minor | Bump::Major => {
                    NextVersion::Updated(VersionTag::parse("v0.7.9-pre.2", "v").unwrap())
                }
                Bump::Release => NextVersion::Updated(VersionTag::parse("v0.7.9", "v").unwrap()),
                Bump::First => NextVersion::Updated(VersionTag::parse("v1.0.0", "v").unwrap()),
            },
            _ => unreachable!("unexpected tag"),
        };

        let (test, _updated_bump) =
            NextVersion::calculate(&current_version, bump.clone(), change_bump.as_ref());

        assert_eq!(expected.version_number(), test.version_number());
    }
}
