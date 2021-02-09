use super::{
    ecs::{
        components::{
            ChoiceName, CurrentPosition, EntityName, FacingDirection, MapName, Player, QuestName,
            StateName, TargetPosition, TaskName,
        },
        resources::TileMap,
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
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct PlayerDefinition {
    pub map: MapName,
    pub position: Position,
    pub direction: GameDirection,
    pub journal: HashMap<QuestName, QuestDefinition>,
}

impl PlayerDefinition {
    pub fn new(map: MapName, position: Position, direction: GameDirection) -> Self {
        let journal = utils::map!();

        Self {
            map,
            position,
            direction,
            journal,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct EntityDefinition {
    pub locations: HashMap<MapName, Position>,
    pub states: HashSet<StateName>,
}

impl EntityDefinition {
    pub fn new() -> Self {
        let locations = utils::map!();
        let states = utils::set!();

        Self { locations, states }
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
    pub entities: HashMap<EntityName, EntityDefinition>,
    pub world: WorldDefinition,
}

impl SaveData {
    pub fn new(
        map: MapName,
        position: Position,
        direction: GameDirection,
        /*difficulty: GameDifficulty, */
    ) -> Self {
        let entities = utils::map!();

        Self {
            entities,
            player: PlayerDefinition::new(map, position, direction),
            world: WorldDefinition::new(),
        }
    }

    pub fn from_game_state(game_state: &mut GameState) -> GameResult<Self> {
        let tile_map = game_state.world.try_fetch::<TileMap>().ok_or_else(|| {
            ggez::GameError::CustomError("Couldn't find tile map resource".to_string())
        })?;

        let (player_c, target_position_c, facing_direction_c): (
            specs::ReadStorage<Player>,
            specs::ReadStorage<TargetPosition>,
            specs::ReadStorage<FacingDirection>,
        ) = game_state.world.system_data();

        for (_, target_position, facing_direction) in
            (&player_c, &target_position_c, &facing_direction_c).join()
        {
            // Help linter
            #[cfg(debug_assertions)]
            let target_position = target_position as &TargetPosition;
            #[cfg(debug_assertions)]
            let facing_direction = facing_direction as &FacingDirection;

            let map = tile_map.current_map.clone();
            let position = Position {
                x: target_position.x,
                y: target_position.y,
            };
            let direction = facing_direction.direction;

            // TODO: Get from game_state
            let task_name = TaskName::some_task();
            let task_status = TaskStatus::Unknown;

            let choice_name = ChoiceName::some_choice();
            let choice = true;

            let tasks = utils::map!(task_name => task_status);
            let choices = utils::map!(choice_name => choice);

            let quest_name = QuestName::some_quest();
            let quest_definition = QuestDefinition { tasks, choices };

            let journal = utils::map!(quest_name => quest_definition);

            let player = PlayerDefinition {
                map,
                position,
                direction,
                journal,
            };

            let entities = utils::map!();

            let states = utils::set!();

            let world = WorldDefinition { states };

            return Ok(Self {
                player,
                entities,
                world,
            });
        }

        return Err(ggez::GameError::CustomError(
            "SaveData couldn't be found from game_state".to_string(),
        ));
    }

    pub fn to_game_state(self, game_state: &mut GameState) -> GameResult {
        let mut tile_map = game_state.world.try_fetch_mut::<TileMap>().ok_or_else(|| {
            ggez::GameError::CustomError("Couldn't find tile map resource".to_string())
        })?;

        tile_map.current_map = self.player.map;

        let (player_c, mut target_position_c, mut current_position_c, mut facing_direction_c): (
            specs::ReadStorage<Player>,
            specs::WriteStorage<TargetPosition>,
            specs::WriteStorage<CurrentPosition>,
            specs::WriteStorage<FacingDirection>,
        ) = game_state.world.system_data();

        for (_, target_position, current_position, facing_direction) in (
            &player_c,
            &mut target_position_c,
            &mut current_position_c,
            &mut facing_direction_c,
        )
            .join()
        {
            // Help linter
            #[cfg(debug_assertions)]
            let target_position = target_position as &mut TargetPosition;
            #[cfg(debug_assertions)]
            let current_position = current_position as &mut CurrentPosition;
            #[cfg(debug_assertions)]
            let facing_direction = facing_direction as &mut FacingDirection;

            let player_entity = tile_map
                .get_tile_mut(target_position.x, target_position.y)
                .entity
                .take()
                .expect(&format!(
                    "Player entity isn't in tile_map @ [{}, {}]\n{:#?}\n{:#?}",
                    target_position.x, target_position.y, current_position, target_position
                ));

            target_position.x = self.player.position.x;
            target_position.y = self.player.position.y;

            target_position.from_x = target_position.x;
            target_position.from_y = target_position.y;

            current_position.x = target_position.x as f32;
            current_position.y = target_position.y as f32;

            facing_direction.direction = self.player.direction;

            tile_map
                .get_tile_mut(target_position.x, target_position.y)
                .entity
                .replace(player_entity);

            return Ok(());
        }

        return Err(ggez::GameError::CustomError(
            "SaveData couldn't be found from game_state".to_string(),
        ));
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
    pub fn new(name: String, map: MapName) -> Self {
        Self {
            name,
            current_map: map,
            seconds_played: 0,
            finished: false,
        }
    }

    pub fn from_game_state(game_state: &mut GameState) -> GameResult<Self> {
        game_state
            .world
            .try_fetch::<Self>()
            .ok_or_else(|| {
                ggez::GameError::CustomError("Couldn't find tile map resource".to_string())
            })
            .map(|meta| (*meta).clone())
    }
}

/*

{
    "player": {
        "map": "pallet_town",
        "position": { "x": 1, "y": 2 },

        "journal": {
            "quest1": {
                "tasks": {
                    "talk to leader": "COMPLETED", // Know and done
                    "stop leader from hurting friend": "FAILED", // Know and can no longer complete
                    "find necklace": "IN PROGRESS", // Known and highlighted in journal
                    "find helmet": "IN PROGRESS", // Known and highlighted in journal
                    "kill dragon": "UNKNOWN", // Unknown task, won't be in journal until learned about
                    "save princess": "NOT STARTED" // Known but not ready to start. Will be in journal as a todo
                },
                "choices": {
                    "joinGang": false,
                    "sneak": true,
                }
            },
            "gatesTo: prif": {
                "tasks": {
                    "opened gates to prif": "UNKNOWN",
                },
                "choices": {}
            },
            "gatesTo: kings landing": {
                "tasks": {
                    "opened gates to kings landing": "UNKNOWN",
                },
                "choices": {}
            }
        },
    },

    "entities": {
        "wise old man": {
            "locations": { // NPCs can have multiple locations, each with a different dialog
                "pallet_town": { "x": 20, "y": 7, "dialogId": 0 },
                "varrock": { "x": 3, "y": 4, "dialogId": 8 },
            },
            "states": ["hasIntroducedPlayer"],
        },

        "bulletin board": {
            "locations": {
                "pallet_town": { "x": 10, "y": 10, "dialogId": 0 }, // Static, if included at all
                ...
            },
            "states": ["0", "1", "3"], // quest ids?
        }
    },

    "world": {
        "states": ["discoveredElves"],
    },
}

*/
