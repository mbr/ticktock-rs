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

pub use crate::clock::Clock;
pub use crate::timer::Timer;
pub use crate::util::NanoSeconds;
pub use crate::util::SecondsFloat;

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
///
/// `Option` is also a valid outcome:
///
/// ```ignore
/// let credentials = vec![("Bob", "secret"), ("Jeff", "hunter2"), ("John", "swordfish")];
///
/// fn try_login(username: &str, password: &str) -> Option<(String, String)> { ... }
///
/// // brute-force our way in
/// let valid_credentials: Option<(String, String)> = credentials
///                                                       .map(|(u, p)| try_login(u, p))
///                                                       .attempt()
///                                                       .unwrap();
/// ```

// note: this could probably be expressed more cleanly by using associated types
// (i.e. `type Outcome = ...`), but a bug in the rust compiler at the time of this writing
// did not allow for it https://github.com/rust-lang/rust/issues/20400

pub trait Attempt<O> {
    /// Consumes until the successful outcome is encountered. In case of failure, returns the last
    /// unsuccessful outcome.
    fn attempt(self) -> Option<O>;
}

impl<T, E, I> Attempt<Result<T, E>> for I
where
    I: Iterator<Item = Result<T, E>>,
{
    fn attempt(self) -> Option<Result<T, E>> {
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

impl<T, I> Attempt<Option<T>> for I
where
    I: Iterator<Item = Option<T>>,
{
    fn attempt(self) -> Option<Option<T>> {
        let mut rv = None;

        for res in self {
            rv = Some(res);

            // do not keep going if we got an Ok
            if let Some(Some(_)) = rv {
                break;
            }
        }

        rv
    }
}

#[cfg(test)]
mod test {
    use super::Attempt;

    #[test]
    fn attempt_works_on_ok_results() {
        let rs = vec![Err(1), Err(2), Err(3), Ok(4), Ok(5)];

        assert_eq!(Some(Ok(4)), rs.into_iter().attempt());
    }

    #[test]
    fn attempt_works_on_err_results() {
        let rs: Vec<Result<(), _>> = vec![Err(1), Err(2), Err(3)];

        assert_eq!(Some(Err(3)), rs.into_iter().attempt());
    }

    #[test]
    fn attempt_works_on_empty_result_vecs() {
        let rs: Vec<Result<(), ()>> = Vec::new();

        assert_eq!(None, rs.into_iter().attempt());
    }

    #[test]
    fn attempt_works_on_some_options() {
        let rs = vec![None, None, None, Some(4), Some(5)];

        assert_eq!(Some(Some(4)), rs.into_iter().attempt());
    }

    #[test]
    fn attempt_works_on_none_options() {
        let rs: Vec<Option<()>> = vec![None, None, None];

        assert_eq!(Some(None), rs.into_iter().attempt());
    }

    #[test]
    fn attempt_works_on_empty_option_vecs() {
        let rs: Vec<Option<()>> = Vec::new();

        assert_eq!(None, rs.into_iter().attempt());
    }
}
