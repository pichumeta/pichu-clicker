use std::time::Duration;

use rand::{distributions::Uniform, thread_rng};
use spin_sleep::sleep;
use winapi::um::winuser::{SendNotifyMessageA, VK_LBUTTON, WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDOWN, WM_LBUTTONUP};

use crate::pichu_clicker::{clicker::{click_recorder::{click_data::ClickData, hot_key::{button::Button, HotKey}, ClickRecorder}, Clicker, DelayMode}, mc_window, set_timer_res};

mod pichu_clicker;

fn main() {
    set_timer_res();
    
    let left_mouse = Button::new(VK_LBUTTON);
    let window = mc_window();

    let mut recorder = ClickRecorder::new(left_mouse);
    recorder.record(1);

    println!("{:?}", recorder.data);
    println!("{:?}", recorder.data.cps());

    let mut rng = thread_rng();
    let generated = ClickData::from_generated(10000, Uniform::new_inclusive(0, 100), &mut rng);
    //println!("{:?}", generated);
    println!("{:?}", generated.cps());

    let mut clicker = Clicker::new(left_mouse, generated, DelayMode::Reuse, 
        HotKey::new(left_mouse, Button::new('F' as i32)), window);
    clicker.run()
}
