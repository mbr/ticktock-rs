//! Utility module
//!
//! Contains convenience traits for converting durations into more easily
//! handled primitives. Note that a loss of precision will occur in most cases,
//! but is negligable in many applications.

use std::time;

const NANOS_PER_SEC: u64 = 1_000_000_000;

/// Convert into 64 bit nanosecond representation.
pub trait NanoSeconds {
    /// Convert duration into `u64`, representing the number of nanoseconds
    /// inside the duration.
    ///
    /// Note that this will panic for durations greater than 584 years
    /// (the maximum number of nanoseconds that fit into 64 bit)
    fn as_ns(&self) -> u64;

    /// Convert `u64` nanoseconds representation into duration.
    ///
    /// Conversion will always work if the target type is larger than 64 bits.
    fn from_ns(ns: u64) -> Self;
}

impl NanoSeconds for time::Duration {
    fn as_ns(&self) -> u64 {
        self.as_secs()
            .checked_mul(NANOS_PER_SEC)
            .expect("overflow during nanosecond conversion")
            .checked_add(self.subsec_nanos() as u64)
            .expect("overflow during nanosecond conversion")
    }

    fn from_ns(ns: u64) -> time::Duration {
        time::Duration::new(ns / NANOS_PER_SEC, (ns % NANOS_PER_SEC) as u32)
    }
}
