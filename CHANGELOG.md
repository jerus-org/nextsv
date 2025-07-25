# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- 🔧 chore(config)-update renovate configuration(pr [#354])

### Fixed

- deps: update ossf/scorecard-action action to v2.4.2(pr [#355])
- deps: update rust crate clap to 4.5.41(pr [#356])
- deps: update rust crate clap-verbosity-flag to 3.0.3(pr [#357])
- deps: update rust crate color-eyre to 0.6.5(pr [#358])
- deps: update rust crate git2 to 0.20.2(pr [#359])
- deps: update rust crate trycmd to 0.15.10(pr [#360])
- deps: update dependency toolkit to v2.12.1(pr [#361])
- deps: update github/codeql-action action to v3.29.4(pr [#362])
- deps: update rust crate autocfg to 1.5.0(pr [#363])
- deps: update rust crate uuid to 1.17.0(pr [#364])

## [0.19.22] - 2025-05-28

### Changed

- 🔧 chore(renovate)-update renovate schedule configuration(pr [#353])

## [0.19.21] - 2025-05-03

### Security

- Dependencies: update rust crate chrono to 0.4.41(pr [#351])

## [0.19.20] - 2025-04-26

### Security

- Dependencies: update github/codeql-action action to v3.28.16(pr [#349])
- Dependencies: update rust crate clap to 4.5.37(pr [#350])

## [0.19.19] - 2025-04-22

### Changed

- 👷 ci(circleci)-update circleci-toolkit orb version(pr [#348])

### Security

- Dependencies: update rust crate clap to 4.5.36(pr [#346])
- Dependencies: update dependency toolkit to v2.8.1(pr [#347])

## [0.19.18] - 2025-04-12

### Security

- Dependencies: update github/codeql-action action to v3.28.15(pr [#345])

## [0.19.17] - 2025-04-05

### Security

- Dependencies: update rust crate clap to 4.5.35(pr [#343])
- Dependencies: update rust crate log to 0.4.27(pr [#342])
- Dependencies: update rust crate env_logger to 0.11.8(pr [#344])

## [0.19.16] - 2025-03-29

### Security

- Dependencies: update github/codeql-action action to v3.28.13(pr [#340])
- Dependencies: update rust crate clap to 4.5.34(pr [#341])

## [0.19.15] - 2025-03-22

### Security

- Dependencies: update dependency toolkit to v2.5.1(pr [#338])
- Dependencies: update actions/upload-artifact action to v4.6.2(pr [#335])
- Dependencies: update github/codeql-action action to v3.28.12(pr [#336])
- Dependencies: update rust crate git2 to 0.20.1(pr [#337])
- Dependencies: update rust crate uuid to 1.16.0(pr [#339])

## [0.19.14] - 2025-03-15

### Security

- Dependencies: update github/codeql-action action to v3.28.11(pr [#331])
- Dependencies: update rust crate clap to 4.5.32(pr [#332])
- Dependencies: update rust crate env_logger to 0.11.7(pr [#333])
- Dependencies: update rust crate cargo_toml to 0.22.1(pr [#334])

## [0.19.13] - 2025-03-08

### Security

- Dependencies: update rust crate thiserror to 2.0.12(pr [#328])
- Dependencies: update dependency toolkit to v2.1.0(pr [#329])
- Dependencies: update rust crate rstest to 0.25.0(pr [#330])

## [0.19.12] - 2025-03-01

### Security

- Dependencies: update actions/upload-artifact action to v4.6.1(pr [#322])
- Dependencies: update github/codeql-action action to v3.28.10(pr [#323])
- Dependencies: update ossf/scorecard-action action to v2.4.1(pr [#324])
- Dependencies: update rust crate chrono to 0.4.40(pr [#325])
- Dependencies: update rust crate clap to 4.5.31(pr [#326])
- Dependencies: update rust crate uuid to 1.15.1(pr [#327])

## [0.19.11] - 2025-02-27

### Added

- ✨ add CmtSummary struct for commit parsing(pr [#318])

### Changed

- 👷 ci(circleci)-update toolkit orb version and simplify workflow(pr [#319])
- 👷 ci(circleci)-update toolkit orb and add path_to_crate(pr [#320])

### Fixed

- 🐛 calculator: include root files in qualified files(pr [#321])

## [0.19.10] - 2025-02-24

### Changed

- 🐛 fix(calculator):  remove unnecessary take and correct typos(pr [#313])
- ✨ fix(version): add package prefix support in version tag search(pr [#314])
- 👷 ci(circleci): add package parameter to save_next_version job(pr [#315])
- 📝 docs(CHANGELOG): remove duplicate version entry(pr [#316])
- 🔧 chore(dependencies): update nextsv package version(pr [#317])

## [0.19.9] - 2025-02-22

### Security

- Dependencies: update rust crate clap to 4.5.30(pr [#310])
- Dependencies: update rust crate uuid to 1.14.0(pr [#311])
- Dependencies: update rust crate log to 0.4.26(pr [#312])

## [0.19.8] - 2025-02-15

### Changed

- chore(config)-migrate renovate config(pr [#309])

### Security

- Dependencies: update github/codeql-action action to v3.28.9(pr [#306])
- Dependencies: update rust crate clap to 4.5.29(pr [#307])
- Dependencies: update rust crate trycmd to 0.15.9(pr [#308])

## [0.19.7] - 2025-02-08

### Security

- Dependencies: update rust crate clap to 4.5.28(pr [#304])
- Dependencies: update rust crate uuid to 1.13.1(pr [#305])

## [0.19.6] - 2025-02-01

### Changed

- 👷 ci(circleci): add package parameter to build configuration(pr [#303])

## [0.19.5] - 2025-02-01

### Security

- Dependencies: update github/codeql-action action to v3.28.8(pr [#300])
- Dependencies: update rust crate git-conventional to 0.12.9(pr [#301])
- Dependencies: update rust crate uuid to 1.12.1(pr [#302])

## [0.19.4] - 2025-01-25

### Changed

- chore(ci)-remove deprecated github release step from CircleCI config(pr [#285])
- refactor-use eq_ignore_ascii_case for pre-release label comparison(pr [#289])
- chore(circleci)-update toolkit orb to version 2.0.0(pr [#290])
- docs-add MIT license file(pr [#291])
- chore-remove dependabot configuration file(pr [#292])
- ci(circleci)-update config for release workflow and toolkit(pr [#296])
- 🔧 chore(version): rollback version from 9.1.4 to 0.19.3(pr [#299])

### Security

- Dependencies: update github/codeql-action action to v3.28.4(pr [#297])
- Dependencies: update rust crate clap to 4.5.27(pr [#298])
- Dependencies: bump github/codeql-action from 3.28.0 to 3.28.1(pr [#287])
- Dependencies: bump actions/upload-artifact from 4.5.0 to 4.6.0(pr [#286])
- Dependencies: bump thiserror from 2.0.10 to 2.0.11(pr [#288])
- Dependencies: update rust crate log to 0.4.25(pr [#293])
- Dependencies: update rust crate uuid to 1.12.0(pr [#294])
- Dependencies: update rust crate colored to v3(pr [#295])

## [0.19.3] - 2025-01-11

### Security

- Dependencies: update rust crate clap to 4.5.26(pr [#281])
- Dependencies: update rust crate thiserror to 2.0.10(pr [#282])
- Dependencies: update rust crate uuid to 1.11.1(pr [#283])
- Dependencies: update rust crate git2 to 0.20.0(pr [#284])

## [0.19.2] - 2025-01-04

### Security

- Dependencies: update rust crate rstest to 0.24.0(pr [#280])

## [0.19.1] - 2024-12-28

### Changed

- chore(circleci)-update toolkit orb and add github release step(pr [#274])

### Fixed

- circleci: update key name from cargo_package to package in config file(pr [#279])

### Security

- Dependencies: bump github/codeql-action from 3.27.9 to 3.28.0(pr [#275])
- Dependencies: bump env_logger from 0.11.5 to 0.11.6(pr [#276])
- Dependencies: bump thiserror from 2.0.8 to 2.0.9(pr [#277])
- Dependencies: update dependency toolkit to v1.23.0(pr [#278])

## [0.19.0] - 2024-12-21

### Changed

- chore(ci)-remove package key from CircleCI config file(pr [#267])

### Security

- Dependencies: bump clap-verbosity-flag from 3.0.1 to 3.0.2(pr [#269])
- Dependencies: bump snapbox from 0.6.20 to 0.6.21(pr [#270])
- Dependencies: bump thiserror from 2.0.6 to 2.0.7(pr [#271])
- Dependencies: bump colored from 2.1.0 to 2.2.0(pr [#268])
- Dependencies: update rust crate thiserror to 2.0.8(pr [#272])
- Dependencies: update actions/upload-artifact action to v4.5.0(pr [#273])

## [0.18.0] - 2024-12-13

### Security

- Dependencies: update dependency toolkit to v1.20.2(pr [#266])
- Dependencies: update github/codeql-action action to v3.27.9(pr [#265])

## [0.17.0] - 2024-12-11

### Added

- add package option to set subdir filter based on workspace package(pr [#264])

### Security

- Dependencies: bump chrono from 0.4.38 to 0.4.39(pr [#262])
- Dependencies: bump thiserror from 2.0.4 to 2.0.6(pr [#263])

## [0.16.0] - 2024-12-07

### Security

- Dependencies: update github/codeql-action action to v3.27.6(pr [#258])
- Dependencies: update rust crate clap to 4.5.23(pr [#259])
- Dependencies: update rust crate thiserror to 2.0.4(pr [#260])
- Dependencies: update rust crate cargo_toml to 0.21.0(pr [#261])

## [0.15.0] - 2024-11-30

### Security

- Dependencies: update rust crate clap-verbosity-flag to 3.0.1(pr [#257])

## [0.14.0] - 2024-11-23

### Security

- Dependencies: update github/codeql-action action to v3.27.5(pr [#254])
- Dependencies: update rust crate clap-verbosity-flag to 2.2.3(pr [#255])
- Dependencies: update rust crate clap-verbosity-flag to v3(pr [#256])

## [0.13.0] - 2024-11-16

### Added

- add scheduling for updates before 6am on Friday in renovate config(pr [#251])

### Security

- Dependencies: update rust crate thiserror to 2.0.2(pr [#247])
- Dependencies: update rust crate thiserror to 2.0.3(pr [#248])
- Dependencies: update github/codeql-action action to v3.27.2(pr [#250])
- Dependencies: update github/codeql-action action to v3.27.4(pr [#252])
- Dependencies: update rust crate clap to 4.5.21(pr [#253])

## [0.12.0] - 2024-11-09

### Changed

- ci-update CircleCI toolkit orb and add code coverage step(pr [#239])
- docs-add codecov badge to README.md(pr [#242])

### Security

- Dependencies: bump thiserror from 1.0.67 to 1.0.68(pr [#238])
- Dependencies: update rust crate snapbox to 0.6.20(pr [#240])
- Dependencies: update rust crate thiserror to v2(pr [#243])
- Dependencies: update dependency toolkit to v1.19.0(pr [#244])
- Dependencies: update github/codeql-action action to v3.27.1(pr [#245])
- Dependencies: update rust crate thiserror to 2.0.1(pr [#246])

## [0.11.0] - 2024-11-04

### Changed

- ci-remove specific_version parameter from CircleCI config(pr [#236])

### Security

- Dependencies: update rust crate thiserror to 1.0.67(pr [#235])

## [0.10.0] - 2024-11-02

### Added

- add option to calculate for specific workspace package(pr [#220])

### Changed

- chore(ci)-remove hardcoded version from CircleCI config(pr [#233])

### Security

- Dependencies: update rust crate thiserror to 1.0.66(pr [#234])

## [0.9.2] - 2024-10-30

### Changed

- chore-update CircleCI toolkit orb and comment out unused configurations(pr [#230])
- chore-remove commented-out code from CircleCI config and update version to 0.9.2(pr [#232])

### Fixed

- downgrade nextsv version to 0.9.1 and update dependencies in Cargo.lock(pr [#231])

### Security

- Dependencies: update rust crate snapbox to 0.6.19(pr [#228])
- Dependencies: update rust crate trycmd to 0.15.8(pr [#229])

## [0.9.1] - 2024-10-29

### Added

- add commands for GitHub and Cargo release processes(pr [#225])
- BREAKING: add subdir option to filter commits by subdirectory(pr [#214])

### Changed

- Reorg into workspace for nextsv and test-utils(pr [#221])
- ci(circleci)-update config to specify version 9.0.1(pr [#222])
- chore-rename CHANGELOG.md to crates/nextsv/CHANGELOG.md(pr [#223])
- chore-rename release.toml to crates/nextsv/release.toml(pr [#224])
- Fix-version-numbering(pr [#226])

### Fixed

- nextsv: update version to 0.8.22 in Cargo and documentation(pr [#227])

### Security

- Dependencies: update crate dependencies to latest versions(pr [#215])
- Dependencies: update rust crate thiserror to 1.0.65(pr [#217])
- Dependencies: update github/codeql-action action to v3.27.0(pr [#216])
- Dependencies: update actions/checkout action to v4.2.2(pr [#218])
- Dependencies: update rust crate regex to 1.11.1(pr [#219])

## [0.8.22] - 2024-10-19

### Security

- Dependencies: update github/codeql-action action to v3.26.13(pr [#211])
- Dependencies: update rust crate uuid to 1.11.0(pr [#213])

## [0.8.21] - 2024-10-12

### Security

- Dependencies: update actions/checkout action to v4.2.1(pr [#203])
- Dependencies: update actions/upload-artifact action to v4.4.3(pr [#202])
- Dependencies: update github/codeql-action action to v3.26.12(pr [#204])
- Dependencies: update rust crate clap to 4.5.20(pr [#208])

## [0.8.20] - 2024-10-05

### Security

- Dependencies: update rust crate rstest to 0.23.0(pr [#195])
- Dependencies: update rust crate regex to 1.11.0(pr [#196])
- Dependencies: update github/codeql-action action to v3.26.10(pr [#197])
- Dependencies: update rust crate clap to 4.5.19(pr [#199])
- Dependencies: update github/codeql-action action to v3.26.11(pr [#200])
- Dependencies: update rust crate snapbox to 0.6.18(pr [#201])

## [0.8.19] - 2024-09-28

### Added

- update renovate config to enable jerus-org/circleci-toolkit with source URL(pr [#191])

### Changed

- chore(circleci)-update toolkit orb version to 1.9.2(pr [#188])

### Security

- Dependencies: update rust crate clap to 4.5.18(pr [#186])
- Dependencies: update rust crate thiserror to 1.0.64(pr [#187])
- Dependencies: update github/codeql-action action to v3.26.9(pr [#189])
- Dependencies: update dependency toolkit to v1.11.0(pr [#192])
- Dependencies: update actions/checkout action to v4.2.0(pr [#190])
- Dependencies: update rust crate clap-verbosity-flag to 2.2.2(pr [#193])
- Dependencies: update rust crate autocfg to 1.4.0(pr [#194])

## [0.8.18] - 2024-09-21

### Security

- Dependencies: update github/codeql-action action to v3.26.8(pr [#185])

## [0.8.17] - 2024-09-14

### Changed

- chore-remove commented-out PAT token instructions from scorecards.yml(pr [#179])

### Security

- Dependencies: update rust crate clap to 4.5.17(pr [#181])
- Dependencies: update rust crate uuid to 1.10.0(pr [#183])
- Dependencies: update rust crate autocfg to 1.3.0(pr [#180])
- Dependencies: update rust crate snapbox to 0.6.17(pr [#182])
- Dependencies: update github/codeql-action action to v3.26.7(pr [#184])

## [0.8.16] - 2024-09-07

### Added

- add dependency dashboard and package grouping rules in renovate.json(pr [#174])

### Changed

- ci(circleci)-update toolkit orb to version 1.5.0 and add label_option parameter(pr [#173])
- chore-update thiserror dependency to version 1.0.63(pr [#176])
- chore-remove commented-out PAT token and publishing instructions from scorecards.yml(pr [#178])

### Fixed

- deps: update actions/upload-artifact action to v4.4.0(pr [#172])

### Security

- Dependencies: update trycmd to version 0.15.7(pr [#175])
- Dependencies: update dependencies in Cargo.toml(pr [#177])

## [0.8.15] - 2024-08-31

### Fixed

- deps: update github/codeql-action action to v3.26.5(pr [#170])
- deps: update github/codeql-action action to v3.26.6(pr [#171])

## [0.8.14] - 2024-08-24

### Fixed

- deps: update github/codeql-action action to v3.26.4(pr [#169])

### Security

- Dependencies: bump github/codeql-action from 3.26.2 to 3.26.3(pr [#168])

## [0.8.13] - 2024-08-17

### Security

- Dependencies: update github/codeql-action action to v3.26.1(pr [#165])
- Dependencies: update github/codeql-action action to v3.26.2(pr [#166])

## [0.8.12] - 2024-08-10

### Security

- Dependencies: update github/codeql-action action to v3.26.0(pr [#164])
- Dependencies: update rstest requirement from 0.21.0 to 0.22.0(pr [#162])
- Dependencies: update actions/upload-artifact action to v4.3.6(pr [#163])

## [0.8.11] - 2024-08-03

### Security

- Dependencies: update actions/upload-artifact action to v4.3.5(pr [#160])

## [0.8.10] - 2024-07-29

### Changed

- ci-add bot-check context to toolkit/make_release workflow(pr [#159])

## [0.8.9] - 2024-07-29

### Security

- Dependencies: update ossf/scorecard-action action to v2.4.0(pr [#158])
- Dependencies: update github/codeql-action action to v3.25.15(pr [#157])

## [0.8.8] - 2024-07-27

### Changed

- chore-switch to custom renovate config(pr [#154])
- ci-update circleci-toolkit version from 0.23.0 to 0.24.1(pr [#156])

### Security

- Dependencies: update github/codeql-action action to v3.25.13(pr [#153])
- Dependencies: update github/codeql-action action to v3.25.14(pr [#155])

## [0.8.7] - 2024-07-20

### Changed

- ci-adopt security job from toolkit(pr [#152])

## [0.8.6] - 2024-07-14

### Added

- add support for pcu to ci script(pr [#143](https://github.com/jerus-org/nextsv/pull/143))

### Changed

- ci-adopt toolkit(pr [#150])
- chore-simplify pre-release replacements in CHANGELOG.md and remove CHANGES.md replacements(pr [#151])

### Security

- Dependencies: update snapbox requirement from 0.5.9 to 0.6.10(pr [#141](https://github.com/jerus-org/nextsv/pull/141))
- Dependencies: update rstest requirement from 0.19.0 to 0.21.0(pr [#139](https://github.com/jerus-org/nextsv/pull/139))
- Dependencies: update github/codeql-action action to v3(pr [#146](https://github.com/jerus-org/nextsv/pull/146))
- Dependencies: update rust crate git2 to 0.19.0(pr [#145](https://github.com/jerus-org/nextsv/pull/145))
- Dependencies: update actions/checkout action to v4.1.7(pr [#144](https://github.com/jerus-org/nextsv/pull/144))
- Dependencies: update github/codeql-action action to v3.25.11(pr [#147])
- Dependencies: update actions/upload-artifact action to v4.3.4(pr [#148])
- Dependencies: update github/codeql-action action to v3.25.12(pr [#149])

## [0.8.5] - 2024-06-11

### Fixed

- Removed dbg macro

## [0.8.4] - 2024-05-18

## [0.8.3] - 2024-05-11

## [0.8.2] - 2024-05-08

## [0.8.1] - 2024-05-08

### Fixed

- Unstable features bump patch

## [0.8.0] - 2024-04-30

### Added

- Export Answer type
- Set environment variable; use Answer type; return error
- Add proc-exit
- Regex implemented to extract version string
- Added builder config struct for VersionCalculator and tests
- [**breaking**] Restructure to allow force to pre-release and first version
- Restructure to allow force to pre-release and first version

### Changed

- Update code in doc comment
- Updated doc tests and removed deleted Answer type
- Added documentation for CalculationConfig
- Documentation review and revision for completeness
- Scenarios describing usage
- Scenarios describing usage
- Licensing scanning only required once.
- Update MSRV to 1.64
- Replace default implementation with derive
- Update MSRV to 1.70
- Update env_logger requirement from 0.10.0 to 0.11.1
- Update minium rust to 1.71 to ensure env_logger works
- Update git2 requirement from 0.15.0 to 0.18.1
- Bump actions/checkout from 3 to 4
- Bump fossa-contrib/fossa-action from 1 to 3
- Bump actions/upload-artifact from 3.1.1 to 4.3.0
- Bump actions/upload-artifact from 4.3.0 to 4.3.1
- Update trycmd requirement from 0.14.5 to 0.15.0
- Updated minium rust version references
- Silenced warnings and started tidy up of redundant code.
- Cleaned up viability to restrict internal use types and functions to pub(crate).
- Trace output from calculation for trouble shooting.
- Add debug logging to PreRelease::new() and VersionTag::parse() functions
- Tidy up source documents
- Cosmetic fixes in changes.
- Ensure all tests passing
- Formatting fix
- Prepare for release
- Bump github/codeql-action from 1 to 3
- Bump ossf/scorecard-action from 2.1.1 to 2.3.1
- Bump actions/upload-artifact from 4.3.1 to 4.3.3
- Tidy up source documents
- Cosmetic fixes in changes.
- Ensure all tests passing
- Formatting fix
- Prepare for release
- Bump github/codeql-action from 1 to 3
- Bump ossf/scorecard-action from 2.1.1 to 2.3.1
- Bump actions/upload-artifact from 4.3.1 to 4.3.3
- Answer type to store result of calculation
- Configure error for cli output
- Configure error for cli output
- Improve update of top type
- Implemented VersionTag
- Split out semantic and add tests
- Extract version_tag and supporting test utils
- Extracted VersionTag to separate module and implement tests
- Extracted PreRelease to module and create tests
- Clean up of mod and testing, testing for first production version.
- Breakup calculator module. Extract ForceLevel, extend options and integrate directly with CLI. Implement tests.
- Split types in calculator and implemented individual tests.
- Conventional as part of calculator and separate module for LevelHierarchy
- Renamed semantic to version and included semantic as part of the overall module.
- Implemented a cleaner approach to calculation.
- Commit type handling in test_repo_with_commit function
- Update tests with new feature
- Update cli tests
- Update expected results from trycmd tests
- Ensure all current tests are passing
- Added rstest module for testing case features
- Added tests for single commit of each type to 0 major
- Added tests for nonprod and breaking case
- Added testing for prod cases with breaking and non-breaking changes
- Added log4rs_test_utils to make logs available in tests
- Implemented failing test for alpha pre-release
- Update help text validation for cli test  with expanded force list.
- Tests for error codes.
- Completed testing for separated types and refactored the test utils for sharing across the crate.
- Validate tests and ensure that they are all passing
- Implemented passing tests for next_version::calculate
- Implemented passing tests for bump::calculate
- Ensured that all tests were passing
- Initial build of integration tests with git repo
- Cli tests for help an help text revision
- Expand testing for bump using rstest and cases
- Add support for pre-release versions in test_repo_with_commit function
- Integration tests and making them work
- Integration tests for different prefixes
- Remove force testing from trycmd
- Outputs of bump only and number only
- Incorrect required files should not short circuit calculation
- First two scenarios
- Scenario with first production pre-releases
- Move out git_utils fto test_utils crate
- Move out git_utils fto test_utils crate
- Test to make work; prep for env variable
- Update CI to use 1.70 as min rust
- Update minimum rust to 1.73
- Updated Minimum rust version to 1.74
- Updated circle ci config to use new cli

### Fixed

- Return type should be answer
- Missing ";"
- Spelling of output in description of set_env
- Scorecards to depend on github token only
- Set publish_results false for scorecards (testing cosign issue)
- Move rust security check to Circle CI
- Added missing build and ci types to TypeHierarchy parse
- Removed second treatment of threshold in reporting method
- Made tag refs optional
- Fix default for level to true so that level is set by default
- Adding a fix for a patch bump
- Adding a fix for a patch bump
- Adding a fix for a patch bump
- Refactor next_version.rs to use semantic_version field in Bump::Custom
- [**breaking**] Check option needs to be at the top level so that it can be applied to calculate and require
- [**breaking**] Remove set-env feature as not workable as thought

## [0.7.9] - 2022-12-21

## [0.7.8] - 2022-12-16

## [0.7.7] - 2022-12-16

## [0.7.6] - 2022-12-16

### Added

- Add check option

### Changed

- Feature set as list
- Fix typo in comment
- Update github/codeql-action action to v2.1.35
- Build script to gate let_else
- Update rust crate trycmd to 0.14.5
- Update ossf/scorecard-action digest to b8b2b68
- Update github/codeql-action digest to 62b14cb
- Make check option optional
- Correct log level of to info
- Return output from calculate
- Implementation of type hierarchy checking
- Fix breaking test as test incorrect
- Update tests for trycmd
- Align tests with code changes
- Split release job into two
- Fix release ready script
- Debug verbosity for nextsv
- Use check in  CI

### Fixed

- Case where major is 0
- Update rust crate clap to 4.0.29
- Correct Enforcelevel values
- Update rust crate env_logger to 0.10.0

## [0.7.5] - 2022-12-05

## [0.7.4] - 2022-11-19

### Changed

- Fix release nextsv specification
- Update github/codeql-action digest
- Update ossf/scorecard-action digest
- Update actions/checkout digest
- Update actions/upload-artifact digest to 6673cd0
- Update ossf/scorecard-action action to v1.1.2
- Update actions/checkout action to v3.1.0
- Update actions/upload-artifact action to v3.1.1
- Update github/codeql-action action to v2
- Update rust crate trycmd to 0.14.4
- Update ossf/scorecard-action action to v2
- Release
- Update cmd line tests as without updates the test should fail
- Removing testing title
- Correct required file check

### Fixed

- Check  backwards
- Rename of variable
- Update rust crate clap to 4.0.26
- Update rust crate env_logger to 0.9.3

## [0.7.3] - 2022-11-05

### Changed

- Update actions/upload-artifact digest to 83fd05a
- Feature enable missing doc
- Feature enable missing_doc_code_example
- Allow unstable feature
- Fix feature unblock by adding config
- Update github/codeql-action digest to 40542d3
- Update ossf/scorecard-action digest to 1455f79
- Update actions/checkout digest to 1f9a0c2
- Update
- Release
- Add tests for CLI expected outputs

### Fixed

- Update rust crate clap to 3.2.23
- Update rust crate env_logger to 0.9.1
- Update rust crate git2 to 0.15.0
- Update rust crate clap to v4
- Update rust crate clap to v4
- Adapt to Clap 4.0

## [0.7.2] - 2022-09-24

### Changed

- Release

## [0.7.1] - 2022-09-18

### Changed

- Tidy up the change logs
- Update security policy
- Preload security in unreleased
- Spacing in change logs
- Update nextsv calculation
- Update enforce flag to -e
- Release
- Rename require-level enforce-level

### Fixed

- Clippy lint failure on not deriving Eq

## [0.7.0] - 2022-08-22

### Added

- ✨ require switch in cli
- Multiple value flag on cli config
- Check that required files are listed
- ✨ have_required method for VersionCalculator
- NoConventionalCommits error
- Error if no commits in struct
- Error message will pass the filename
- MissingRequired File error
- No files listed and file list
- Has_required function
- Collect file names during walk
- Use diff to get file list as OsStrings
- Required_level to enforce

### Changed

- (ci) remove redundant rustup in docs job
- Simplify options
- Rename commits walk_commits
- Trace file names found
- Use HashSet

### Fixed

- Files check as part of the calculation
- Pass vec and not reference to vec

## [0.6.2] - 2022-08-20

## [0.6.1] - 2022-08-14

### Changed

- Release

### Fixed

- (docs) minimum rust release graphic

## [0.6.0] - 2022-08-14

### Added

- Custom image for execution environment

### Changed

- (ci) remove rustup
- Release
- Remove installs included in custom executor

### Fixed

- (docs) update min rust version to 1.60
- (crate) update rust-version to 1.60

## [0.5.2] - 2022-08-08

### Changed

- If test publish only not none
- Release
- Add else block to halt instead of fail.

### Fixed

- Allow none as valid response
- Clippy lint on unused Level

## [0.5.1] - 2022-08-07

### Added

- ✨ Add logging feature to crate
- ✨ Add logging to the CLI.
- ✨ Log the command running and errors
- ✨ Logging for calculator
- 🎨 Report level with   version number
- Exit with an error

### Changed

- (ci) update address for crates.io
- Release
- 🎨 Check using nextsv to fail quickly
- Update Changelogs
- 🎨 Remove count fields from the struct
- 🎨 replace old methods with new
- 🎨 replace specific functions with generic in verbosity
- Tidy up use statement for nextsv
- Update version help text
- Update log messages
- Help text for CLI command level
- Simplify interface by removing the subcommands
- Single function to implement force options
- Use increment_counts
- Feature flags no longer required
- Update call to nextsv in CI

### Fixed

- Registry must be a https:// link not a ssh link
- Correct specification of registry
- Align documentation tests

## [0.4.0] - 2022-07-31

### Added

- Create enum of bump levels
- ✨ add patch level of none when no conventional commits are found
- Instead of Level::None return and error NoLevelChange
- Add error for no level change

### Changed

- 🎨 Update changelogs
- Update github/codeql-action digest to d8c9c72
- Update ossf/scorecard-action digest to 88c5e32
- Update dependency cimg/rust to v1.62
- Update ossf/scorecard-action digest to d434c40
- Update ossf/scorecard-action digest to ccd0038
- Update github/codeql-action digest to ba95eeb
- Update github/codeql-action digest to b8bd06e
- Update ossf/scorecard-action digest to 0c37758
- Update github/codeql-action digest to 8171514
- Update ossf/scorecard-action digest to 3155d13
- ✨ Add workflow to check  for and release

### Fixed

- Update rust crate clap to 3.2.11
- Update rust crate clap to 3.2.12
- Update rust crate git-conventional to 0.12.0
- Update rust crate clap to 3.2.13
- Update rust crate clap to 3.2.14
- 🐛 Spelling error in error text

## [0.3.1] - 2022-07-11

### Fixed

- Errors found after cargo release run

## [0.3.0] - 2022-07-11

### Added

- Create enum of bump levels
- ✨ Features for calculation of level or version number
- ✨ Error for case where no conventional commits have been found
- ✨ function to calculate next level based on recent commits
- ✨ Implement display for semantic::Level

### Changed

- ✨ Commit based changelog using git cliff application
- 🎨 separate version calculation into a dedicated function version
- 🎨 move level printing code to separate function for level
- 🎨 Two subcommands for version and level output
- 🎨 Tidy off testing aids

### Fixed

- Fix errors in drafted Level code
- 🐛 replace tag identification using 'v' with prefix variable

## [0.2.0] - 2022-06-27

### Added

- ✨ cli based on clap with verbose setting
- ✨ force option on cli to force a specific level of update

### Changed

- 🔥 Remove dbg! macros
- 📝 Update release version in Cargo.toml to 0.1.1
- Update version in Cargo.toml to 0.2.0

### Fixed

- 🐛 Set lower components to 0 on increment

## [0.1.1] - 2022-06-26

### Fixed

- 🐛 Fix failure to detect separate tag and correct calculation of the next version
- 🐛 Test both other and fix_commits values for patch increment (major=0)

## [0.1.0] - 2022-06-25

### Added

- ✨ Add Semantic version struct and methods to display and increment components
- ✨ Add error module for nextsv library
- ✨ Add dependencies for error ,management
- ✨ add parse method to parse a git tag into a semantic version
- Count conventional commits to last tag
- ✨ abstraction for conventional commit
- ✨ describe a version tag
- Add module references to library and testing code in main, settings updates
- ✨ create function to calculate next semantic version

### Changed

- 📝 Update documentation for semantic module to refer to semver standard
- ✨ Initial announcement to reserve crate name
- Add CI to test and check the code
- Update security and changelog notices
- Add cargo release pre-release replacements
- 🎨 Refactor into library and binary
- Tuning updates

[#147]: https://github.com/jerus-org/nextsv/pull/147
[#148]: https://github.com/jerus-org/nextsv/pull/148
[#149]: https://github.com/jerus-org/nextsv/pull/149
[#150]: https://github.com/jerus-org/nextsv/pull/150
[#151]: https://github.com/jerus-org/nextsv/pull/151
[#152]: https://github.com/jerus-org/nextsv/pull/152
[#154]: https://github.com/jerus-org/nextsv/pull/154
[#153]: https://github.com/jerus-org/nextsv/pull/153
[#155]: https://github.com/jerus-org/nextsv/pull/155
[#156]: https://github.com/jerus-org/nextsv/pull/156
[#158]: https://github.com/jerus-org/nextsv/pull/158
[#157]: https://github.com/jerus-org/nextsv/pull/157
[#159]: https://github.com/jerus-org/nextsv/pull/159
[#160]: https://github.com/jerus-org/nextsv/pull/160
[#164]: https://github.com/jerus-org/nextsv/pull/164
[#162]: https://github.com/jerus-org/nextsv/pull/162
[#163]: https://github.com/jerus-org/nextsv/pull/163
[#165]: https://github.com/jerus-org/nextsv/pull/165
[#166]: https://github.com/jerus-org/nextsv/pull/166
[#168]: https://github.com/jerus-org/nextsv/pull/168
[#169]: https://github.com/jerus-org/nextsv/pull/169
[#170]: https://github.com/jerus-org/nextsv/pull/170
[#171]: https://github.com/jerus-org/nextsv/pull/171
[#172]: https://github.com/jerus-org/nextsv/pull/172
[#173]: https://github.com/jerus-org/nextsv/pull/173
[#174]: https://github.com/jerus-org/nextsv/pull/174
[#175]: https://github.com/jerus-org/nextsv/pull/175
[#176]: https://github.com/jerus-org/nextsv/pull/176
[#177]: https://github.com/jerus-org/nextsv/pull/177
[#178]: https://github.com/jerus-org/nextsv/pull/178
[#179]: https://github.com/jerus-org/nextsv/pull/179
[#181]: https://github.com/jerus-org/nextsv/pull/181
[#183]: https://github.com/jerus-org/nextsv/pull/183
[#180]: https://github.com/jerus-org/nextsv/pull/180
[#182]: https://github.com/jerus-org/nextsv/pull/182
[#184]: https://github.com/jerus-org/nextsv/pull/184
[#185]: https://github.com/jerus-org/nextsv/pull/185
[#186]: https://github.com/jerus-org/nextsv/pull/186
[#187]: https://github.com/jerus-org/nextsv/pull/187
[#188]: https://github.com/jerus-org/nextsv/pull/188
[#189]: https://github.com/jerus-org/nextsv/pull/189
[#191]: https://github.com/jerus-org/nextsv/pull/191
[#192]: https://github.com/jerus-org/nextsv/pull/192
[#190]: https://github.com/jerus-org/nextsv/pull/190
[#193]: https://github.com/jerus-org/nextsv/pull/193
[#194]: https://github.com/jerus-org/nextsv/pull/194
[#195]: https://github.com/jerus-org/nextsv/pull/195
[#196]: https://github.com/jerus-org/nextsv/pull/196
[#197]: https://github.com/jerus-org/nextsv/pull/197
[#199]: https://github.com/jerus-org/nextsv/pull/199
[#200]: https://github.com/jerus-org/nextsv/pull/200
[#201]: https://github.com/jerus-org/nextsv/pull/201
[#203]: https://github.com/jerus-org/nextsv/pull/203
[#202]: https://github.com/jerus-org/nextsv/pull/202
[#204]: https://github.com/jerus-org/nextsv/pull/204
[#208]: https://github.com/jerus-org/nextsv/pull/208
[#211]: https://github.com/jerus-org/nextsv/pull/211
[#213]: https://github.com/jerus-org/nextsv/pull/213
[#214]: https://github.com/jerus-org/nextsv/pull/214
[#215]: https://github.com/jerus-org/nextsv/pull/215
[#217]: https://github.com/jerus-org/nextsv/pull/217
[#216]: https://github.com/jerus-org/nextsv/pull/216
[#218]: https://github.com/jerus-org/nextsv/pull/218
[#219]: https://github.com/jerus-org/nextsv/pull/219
[#221]: https://github.com/jerus-org/nextsv/pull/221
[#222]: https://github.com/jerus-org/nextsv/pull/222
[#223]: https://github.com/jerus-org/nextsv/pull/223
[#224]: https://github.com/jerus-org/nextsv/pull/224
[#225]: https://github.com/jerus-org/nextsv/pull/225
[#227]: https://github.com/jerus-org/nextsv/pull/227
[#226]: https://github.com/jerus-org/nextsv/pull/226
[#228]: https://github.com/jerus-org/nextsv/pull/228
[#229]: https://github.com/jerus-org/nextsv/pull/229
[#230]: https://github.com/jerus-org/nextsv/pull/230
[#231]: https://github.com/jerus-org/nextsv/pull/231
[#232]: https://github.com/jerus-org/nextsv/pull/232
[#233]: https://github.com/jerus-org/nextsv/pull/233
[#220]: https://github.com/jerus-org/nextsv/pull/220
[#234]: https://github.com/jerus-org/nextsv/pull/234
[#235]: https://github.com/jerus-org/nextsv/pull/235
[#236]: https://github.com/jerus-org/nextsv/pull/236
[#238]: https://github.com/jerus-org/nextsv/pull/238
[#239]: https://github.com/jerus-org/nextsv/pull/239
[#242]: https://github.com/jerus-org/nextsv/pull/242
[#240]: https://github.com/jerus-org/nextsv/pull/240
[#243]: https://github.com/jerus-org/nextsv/pull/243
[#244]: https://github.com/jerus-org/nextsv/pull/244
[#245]: https://github.com/jerus-org/nextsv/pull/245
[#246]: https://github.com/jerus-org/nextsv/pull/246
[#247]: https://github.com/jerus-org/nextsv/pull/247
[#248]: https://github.com/jerus-org/nextsv/pull/248
[#250]: https://github.com/jerus-org/nextsv/pull/250
[#251]: https://github.com/jerus-org/nextsv/pull/251
[#252]: https://github.com/jerus-org/nextsv/pull/252
[#253]: https://github.com/jerus-org/nextsv/pull/253
[#254]: https://github.com/jerus-org/nextsv/pull/254
[#255]: https://github.com/jerus-org/nextsv/pull/255
[#256]: https://github.com/jerus-org/nextsv/pull/256
[#257]: https://github.com/jerus-org/nextsv/pull/257
[#258]: https://github.com/jerus-org/nextsv/pull/258
[#259]: https://github.com/jerus-org/nextsv/pull/259
[#260]: https://github.com/jerus-org/nextsv/pull/260
[#261]: https://github.com/jerus-org/nextsv/pull/261
[#262]: https://github.com/jerus-org/nextsv/pull/262
[#263]: https://github.com/jerus-org/nextsv/pull/263
[#264]: https://github.com/jerus-org/nextsv/pull/264
[#266]: https://github.com/jerus-org/nextsv/pull/266
[#265]: https://github.com/jerus-org/nextsv/pull/265
[#267]: https://github.com/jerus-org/nextsv/pull/267
[#269]: https://github.com/jerus-org/nextsv/pull/269
[#270]: https://github.com/jerus-org/nextsv/pull/270
[#271]: https://github.com/jerus-org/nextsv/pull/271
[#268]: https://github.com/jerus-org/nextsv/pull/268
[#272]: https://github.com/jerus-org/nextsv/pull/272
[#273]: https://github.com/jerus-org/nextsv/pull/273
[#274]: https://github.com/jerus-org/nextsv/pull/274
[#275]: https://github.com/jerus-org/nextsv/pull/275
[#276]: https://github.com/jerus-org/nextsv/pull/276
[#277]: https://github.com/jerus-org/nextsv/pull/277
[#278]: https://github.com/jerus-org/nextsv/pull/278
[#279]: https://github.com/jerus-org/nextsv/pull/279
[#280]: https://github.com/jerus-org/nextsv/pull/280
[#281]: https://github.com/jerus-org/nextsv/pull/281
[#282]: https://github.com/jerus-org/nextsv/pull/282
[#283]: https://github.com/jerus-org/nextsv/pull/283
[#284]: https://github.com/jerus-org/nextsv/pull/284
[#285]: https://github.com/jerus-org/nextsv/pull/285
[#287]: https://github.com/jerus-org/nextsv/pull/287
[#286]: https://github.com/jerus-org/nextsv/pull/286
[#288]: https://github.com/jerus-org/nextsv/pull/288
[#289]: https://github.com/jerus-org/nextsv/pull/289
[#290]: https://github.com/jerus-org/nextsv/pull/290
[#291]: https://github.com/jerus-org/nextsv/pull/291
[#292]: https://github.com/jerus-org/nextsv/pull/292
[#293]: https://github.com/jerus-org/nextsv/pull/293
[#294]: https://github.com/jerus-org/nextsv/pull/294
[#295]: https://github.com/jerus-org/nextsv/pull/295
[#296]: https://github.com/jerus-org/nextsv/pull/296
[#297]: https://github.com/jerus-org/nextsv/pull/297
[#298]: https://github.com/jerus-org/nextsv/pull/298
[#299]: https://github.com/jerus-org/nextsv/pull/299
[#300]: https://github.com/jerus-org/nextsv/pull/300
[#301]: https://github.com/jerus-org/nextsv/pull/301
[#302]: https://github.com/jerus-org/nextsv/pull/302
[#303]: https://github.com/jerus-org/nextsv/pull/303
[#304]: https://github.com/jerus-org/nextsv/pull/304
[#305]: https://github.com/jerus-org/nextsv/pull/305
[#306]: https://github.com/jerus-org/nextsv/pull/306
[#307]: https://github.com/jerus-org/nextsv/pull/307
[#308]: https://github.com/jerus-org/nextsv/pull/308
[#309]: https://github.com/jerus-org/nextsv/pull/309
[#310]: https://github.com/jerus-org/nextsv/pull/310
[#311]: https://github.com/jerus-org/nextsv/pull/311
[#312]: https://github.com/jerus-org/nextsv/pull/312
[#313]: https://github.com/jerus-org/nextsv/pull/313
[#314]: https://github.com/jerus-org/nextsv/pull/314
[#315]: https://github.com/jerus-org/nextsv/pull/315
[#316]: https://github.com/jerus-org/nextsv/pull/316
[#317]: https://github.com/jerus-org/nextsv/pull/317
[#318]: https://github.com/jerus-org/nextsv/pull/318
[#319]: https://github.com/jerus-org/nextsv/pull/319
[#320]: https://github.com/jerus-org/nextsv/pull/320
[#321]: https://github.com/jerus-org/nextsv/pull/321
[#322]: https://github.com/jerus-org/nextsv/pull/322
[#323]: https://github.com/jerus-org/nextsv/pull/323
[#324]: https://github.com/jerus-org/nextsv/pull/324
[#325]: https://github.com/jerus-org/nextsv/pull/325
[#326]: https://github.com/jerus-org/nextsv/pull/326
[#327]: https://github.com/jerus-org/nextsv/pull/327
[#328]: https://github.com/jerus-org/nextsv/pull/328
[#329]: https://github.com/jerus-org/nextsv/pull/329
[#330]: https://github.com/jerus-org/nextsv/pull/330
[#331]: https://github.com/jerus-org/nextsv/pull/331
[#332]: https://github.com/jerus-org/nextsv/pull/332
[#333]: https://github.com/jerus-org/nextsv/pull/333
[#334]: https://github.com/jerus-org/nextsv/pull/334
[#338]: https://github.com/jerus-org/nextsv/pull/338
[#335]: https://github.com/jerus-org/nextsv/pull/335
[#336]: https://github.com/jerus-org/nextsv/pull/336
[#337]: https://github.com/jerus-org/nextsv/pull/337
[#339]: https://github.com/jerus-org/nextsv/pull/339
[#340]: https://github.com/jerus-org/nextsv/pull/340
[#341]: https://github.com/jerus-org/nextsv/pull/341
[#343]: https://github.com/jerus-org/nextsv/pull/343
[#342]: https://github.com/jerus-org/nextsv/pull/342
[#344]: https://github.com/jerus-org/nextsv/pull/344
[#345]: https://github.com/jerus-org/nextsv/pull/345
[#346]: https://github.com/jerus-org/nextsv/pull/346
[#347]: https://github.com/jerus-org/nextsv/pull/347
[#348]: https://github.com/jerus-org/nextsv/pull/348
[#349]: https://github.com/jerus-org/nextsv/pull/349
[#350]: https://github.com/jerus-org/nextsv/pull/350
[#351]: https://github.com/jerus-org/nextsv/pull/351
[#353]: https://github.com/jerus-org/nextsv/pull/353
[#354]: https://github.com/jerus-org/nextsv/pull/354
[#355]: https://github.com/jerus-org/nextsv/pull/355
[#356]: https://github.com/jerus-org/nextsv/pull/356
[#357]: https://github.com/jerus-org/nextsv/pull/357
[#358]: https://github.com/jerus-org/nextsv/pull/358
[#359]: https://github.com/jerus-org/nextsv/pull/359
[#360]: https://github.com/jerus-org/nextsv/pull/360
[#361]: https://github.com/jerus-org/nextsv/pull/361
[#362]: https://github.com/jerus-org/nextsv/pull/362
[#363]: https://github.com/jerus-org/nextsv/pull/363
[#364]: https://github.com/jerus-org/nextsv/pull/364
[Unreleased]: https://github.com/jerus-org/nextsv/compare/v0.19.22...HEAD
[0.19.22]: https://github.com/jerus-org/nextsv/compare/v0.19.21...v0.19.22
[0.19.21]: https://github.com/jerus-org/nextsv/compare/v0.19.20...v0.19.21
[0.19.20]: https://github.com/jerus-org/nextsv/compare/v0.19.19...v0.19.20
[0.19.19]: https://github.com/jerus-org/nextsv/compare/v0.19.18...v0.19.19
[0.19.18]: https://github.com/jerus-org/nextsv/compare/v0.19.17...v0.19.18
[0.19.17]: https://github.com/jerus-org/nextsv/compare/v0.19.16...v0.19.17
[0.19.16]: https://github.com/jerus-org/nextsv/compare/v0.19.15...v0.19.16
[0.19.15]: https://github.com/jerus-org/nextsv/compare/v0.19.14...v0.19.15
[0.19.14]: https://github.com/jerus-org/nextsv/compare/v0.19.13...v0.19.14
[0.19.13]: https://github.com/jerus-org/nextsv/compare/v0.19.12...v0.19.13
[0.19.12]: https://github.com/jerus-org/nextsv/compare/v0.19.11...v0.19.12
[0.19.11]: https://github.com/jerus-org/nextsv/compare/v0.19.10...v0.19.11
[0.19.10]: https://github.com/jerus-org/nextsv/compare/v0.19.9...v0.19.10
[0.19.9]: https://github.com/jerus-org/nextsv/compare/v0.19.8...v0.19.9
[0.19.8]: https://github.com/jerus-org/nextsv/compare/v0.19.7...v0.19.8
[0.19.7]: https://github.com/jerus-org/nextsv/compare/v0.19.6...v0.19.7
[0.19.6]: https://github.com/jerus-org/nextsv/compare/v0.19.5...v0.19.6
[0.19.5]: https://github.com/jerus-org/nextsv/compare/v0.19.4...v0.19.5
[0.19.4]: https://github.com/jerus-org/nextsv/compare/v0.19.3...v0.19.4
[0.19.3]: https://github.com/jerus-org/nextsv/compare/v0.19.2...v0.19.3
[0.19.2]: https://github.com/jerus-org/nextsv/compare/v0.19.1...v0.19.2
[0.19.1]: https://github.com/jerus-org/nextsv/compare/v0.19.0...v0.19.1
[0.19.0]: https://github.com/jerus-org/nextsv/compare/v0.18.0...v0.19.0
[0.18.0]: https://github.com/jerus-org/nextsv/compare/v0.17.0...v0.18.0
[0.17.0]: https://github.com/jerus-org/nextsv/compare/v0.16.0...v0.17.0
[0.16.0]: https://github.com/jerus-org/nextsv/compare/v0.15.0...v0.16.0
[0.15.0]: https://github.com/jerus-org/nextsv/compare/v0.14.0...v0.15.0
[0.14.0]: https://github.com/jerus-org/nextsv/compare/v0.13.0...v0.14.0
[0.13.0]: https://github.com/jerus-org/nextsv/compare/v0.12.0...v0.13.0
[0.12.0]: https://github.com/jerus-org/nextsv/compare/v0.11.0...v0.12.0
[0.11.0]: https://github.com/jerus-org/nextsv/compare/v0.10.0...v0.11.0
[0.10.0]: https://github.com/jerus-org/nextsv/compare/v0.9.2...v0.10.0
[0.9.2]: https://github.com/jerus-org/nextsv/compare/v0.9.1...v0.9.2
[0.9.1]: https://github.com/jerus-org/nextsv/compare/v0.8.22...v0.9.1
[0.8.22]: https://github.com/jerus-org/nextsv/compare/v0.8.21...v0.8.22
[0.8.21]: https://github.com/jerus-org/nextsv/compare/v0.8.20...v0.8.21
[0.8.20]: https://github.com/jerus-org/nextsv/compare/v0.8.19...v0.8.20
[0.8.19]: https://github.com/jerus-org/nextsv/compare/v0.8.18...v0.8.19
[0.8.18]: https://github.com/jerus-org/nextsv/compare/v0.8.17...v0.8.18
[0.8.17]: https://github.com/jerus-org/nextsv/compare/v0.8.16...v0.8.17
[0.8.16]: https://github.com/jerus-org/nextsv/compare/v0.8.15...v0.8.16
[0.8.15]: https://github.com/jerus-org/nextsv/compare/v0.8.14...v0.8.15
[0.8.14]: https://github.com/jerus-org/nextsv/compare/v0.8.13...v0.8.14
[0.8.13]: https://github.com/jerus-org/nextsv/compare/v0.8.12...v0.8.13
[0.8.12]: https://github.com/jerus-org/nextsv/compare/v0.8.11...v0.8.12
[0.8.11]: https://github.com/jerus-org/nextsv/compare/v0.8.10...v0.8.11
[0.8.10]: https://github.com/jerus-org/nextsv/compare/v0.8.9...v0.8.10
[0.8.9]: https://github.com/jerus-org/nextsv/compare/v0.8.8...v0.8.9
[0.8.8]: https://github.com/jerus-org/nextsv/compare/v0.8.7...v0.8.8
[0.8.7]: https://github.com/jerus-org/nextsv/compare/v0.8.6...v0.8.7
[0.8.6]: https://github.com/jerus-org/nextsv/compare/v0.8.5...v0.8.6
[0.8.5]: https://github.com/jerus-org/nextsv/compare/v0.8.4...v0.8.5
[0.8.4]: https://github.com/jerus-org/nextsv/compare/v0.8.3...v0.8.4
[0.8.3]: https://github.com/jerus-org/nextsv/compare/v0.8.2...v0.8.3
[0.8.2]: https://github.com/jerus-org/nextsv/compare/v0.8.1...v0.8.2
[0.8.1]: https://github.com/jerus-org/nextsv/compare/v0.8.0...v0.8.1
[0.8.0]: https://github.com/jerus-org/nextsv/compare/v0.7.9...v0.8.0
[0.7.9]: https://github.com/jerus-org/nextsv/compare/v0.7.8...v0.7.9
[0.7.8]: https://github.com/jerus-org/nextsv/compare/v0.7.7...v0.7.8
[0.7.7]: https://github.com/jerus-org/nextsv/compare/v0.7.6...v0.7.7
[0.7.6]: https://github.com/jerus-org/nextsv/compare/v0.7.5...v0.7.6
[0.7.5]: https://github.com/jerus-org/nextsv/compare/v0.7.4...v0.7.5
[0.7.4]: https://github.com/jerus-org/nextsv/compare/v0.7.3...v0.7.4
[0.7.3]: https://github.com/jerus-org/nextsv/compare/v0.7.2...v0.7.3
[0.7.2]: https://github.com/jerus-org/nextsv/compare/v0.7.1...v0.7.2
[0.7.1]: https://github.com/jerus-org/nextsv/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/jerus-org/nextsv/compare/v0.6.2...v0.7.0
[0.6.2]: https://github.com/jerus-org/nextsv/compare/v0.6.1...v0.6.2
[0.6.1]: https://github.com/jerus-org/nextsv/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/jerus-org/nextsv/compare/v0.5.2...v0.6.0
[0.5.2]: https://github.com/jerus-org/nextsv/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/jerus-org/nextsv/compare/v0.4.0...v0.5.1
[0.4.0]: https://github.com/jerus-org/nextsv/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/jerus-org/nextsv/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/jerus-org/nextsv/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/jerus-org/nextsv/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/jerus-org/nextsv/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/jerus-org/nextsv/releases/tag/v0.1.0
