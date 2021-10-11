use std::{
    cmp::{Eq, Ord, Ordering, PartialEq},
    marker::PhantomData,
    ops::{Add, Sub},
};

#[derive(Eq, PartialEq)]
pub struct Instant {
    tick: u32,
}

impl Instant {
    pub unsafe fn new(tick: u32) -> Self {
        Instant { tick }
    }
}

impl Add for Instant {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            tick: self.tick.wrapping_add(other.tick),
        }
    }
}

impl Sub for Instant {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            tick: self.tick.wrapping_sub(other.tick),
        }
    }
}

impl Ord for Instant {
    fn cmp(&self, other: &Self) -> Ordering {
        self.tick.cmp(&other.tick)
    }
}

impl PartialOrd for Instant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
