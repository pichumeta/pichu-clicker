use std::{sync::{atomic::Ordering, Arc, Mutex}, thread};

use rand::thread_rng;
use spin_sleep::sleep;
use winapi::shared::windef::HWND;

use self::click_recorder::{click_data::{cps::delay::Delay, ClickData}, hot_key::{button::Button, HotKey}};

#[path ="click_recorder.rs"] pub mod click_recorder;

#[derive(Clone, Copy, Debug)]
pub enum DelayMode {
    Gen,
    Reuse
}

#[derive(Clone, Debug)]
pub struct Clicker {
    button: Button,
    clicks: usize,
    data: ClickData,
    delay_mode: DelayMode,
    hot_key: Arc<Mutex<HotKey>>,
    window: HWND
}

impl Clicker {
    pub fn new(button: Button, data: ClickData, delay_mode: DelayMode, hot_key: HotKey, window: HWND) -> Self {
        Self { button, data, delay_mode, clicks: 0, hot_key: Arc::new(Mutex::new(hot_key)), window }
    }

    pub fn run(&mut self) {
        loop {
            if self.hot_key.lock().unwrap().check() {
                self.click()
            }
        }
    }

    fn click(&mut self) {
        let delay = self.delay();

        self.button.down(self.window);
        sleep(delay.down);

        self.button.up(self.window);
        sleep(delay.up);

        self.clicks += 1
    }

    fn delay(&mut self) -> Delay {
        let mut clicks = self.clicks;

        let num = self.data.num();
        let diff = clicks as isize - num as isize;

        if diff >= 0 {
            match self.delay_mode {
                DelayMode::Gen => {
                    self.data.generate(clicks, self.data.cps().distribution(), &mut thread_rng());
                }
                
                DelayMode::Reuse => {
                    clicks -= num * (diff as usize / num + 1);
                }
            }
        }

        self.data.delays[clicks]
    }
}