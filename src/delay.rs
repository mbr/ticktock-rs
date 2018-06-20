//! Delay iterator
//!
//! A simpler iterator than `clock::Clock` that delays between executions with non-adaptive
//! intervals.

use std::{iter, thread, time};

/// Simple iterable delay
///
/// Iterating over this structure will insert `delay` between each iteration, starting after the
/// first.
pub struct Delay {
    /// Delay duration
    delay: time::Duration,

    /// Notes whether or not we are on the first tick. Used to skip the delay on first iteration.
    first_tick: bool,
}

impl Delay {
    /// Creates a new delay
    #[inline]
    pub fn new(delay: time::Duration) -> Delay {
        Delay {
            delay,
            first_tick: true,
        }
    }

    /// Creates a new delay that delays first
    #[inline]
    pub fn delayed(delay: time::Duration) -> Delay {
        Delay {
            delay,
            first_tick: false,
        }
    }
}

impl<'a> iter::Iterator for Delay {
    type Item = ();

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.first_tick {
            self.first_tick = false;
        } else {
            thread::sleep(self.delay);
        }

        Some(())
    }
}
