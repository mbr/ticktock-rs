//! Frame clock module
//!
//! Contains a clocking that ticks in a fixed interval as precisely as
//! possible.

use std::{iter, time, thread};
use ::util::NanoConv;

/// Clock structure.
pub struct Clock {
    /// Start time of the clock, in ns since epoch
    start: time::Instant,
    /// Tick length
    tick_len: time::Duration,
}

/// A clock iterator
///
/// Used to iterate over the clock:
///
/// ```
/// extern crate ticktock;
///
/// use std::time;
/// use ticktock::clock::Clock;
///
/// fn main() {
///     let start = time::Instant::now();
///     // ticks once per second
///     let mut clock = Clock::new(time::Duration::from_secs(1));
///
///     // as soon as the clock starts, it will wait for the next time.
///     // in this case, we'll start at t = 1 second
///     for tick in clock.iter() {
///         // ...
///
///         // a simple break will exit
///         break;
///     }
///
///    let end = time::Instant::now();
///
///    assert!(time::Duration::from_secs(1) < end-start);
/// }
/// ```
pub struct ClockIter<'a>(&'a mut Clock);

impl Clock {
    /// Creates a new clock.
    ///
    /// Create a clock with a tick size of `tick_len_ms`, in ms.
    pub fn new(tick_len: time::Duration) -> Clock{
        Clock::new_with_start_time(tick_len, time::Instant::now())
    }

    /// Creates a new clock with a specified start time
    pub fn new_with_start_time(tick_len: time::Duration,
                               start: time::Instant) -> Clock {
        Clock{
            start: start,
            tick_len: tick_len,
        }
    }

    /// Creates a new clock with a different tick length that is synced to
    /// the original clock
    pub fn synced(&self, tick_len: time::Duration) -> Clock{
        Clock{
            start: self.start,
            tick_len: tick_len,
        }
    }

    /// Get start time
    pub fn start(&self) -> time::Instant {
        self.start
    }

    /// Waits for the next clock tick.
    ///
    /// Will wait until the next tick and return the current tick count.
    pub fn wait_until_tick(&self) -> (u64, time::Instant) {
        // uses signed math because ntp might put us in the past
        let now = time::Instant::now();

        let elapsed_ns = (now - self.start).as_ns();
        let tick_len_ns = self.tick_len.as_ns();

        let current_tick_num = elapsed_ns / tick_len_ns;
        let next_tick_num = current_tick_num + 1;

        let next_tick = self.start + self.tick_len * next_tick_num as u32;
        let until_next: time::Duration = next_tick - now;

        thread::sleep(until_next);
        return (next_tick_num, next_tick)
    }

    /// Creates a clock iterator.
    ///
    /// The iterator will iterate forever, calling `wait_until_tick` on each
    /// iteration. It will panic after about 293 years.
    ///
    /// Returns (current tick number, absolute time) on each iteration, where
    /// absolute time is relative to a fixed offset that depends on the machine
    /// (see `Instant`).
    pub fn iter(&mut self) -> ClockIter {
        ClockIter(self)
    }

    /// Create a relative clock iterator.
    ///
    /// Similar to `iter()`, but the resulting iterator will return a tuple of
    /// (current tick number, relative time), with relative time being a
    /// `time::Duration` from the start of the clock.
    pub fn rel_iter(&mut self) -> ClockIterRelative {
        ClockIterRelative(self)
    }
}

impl<'a> iter::Iterator for ClockIter<'a> {
    type Item = (u64, time::Instant);

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0.wait_until_tick())
    }
}

/// Similar to `ClockIter`, but returns a relative time instead.
///
/// The resulting returned tuple will be of the form `(tick_number,
/// duration_since_clock_start)`
pub struct ClockIterRelative<'a>(&'a mut Clock);

impl<'a> iter::Iterator for ClockIterRelative<'a> {
    type Item = (u64, time::Duration);

    fn next(&mut self) -> Option<Self::Item> {
        let (n, t) = self.0.wait_until_tick();
        Some((n, t - self.0.start))
    }
}
