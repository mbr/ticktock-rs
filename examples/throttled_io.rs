//! Example application that illustrates throttled IO.
//!
//! Read from `/dev/zero` and writes to `/dev/null` with a fixed bitrate.

use std::io::{Read, Write};
use std::{fs, time};
use ticktock::throttled_io::ThrottledIo;

const BUFFER_SIZE: usize = 500;
const TOTAL_BYTES: usize = 1000 * 1000;
const SPEED: u32 = 250 * 1000;

fn main() {
    println!("Reading 1 mb from /dev/zero at 250 kb/s.");
    let mut buf = [0; BUFFER_SIZE];

    let mut zero = ThrottledIo::new(fs::File::open("/dev/zero").unwrap(), SPEED);
    let mut remaining = TOTAL_BYTES;

    let start = time::Instant::now();
    // We're cheating a bit here and drop the last BUFFER_SIZE, but this should not matter much.
    while remaining >= BUFFER_SIZE {
        assert_eq!(zero.read(&mut buf).unwrap(), BUFFER_SIZE);

        remaining -= BUFFER_SIZE;
    }

    let end = time::Instant::now();
    println!("Read took {:?}", end - start);

    println!("Writing 1 mb to /dev/null at 250 kb/s.");
    let input = [0xFF; BUFFER_SIZE];
    let mut null = ThrottledIo::new(fs::File::create("/dev/null").unwrap(), SPEED);

    let mut remaining = TOTAL_BYTES;

    let start = time::Instant::now();
    while remaining >= BUFFER_SIZE {
        let bytes_written = null.write(&input).unwrap();

        assert_ne!(bytes_written, 0);

        remaining -= BUFFER_SIZE;
    }
    let end = time::Instant::now();
    println!("Write took {:?}", end - start);
}
