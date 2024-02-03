use rand::{distributions::Distribution, Rng};

use self::cps::{delay::Delay, CPS};

#[path ="cps.rs"] pub mod cps;

#[derive(Clone, Debug)]
pub struct ClickData {
    pub delays: Vec<Delay>
}

impl ClickData {
    pub fn new(delays: Vec<Delay>) -> Self {
        Self { delays }
    }

    pub fn from_generated<Distr, R>(clicks: usize, distr: Distr, rng: &mut R) -> Self
    where
        Distr: Distribution<u64>,
        R: Rng
    {
        let mut instance = Self::new(vec![]);
        instance.generate(clicks, distr, rng);
        instance
    }

    pub fn generate<Distr, R>(&mut self, clicks: usize, distr: Distr, rng: &mut R)
    where
        Distr: Distribution<u64>,
        R: Rng
    {
        let delay_u64s: Vec<u64> = distr.sample_iter(rng).take(clicks * 2).collect();
        let mut delay_tuples: Vec<(u64, u64)> = vec![];

        for i in 0..delay_u64s.len() {
            if i % 2 != 0 { continue; }
            delay_tuples.push((delay_u64s[i], delay_u64s[i + 1]))
        }

        for delay_tuple in &delay_tuples {
            self.delays.push(Delay::from_millis(delay_tuple.0, delay_tuple.1))
        }
    }

    pub fn cps(&self) -> CPS {
        let average_delay = self.average_delay();
        CPS::from_delay(average_delay)
    }

    pub fn num(&self) -> usize {
        self.delays.len()
    }

    fn average_delay(&self) -> Delay {
        let num = self.delays.len() as u128;
        let total_delay = self.total_delay();
        Delay::from_millis((total_delay.down.as_millis() / num) as u64, 
            (total_delay.up.as_millis() / num) as u64)
    }

    fn total_delay(&self) -> Delay {
        let mut total_delay = Delay::from_millis(0, 0);

        for delay in &self.delays {
            total_delay += *delay;
        }

        total_delay
    }
}