use std::time;
use util::AsNanoSeconds;

#[derive(Debug)]
pub struct TimerBuilder<F, V, R>
where
    F: Fn(time::Duration, &mut V) -> R,
{
    func: F,
    initial: V,
    interval: Option<time::Duration>,
    repeat: bool,
}

impl<F, V, R> TimerBuilder<F, V, R>
where
    F: Fn(time::Duration, &mut V) -> R,
{
    #[inline]
    fn new(func: F, initial: V) -> TimerBuilder<F, V, R> {
        TimerBuilder {
            func: func,
            initial: initial,
            interval: None,
            repeat: true,
        }
    }

    #[inline]
    pub fn every(mut self, interval: time::Duration) -> Self {
        self.interval = Some(interval);
        self.repeat = true;
        self
    }

    #[inline]
    pub fn once(mut self, delay: time::Duration) -> Self {
        self.interval = Some(delay);
        self.repeat = false;
        self
    }

    pub fn start(self, now: time::Instant) -> Timer<F, V, R> {
        let interval = self.interval.expect("no timing set");
        let next_tick = now + interval;

        Timer {
            func: self.func,
            value: self.initial,
            interval: interval,
            interval_ns: interval.as_ns(),
            next_tick: next_tick,
        }
    }
}

#[derive(Debug)]
pub struct Timer<F, V, R>
where
    F: Fn(time::Duration, &mut V) -> R,
{
    func: F,
    value: V,
    interval: time::Duration,
    interval_ns: u64,
    next_tick: time::Instant,
}

impl<F, V, R> Timer<F, V, R>
where
    F: Fn(time::Duration, &mut V) -> R,
{
    #[inline]
    pub fn apply(func: F, initial: V) -> TimerBuilder<F, V, R> {
        TimerBuilder::new(func, initial)
    }

    pub fn interval(&self) -> time::Duration {
        self.interval
    }

    pub fn set_value(&mut self, value: V) {
        self.value = value;
    }

    pub fn update(&mut self, now: time::Instant) -> Option<R> {
        // check if timer needs to fire
        if self.next_tick > now {
            return None;
        }

        // calculate delta and update tick
        let dt = now - self.next_tick + self.interval;

        // calculate how many ticks we already passed
        let dt_ns = dt.as_ns();
        let ticks = dt_ns / self.interval_ns;

        // next tick
        self.next_tick += self.interval * ticks as u32;

        // handle tick, update value
        Some((&self.func)(dt, &mut self.value))
    }
}

impl<F, V: Clone, R> Timer<F, V, R>
where
    F: Fn(time::Duration, &mut V) -> R,
{
    #[inline]
    pub fn value(&self) -> V {
        self.value.clone()
    }
}

impl<F, V, R> AsRef<V> for Timer<F, V, R>
where
    F: Fn(time::Duration, &mut V) -> R,
{
    #[inline(always)]
    fn as_ref(&self) -> &V {
        &self.value
    }
}

impl<F, V, R> AsMut<V> for Timer<F, V, R>
where
    F: Fn(time::Duration, &mut V) -> R,
{
    #[inline(always)]
    fn as_mut(&mut self) -> &mut V {
        &mut self.value
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction() {
        let now = time::Instant::now();
        Timer::apply(|_, _| (), 123)
            .every(time::Duration::from_millis(500))
            .start(now);
        Timer::apply(
            |_, v| {
                *v += 1;
                *v
            },
            12,
        ).every(time::Duration::from_millis(500))
            .start(now);
    }

    #[test]
    fn value_retrieval() {
        let now = time::Instant::now();
        let mut t = Timer::apply(|_, _| (), 123)
            .every(time::Duration::from_millis(500))
            .start(now);

        assert_eq!(*t.as_ref(), 123);
        assert_eq!(*t.as_mut(), 123);
        assert_eq!(t.value(), 123);
    }

    #[test]
    fn test_single_timer() {
        let now = time::Instant::now();
        let mut timer = Timer::apply(|_, count| *count += 1, 0)
            .every(time::Duration::from_millis(50))
            .start(now);


        assert_eq!(timer.value(), 0);
        let future = now + time::Duration::from_millis(49);
        timer.update(future);
        assert_eq!(timer.value(), 0);
        timer.update(future);
        assert_eq!(timer.value(), 0);

        let future2 = now + time::Duration::from_millis(50);
        timer.update(future2);
        assert_eq!(timer.value(), 1);
        timer.update(future2);
        assert_eq!(timer.value(), 1);

        let future3 = now + time::Duration::from_millis(51);
        timer.update(future3);
        assert_eq!(timer.value(), 1);
        timer.update(future3);
        assert_eq!(timer.value(), 1);

        let future4 = now + time::Duration::from_millis(100);
        timer.update(future4);
        assert_eq!(timer.value(), 2);
        timer.update(future4);
        assert_eq!(timer.value(), 2);

        let future5 = now + time::Duration::from_millis(10000);
        timer.update(future5);
        assert_eq!(timer.value(), 3);
        timer.update(future5);
        assert_eq!(timer.value(), 3);
    }

    #[test]
    fn test_just_called() {
        let now = time::Instant::now();
        let mut timer = Timer::apply(|_, fired| *fired = true, false)
            .every(time::Duration::from_millis(100))
            .start(now);

        timer.update(now);

        if timer.value() {
            // reset after it fired
            timer.set_value(false);
        }
    }
}
