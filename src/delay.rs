//! Delay iterator
//!
//! A simpler iterator than `clock::Clock` that delays between executions with non-adaptive
//! intervals. Begins immediately:
//!
//! ```rust
//! use std::net::TcpStream;
//! use std::time::Duration;
//! use ticktock::delay::Delay;
//!
//! const RETRY_DELAY: Duration = Duration::from_millis(250);
//!
//! // attempt to connect to localhost:12348 three times, before giving up.
//! // in total, 500 ms of delay will be inserted
//! let conn = Delay::new(RETRY_DELAY)
//!                .take(3)
//!                .filter_map(|_| TcpStream::connect("localhost:12348").ok())
//!                .next();
//!
//! # // our test will fail, because there is noting listening at 12348
//! # assert!(conn.is_none());
//! ```

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
