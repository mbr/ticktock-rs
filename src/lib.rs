//! Timing module for frame-based applications
//!
//! Contains methods for slowing down to a fixed framerate, as well as
//! measuring actual frames per second.

extern crate time;

pub mod clock;
pub mod framecounter;

pub const SECOND: u64 = MILISECOND * 1000;
pub const MILISECOND: u64 = MICROSECOND * 1000;
pub const MICROSECOND: u64 = NANOSECOND * 1000;
pub const NANOSECOND: u64 = 1;
