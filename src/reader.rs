//! Simulated slow reads.
//!
//! This module allows simulating limited bandwidth by lengthening the duration
//! of calls to `io::Read::read` to meet a specific upper bound on the bitrate.

use std::io::Read;
use std::{io, thread, time};

/// A reader that limits the maximum read-rate.
///
/// When asked to read bytes, the reader will always pause after a successful
/// read to never exceed the specified maximum read rate.
#[derive(Debug)]
pub struct ThrottledReader<R> {
    ns_per_byte: u128,
    total_read: u128,
    start: time::Instant,
    reader: R,
}

impl<R> ThrottledReader<R> {
    /// Create a new throttled reader with a specified maximum bitrate.
    pub fn new(reader: R, bits_per_second: u32) -> ThrottledReader<R> {
        Self::new_with_start_time(reader, bits_per_second, time::Instant::now())
    }

    /// Create a new throttled reader, with specified start time.
    ///
    /// Note: If `now` is in the future, calls to `read` will likely panic.
    pub fn new_with_start_time(
        reader: R,
        bits_per_second: u32,
        now: time::Instant,
    ) -> ThrottledReader<R> {
        let ns_per_byte = 8_000_000_000 / (bits_per_second as u128);

        ThrottledReader {
            ns_per_byte,
            total_read: 0,
            start: now,
            reader,
        }
    }
}

impl<T> Read for ThrottledReader<T>
where
    T: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes_read = self.reader.read(buf)?;
        self.total_read += bytes_read as u128;

        // Determine if we're ahead of schedule.
        let elapsed = time::Instant::now() - self.start;
        let max = elapsed.as_nanos() / self.ns_per_byte;

        // Delay until we're actually supposed to be done.
        if max < self.total_read {
            let remainder_ns = (self.total_read - max) * self.ns_per_byte;

            thread::sleep(time::Duration::from_nanos(remainder_ns as u64))
        }

        Ok(bytes_read)
    }
}
