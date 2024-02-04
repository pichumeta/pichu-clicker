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
    
    pub fn from_durations(down: Duration, up: Duration) -> Self {
        Self { down, up }
    }

    pub fn from_millis(down: u64, up: u64) -> Self {
        Self { down: Duration::from_millis(down), up: Duration::from_millis(up) }
    }

    pub fn from_nanos(down: u64, up: u64) -> Self {
        Self { down: Duration::from_nanos(down), up: Duration::from_nanos(up) }
    }

    pub fn duration(&self) -> Duration {
        self.down + self.up
    }

    pub fn total_millis(&self) -> u128 {
        self.duration().as_millis()
    }
}

impl Add<Delay> for Delay {
    type Output = Delay;

    fn add(self, rhs: Delay) -> Delay {
        Delay::from_durations(self.down + rhs.down, self.up + rhs.up)
    }
}

impl AddAssign<Delay> for Delay {
    fn add_assign(&mut self, rhs: Delay) {
        *self = *self + rhs;
    }
}