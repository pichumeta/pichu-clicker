use delay::Delay;
use rand::distributions::Uniform;

#[path = "delay.rs"] pub mod delay;

pub const SECOND: f32 = 1000.0;

#[derive(Clone, Copy, Debug)]
pub struct CPS {
    pub cps: f32
}

impl CPS {
    pub fn new(cps: f32) -> Self {
        Self { cps }
    }

    pub fn from_delay(delay: Delay) -> Self {
        Self { cps: Self::delay_to_cps(delay) }
    }

    pub fn delay(&self) -> Delay {
        Self::cps_to_delay(*self)
    }

    pub fn distribution(&self) -> Uniform<u64> {
        let delay = self.delay();
        dbg!(delay);
        Uniform::new_inclusive(0, (delay.down + delay.up).as_millis() as u64)
    }

    fn cps_to_delay(cps: CPS) -> Delay {
        let each_delay = (SECOND / cps.cps / 2.0) as u64;
        Delay::from_millis(each_delay, each_delay)
    }

    fn delay_to_cps(delay: Delay) -> f32 {
        SECOND / delay.total_millis() as f32
    }
}