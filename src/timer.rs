//! Non self-updating interval timers

use std::time;
use ::util::NanoConv;

/// Interval-timer
///
/// Interval timers can be used to track and act on fixed time intervals when
/// said timers are neither in charge of delay/run decision-making nor running
/// in a separate thread:
///
/// ```
/// extern crate ticktock;
/// use std::time;
/// use std::thread;
/// use ticktock::timer::Timer;
///
/// fn main() {
///     // create two independent timers
///
///     let mut t1 = Timer::new(time::Duration::from_millis(50));
///     let mut t2 = Timer::new(time::Duration::from_millis(150));
///
///     let delay = time::Duration::from_millis(100);
///
///     // for testing purpose, run 10 times only
///     for _ in 1..10 {
///         let now = time::Instant::now();
///
///         if t1.has_fired(now) {
///             println!("T1 has fired!");
///
///             // notify the timer it has run for this tick or it will attempt
///             // to run again.
///             t1.reset(now);
///         }
///
///         // the `handle()` function works like "has_fired", but updates
///         // the timer automatically
///         if t2.handle(now) {
///             println!("T2 has fired!");
///         }
///
///         // sleep 100 ms
///         thread::sleep(delay);
///     }
/// }
///
/// ```
pub struct Timer {
    next_tick: time::Instant,
    tick_len: time::Duration,
}


impl Timer {
    /// Creates a new timer.
    ///
    /// Create a timer with a tick length of `tick_len_ms` in ms.
    pub fn new(tick_len: time::Duration) -> Timer{
        Timer::new_with_start_time(tick_len, time::Instant::now())
    }

    /// Creates a new timer with a specific start time
    pub fn new_with_start_time(tick_len: time::Duration,
                               start: time::Instant) -> Timer {
        Timer{
            next_tick: start + tick_len,
            tick_len: tick_len,
        }
    }

    /// Combines `has_fired()` and `reset()`.
    pub fn handle(&mut self, t: time::Instant) -> bool {
        if self.has_fired(t) {
            self.reset(t);
            true
        } else {
            false
        }
    }

    /// Returns true if the timer has fired since the last time passed to
    /// `reset()`.
    ///
    /// `t` is the current time and should be passed in.
    #[inline(always)]
    pub fn has_fired(&self, t: time::Instant) -> bool {
        self.next_tick <= t
    }

    #[inline(always)]
    /// Remaining time until the timer will fire again.
    pub fn remaining(&self, t: time::Instant) -> time::Duration {
        if self.next_tick <= t {
            time::Duration::new(0, 0)
        } else {
            self.next_tick - t
        }
    }

    /// Notify the timer it has been executed.
    #[inline(always)]
    pub fn reset(&mut self, t: time::Instant) -> u32 {
        if t >= self.next_tick {
            // calculate how many ticks we already passed
            let skipped_ns = (t - self.next_tick).as_ns();
            let lost_ticks = (skipped_ns / self.tick_len.as_ns()) as u32;
            self.next_tick = self.next_tick + self.tick_len * (1 + lost_ticks);
            lost_ticks
        } else {
            0
        }
    }

    pub fn set_tick_len(&mut self, tick_len: time::Duration) {
        self.tick_len = tick_len
    }

    fn tick_len(&self) -> time::Duration {
        self.tick_len
    }
}


#[cfg(test)]
mod tests {
    use std::{thread, time};
    use super::*;

    #[test]
    fn test_single_timer() {
        let timer = Timer::new(time::Duration::from_millis(50));
        assert!(!timer.has_fired(time::Instant::now()));
        thread::sleep(time::Duration::from_millis(50));
        assert!(timer.has_fired(time::Instant::now()));
    }

    #[test]
    fn test_minimum_duration() {
        let now = time::Instant::now();

        let mut t1 = Timer::new_with_start_time(time::Duration::from_millis
                                            (10), now);
        let mut t2 = Timer::new_with_start_time(time::Duration::from_millis
                                            (50), now);

        let later = now + time::Duration::from_millis(15);

        // check if timers fire correctly
        assert!(t1.has_fired(later));
        assert!(!t2.has_fired(later));

        // check remaining time
        assert_eq!(t1.remaining(later), time::Duration::from_millis(0));
        assert_eq!(t2.remaining(later), time::Duration::from_millis(35));

        // reset timers
        t1.reset(later);
        t2.reset(later);

        assert!(!t1.has_fired(later));
        assert!(!t2.has_fired(later));

        // check updated times
        assert_eq!(t1.remaining(later), time::Duration::from_millis(5));
        assert_eq!(t2.remaining(later), time::Duration::from_millis(35));

        // try multiple timers
        let timers = vec![t2, t1];

        // find max and min
        assert_eq!(timers.iter().map(|t| t.remaining(later)).min().unwrap(),
                   time::Duration::from_millis(5));
        assert_eq!(timers.iter().map(|t| t.remaining(later)).max().unwrap(),
                   time::Duration::from_millis(35));
    }
}
