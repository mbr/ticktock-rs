//! Frame clock module
//!
//! Contains a clocking that ticks in a fixed interval as precisely as
//! possible.

// FIXME: clock should start immediately, not waiting the initial interval

use std::{iter, thread, time};
use util::{NanoSeconds, SecondsFloat};

/// Clock structure.
pub struct Clock {
    /// Start time of the clock, in ns since epoch
    started_at: time::Instant,
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
/// use ticktock::Clock;
///
/// fn main() {
///     let start = time::Instant::now();
///     // ticks once per second
///     let mut clock = Clock::new(time::Duration::from_secs(1));
///
///     // as soon as the clock starts, it will wait for the next tick.
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
pub struct ClockIter<'a>(&'a Clock);

impl Clock {
    /// Creates a new clock.
    ///
    /// Create a clock with a tick size of `tick_len_ms`, in ms.
    #[inline]
    pub fn new(tick_len: time::Duration) -> Clock {
        Clock::new_with_start_time(tick_len, time::Instant::now())
    }

    /// Creates a new clock with a specified start time
    #[inline]
    pub fn new_with_start_time(tick_len: time::Duration, start: time::Instant) -> Clock {
        Clock {
            started_at: start,
            tick_len: tick_len,
        }
    }

    /// Creates a new fixed-framerate clock
    #[inline]
    pub fn framerate(fps: f64) -> Clock {
        Clock::framerate_with_start_time(fps, time::Instant::now())
    }

    /// Creates a new fixed-framerate clock with a specified sart time
    #[inline]
    pub fn framerate_with_start_time(fps: f64, start: time::Instant) -> Clock {
        let frame_time_s = 1.0 / fps;

        Clock::new_with_start_time(time::Duration::from_fsecs(frame_time_s), start)
    }

    /// Creates a new clock with a different tick length that is synced to
    /// the original clock
    #[inline]
    pub fn synced(&self, tick_len: time::Duration) -> Clock {
        Clock {
            started_at: self.started_at,
            tick_len: tick_len,
        }
    }

    /// Get start time
    #[inline]
    pub fn started_at(&self) -> time::Instant {
        self.started_at
    }

    /// Returns the tick number preceding an specific instant in time
    #[inline]
    pub fn tick_num_at(&self, now: time::Instant) -> u64 {
        (now - self.started_at).as_ns() / self.tick_len.as_ns()
    }

    /// Waits for the next clock tick.
    ///
    /// Will wait until the next tick and return the current tick count.
    #[inline]
    pub fn wait_until_tick(&self) -> (u64, time::Instant) {
        // uses signed math because ntp might put us in the past
        let now = time::Instant::now();

        let current_tick_num = self.tick_num_at(now);
        let next_tick_num = current_tick_num + 1;

        let next_tick = self.started_at + self.tick_len * next_tick_num as u32;
        let until_next: time::Duration = next_tick - now;

        thread::sleep(until_next);
        return (next_tick_num, next_tick);
    }

    /// Creates a clock iterator.
    ///
    /// The iterator will iterate forever, calling `wait_until_tick` on each
    /// iteration. It will panic after about 293 years.
    ///
    /// Returns (current tick number, absolute time) on each iteration, where
    /// absolute time is relative to a fixed offset that depends on the machine
    /// (see `Instant`).
    #[inline]
    pub fn iter(&self) -> ClockIter {
        ClockIter(self)
    }

    /// Create a relative clock iterator.
    ///
    /// Similar to `iter()`, but the resulting iterator will return a tuple of
    /// (current tick number, relative time), with relative time being a
    /// `time::Duration` from the start of the clock.
    #[inline]
    pub fn rel_iter(&self) -> ClockIterRelative {
        ClockIterRelative(self)
    }
}

impl<'a> iter::Iterator for ClockIter<'a> {
    type Item = (u64, time::Instant);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0.wait_until_tick())
    }
}

/// Similar to `ClockIter`, but returns a relative time instead.
///
/// The resulting returned tuple will be of the form `(tick_number,
/// duration_since_clock_start)`
pub struct ClockIterRelative<'a>(&'a Clock);

impl<'a> iter::Iterator for ClockIterRelative<'a> {
    type Item = (u64, time::Duration);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (n, t) = self.0.wait_until_tick();
        Some((n, t - self.0.started_at))
    }
}
