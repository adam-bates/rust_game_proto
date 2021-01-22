use super::TargetPosition;

use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Default, Component, Debug)]
#[storage(VecStorage)]
pub struct CurrentPosition {
    pub x: f32,
    pub y: f32,
}

impl CurrentPosition {
    pub fn lerp(&mut self, target_position: &TargetPosition, percent: f32) {
        if percent == 1.0 {
            self.x = target_position.x as f32;
            self.y = target_position.y as f32;
        } else {
            self.x += (target_position.x as f32 - self.x) * percent;
            self.y += (target_position.y as f32 - self.y) * percent;
        }
    }
}
