//! Timing module for frame-based applications
//!
//! Contains methods for slowing down to a fixed framerate, as well as
//! measuring actual frames per second.

pub mod clock;
pub mod util;
pub mod timer;
pub mod timer2;


// Module currently disabled, until I have time (no pun intended) to update
// the API:
// pub mod framecounter;
