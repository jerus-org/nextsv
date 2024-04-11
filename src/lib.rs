#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(rustdoc_missing_doc_code_examples))]
#![cfg_attr(docsrs, warn(rustdoc::missing_doc_code_examples))]
#![cfg_attr(docsrs, warn(rustdoc::invalid_codeblock_attributes))]

//! Semantic Versioning Management
//!
//! Calculates the next semantic version number and level based on
//! the current version number and the conventional commits made
//! since the last version has been released.
//!
//! ## Usage
//!
//! Add the dependency to Cargo.toml
//!
//! ```toml
//!
//! [dependencies]
//! nextsv = "0.7.9"
//!
//! ```
//!
//! ```no_run
//! # fn main() -> Result<(), nextsv::Error> {
//!     use nextsv::VersionCalculator;
//!     let version_prefix = "v"; // Identifies a version tag
//!
//!     let mut latest_version = VersionCalculator::new(version_prefix)?;
//!
//!     latest_version.walk_commits()?;
//!     latest_version.calculate();
//!
//!     println!("Next Version: {}\nNext Level: {}", latest_version.next_version_number(), latest_version.bump_level());
//!
//! #    Ok(())
//! # }
//! ```

mod calculator;
mod error;
mod version;

pub use calculator::{ForceLevel, LevelHierarchy, VersionCalculator};
pub use error::Error;
pub use version::{Level, VersionTag};
