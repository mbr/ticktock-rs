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

/// Iterator attempt
///
/// Given an iterator of outcomes, iterates returning either
///
/// * the first successful outcome
/// * the last unsuccessful outcome
/// * `None` if the iterator was empty
///
/// `Result` is often used as an outcome, e.g. when trying to reconnect multiple times:
///
/// ```rust
/// use std::net::TcpStream;
/// use std::time::Duration;
/// use ticktock::Attempt;
/// use ticktock::delay::Delay;
///
/// const RETRY_DELAY: Duration = Duration::from_millis(250);
///
/// // attempt to connect to localhost:12348 three times, before giving up.
/// // in total, 500 ms of delay will be inserted
/// let conn = Delay::new(RETRY_DELAY)
///                .map(|_| TcpStream::connect("localhost:12348"))
///                .take(3)
///                .attempt()
///                .unwrap();
///
/// # // our test will fail, because there is noting listening at 12348
/// # assert!(conn.is_err());
/// ```

pub trait Attempt {
    type Outcome;

    /// Consumes until the successful outcome is encountered. In case of failure, returns the last
    /// unsuccessful outcome.
    fn attempt(self) -> Option<Self::Outcome>;
}

impl<T, E, I> Attempt for I
where
    I: Iterator<Item = Result<T, E>>,
{
    type Outcome = Result<T, E>;

    fn attempt(self) -> Option<Self::Outcome> {
        let mut rv = None;

        for res in self {
            rv = Some(res);

            // do not keep going if we got an Ok
            if let Some(Ok(_)) = rv {
                break;
            }
        }

        rv
    }
}
