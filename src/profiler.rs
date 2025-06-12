//! A simple profiler for measuring elapsed time in Rust.
//!
//! This module provides a `TimeLapse` struct and macros to start and end profiling.
//! Macros `profile_start!` and `profile_end!` are used to simplify the profiling process.
//!
//! See the tesing examples at the end of this source file for usage.

#![allow(unused)]
use log::info;
use std::time::{Duration, Instant};

/// The `profile_start!` macro initializes a `TimeLapse` instance to start profiling.
/// It takes an identifier as an argument, which will be instantiated and used to reference the profiler instance.
///
/// # Usage
/// ```rust
/// use std::time::Duration;
/// use timelapse::{TimeLapse, profile_start, profile_end};
///
/// profile_start!(my_profiler);
///
/// std::thread::sleep(Duration::from_millis(100));
/// assert!(my_profiler.elapsed().as_millis() >= 100);
///
/// profile_end!(my_profiler);
/// ```
#[macro_export]
macro_rules! profile_start {
    ($name:ident) => {
        let $name = TimeLapse::new();
    };
}

/// The `profile_end!` macro logs the elapsed time of the profiling instance created by `profile_start!`.
/// These macros are useful for quick profiling without needing to manually create and manage `TimeLapse` instances.
/// # Usage
/// ```rust
/// use std::time::Duration;
/// use timelapse::{TimeLapse, profile_start, profile_end};
///
/// profile_start!(my_profiler);
///
/// std::thread::sleep(Duration::from_millis(100));
/// assert!(my_profiler.elapsed().as_millis() >= 100);
///
/// profile_end!(my_profiler);
/// ```
#[macro_export]
macro_rules! profile_end {
    ($name:ident) => {
        $name.log(stringify!($name));
    };
}

/// The `TimeLapse` struct is used to measure elapsed time in Rust applications.
/// It provides methods to start, reset, and log the elapsed time.
/// It can be used to profile code execution and is useful for performance analysis.
/// It implements the `Display` and `Debug` traits for easy formatting and logging.
pub struct TimeLapse {
    start_time: Instant,
}

impl TimeLapse {
    /// Creates a new `TimeLapse` instance, starting the timer immediately.
    pub fn new() -> Self {
        TimeLapse {
            start_time: Instant::now(),
        }
    }

    /// Returns the elapsed time since the `TimeLapse` instance was created or reset.
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Resets the timer, starting a new measurement from the current time.
    pub fn reset(&mut self) {
        self.start_time = Instant::now();
    }
}

/// Implements the `Display` trait for the `TimeLapse` struct.
impl std::fmt::Display for TimeLapse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Elapsed time: {:?}", self.elapsed())
    }
}

/// Implements the `Debug` trait for the `TimeLapse` struct.
impl std::fmt::Debug for TimeLapse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TimeLapse {{ elapsed: {:?} }}", self.elapsed())
    }
}

/// Implements the `Default` trait for the `TimeLapse` struct, allowing it to be created with default values.
impl std::default::Default for TimeLapse {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeLapse {
    /// Logs the elapsed time with a given name.
    pub fn log(&self, name: &str) {
        info!("TimeLapse {} - Elapsed time: {:?}", name, self.elapsed());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profiler() {
        let profiler = TimeLapse::new();
        std::thread::sleep(Duration::from_millis(100));
        assert!(profiler.elapsed().as_millis() >= 100);
        profiler.log("test");
    }

    #[test]
    fn test_profiler_macros() {
        profile_start!(the_profile);
        std::thread::sleep(Duration::from_millis(100));
        assert!(the_profile.elapsed().as_millis() >= 100);
        profile_end!(the_profile);
    }

    #[test]
    fn test_profiler_reset() {
        let mut profiler = TimeLapse::new();
        std::thread::sleep(Duration::from_millis(100));
        profiler.reset();
        assert!(profiler.elapsed().as_millis() < 50);
    }
}
