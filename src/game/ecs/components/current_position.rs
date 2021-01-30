use super::TargetPosition;

use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Default, Component, Debug)]
#[storage(VecStorage)]
pub struct CurrentPosition {
    pub x: f32,
    pub y: f32,
}
