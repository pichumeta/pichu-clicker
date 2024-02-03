use std::{sync::atomic::{AtomicBool, Ordering}, time::Duration};

use spin_sleep::sleep;
use winapi::um::winuser::GetAsyncKeyState;

use self::button::Button;

#[path = "button.rs"] pub mod button;

#[derive(Debug)]
pub struct HotKey {
    hold_key: Button,
    hot_key: Button,

    pub enabled: bool,
    pub run: bool,

    wait: Duration
}

impl HotKey {
    pub fn new(hold_key: Button, hot_key: Button) -> Self {
        Self { hold_key, hot_key, enabled: false, run: true, wait: Duration::from_millis(10) }
    }

    pub fn check(&mut self) -> bool {
        self.enabled = self.hold_key() || self.hot_key();
        self.enabled
    }

    pub fn run(&mut self) {
        while self.run {
            self.check();
            sleep(self.wait)
        }
    }

    fn hold_key(&self) -> bool {
        unsafe {
            GetAsyncKeyState(self.hold_key.button) != 0
        }
    }

    fn hot_key(&self) -> bool {
        if unsafe {
            GetAsyncKeyState(self.hot_key.button)
        } != 0 {
            return !self.enabled;
        }

        self.enabled
    }
}