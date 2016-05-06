use std::time;

const NANOS_PER_SEC: u64 = 1_000_000_000;

pub trait NanoConv {
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
