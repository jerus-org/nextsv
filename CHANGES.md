<!-- markdownlint-disable MD024 -->
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Fix

- removed dbg macro

### Security

- Dependency Updates

## [0.8.4] - 2024-05-18

### Security

- Dependency Updates

## [0.8.3] - 2024-05-11

### Security

- Dependency Updates

## [0.8.2] - 2024-05-08

### Security

- Dependency Updates

## [0.8.1] - 2024-05-08

### Bug Fixes

- Unstable features bump patch

### Security

- Dependency Updates

## [0.8.0] - 2024-04-30

### Features

- Support the setting of pre-release tagged version numbers
- Clearing the pre-release tag to release the planned version
- Supporting a first version that includes a pre-release tag
- Restructure of CLI to provide
  - calculate, force and require
  - calculate performs the calculation
  - force forces the bump result with a flag to create a first version
  - require enforces changes to specific files if a change threshold is met

### Testing

- Provide more comprehensive testing

### Security

- Dependency Updates

## [0.7.9] - 2022-12-21

### Security

- Dependency Updates

## [0.7.8] - 2022-12-16

### Security

- Dependency Updates

## [0.7.7] - 2022-12-16

### Security

- Dependency Updates

## [0.7.6] - 2022-12-16

### Security

- Dependency Updates

### Features

- add check option to check if a minimum level of release has been met. Program exits with error of the level has not been met.

### Refactor

- Remove EnforceLevel type and use TypeHierarchy defined in conventional instead for CLI flags. Define level when building conventional.

## [0.7.5] - 2022-12-05

### Security

- Dependency Updates

## [0.7.4] - 2022-11-19

### Security

- Dependency Updates

## [0.7.3] - 2022-11-05

### Security

- Dependency Updates

## [0.7.2] - 2022-09-24

### Security

- Dependency Updates

## [0.7.1] - 2022-09-18

### Security

- Dependency Updates

## Refactor

- Update release enforce flag to -e

## [0.7.0] - 2022-08-22

### Features

- required - list documents that are required to be included in the release. The list of files changed since the last release will be checked the programme exits with an error if any of the required files have not been changed since the last release.
- required-level - set a level of changes from which the required files list will be enforced. Four levels are available: breaking, feature, fix, other. The `breaking` level enforces the required files list only for breaking conventional commits. The `other` level enforces the change for all conventional commits. The feature allows a required files list to be used but not enforced for minor changes (e.g. non-code changes such as chores).

## [0.6.2] - 2022-08-20

## [0.6.1] - 2022-08-14

## [0.6.0] - 2022-08-14

### Bug fixes

- Update minimum rust version to 1.60 required for git2

## [0.5.2] - 2022-08-08

## [0.5.1] - 2022-08-07

## [0.5.0] - 2022-08-06

### Bug Fixes

- Align documentation tests

### Features

- Logging for the crate
- Flags to report both level and version number or either
- Report level by default
- Exit with an error when no level change

### Miscellaneous Tasks

- Update Changelogs

### Refactor

- Remove count fields from the struct
- Replace old methods with new
- Replace specific functions with generic
- Tidy up use statement for nextsv
- Update version help text
- Update log messages
- Help text for CLI command level
- Simplify interface
  - remove the subcommands
  - flags for number and level
- Single function to implement force options
- Use increment_counts with type instead of dedicated functions

## [0.4.0] - 2202-07-31

### Bug Fixes

- Update dependencies
- 🐛 Spelling error in error text

### Features

- Create enum of bump levels
- ✨ add patch level of none when no conventional commits are found
- Instead of Level::None return and error NoLevelChange
- Add error for no level change

### Miscellaneous Tasks

- Update changelogs
- Update CI dependencies
- ✨ Add workflow to check for and release

## [0.3.1] - 2022-07-11

## Fix

- Errors found after cargo release run

## [0.3.0] - 2022-07-11

### Documentation

- ✨ Commit based changelog using git cliff application

### Features

- Features for calculation of level or version number
- Error for case where no conventional commits have been found
- Calculate next level based on recent commits

### Refactor

- separate version calculation into a dedicated function version
- move level printing code to separate function for level
- Two cli subcommands for version and level output

## [0.2.0] - 2022-06-27

### Features

- cli based on clap with verbose setting
- force option on cli to force a specific level of update

## [0.1.0] - 2022-06-25

### Documentation

- Initial draft of documentation

### Features

- Initial library

### Miscellaneous Tasks

- ✨ Initial announcement to reserve crate name

<!-- generated by git-cliff -->
<!-- next-url -->
[Unreleased]: https://github.com/jerus-org/nextsv/compare/v0.8.4...HEAD
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
[0.7.0]: <https://github.com/jerus-org/nextsv/compare/v0.6.2...v0.7.0>
[0.6.2]: <https://github.com/jerus-org/nextsv/compare/v0.6.1...v0.6.2>
[0.6.1]: <https://github.com/jerus-org/nextsv/compare/v0.6.0...v0.6.1>
[0.6.0]: <https://github.com/jerus-org/nextsv/compare/v0.5.2...v0.6.0>
[0.5.2]: <https://github.com/jerus-org/nextsv/compare/v0.5.1...v0.5.2>
[0.5.1]: <https://github.com/jerus-org/nextsv/compare/v0.5.0...v0.5.1>
[0.5.0]: <https://github.com/jerus-org/nextsv/compare/v0.4.0...v0.5.0>
[0.4.0]: <https://github.com/jerus-org/nextsv/compare/v0.3.1...v0.4.0>
[0.3.1]: <https://github.com/jerus-org/nextsv/compare/v0.3.0...v0.3.1>
[0.3.0]: <https://github.com/jerus-org/nextsv/compare/v0.2.0...v0.3.0>"
[0.2.0]: <https://github.com/jerudp/nextsv/compare/v0.1.1...v0.2.0>
[0.1.0]: <https://github.com/jerudp/nextsv/compare/...v0.1.0>
