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

    pub fn is_down(&mut self) -> bool {
        unsafe {
            GetAsyncKeyState(self.button) != 0
        }
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