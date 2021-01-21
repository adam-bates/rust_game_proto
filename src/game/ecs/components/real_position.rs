use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Default, Component)]
#[storage(VecStorage)]
pub struct RealPosition {
    pub x: f32,
    pub y: f32,
}
