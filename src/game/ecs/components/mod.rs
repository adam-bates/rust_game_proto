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

use super::super::{game_state::GameState, input, scenes};
use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};
use specs_derive::Component;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Component, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
#[storage(VecStorage)]
pub enum EntityName {
    WiseOldMan,
}

impl EntityName {
    pub fn new_entity(game_state: &mut GameState, ctx: &mut ggez::Context) {
        // TODO: find and move, or insert
    }
}

#[derive(Debug, Component, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
#[storage(VecStorage)]
pub enum MapName {
    PalletTown,
}

impl MapName {
    pub fn scene_builder(&self) -> scenes::types::SceneBuilder {
        match self {
            Self::PalletTown => Box::new(|game_state, ctx| {
                let scene = scenes::PalletTownOverworldScene::new(game_state, ctx)?;
                Ok(Rc::new(RefCell::new(scene)))
            }),
        }
    }
}

#[derive(Debug, Component, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
#[storage(VecStorage)]
pub enum QuestName {
    TestQuest,
}

#[derive(Debug, Component, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
#[storage(VecStorage)]
pub enum TaskName {
    TestTask,
}

#[derive(Debug, Component, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
#[storage(VecStorage)]
pub enum ChoiceName {
    TestChoice,
}

#[derive(Debug, Component, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
#[storage(VecStorage)]
pub enum StateName {
    TestState,
}
