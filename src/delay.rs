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

#[inline]
pub fn retry<T, E, F>(retries: usize, delay: time::Duration, f: F) -> Result<T, E>
where
    F: Fn() -> Result<T, E>,
{
    // initial attempt
    let mut rv = f();

    if rv.is_ok() {
        return rv;
    }

    // retry `retries` more times after the first
    for res in Delay::delayed(delay).take(retries).map(|_| f()) {
        rv = res;

        if rv.is_ok() {
            break;
        }
    }

    rv
}

#[inline]
pub fn retry_opt<T, F>(retries: usize, delay: time::Duration, f: F) -> Option<T>
where
    F: Fn() -> Option<T>,
{
    retry(retries, delay, || f().ok_or(())).ok()
}

trait Attempt {
    type Result;

    fn attempt(self, retries: usize) -> Option<Self::Result>;
}

impl<T, E, I> Attempt for I
where
    I: Iterator<Item = Result<T, E>>,
{
    type Result = Result<T, E>;

    fn attempt(mut self, retries: usize) -> Option<Result<T, E>> {
        let mut rv = None;

        for res in self.take(retries) {
            rv = Some(res);

            // do not keep going if we got an Ok
            if let Some(Ok(_)) = rv {
                break;
            }
        }

        rv
    }
}

// ex: Delay::new(RETRY_DELAY).map(|_| TcpStream::connect("localhost:12348")).attempt(5)
// ex: Delay::new(RETRY_DELAY).map(|_| TcpStream::connect("localhost:12348")).take(5).attempt()
// vs: Delay::new(RETRY_DELAY).filter_map(|_| TcpStream::connect("localhost:12348"))...
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

    pub fn retry<T, E, F>(&mut self, retries: usize, f: F) -> Result<T, E>
    where
        F: Fn() -> Result<T, E>,
    {
        // initial attempt
        let mut rv = f();

        if rv.is_ok() {
            return rv;
        }

        // retry `retries` more times after the first
        for res in self.map(|_| f()) {
            rv = res;

            if rv.is_ok() {
                break;
            }
        }

        rv
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
