use std::{collections::HashMap, error::Error, fs::{read_to_string, write, OpenOptions}, io::{self, Write}, time::Duration};

use plotters::prelude::*;

use rand::{distributions::Distribution, Rng};

use self::cps::{delay::Delay, CPS};

#[path ="cps.rs"] pub mod cps;

#[derive(Clone, Debug)]
pub struct ClickData {
    pub delays: Vec<Delay>
}

const DELIM: &str = "|";

impl ClickData {
    pub fn new(delays: Vec<Delay>) -> Self {
        Self { delays }
    }

    pub fn from_lines(lines: Vec<String>) -> Result<Self, ()> {
        let mut delays: Vec<Delay> = vec![];

        for line in lines {
            if line.is_empty() { continue; }

            let delay_millis: Vec<&str> = line.split(DELIM).collect();
            if delay_millis.len() != 2 {
                return Err(());
            }

            let down_millis = match delay_millis[0].parse::<u64>() {
                Ok(millis) => millis,
                Err(_) => return Err(())
            };

            let up_millis = match delay_millis[1].parse::<u64>() {
                Ok(millis) => millis,
                Err(_) => return Err(())
            };

            delays.push(Delay::from_nanos(down_millis, up_millis))
        }

        Ok(Self::new(delays))
    }

    pub fn from_string(str: String) -> Result<Self, ()> {
        let lines: Vec<String> = str.lines().map(String::from).collect();
        Self::from_lines(lines)
    }

    pub fn from_file(file_path: &str) -> Result<Self, ()> {
        match read_to_string(file_path) {
            Ok(str) => Self::from_string(str),
            Err(_) => Err(())
        }
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
        let delay_u64s: Vec<u64> = distr
            .sample_iter(rng)
            .take(clicks * 2)
            .collect();
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

    pub fn string(&self) -> String {
        let mut string = String::new();

        for delay in &self.delays {
            string += format!("{}{}{}\n", delay.down.as_nanos(), DELIM, delay.up.as_nanos()).as_str()
        };

        string
    }

    pub fn write_to_file(&self, file_path: &str) -> Result<(), io::Error> {
        write(file_path, self.string())
    }

    pub fn append_to_file(&self, file_path: &str) -> Result<(), io::Error> {
        let mut data_file = OpenOptions::new()
            .append(true)
            .open(file_path)?;

        data_file.write_all(self.string().as_bytes())
    }

    pub fn down_durations(&self) -> Vec<Duration> {
        self.delays.iter().map(|delay| delay.down).collect()
    }

    pub fn up_durations(&self) -> Vec<Duration> {
        self.delays.iter().map(|delay| delay.up).collect()
    }

    pub fn total_durations(&self) -> Vec<Duration> {
        self.delays.iter().map(|delay| delay.duration()).collect()
    }

    pub fn total_durations_as_millis(&self) -> Vec<u128> {
        self.total_durations().iter().map(|delay| delay.as_millis()).collect()
    }

    pub fn total_frequencies(&self) -> HashMap<u128, usize> {
        let mut frequencies = HashMap::new();

        for val in &self.total_durations_as_millis() {
            *frequencies.entry(*val).or_insert(0) += 1;
        }

        frequencies
    }

    pub fn filter_by_freq(&mut self, min_frequency: usize) -> Result<(), ()> {
        let frequencies = self.total_frequencies();

        let mut new_delays = vec![];

        for delay in &self.delays {
            let freq = match frequencies.get(&delay.total_millis()) {
                Some(freq) => *freq,
                None => return Err(())
            };

            if freq > min_frequency {
                new_delays.push(*delay);
            }
        }

        self.delays = new_delays;

        Ok(())
    }

    pub fn filter_by_delay(&mut self, max_delay: Delay) {
        self.filter_by_duration(max_delay.duration())
    }

    pub fn filter_by_duration(&mut self, max_duration: Duration) {
        let mut new_delays = vec![];

        for delay in &self.delays {
            if delay.duration() <= max_duration {
                new_delays.push(*delay)
            }
        }

        self.delays = new_delays;
    }

    pub fn plot_histogram(&self, file_path: &str, dimensions: (u32, u32)) -> Result<(), Box<dyn Error>> {
        let data: Vec<u128> = self.total_durations_as_millis();
        let min = data.iter().min().unwrap();
        let max = data.iter().max().unwrap();

        let frequencies = self.total_frequencies();

        let freq_max = frequencies.iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(_k, v)| v)
            .unwrap();

        let root = BitMapBackend::new(file_path, dimensions).into_drawing_area();
        root.fill(&BLACK)?;

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .margin(5)
            .caption("Click Data Histogram", ("Calibri", 40, &BLUE))
            .build_cartesian_2d((*min..*max + 1).into_segmented(), 0..*freq_max + 1)?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .bold_line_style(WHITE.mix(0.9))
            .y_desc("Frequency")
            .x_desc("Delay")
            .axis_desc_style(("Calibri", 25, &BLUE.mix(0.9)))
            .axis_style(&YELLOW)
            .label_style(&YELLOW)
            .draw()?;

        chart.draw_series(
            Histogram::vertical(&chart)
                .style(BLUE.filled())
                .data(data.iter().map(|x| (*x, 1))),
        )?;

        Ok(())
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