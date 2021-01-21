use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Default, Component)]
#[storage(VecStorage)]
pub struct TargetPosition {
    pub x: f32,
    pub y: f32,
}
