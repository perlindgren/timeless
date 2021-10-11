mod instant;
use instant::*;

pub trait Clock {
    fn now(&self) -> Instant;
}

#[derive(Debug)]
pub struct Freq {
    hz: u32,
}
