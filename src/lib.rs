//! A simple profiler for Rust applications.
//!
//! This crate provides a basic profiling utility that can be used to measure the elapsed time of code execution.
//! See the `CHANGELOG.md`` for updates and changes.
//!
//! The TimeLapse profiler is open-source and can be freely used and modified under the terms of the MIT license.

pub mod profiler;

pub use profiler::TimeLapse;
