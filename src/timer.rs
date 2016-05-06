use std::time;

const NANOS_PER_SEC: u64 = 1_000_000_000;

trait NanoConv {
    fn as_ns(&self) -> u64;
    fn from_ns(ns: u64) -> Self;
}

impl NanoConv for time::Duration {
    fn as_ns(&self) -> u64 {
        self.as_secs().checked_mul(NANOS_PER_SEC)
            .expect("overflow during nanosecond conversion")
            .checked_add(self.subsec_nanos() as u64)
            .expect("overflow during nanosecond conversion")
    }

    fn from_ns(ns: u64) -> time::Duration {
        time::Duration::new(ns / NANOS_PER_SEC, (ns % NANOS_PER_SEC) as u32)
    }
}

/// Interval-timer
///
/// An interval timer
pub struct Timer {
    next_tick: time::Instant,
    tick_len: time::Duration,
}


impl Timer {
    /// Creates a new timer.
    ///
    /// Create a timer with a tick length of `tick_len_ms`, in ms.
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

    #[inline(always)]
    pub fn has_fired(&self, t: time::Instant) -> bool {
        self.next_tick <= t
    }

    #[inline(always)]
    pub fn remaining(&self, t: time::Instant) -> time::Duration {
        if self.next_tick <= t {
            time::Duration::new(0, 0)
        } else {
            self.next_tick - t
        }
    }

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
