//! Simulated slow reads/writes.
//!
//! This module allows simulating limited bandwidth by lengthening the duration
//! of calls to `Read`/`Write` to meet a specific upper bound on the rate.

use std::io::{Read, Write};
use std::{io, thread, time};

const NS_PER_SECOND: u128 = 1_000_000_000;

/// A wrapper that limits the maximum read/write-rate.
///
/// When asked to read bytes, the reader will always pause after a successful
/// read to never exceed the specified maximum read rate.
#[derive(Debug)]
pub struct ThrottledIo<T> {
    /// Desired nanoseconds per byte.
    bytes_per_second: u32,
    /// Total bytes read since `start`.
    total_read: u128,
    /// Total bytes written since `start`.
    total_written: u128,
    /// Start time.
    start: time::Instant,
    /// Inner IO type.
    io: T,
}

impl<T> ThrottledIo<T> {
    /// Create a new throttled reader with a specified maximum rate.
    #[inline]
    pub fn new(io: T, bytes_per_second: u32) -> ThrottledIo<T> {
        Self::new_with_start_time(io, bytes_per_second, time::Instant::now())
    }

    /// Create a new throttled reader, with specified start time.
    ///
    /// Note: If `now` is in the future, calls to `read` will likely panic.
    #[inline]
    pub fn new_with_start_time(io: T, bytes_per_second: u32, now: time::Instant) -> ThrottledIo<T> {
        ThrottledIo {
            bytes_per_second,
            total_read: 0,
            total_written: 0,
            start: now,
            io,
        }
    }

    /// Return the inner reader/writer.
    #[inline]
    pub fn into_inner(self) -> T {
        self.io
    }

    #[inline]
    fn delay(&self, total: u128) {
        let elapsed = time::Instant::now() - self.start;
        let max_bytes = (elapsed.as_nanos() * self.bytes_per_second as u128) / NS_PER_SECOND;

        // Delay until we're actually supposed to be done.
        if max_bytes < total {
            let remainder_ns = (total - max_bytes) * NS_PER_SECOND / self.bytes_per_second as u128;

            thread::sleep(time::Duration::from_nanos(remainder_ns as u64))
        }
    }
}

impl<T> Read for ThrottledIo<T>
where
    T: Read,
{
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes_read = self.io.read(buf)?;
        self.total_read += bytes_read as u128;

        self.delay(self.total_read);

        Ok(bytes_read)
    }
}

impl<T> Write for ThrottledIo<T>
where
    T: Write,
{
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        let bytes_written = self.io.write(data)?;
        self.total_written += bytes_written as u128;

        self.delay(self.total_written);

        Ok(bytes_written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.io.flush()
    }
}
