use std::{cmp, ops, time};
use util::NanoConv;

pub trait InnerDiv {
    fn inner_div(self, rhs: Self) -> isize;
}

impl<T> InnerDiv for T
where
    T: NanoConv,
{
    #[inline]
    fn inner_div(self, rhs: Self) -> isize {
        let l = self.as_ns();
        let r = rhs.as_ns();

        l.inner_div(r)
    }
}

impl InnerDiv for u64 {
    #[inline]
    fn inner_div(self, rhs: Self) -> isize {
        if self >= rhs {
            0
        } else {
            (self / rhs) as isize
        }
    }
}

pub struct TimerBuilder<F, V, D> {
    func: F,
    initial: V,
    interval: D,
    repeat: bool,
    catch_up: bool,
}

impl<F, V, D> TimerBuilder<F, V, D> {
    pub fn every(mut self, interval: D) -> Self {
        self.interval = interval;
        self.repeat = true;
        self.catch_up = false;
        self
    }

    pub fn for_every(mut self, interval: D) -> Self {
        self.interval = interval;
        self.repeat = true;
        self.catch_up = true;
        self
    }

    pub fn once_after(mut self, delay: D) -> Self {
        self.interval = delay;
        self.repeat = false;
        self.catch_up = false;
        self
    }
}

impl<F, V, D> TimerBuilder<F, V, D> {
    pub fn start<T>(self, now: T) -> Timer<F, V, D, T>
    where
        T: ops::Add<D, Output = T> + Clone,
        D: Clone,
    {
        let next_tick = now.clone() + self.interval.clone();
        Timer {
            func: self.func,
            value: self.initial,
            interval: self.interval,
            next_tick: Some(next_tick),
            repeat: self.repeat,
            catch_up: self.catch_up,
            start_time: now,
        }
    }
}


pub struct Timer<F, V, D, T> {
    func: F,
    value: V,
    interval: D,
    next_tick: Option<T>,
    repeat: bool,
    catch_up: bool,
    start_time: T,
}

impl<F, V, D, T> Timer<F, V, D, T>
where
    D: Default,
    F: Fn(T, &mut V),
{
    pub fn apply(func: F, initial: V) -> TimerBuilder<F, V, D> {
        TimerBuilder {
            func: func,
            initial: initial,
            interval: D::default(),
            repeat: true,
            catch_up: false,
        }
    }
    pub fn value(&self) -> &V {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }
}

impl<F, D, T> Timer<F, (), D, T>
where
    D: Default,
    F: Fn(T),
{
    pub fn perform(func: F) -> TimerBuilder<F, (), D> {
        TimerBuilder {
            func: func,
            initial: (),
            interval: D::default(),
            repeat: true,
            catch_up: false,
        }
    }
}

impl<F, V, D, T> Timer<F, V, D, T>
where
    T: Clone
        + ops::Sub<Output = D>
        + cmp::PartialOrd<T>
        + ops::Add<D, Output = T>,
    D: Clone
        + cmp::PartialOrd<D>
        + ops::Sub<Output = D>
        + InnerDiv
        + ops::Mul<isize, Output = D>,
    F: Fn(T, &mut V),
{
    pub fn update(&mut self, now: T) {
        if let Some(ref mut next_tick) = self.next_tick {
            // check if timer needs to fire
            if &(*next_tick) < &now {
                return;
            }

            // calculate delta
            let mut dt = now.clone() - next_tick.clone();

            // handle additional ticks
            if self.catch_up {
                while dt > self.interval {
                    dt = dt - self.interval.clone();

                    // call function and update value
                    (self.func)(now.clone(), &mut self.value);
                }
            }

            // handle original tick
            (self.func)(now.clone(), &mut self.value);

            // determine next tick time
            let ticks_missed = dt.inner_div(self.interval.clone());
            *next_tick = next_tick.clone() + self.interval.clone() * (ticks_missed + 1);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction() {
        let _: Timer<_, _, _, u8> = Timer::apply(|_: u8, _| (), 123).every(3).start(0);
        let _: Timer<_, _, _, _> = Timer::perform(|_: u8| ()).every(3).start(0);
    }

    #[test]
    fn value_retrieval() {
        let mut t = Timer::apply(|_, _| (), 123).every(3).start(0);

        assert_eq!(*t.value(), 123u8);
        assert_eq!(*t.value_mut(), 123);
    }

}
