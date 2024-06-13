<!-- markdownlint-disable MD024 -->
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

### Changed

- Update code in doc comment
- Updated doc tests and removed deleted Answer type
- Added documentation for CalculationConfig
- Documentation review and revision for completeness
- Scenarios describing usage
- Scenarios describing usage

### Added

- Export Answer type
- Set environment variable; use Answer type; return error
- Add proc-exit
- Regex implemented to extract version string
- Added builder config struct for VersionCalculator and tests
- [**breaking**] Restructure to allow force to pre-release and first version
- Restructure to allow force to pre-release and first version

### Changed

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

### Changed

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

### Changed

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

### Changed

- Test to make work; prep for env variable
- Update CI to use 1.70 as min rust
- Update minimum rust to 1.73
- Updated Minimum rust version to 1.74
- Updated circle ci config to use new cli

## [0.7.9] - 2022-12-21

## [0.7.8] - 2022-12-16

## [0.7.7] - 2022-12-16

## [0.7.6] - 2022-12-16

### Fixed

- Case where major is 0
- Update rust crate clap to 4.0.29
- Correct Enforcelevel values
- Update rust crate env_logger to 0.10.0

### Changed

- Feature set as list
- Fix typo in comment

### Added

- Add check option

### Changed

- Update github/codeql-action action to v2.1.35
- Build script to gate let_else
- Update rust crate trycmd to 0.14.5
- Update ossf/scorecard-action digest to b8b2b68
- Update github/codeql-action digest to 62b14cb

### Changed

- Make check option optional
- Correct log level of to info
- Return output from calculate
- Implementation of type hierarchy checking

### Changed

- Fix breaking test as test incorrect
- Update tests for trycmd
- Align tests with code changes

### Changed

- Split release job into two
- Fix release ready script
- Debug verbosity for nextsv
- Use check in  CI

## [0.7.5] - 2022-12-05

## [0.7.4] - 2022-11-19

### Fixed

- Check  backwards
- Rename of variable
- Update rust crate clap to 4.0.26
- Update rust crate env_logger to 0.9.3

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

### Changed

- Update cmd line tests as without updates the test should fail
- Removing testing title
- Correct required file check

## [0.7.3] - 2022-11-05

### Fixed

- Update rust crate clap to 3.2.23
- Update rust crate env_logger to 0.9.1
- Update rust crate git2 to 0.15.0
- Update rust crate clap to v4
- Update rust crate clap to v4
- Adapt to Clap 4.0

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

### Changed

- Add tests for CLI expected outputs

## [0.7.2] - 2022-09-24

### Changed

- Release

## [0.7.1] - 2022-09-18

### Fixed

- Clippy lint failure on not deriving Eq

### Changed

- Tidy up the change logs
- Update security policy
- Preload security in unreleased
- Spacing in change logs
- Update nextsv calculation
- Update enforce flag to -e
- Release

### Changed

- Rename require-level enforce-level

## [0.7.0] - 2022-08-22

### Fixed

- Files check as part of the calculation
- Pass vec and not reference to vec

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

### Changed

- Simplify options
- Rename commits walk_commits
- Trace file names found
- Use HashSet

## [0.6.2] - 2022-08-20

## [0.6.1] - 2022-08-14

### Fixed

- (docs) minimum rust release graphic

### Changed

- Release

## [0.6.0] - 2022-08-14

### Fixed

- (docs) update min rust version to 1.60
- (crate) update rust-version to 1.60

### Added

- Custom image for execution environment

### Changed

- (ci) remove rustup
- Release

### Changed

- Remove installs included in custom executor

## [0.5.2] - 2022-08-08

### Fixed

- Allow none as valid response
- Clippy lint on unused Level

### Changed

- If test publish only not none
- Release

### Changed

- Add else block to halt instead of fail.

## [0.5.1] - 2022-08-07

### Fixed

- Registry must be a https:// link not a ssh link
- Correct specification of registry

### Changed

- (ci) update address for crates.io
- Release

### Fixed

- Align documentation tests

### Added

- ✨ Add logging feature to crate
- ✨ Add logging to the CLI.
- ✨ Log the command running and errors
- ✨ Logging for calculator
- 🎨 Report level with   version number
- Exit with an error

### Changed

- 🎨 Check using nextsv to fail quickly
- Update Changelogs

### Changed

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

## [0.4.0] - 2022-07-31

### Fixed

- Update rust crate clap to 3.2.11
- Update rust crate clap to 3.2.12
- Update rust crate git-conventional to 0.12.0
- Update rust crate clap to 3.2.13
- Update rust crate clap to 3.2.14
- 🐛 Spelling error in error text

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

## [0.3.1] - 2022-07-11

### Fixed

- Errors found after cargo release run

## [0.3.0] - 2022-07-11

### Fixed

- Fix errors in drafted Level code
- 🐛 replace tag identification using 'v' with prefix variable

### Changed

- ✨ Commit based changelog using git cliff application

### Added

- Create enum of bump levels
- ✨ Features for calculation of level or version number
- ✨ Error for case where no conventional commits have been found
- ✨ function to calculate next level based on recent commits
- ✨ Implement display for semantic::Level

### Changed

- 🎨 separate version calculation into a dedicated function version
- 🎨 move level printing code to separate function for level
- 🎨 Two subcommands for version and level output
- 🎨 Tidy off testing aids

## [0.2.0] - 2022-06-27

### Fixed

- 🐛 Set lower components to 0 on increment

### Added

- ✨ cli based on clap with verbose setting
- ✨ force option on cli to force a specific level of update

### Changed

- 🔥 Remove dbg! macros
- 📝 Update release version in Cargo.toml to 0.1.1
- Update version in Cargo.toml to 0.2.0

## [0.1.1] - 2022-06-26

### Fixed

- 🐛 Fix failure to detect separate tag and correct calculation of the next version
- 🐛 Test both other and fix_commits values for patch increment (major=0)

## [0.1.0] - 2022-06-25

### Changed

- 📝 Update documentation for semantic module to refer to semver standard

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

- ✨ Initial announcement to reserve crate name
- Add CI to test and check the code
- Update security and changelog notices
- Add cargo release pre-release replacements

### Changed

- 🎨 Refactor into library and binary
- Tuning updates

[Unreleased]: https://github.com/jerusdp/nextsv/compare/v0.8.5...HEAD
[0.8.5]: https://github.com/jerus-org/nextsv/compare/v0.8.4...v0.8.5
[0.8.4]: https://github.com/jerus-org/nextsv/compare/v0.8.3...v0.8.4
[0.8.3]: https://github.com/jerus-org/nextsv/compare/v0.8.2...v0.8.3
[0.8.2]: https://github.com/jerus-org/nextsv/compare/v0.8.1...v0.8.2
[0.8.1]: https://github.com/jerus-org/nextsv/compare/v0.8.0...v0.8.1
[0.8.0]: https://github.com/jerus-org/nextsv/compare/v0.7.9...v0.8.0
[0.7.9]: https://github.com/jerus-org/nextsv/compare/v0.7.8...v0.7.9
[0.7.8]: https://github.com/jerus-org/nextsv/compare/v0.7.7...v0.7.8
[0.7.7]: https://github.com/jerus-org/nextsv/compare/v0.7.6...v0.7.7
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
[0.5.1]: https://github.com/jerus-org/nextsv/compare/v0.5.0...v0.5.1
[0.4.0]: https://github.com/jerus-org/nextsv/compare/v0.3.1...V0.4.0
[0.3.1]: https://github.com/jerus-org/nextsv/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/jerus-org/nextsv/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/jerudp/nextsv/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/jerudp/nextsv/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/jerudp/nextsv/compare/...v0.1.0
