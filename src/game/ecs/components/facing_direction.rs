use super::input::types::GameDirection;
use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct FacingDirection {
    pub direction: GameDirection,
}
