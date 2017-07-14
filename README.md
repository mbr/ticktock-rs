Frame-clock and timers
======================

`ticktock` makes it very easy to access frame-timing iterators to achieve
constant framerates:

```rust
// run with a constant framerate of 30 fps
for (tick, now) in Clock::framerate(30.0).iter() {
  // ...
}
```

See the [documentation](https://docs.rs/ticktock) for details.
