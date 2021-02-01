use super::scenes::types::SceneBuilder;
use specs::{Component, Entity, VecStorage};
use specs_derive::Component;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Interactable {
    pub handler: Box<dyn Fn(Entity, Entity) -> Option<SceneBuilder> + Send + Sync>,
}
