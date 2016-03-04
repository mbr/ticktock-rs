//! Frame clock module
//!
//! Contains a clocking that ticks in a fixed interval as precisely as
//! possible.

use std::iter;
use std::thread;
use time::precise_time_ns;
use ::MILISECOND;

/// Clock structure.
pub struct Clock {
    /// Start time of the clock, in ns since epoch
    pub start_ns: u64,
    /// Tick length
    tick_len_ns: u64,
}

/// A clock iterator
///
/// Used to iterate over the clock:
///
/// ```
/// use ticktock::SECOND;
/// use ticktock::clock;
/// // ticks once per second
/// let mut clock = clock::Clock::new(1 * SECOND);
///
/// for tick in clock.iter() {
///     // ...
///     // will wait for the remainder of the time left until the next second
/// }
/// ```
pub struct ClockIter<'a>(&'a mut Clock);

impl Clock {
    /// Creates a new clock.
    ///
    /// Create a clock with a tick size of `tick_len_ms`, in ms.
    pub fn new(tick_len_ns: u64) -> Clock {
        Clock{
            start_ns: precise_time_ns(),
            tick_len_ns: tick_len_ns,
        }
    }

    /// Restarts the clock.
    ///
    /// Sets the clock start time to the current time.
    pub fn restart(&mut self) {
        self.start_ns = precise_time_ns();
    }

    /// Waits for the next clock tick.
    ///
    /// Will wait until the next tick and return the current tick count.
    /// If the clock has become nonsensical (for example due to do a change of
    /// the system clock into the past/future, will return `None`.
    pub fn wait_until_tick(&self) -> Option<(u32, u64)> {
        // uses signed math because ntp might put us in the past

        let now_ns = precise_time_ns();
        let current_tick = (now_ns-self.start_ns) / self.tick_len_ns;
        let next_tick_ns = self.start_ns +
                           (current_tick + 1) * self.tick_len_ns;
        let mut until_next_ms = (next_tick_ns - now_ns) / MILISECOND;

        if until_next_ms <= 0 {
            // we cannot wait with ns precision, so use slice length
            until_next_ms = self.tick_len_ns / MILISECOND;
        }

        thread::sleep_ms((until_next_ms) as u32);
        return Some(((current_tick+1) as u32, now_ns))
    }

    /// Creates a clock iterator.
    ///
    /// The iterator will iterate forever, calling `wait_until_tick` on each
    /// iteration. If an error occurs, it will reset the clock using `restart`,
    /// then resume operatior.
    ///
    /// Returns the current tick number on each iteration.
    pub fn iter(&mut self) -> ClockIter {
        ClockIter(self)
    }
}

impl<'a> iter::Iterator for ClockIter<'a> {
    type Item = (u32, u64);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.wait_until_tick() {
                Some(v) => return Some(v),
                None => self.0.restart()
            }
        }
    }
}
