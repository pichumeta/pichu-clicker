use std::{ffi::OsString, iter::once, os::windows::ffi::OsStrExt, ptr::null};

use winapi::{shared::windef::HWND, um::{processthreadsapi::{GetCurrentProcess, SetProcessInformation}, winuser::FindWindowW}};

#[path ="clicker.rs"] pub mod clicker;

const AUTHOR: &str = "elecreal";
const CLASS: &str = "LWJGL";
const NAME: &str = "pichu-clicker";
const VERSION: f32 = 0.01;

pub fn version_string() -> String {
    format!("v{}", VERSION)
}

pub fn name() -> String {
    format!("{} {} by: {}", NAME, version_string(), AUTHOR)
}

pub fn class() -> Vec<u16> {
    let window_class = OsString::from(CLASS);
    window_class
        .as_os_str()
        .encode_wide()
        .chain(once(0))
        .collect()
}

pub fn set_timer_res() {
    unsafe {
        SetProcessInformation(GetCurrentProcess(), 4, core::ptr::null_mut(), 12)
    };
}

pub fn mc_window() -> HWND {
    unsafe {
        FindWindowW(class().as_ptr(), null())
    }
}