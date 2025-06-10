//! A simple profiler for measuring elapsed time in Rust.
//!
//! This module provides a `TimeLapse` struct and macros to start and end profiling.
//! Macros `profile_start!` and `profile_end!` are used to simplify the profiling process.
//!
//! See the tesing examples at the end of this source file for usage.

#![allow(unused)]
use log::info;
use std::time::{Duration, Instant};

#[macro_export]
macro_rules! profile_start {
    ($name:ident) => {
        let $name = TimeLapse::new();
    };
}

#[macro_export]
macro_rules! profile_end {
    ($name:ident) => {
        $name.log(stringify!($name));
    };
}

pub struct TimeLapse {
    start_time: Instant,
}

impl TimeLapse {
    pub fn new() -> Self {
        TimeLapse {
            start_time: Instant::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn reset(&mut self) {
        self.start_time = Instant::now();
    }
}

impl std::fmt::Display for TimeLapse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Elapsed time: {:?}", self.elapsed())
    }
}

impl std::fmt::Debug for TimeLapse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TimeLapse {{ elapsed: {:?} }}", self.elapsed())
    }
}

impl std::default::Default for TimeLapse {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeLapse {
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
