//! Semantic Versioning Management
//!
//! Calculates the next semantic version number based on the current version
//! number and the conventional commits that have been made since the
//! last version has been released.
//!
//! Add the dependency to Cargo.toml
//!
//! ```toml
//! [dependencies]
//! nextsv = "0.3.1"
//! ```
//!
//! ```rust
//!
//!     let current_version = get_latest_version_tag();
//!     let new_version = current_version.bump();
//!
//! ```

mod conventional;
mod error;
mod semantic;
mod version_tag;

pub(crate) use conventional::ConventionalCommits;
pub use error::Error;
pub use semantic::{Level, Semantic};
pub use version_tag::VersionTag;
