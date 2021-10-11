# Timeless

Time with Lest Effort Safe and Sound abstractions for embedded Rust.

---

## Partially monotonic time.

We want to represent instances in time using bounded size representation.

- Assume time instances `i1`, and `i2`, where `i2` is taken at a later point in time. We would assume `i1 < i2` to always hold (monotonic assumption). 
  
- Under bounded size `m` then we have the representation of `i1` (`i2`) as `i1 % m` (`i2 % m`).

- The best we can get is `i2 - i1 < m / 2 -> i1 < i2 <-> i1 % m < i2 % m`. That is if the duration between `i2` and `i1` is less than half our range, then comparison on the monotonic assumption will hold (the test on modular instances complies to the test on monotonic (unbounded instances)).

---

## u16, u32, u64

The intent is to support partially monotonic time represented by `u16`, `u32` and `u64`. This is motivated by requirements derived from embedded development, where we typically have a mix of 16 and 32 bit timers in hardware (64 bit timers are more rare but can be emulated by software). 

As used in real-time applications we want arithmetics and comparisons to be CPU and memory efficient.

## API Design


