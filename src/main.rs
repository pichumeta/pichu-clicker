use rand::thread_rng;
use winapi::um::winuser::{VK_DELETE, VK_LBUTTON};

use crate::pichu_clicker::{clicker::{click_recorder::{click_data::{cps::CPS, ClickData}, hot_key::{button::Button, HotKey}, ClickRecorder}, Clicker, DelayMode}, mc_window, set_timer_res};

mod pichu_clicker;

fn main() {
    set_timer_res();
    
    let left_mouse = Button::new(VK_LBUTTON);
    let window = mc_window();

    // let mut recorder = ClickRecorder::new(left_mouse);
    // recorder.record(1000);

    let file_path = "test.clicks";

    // recorder.data.write_to_file(file_path).unwrap();
    let mut clicks = ClickData::from_file(file_path).unwrap();
    clicks.filter_by_freq(7).unwrap();

    // dbg!(recorder.data.cps());
    /*dbg!(clicks.cps());

    let dimensions = (600, 600);

    clicks.plot_histogram("test.png", dimensions).unwrap();

    let cps_10 = CPS::new(10.0);
    let dist = cps_10.distribution();

    let mut generated = ClickData::from_generated(1000, dist, &mut thread_rng());
    generated.filter_by_freq(5).unwrap();

    dbg!(generated.cps());

    generated.plot_histogram("test_generated.png", dimensions).unwrap();*/

    let mut clicker = Clicker::new(left_mouse, clicks, DelayMode::Reuse, 
         HotKey::new(left_mouse, Button::new(VK_DELETE)), window);
    clicker.run()
}
