#![no_std]
mod instant;
pub use instant::*;

pub trait Clock<const F: u32> {
    type P;
    fn now(&self) -> Instant<i32, Self::P>;
}

#[derive(Debug)]
pub struct Freq {
    hz: u32,
}
