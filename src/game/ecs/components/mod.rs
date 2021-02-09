mod current_position;
mod drawable;
mod facing_direction;
mod id;
mod interactable;
mod is_static;
mod player;
mod sprite_sheet;
mod target_position;
mod timer;

pub use current_position::CurrentPosition;
pub use drawable::Drawable;
pub use facing_direction::FacingDirection;
pub use id::Id;
pub use interactable::Interactable;
pub use is_static::IsStatic;
pub use player::Player;
pub use sprite_sheet::{SpriteRow, SpriteSheet};
pub use target_position::TargetPosition;
pub use timer::Timer;

use super::super::{input, scenes};
use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Debug, Component, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
#[storage(VecStorage)]
pub struct EntityName(String);

impl EntityName {
    pub fn value(&self) -> String {
        self.0.clone()
    }

    pub fn some_entity() -> Self {
        Self("some_entity".to_string())
    }
}

#[derive(Debug, Component, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
#[storage(VecStorage)]
pub struct MapName(String);

impl MapName {
    pub fn value(&self) -> String {
        self.0.clone()
    }

    pub fn pallet_town() -> Self {
        Self("pallet_town".to_string())
    }
}

#[derive(Debug, Component, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
#[storage(VecStorage)]
pub struct QuestName(String);

impl QuestName {
    pub fn value(&self) -> String {
        self.0.clone()
    }

    pub fn some_quest() -> Self {
        Self("some_quest".to_string())
    }
}

#[derive(Debug, Component, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
#[storage(VecStorage)]
pub struct TaskName(String);

impl TaskName {
    pub fn value(&self) -> String {
        self.0.clone()
    }

    pub fn some_task() -> Self {
        Self("some_task".to_string())
    }
}

#[derive(Debug, Component, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
#[storage(VecStorage)]
pub struct ChoiceName(String);

impl ChoiceName {
    pub fn value(&self) -> String {
        self.0.clone()
    }

    pub fn some_choice() -> Self {
        Self("some_choice".to_string())
    }
}

#[derive(Debug, Component, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
#[storage(VecStorage)]
pub struct StateName(String);

impl StateName {
    pub fn value(&self) -> String {
        self.0.clone()
    }

    pub fn some_state() -> Self {
        Self("some_state".to_string())
    }
}
