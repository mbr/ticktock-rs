//! Utility module
//!
//! Contains a few convenience traits for converting durations into more easily
//! handled primitives. Note that a loss of precision will occur in most cases,
//! but is negligable in many applications.

use std::time;

const NANOS_PER_SEC: u64 = 1_000_000_000;

/// Convert into 64 bit nanosecond representation.
pub trait AsNanoSeconds {
    /// Converts duration into `u64`, representing the number of nanoseconds
    /// inside the duration. Note that this will panic for durations greater
    /// than 584 years (the maximum number of nanoseconds that fit into 64 bit)
    fn as_ns(&self) -> u64;

    /// Converts `u64` nanoseconds representation into duration.
    ///
    /// Conversion will always work if the target type is larger than 64 bits.
    fn from_ns(ns: u64) -> Self;
}

impl AsNanoSeconds for time::Duration {
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

/// Floating point conversion for time values.
///
/// Conversions will never fail, but are likely to lose precision if they
/// cannot accurately be represented as floating point numbers. Larger values
/// often result in smaller sub-second accuracy; large values may even lose
/// precision at the second or greater resolution.
pub trait AsSecondsFloat {
    /// Convert duration to `f64`.
    ///
    /// Precision-loss may occur.
    fn as_fsecs(&self) -> f64;

    /// Convert `f64` values to time durations
    fn from_fsecs(&self, fsecs: f64) -> Self;
}

impl AsSecondsFloat for time::Duration {
    fn as_fsecs(&self) -> f64 {
        // no issues here, just loss of precision
        let mut fsecs = self.as_secs() as f64;
        fsecs += (self.subsec_nanos() as f64) / NANOS_PER_SEC as f64;
        fsecs
    }

    fn from_fsecs(&self, fsecs: f64) -> time::Duration {
        // https://github.com/rust-lang/rust/issues/10184
        let secs = fsecs.trunc() as u64;
        // size will always be <= 999_999_999, which is < 2^32
        let subsec_nanos = (fsecs.fract() * NANOS_PER_SEC as f64) as u32;
        time::Duration::new(secs, subsec_nanos)
    }
}

pub trait FromSecondsFloat {
    fn from_fsecs(self: Self) -> time::Duration;
}

impl FromSecondsFloat for f64 {
    fn from_fsecs(self) -> time::Duration {
        // https://github.com/rust-lang/rust/issues/10184
        let secs = self.round() as u64;
        // size will always be <= 999_999_999, which is < 2^32
        let nanos = ((self % 1.0) * 1_000_000_000.0) as u32;
        time::Duration::new(secs, nanos)
    }
}
