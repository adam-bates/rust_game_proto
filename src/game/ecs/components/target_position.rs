use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Default, Component, Debug)]
#[storage(VecStorage)]
pub struct TargetPosition {
    pub x: u32,
    pub y: u32,
}
