//! Error types for nextsv

use std::ffi::OsString;

use proc_exit::{Code, Exit};
use thiserror::Error;

const EXIT_UNEXPECTED_ERROR: i32 = 10;
const EXIT_NOT_CALCULATED_CODE: i32 = 12;
const EXIT_MISSING_REQUIRED_CODE: i32 = 13;
const EXIT_NOT_REQUIRED_LEVEL: i32 = 14;
const EXIT_NO_FILES_LISTED: i32 = 15;

/// The error type for nextsv.
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Error {
    /// The tag provided is not a version tag as it does not
    /// start with the provided prefix string.
    #[error("Version tags must start with \"{0}\" but tag is {1}")]
    NotVersionTag(String, String),
    /// Too many components found.
    #[error("Version must have three components but at least {0} were found")]
    TooManyComponents(usize),
    /// Too few components found.
    #[error("Version must have three components but only {0} found")]
    TooFewComponents(usize),
    /// The component must be a digit
    #[error("Version must be a number but found {0}")]
    MustBeNumber(String),
    /// No valid version tag was found in the repository
    #[error("No valid version tag found in the repository")]
    NoVersionTag,
    /// The first production release (1.0.0) has already been made
    #[error("First production release already deployed. Current major version: {0}")]
    MajorAlreadyUsed(String),
    /// No conventional commits in the VersionCalculator struct
    #[error("No conventional commits have been loaded into the VersionCalculator struct. May have been called before `commits`.")]
    NoConventionalCommits,
    /// Missing required file found.
    #[error("Missing the required file(s): {0:?}.")]
    MissingRequiredFile(Vec<OsString>),
    /// Not a valid Type Hierachy name.
    #[error("{0} is not a valid type hierarchy namne.")]
    NotTypeHierachyName(String),
    /// List of files has not been generated yet (or there are no commits). Call `commits` to generate the list by walking back to the current version tag.
    #[error("No files have been listed. May have been called before `commits`.")]
    NoFilesListed,
    /// The minimum change level set for check has been met.
    #[error("Minimum change Level has been met.")]
    MinimumChangeLevelMet,
    /// The minimum change level set for check has not been met.
    #[error("Minimum change level has not been met.")]
    MinimumChangeLevelNotMet,
    /// Error passed up from git2
    #[error("0:?")]
    Git2(#[from] git2::Error),
    /// Invalid Pre-Release Format
    #[error("Invalid PreRelease format: {0}")]
    InvalidPreReleaseFormat(String),
}

impl From<Error> for Exit {
    fn from(err: Error) -> Self {
        match err {
            Error::Git2(_) => {
                Exit::new(Code::new(EXIT_NOT_CALCULATED_CODE)).with_message(err.to_string())
            }
            Error::MissingRequiredFile(_) => {
                Exit::new(Code::new(EXIT_MISSING_REQUIRED_CODE)).with_message(err.to_string())
            }
            Error::NoFilesListed => {
                Exit::new(Code::new(EXIT_NO_FILES_LISTED)).with_message(err.to_string())
            }
            Error::MinimumChangeLevelMet => Exit::new(Code::SUCCESS).with_message(err.to_string()),
            Error::MinimumChangeLevelNotMet => {
                Exit::new(Code::new(EXIT_NOT_REQUIRED_LEVEL)).with_message(err.to_string())
            }
            _ => Exit::new(Code::new(EXIT_UNEXPECTED_ERROR)),
        }
    }
}
