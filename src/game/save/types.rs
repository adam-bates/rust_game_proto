use super::{
    ecs::components::{
        ChoiceName, CurrentPosition, EntityName, FacingDirection, MapName, QuestName, StateName,
        TargetPosition, TaskName,
    },
    input::types::GameDirection,
    utils, GameResult, GameState,
};
use serde::{Deserialize, Serialize};
use specs::Join;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum TaskStatus {
    Unknown,
    NotStarted,
    Active,
    Complete,
    Failed,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct QuestDefinition {
    pub tasks: HashMap<TaskName, TaskStatus>,
    pub choices: HashMap<ChoiceName, bool>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
    pub facing: Option<GameDirection>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct PlayerDefinition {
    pub map: MapName,
    pub position: Position,
    pub journal: HashMap<QuestName, QuestDefinition>,
}

impl PlayerDefinition {
    pub fn new(map: MapName, position: Position) -> Self {
        let journal = utils::map!();

        Self {
            map,
            position,
            journal,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct EntityInstanceDefinition {
    pub position: Position,
    pub dialog_id: usize,
}

impl EntityInstanceDefinition {
    pub fn insert_into_world(
        &self,
        game_state: &mut GameState,
        ctx: &mut ggez::Context,
        name: &EntityName,
    ) -> GameResult {
        match name {
            EntityName::WiseOldMan => {
                let (
                    entity_name_c,
                    mut target_position_c,
                    mut current_position_c,
                    mut facing_direction_c,
                ): (
                    specs::ReadStorage<EntityName>,
                    specs::WriteStorage<TargetPosition>,
                    specs::WriteStorage<CurrentPosition>,
                    specs::WriteStorage<FacingDirection>,
                ) = game_state.world.system_data();

                for (entity_name, target_position, current_position, facing_direction) in (
                    &entity_name_c,
                    &mut target_position_c,
                    &mut current_position_c,
                    &mut facing_direction_c,
                )
                    .join()
                {
                    // Help linter
                    #[cfg(debug_assertions)]
                    let entity_name = entity_name as &EntityName;
                    #[cfg(debug_assertions)]
                    let target_position = target_position as &mut TargetPosition;
                    #[cfg(debug_assertions)]
                    let current_position = current_position as &mut CurrentPosition;
                    #[cfg(debug_assertions)]
                    let facing_direction = facing_direction as &mut FacingDirection;

                    // Already exists in world
                    if *name == *entity_name {
                        return Ok(());
                    }
                }

                // Needs to be initialized
            }
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct MapDefinition {
    pub entity_instances: HashMap<EntityName, EntityInstanceDefinition>,
    // pub bulletins: HashMap<BulletinName, BulletinDefinition>,
    pub states: HashSet<StateName>,
}

impl MapDefinition {
    pub fn new(entity_instances: HashMap<EntityName, EntityInstanceDefinition>) -> Self {
        Self {
            entity_instances,
            states: utils::set!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct WorldDefinition {
    pub states: HashSet<StateName>,
}

impl WorldDefinition {
    pub fn new() -> Self {
        let states = utils::set!();

        Self { states }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct SaveData {
    pub player: PlayerDefinition,
    pub world: WorldDefinition,
    pub maps: HashMap<MapName, MapDefinition>,
    pub entity_states: HashMap<EntityName, HashSet<StateName>>,
}

impl SaveData {
    pub fn new(/*difficulty: GameDifficulty, */) -> Self {
        let pallet_town_entity_instances = utils::map!(
            EntityName::WiseOldMan => EntityInstanceDefinition {
                position: Position { x: 5, y: 5, facing: Some(GameDirection::Right) },
                dialog_id: 0,
            },
        );
        let varrock_entity_instances = utils::map!(
            EntityName::WiseOldMan => EntityInstanceDefinition {
                position: Position { x: 7, y: 7, facing: Some(GameDirection::Down) },
                dialog_id: 0,
            },
        );

        let maps = utils::map!(
            MapName::PalletTown => MapDefinition::new(pallet_town_entity_instances),
            MapName::Varrock => MapDefinition::new(varrock_entity_instances),
        );

        let entity_states = utils::map!(
            EntityName::WiseOldMan => utils::set!(),
        );

        Self {
            player: PlayerDefinition::new(
                MapName::Varrock,
                Position {
                    x: 4,
                    y: 8,
                    facing: Some(GameDirection::Down),
                },
            ),
            world: WorldDefinition::new(),
            maps,
            entity_states,
        }
    }

    pub fn from_game_state(game_state: &mut GameState) -> GameResult<Self> {
        game_state
            .world
            .try_fetch::<Self>()
            .ok_or_else(|| {
                ggez::GameError::CustomError("Couldn't find SaveData resource".to_string())
            })
            .map(|save| (*save).clone())
    }

    pub fn to_game_state(self, game_state: &mut GameState) -> GameResult {
        game_state.world.insert(self);
        return Ok(());
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct MetaSaveData {
    pub name: String,
    pub current_map: MapName,
    pub seconds_played: usize,
    pub finished: bool,
}

impl MetaSaveData {
    pub fn new(name: String) -> Self {
        Self {
            name,
            current_map: MapName::Varrock,
            seconds_played: 0,
            finished: false,
        }
    }

    pub fn from_game_state(game_state: &mut GameState) -> GameResult<Self> {
        game_state
            .world
            .try_fetch::<Self>()
            .ok_or_else(|| {
                ggez::GameError::CustomError("Couldn't find MetaSaveData resource".to_string())
            })
            .map(|save| (*save).clone())
    }
}

/*

// Meta
{
    "name": "Convergent",
    "current_map": "varrock",
    "seconds_played": 12345,
    "complete": false,
}

// Save
{
    "player": {
        "map": "varrock",
        "position": { "x": 1, "y": 2 },
        "journal": {
            "quest1": {
                "tasks": {
                    "talk to leader": "COMPLETED", // Known and done
                    "stop leader from hurting friend": "FAILED", // Known but can no longer complete
                    "find necklace": "IN PROGRESS", // Known and highlighted in journal
                    "find helmet": "IN PROGRESS", // Known and highlighted in journal
                    "kill dragon": "UNKNOWN", // Unknown task, won't be in journal until learned about
                    "save princess": "NOT STARTED" // Known but not ready to start. Will be in journal as a todo
                },
                "choices": {
                    "joinGang": false,
                    "sneak": true,
                },
                "complete": false,
            },
            "gatesTo: varrock": {
                "tasks": {
                    "opened gates to varrock": "COMPLETED",
                },
                "choices": {},
                "complete": true,
            },
            "gatesTo: kings landing": {
                "tasks": {
                    "opened gates to kings landing": "NOT STARTED",
                },
                "choices": {},
                "complete": false,
            }
        },
    },
    "world": {
        "states": ["discoveredElves"],
    },
    "maps": {
        "pallet_town": {
            "entity_instances": {
                "wise old man": {
                    "position": { "x": 20, "y": 7 },
                    "dialogId": 0,
                },
            },
            "bulletins": {
                "bulletin board 1": {
                    "dialogId": 0,
                    "questIds": [0, 1, 3],
                }
            },
            "states": [],
        },
        "varrock": {
            "entity_instances": {
                "wise old man": {
                    "position": { "x": 3, "y": 4 },
                    "dialogId": 8,
                    "states": [],
                },
            }
            "bulletins": {},
            "states": ["dark"],
        }
    },
    "entity_states": {
        "wise old man": ["hasIntroducedPlayer"],
    },
}

*/
