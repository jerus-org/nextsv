#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(
    missing_docs,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links,
    rustdoc::invalid_rust_codeblocks,
    rustdoc::invalid_codeblock_attributes
)]
#![cfg_attr(docsrs, feature(rustdoc_missing_doc_code_examples))]
#![cfg_attr(docsrs, warn(rustdoc::missing_doc_code_examples))]
#![cfg_attr(docsrs, warn(rustdoc::invalid_codeblock_attributes))]

//! # Calculate next semantic bump and/or version number
//!
//! Calculates the next semantic bump and/or version number based on
//! the current version number and the conventional commits made
//! since the last version has been released.
//!
//! ## Usage
//!
//! Add the dependency to Cargo.toml
//!
//! ```toml
//! [dependencies]
//! nextsv = "0.9.2"
//! ```
//!
//! Calculation workflow:
//! 1. Create the configuration
//! 2. Build the calculator
//! 3. Report the calculation
//!
//! Report the results from the calculator
//!
//! ```no_run
//! #   use nextsv::{CalculatorConfig, ForceBump, Hierarchy};
//! #   use std::ffi::OsString;
//! #
//! #   fn main() -> Result<(), nextsv::Error> {
//! #   struct Args {
//! #       prefix: String,
//! #       level: bool,
//! #       number: bool,
//! #       force: Option<ForceBump>,
//! #       require: Vec<OsString>,
//! #       enforce_level: Hierarchy,
//! #       check: Option<Hierarchy>,
//! #       
//! #   };
//!     // arguments collected from CLI
//!     let args = Args {
//!         prefix: String::from("v"),
//!         level: true,
//!         number: true,
//!         force: None,
//!         require: vec![OsString::from("README.md"), OsString::from("CHANGES.md"), ],
//!         enforce_level: Hierarchy::Feature,
//!         check: None,
//!     };
//!
//!     // 1. Create the configuration
//!
//!     let mut calculator_config = CalculatorConfig::new();
//!
//!     // Set the version number prefix
//!     calculator_config = calculator_config.set_prefix(&args.prefix);
//!
//!     // What do we want to output?    
//!     calculator_config = calculator_config.set_bump_report(args.level);
//!     calculator_config = calculator_config.set_version_report(args.number);
//!
//!     // Is the bump level being forced?
//!     if let Some(force) = args.force {
//!         calculator_config = calculator_config.set_force_bump(force);
//!     };
//!
//!     // Are there files that must be updated? What change level should they be enforced at?
//!     if !args.require.is_empty() {
//!         calculator_config = calculator_config.add_required_files(args.require);
//!         calculator_config = calculator_config.set_required_enforcement(args.enforce_level);
//!     };
//!
//!     // Is three a threshold set that must be met before proceeding with a change?
//!     if let Some(check_level) = args.check {
//!         calculator_config = calculator_config.set_reporting_threshold(check_level);
//!     }
//!
//!     // 2. Build the calculator
//!     // Apply the config and create a calculator
//!     let calculator = calculator_config.build()?;
//!     
//!     // 3. Report the calculations
//!     println!("{}", calculator.report());
//!
//! #    Ok(())
//! # }
//! ```

mod calculator;
mod error;
#[cfg(test)]
mod test_utils;
mod version;

pub use calculator::{Calculator, CalculatorConfig, ForceBump, Hierarchy};
pub use error::Error;
// pub use version::VersionTag;
