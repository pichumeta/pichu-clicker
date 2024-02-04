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
        Self { hold_key, hot_key, enabled: false, run: true, wait: Duration::from_millis(30) }
    }

    pub fn check(&mut self) -> bool {
        let hold_key = self.hold_key();
        let hot_key = self.hot_key();

        self.enabled = hot_key;

        self.enabled || hold_key
    }

    pub fn run(&mut self) {
        while self.run {
            self.check();
            sleep(self.wait)
        }
    }

    fn hold_key(&self) -> bool {
        let down = self.hold_key.is_down();
        down
    }

    fn hot_key(&self) -> bool {
        if self.hot_key.is_down() {
            return !self.enabled;
        }

        self.enabled
    }
}