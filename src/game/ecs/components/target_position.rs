use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Default, Component, Debug)]
#[storage(VecStorage)]
pub struct TargetPosition {
    pub x: usize,
    pub y: usize,
    pub is_moving: bool,
}
