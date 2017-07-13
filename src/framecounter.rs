//! Frame(s per second) counter
//!
//! Records the start time and outputs frame per second when printed.

use std::fmt;
use time::precise_time_ns;
use SECOND;


#[derive(Debug)]
pub struct FrameCounter {
    start_ns: u64,
    frame_count: u32,
    slice_size_ns: u64,
    fps: f32,
}

/// Frame counter.
///
/// Print using "{}" to show frames per second as "12.34 FPS"
impl FrameCounter {
    /// Creates a new frame counter with a specific slice size.
    pub fn new_with_slice_size(slice_size_ns: u64) -> FrameCounter {
        let now_ns = precise_time_ns();
        FrameCounter {
            start_ns: now_ns,
            frame_count: 0,
            slice_size_ns: slice_size_ns,
            fps: 0.0,
        }
    }

    /// Creates a new frame counter with a default slice size of 1 second.
    pub fn new() -> FrameCounter {
        Self::new_with_slice_size(1 * SECOND as u64)
    }

    /// Increments the internal frame counter by one.
    ///
    /// Returns true if a measuring period ended (a good time to print out
    /// the current fps value).
    pub fn next_frame(&mut self) -> bool {
        let now_ns = precise_time_ns();
        let mut slice_completed = false;

        let slices_passed = (now_ns - self.start_ns) / self.slice_size_ns as u64;
        if slices_passed > 0 {
            let duration_s = (now_ns - self.start_ns) as f32 / SECOND as f32;
            self.fps = self.frame_count as f32 / duration_s;
            slice_completed = true;

            // prep for next slice
            self.frame_count = 0;
            self.start_ns += self.slice_size_ns as u64 * slices_passed;
        }
        self.frame_count += 1;
        slice_completed
    }
}

impl fmt::Display for FrameCounter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} FPS", self.fps)
    }
}
