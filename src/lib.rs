//! A simple profiler for Rust applications.
//!
//! This crate provides a basic profiling utility that can be used to measure the elapsed time of code execution.
//! See the Changelog sub-section below for updates and changes.
//!
//! The TimeLapse profiler is open-source and can be freely used and modified under the terms of the MIT license.
//!
//! ## Changelog
//!
//! ### [0.1.3] - 2025-06-28
//!
//! - Added two new macros:
//!   - `profile_end_print!()` - Use println! instead of a `std::log` instance.
//!   - `profile_end_log!()` - Additional parameter to select the log's `Level` to use.
//!
//!   See the documentation for usage details.
//!
//! ### [0.1.2] - 2025-06-12
//!
//! - Added example of usage for the defined macros.
//! - Added a CHANGELOG.md file
//!
//! ### [0.1.1] - 2025-06-11
//!
//! - Added  `pub use` entries in `lib.rs` to simplify usage in calling applications.
//!
//! ### [0.1.0] - 2025-06-10
//!
//! Initial release

pub mod profiler;

pub use profiler::TimeLapse;
