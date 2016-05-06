//! Timing module for frame-based applications
//!
//! Contains methods for slowing down to a fixed framerate, as well as
//! measuring actual frames per second.

pub mod clock;
pub mod timer;

mod util;

// Module currently disabled, until I have time (no pun intended) to update
// the API:
// pub mod framecounter;
