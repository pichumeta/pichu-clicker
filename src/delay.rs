use std::{ops::{Add, AddAssign}, time::Duration};

#[derive(Debug, Clone, Copy)]
pub struct Delay {
    pub down: Duration,
    pub up: Duration
}

impl Delay {
    pub fn new(down: Duration, up: Duration) -> Self {
        Self { down, up }
    }
    
    pub fn from_durations(delays: (Duration, Duration)) -> Self {
        Self { down: delays.0, up: delays.1 }
    }

    pub fn from_millis(down: u64, up: u64) -> Self {
        Self { up: Duration::from_millis(up), down: Duration::from_millis(down) }
    }

    pub fn durations(&self) -> (Duration, Duration) {
        (self.down, self.up)
    }

    pub fn total_millis(&self) -> u128 {
        self.up.as_millis() + self.down.as_millis()
    }
}

impl Add<Delay> for Delay {
    type Output = Delay;

    fn add(self, rhs: Delay) -> Delay {
        Delay::from_millis((self.down.as_millis() + rhs.down.as_millis()) as u64, 
            (self.up.as_millis() + rhs.up.as_millis()) as u64)
    }
}

impl AddAssign<Delay> for Delay {
    fn add_assign(&mut self, rhs: Delay) {
        *self = *self + rhs;
    }
}