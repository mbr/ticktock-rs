//! Frame clock module
//!
//! Contains a clocking that ticks in a fixed interval as precisely as
//! possible.

use std::{self, iter, thread};
use time;

// FIXME: factor this out into another crate
pub fn time_to_std(d_in: &time::Duration) -> std::time::Duration {
    std::time::Duration::new(
        d_in.num_seconds() as u64,
        match d_in.num_nanoseconds() {
            None => 0,
            Some(v) => (v % 1_000_000_000) as u32
        }
    )
}

/// Clock structure.
pub struct Clock {
    /// Start time of the clock, in ns since epoch
    start: time::SteadyTime,
    /// Tick length
    tick_len: time::Duration,
}

/// A clock iterator
///
/// Used to iterate over the clock:
///
/// ```
/// extern crate time;
/// use ticktock::clock::Clock;
///
/// // ticks once per second
/// let mut mclock = Clock::new(time::Duration::seconds(1));
///
/// for tick in mclock.iter() {
///     // ...
///     // will wait for the remainder of the time left until the next second
///
///     // a simple break will exit
///     break;
/// }
/// ```
pub struct ClockIter<'a>(&'a mut Clock);

impl Clock {
    /// Creates a new clock.
    ///
    /// Create a clock with a tick size of `tick_len_ms`, in ms.
    pub fn new(tick_len: time::Duration) -> Clock {
        Clock{
            start: time::SteadyTime::now(),
            tick_len: tick_len,
        }
    }

    pub fn start(&self) -> time::SteadyTime {
        self.start
    }

    /// Waits for the next clock tick.
    ///
    /// Will wait until the next tick and return the current tick count.
    /// Returns None on clock arithmetic overflow
    pub fn wait_until_tick(&self) -> Option<(i64, time::SteadyTime)> {
        // uses signed math because ntp might put us in the past
        let now = time::SteadyTime::now();

        let elapsed_ns = match (now - self.start).num_nanoseconds() {
            None => return None,
            Some(v) => v
        };

        let tick_len_ns = match self.tick_len.num_nanoseconds() {
            None => return None,
            Some(v) => v
        };

        let current_tick_num = elapsed_ns / tick_len_ns;
        let next_tick_num = current_tick_num + 1;

        let next_tick = self.start + self.tick_len * next_tick_num as i32;
        let until_next: time::Duration = next_tick - now;

        thread::sleep(time_to_std(&until_next));
        return Some((next_tick_num, next_tick))
    }

    /// Creates a clock iterator.
    ///
    /// The iterator will iterate forever, calling `wait_until_tick` on each
    /// iteration. It will panic after about 293 years.
    ///
    /// Returns the current tick number on each iteration.
    pub fn iter(&mut self) -> ClockIter {
        ClockIter(self)
    }
}

impl<'a> iter::Iterator for ClockIter<'a> {
    type Item = (i64, time::SteadyTime);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.wait_until_tick() {
                Some(v) => return Some(v),
                None => panic!("Hacking too much time")
            }
        }
    }
}
