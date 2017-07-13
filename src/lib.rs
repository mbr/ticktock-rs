//! Timing module for frame-based applications
//!
//! Contains methods for slowing down to a fixed framerate, as well as
//! measuring actual frames per second.

pub mod clock;
pub mod util;
pub mod timer;

pub use clock::Clock;
pub use timer::Timer;
pub use util::NanoSeconds;
pub use util::SecondsFloat;
