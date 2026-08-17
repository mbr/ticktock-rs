# ticktock

`ticktock` is deprecated. For synchronous interval timers, use [`spin_sleep_util`](https://crates.io/crates/spin_sleep_util) and call `Interval::tick_no_spin()` to wait without busy-spinning.

## Migration

Replace `ticktock::Clock` with a non-spinning interval:

```rust
use spin_sleep_util::interval_at;
use std::time::{Duration, Instant};

let period = Duration::from_secs_f64(1.0 / 30.0);
let mut interval = interval_at(Instant::now() + period, period);

loop {
    let scheduled = interval.tick_no_spin();

    // update, render, etc.
}
```

Using `interval_at` preserves `ticktock::Clock`'s delayed first tick. Use `spin_sleep_util::interval` when the first tick should be immediate. Maintain a local counter if the tick number is needed.
