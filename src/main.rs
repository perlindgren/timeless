use timeless::*;
// Example of a Timer implementation

#[derive(Debug)]
struct Timer {
    tick: u32,
    freq: Freq,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            tick: 0,
            freq: Freq::new(1000),
        }
    }

    fn tick(&mut self) {
        self.tick += 1;
    }
}

impl Clock for Timer {
    fn now(&self) -> Instant {
        unsafe { Instant::new(self.tick) }
    }
}

fn main() {
    let mut clk = Timer::new();
    println!("clk {:?}", clk);
    clk.tick();
    println!("clk {:?}", clk);
    clk.tick();
    println!("clk {:?}", clk);
    clk.tick();
    println!("clk {:?}", clk);
}

#[test]
fn t() {
    print!("t");
}
