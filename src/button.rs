use std::time::{Duration, Instant};

use spin_sleep::sleep;
use winapi::{shared::windef::HWND, um::winuser::{GetAsyncKeyState, SendNotifyMessageA, VK_LBUTTON, VK_RBUTTON, WM_KEYDOWN, WM_LBUTTONDOWN, WM_RBUTTONDOWN}};

#[derive(Clone, Copy, Debug)]
pub struct Button {
    pub button: i32,
    pub msg: u32,
    pub lparam: isize
}

impl Button {
    pub fn new(button: i32) -> Self {
        Self {
            button,
            msg: Self::make_msg(button),
            lparam: Self::make_lparam(button)
        }
    }

    pub fn down(&self, window: HWND) {
        unsafe { SendNotifyMessageA(window, self.msg, self.button as usize, 0) };
    }

    pub fn up(&self, window: HWND) {
        unsafe { SendNotifyMessageA(window, self.msg + 1, self.button as usize, self.lparam) };
    }

    pub fn is_down(&self) -> bool {
        unsafe {
            GetAsyncKeyState(self.button) != 0
        }
    }

    pub fn while_down(&self) {
        while self.is_down() {
            sleep(Duration::from_nanos(1000))
        }
    }

    pub fn while_up(&self) {
        while !self.is_down() {
            sleep(Duration::from_nanos(1000))
        }
    }

    pub fn time_down(&self) -> Duration {
        let start = Instant::now();
        self.while_down();
        Instant::now().duration_since(start)
    }

    pub fn time_up(&self) -> Duration {
        let start = Instant::now();
        self.while_up();
        Instant::now().duration_since(start)
    }

    fn make_msg(button: i32) -> u32 {
        match button {
            VK_LBUTTON => WM_LBUTTONDOWN,
            VK_RBUTTON => WM_RBUTTONDOWN,
            _ => WM_KEYDOWN
        }
    }

    fn make_lparam(button: i32) -> isize {
        0
		| ((button as isize) << 16)
		| (0 << 24)
		| (0 << 29)
		| (1 << 30)
		| (1 << 31)
    }
}