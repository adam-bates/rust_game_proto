mod current_position;
mod door;
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
pub use door::Door;
pub use drawable::Drawable;
pub use facing_direction::FacingDirection;
pub use id::Id;
pub use interactable::Interactable;
pub use is_static::IsStatic;
pub use player::Player;
pub use sprite_sheet::{SpriteRow, SpriteSheet};
pub use target_position::TargetPosition;
pub use timer::Timer;

use super::super::{
    error::types::GameResult,
    game_state::GameState,
    input::{self, types::GameDirection},
    maps,
    save::{MetaSaveData, SaveData},
    scenes,
};
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
    Varrock,
}

impl MapName {
    pub fn scene_builder(&self) -> scenes::types::SceneBuilder {
        match self {
            Self::PalletTown => Box::new(|game_state, ctx| {
                let scene = scenes::PalletTownOverworldScene::new(game_state, ctx)?;
                Ok(Rc::new(RefCell::new(scene)))
            }),
            Self::Varrock => Box::new(|game_state, ctx| {
                let scene = scenes::VarrockOverworldScene::new(game_state, ctx)?;
                Ok(Rc::new(RefCell::new(scene)))
            }),
        }
    }

    pub fn scene_builder_from_door(
        &self,
        door_id: usize,
    ) -> GameResult<scenes::types::SceneBuilder> {
        let (position, direction) = self.get_door_position(door_id).ok_or_else(|| {
            ggez::GameError::CustomError(format!(
                "No door found for door_id [{}] for map: {:#?}",
                door_id, self
            ))
        })?;

        let map_scene_builder: scenes::types::SceneBuilder = self.scene_builder();

        let map = self.clone();

        Ok(Box::new(move |game_state: &mut GameState, ctx| {
            {
                let mut save_data = game_state.world.fetch_mut::<SaveData>();
                save_data.player.map = map.clone();

                let delta_xy = direction.to_xy();

                save_data.player.position.x = (position.0 as isize + delta_xy.0) as usize;
                save_data.player.position.y = (position.1 as isize + delta_xy.1) as usize;
                save_data.player.position.facing = Some(direction);
            }
            {
                let mut meta_save_data = game_state.world.fetch_mut::<MetaSaveData>();
                meta_save_data.current_map = map.clone();
            }

            map_scene_builder(game_state, ctx)
        }))
    }

    pub fn get_door_position(&self, door_id: usize) -> Option<((usize, usize), GameDirection)> {
        match self {
            Self::PalletTown => match door_id {
                0 => Some(((19, 19), GameDirection::Up)),
                _ => None,
            },
            Self::Varrock => match door_id {
                0 => Some(((13, 0), GameDirection::Down)),
                _ => None,
            },
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
