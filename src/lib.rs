use std::marker::PhantomData;

#[derive(Debug)]
pub struct Freq {
    hz: u32,
}

impl Freq {
    pub const fn new(hz: u32) -> Self {
        Freq { hz }
    }
}

// pub struct Instant<T> {
//     tick: u32,
//     _unique: PhantomData<T>,
// }

// pub trait Clock<T> {
//     fn now(&self) -> Instant<T>;
// }

// #[test]
// fn freq() {
//     let f = Freq::new(10);
//     println!("f {:?}", f)
// }

pub struct Instant {
    tick: u32,
}

impl Instant {
    pub unsafe fn new(tick: u32) -> Self {
        Instant { tick }
    }
}

pub trait Clock {
    fn now(&self) -> Instant;
}
