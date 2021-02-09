use super::{CurrentPosition, Timer};
use serde_derive::{Deserialize, Serialize};
use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Default, Component, Debug, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct TargetPosition {
    pub x: usize,
    pub y: usize,

    #[serde(skip)]
    pub from_x: usize,

    #[serde(skip)]
    pub from_y: usize,

    #[serde(skip)]
    pub is_moving: bool,
}

impl TargetPosition {
    pub fn get_current_position(&self, timer: &Timer) -> CurrentPosition {
        let x = self.x as f32;
        let y = self.y as f32;
        let from_x = self.from_x as f32;
        let from_y = self.from_y as f32;

        let percent = timer.percent();

        CurrentPosition {
            x: from_x + (x - from_x) * percent,
            y: from_y + (y - from_y) * percent,
        }
    }
}
