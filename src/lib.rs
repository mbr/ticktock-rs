//! Timing module for frame-based applications
//!
//! Contains methods for slowing down to a fixed framerate, as well as
//! measuring actual frames per second.
//!
//! An example game loop:
//!
//! ```ignore
//! extern crate ticktock;
//!
//! use std::time;
//! use ticktock::{Clock, SecondsFloat, Timer};
//!
//! fn main() {
//!     let now = time::Instant::now();
//!
//!     // initialize game
//!     // ...
//!
//!     // show some fps measurements every 5 seconds
//!     let mut fps_counter = Timer::apply(|delta_t, prev_tick|
//!                                        (delta_t, *prev_tick), 0)
//!                                 .every(time::Duration::from_secs(5))
//!                                 .start(now);
//!
//!     // run with a constant framerate of 30 fps
//!     for (tick, now) in Clock::framerate(30.0).iter() {
//!         // this loop will run approx. every 33.3333 ms
//!
//!         // update, render, etc
//!         // ...
//!
//!         // update or display fps count
//!         if let Some((delta_t, prev_tick)) = fps_counter.update(now) {
//!             fps_counter.set_value(tick);
//!
//!             let fps = (tick - prev_tick) as f64 / delta_t.as_fsecs();
//!             println!("FPS: {}", fps);
//!         }
//!         break;  // ignore, for doctests
//!     }
//! }
//! ```

pub mod clock;
pub mod delay;
pub mod timer;
pub mod util;

pub use clock::Clock;
pub use timer::Timer;
pub use util::NanoSeconds;
pub use util::SecondsFloat;
