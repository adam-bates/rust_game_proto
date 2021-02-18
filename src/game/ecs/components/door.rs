use super::{MapName, TargetPosition};

use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Door {
    pub id: usize,
    pub to_map: MapName,
    pub to_id: usize,
}
