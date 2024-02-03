use std::time::Duration;

use spin_sleep::sleep;
use winapi::um::winuser::GetAsyncKeyState;

use self::{click_data::{cps::delay::Delay, ClickData}, hot_key::button::Button};

#[path ="click_data.rs"] pub mod click_data;
#[path ="hot_key.rs"] pub mod hot_key;

#[derive(Clone, Debug)]
pub struct ClickRecorder {
    pub button: Button,
    pub data: ClickData
}

pub static WAIT: u64 = 1;

impl ClickRecorder {
    pub fn wait() -> Duration {
        Duration::from_millis(WAIT)
    }

    pub fn new(button: Button) -> Self {
        Self { button, data: ClickData::new(vec![]) }
    }

    pub fn record(&mut self, clicks: usize) {
        let wait = Self::wait();

        while !self.button.is_down() {
            sleep(wait);
        }

        while clicks >= self.data.num() {
            let mut up = 0_u64;
            let mut down = 0_u64;

            while self.button.is_down() {
                sleep(wait);
                down += WAIT;
            }

            while !self.button.is_down() {
                sleep(wait);
                up += WAIT;
            }

            self.data.delays.push(Delay::from_millis(down, up))
        }

        self.data.delays.remove(0);
    }
}