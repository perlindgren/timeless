use std::{
    cmp::{Eq, Ord, Ordering, PartialEq},
    marker::PhantomData,
    ops::{Add, Sub},
};

#[derive(Eq, PartialEq, Debug)]
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

#[test]
fn cmp_instant() {
    let i1 = unsafe { Instant::new(0) };
    let i2 = unsafe { Instant::new(1) };
    assert_ne!(i1, i2);

    assert!(i1 != i2);

    assert!(i1 < i2);

    assert!(i2 > i1);

    let i1 = unsafe { Instant::new(0x8000_0000) };
    let i2 = unsafe { Instant::new(0) };

    assert!(i1 > i2);
}
