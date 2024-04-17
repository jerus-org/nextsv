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
//!     
//!
//!     // get arguments from CLI
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
//!     let mut calculator_config = CalculatorConfig::new(&args.prefix);
//!
//!     // What do we want to output?    
//!     calculator_config.set_print_bump(args.level);
//!     calculator_config.set_print_version_number(args.number);
//!
//!     // Is the bump level being forced?
//!     if let Some(force) = args.force {
//!         calculator_config.set_force_level(force);
//!     };
//!
//!     // Are there files that must be updated? What change level should they be enforced at?
//!     if !args.require.is_empty() {
//!         calculator_config.add_required_files(args.require);
//!         calculator_config.set_file_requirement_enforcement_level(args.enforce_level);
//!     };
//!
//!     // Is three a threshold set that must be met before proceeding with a change?
//!     if let Some(check_level) = args.check {
//!         calculator_config.set_threshold(check_level);
//!     }
//!
//!     // Apply the config and create a calculator
//!     let calculator = calculator_config.build_calculator()?;
//!     
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
pub use version::VersionTag;
