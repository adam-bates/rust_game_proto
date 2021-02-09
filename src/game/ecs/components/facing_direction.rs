use super::input::types::GameDirection;
use serde_derive::{Deserialize, Serialize};
use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Component, Debug, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct FacingDirection {
    pub direction: GameDirection,
}
