use self::{click_data::{cps::delay::Delay, ClickData}, hot_key::button::Button};

#[path ="click_data.rs"] pub mod click_data;
#[path ="hot_key.rs"] pub mod hot_key;

#[derive(Clone, Debug)]
pub struct ClickRecorder {
    pub button: Button,
    pub data: ClickData
}

impl ClickRecorder {
    pub fn new(button: Button) -> Self {
        Self { button, data: ClickData::new(vec![]) }
    }

    pub fn record(&mut self, clicks: usize) {
        self.button.while_up();

        while clicks >= self.data.num() {
            let down = self.button.time_down();
            let up = self.button.time_up();

            self.data.delays.push(Delay::from_durations(down, up))
        }

        self.data.delays.remove(0);
    }
}