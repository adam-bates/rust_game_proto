use super::{
    ecs::{
        components::{CurrentPosition, Player, TargetPosition},
        resources::TileMap,
    },
    error::types::GameResult,
    game_state::GameState,
};
use serde::{Deserialize, Serialize};
use specs::Join;

const SAVE_FILE_DIR: &str = "/saves";
const SAVE_FILE_EXT: &str = "sav";
const BACKUP_FILE_EXT: &str = "backup.sav";

// TODO: Look into making save/load modular.
// Entities should "save themselves" (Maybe a saveable component? That has a save_handler and load_handler functions? With factory functions for easy creation?)
// Every entity has mutable state (even signs, they might change text), but what they save/load is different per entity
// Can we save/load dynamic structures? Maybe a hashmap? (more prone to error, technically slower but not noticeable)
// Or we could just hard-code everything we need ... Just might not be as flexible

fn get_user_data_vfs(ctx: &mut ggez::Context) -> GameResult<&Box<dyn ggez::vfs::VFS>> {
    ctx.filesystem
        .find_vfs(&ctx.filesystem.user_data_path)
        .ok_or_else(|| ggez::GameError::FilesystemError("Couldn't find user data vfs".to_string()))
}

pub fn save(game_state: &mut GameState, ctx: &mut ggez::Context, slot: SaveSlot) -> GameResult {
    let vfs = get_user_data_vfs(ctx)?;

    let saves_path = std::path::PathBuf::from(SAVE_FILE_DIR);

    if !vfs.exists(&saves_path) {
        vfs.mkdir(&saves_path)?;
    }

    let save_filename = &format!("{}.{}", slot.id(), SAVE_FILE_EXT);
    let backup_filename = &format!("{}.{}", slot.id(), BACKUP_FILE_EXT);

    let save_file_path = saves_path.join(save_filename);
    let backup_file_path = saves_path.join(backup_filename);

    if vfs.exists(&save_file_path) {
        let save_file = vfs.open(&save_file_path)?;
        let old_save_data: Option<SaveData> = match bincode::deserialize_from(save_file) {
            Ok(old_save_data) => Some(old_save_data),
            Err(e) => {
                println!("Error deserializing old save data: {}", e);
                None
            }
        };

        if let Some(old_save_data) = old_save_data {
            let backup_file = vfs.create(&backup_file_path)?;
            if let Err(e) = bincode::serialize_into(backup_file, &old_save_data) {
                println!("Error serializing old save data into backup file: {}", e);
            }
        }
    }

    let save_file = vfs.create(&save_file_path)?;
    let save_data = SaveData::from_game_state(game_state)?;
    bincode::serialize_into(save_file, &save_data).map_err(|e| {
        ggez::GameError::CustomError(format!(
            "Error serializing save data into save file: {:?}\n{}",
            save_data, e
        ))
    })?;

    let save_file = vfs.open(&save_file_path)?;
    let save_data_check: SaveData = bincode::deserialize_from(save_file).map_err(|e| {
        ggez::GameError::CustomError(format!(
            "Failed to read save data: {:?}\n{}",
            save_file_path, e
        ))
    })?;

    if save_data != save_data_check {
        return Err(ggez::GameError::CustomError(format!(
            "Error saving data, save file doesn't match save data.\nSave data: {:#?}\nSaved file: {:#?}",
            save_data, save_data_check
        )));
    }

    // Delete backup save now that main save is confirmed valid
    if vfs.exists(&backup_file_path) {
        vfs.rm(&backup_file_path)?;
    }

    Ok(())
}

pub fn load(game_state: &mut GameState, ctx: &mut ggez::Context, slot: SaveSlot) -> GameResult {
    let vfs = get_user_data_vfs(ctx)?;

    let save_file_path = std::path::PathBuf::from(&format!(
        "{}/{}.{}",
        SAVE_FILE_DIR,
        slot.id(),
        SAVE_FILE_EXT
    ));

    let save_file = vfs.open(&save_file_path)?;
    let save_data: SaveData = bincode::deserialize_from(save_file).map_err(|e| {
        ggez::GameError::CustomError(format!("Error deserializing old save data: {}", e))
    })?;

    save_data.to_game_state(game_state)
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum SaveSlot {
    One,
    Two,
    Three,
}

impl SaveSlot {
    pub fn id(&self) -> usize {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
        }
    }

    pub fn from_id(id: usize) -> Option<Self> {
        match id {
            1 => Some(Self::One),
            2 => Some(Self::Two),
            3 => Some(Self::Three),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct SaveData {
    player_position_x: usize,
    player_position_y: usize,
}

impl SaveData {
    fn from_game_state(game_state: &GameState) -> GameResult<Self> {
        let (player_c, target_position_c): (
            specs::ReadStorage<Player>,
            specs::ReadStorage<TargetPosition>,
        ) = game_state.world.system_data();

        for (_, target_position) in (&player_c, &target_position_c).join() {
            // Help linter
            #[cfg(debug_assertions)]
            let target_position = target_position as &TargetPosition;

            return Ok(SaveData {
                player_position_x: target_position.from_x,
                player_position_y: target_position.from_y,
            });
        }

        Err(ggez::GameError::CustomError(format!(
            "Unable to read component data when creating SaveData. GameState: {:#?}",
            game_state
        )))
    }

    fn to_game_state(self, game_state: &mut GameState) -> GameResult {
        // MOVE PLAYER TO POSITION
        let world = &game_state.world;
        let mut tile_map = world.try_fetch_mut::<TileMap>().ok_or_else(|| {
            ggez::GameError::CustomError("Tilemap resource not found".to_string())
        })?;

        let position = (self.player_position_x, self.player_position_y);

        let (player_c, mut current_position_c, mut target_position_c): (
            specs::ReadStorage<Player>,
            specs::WriteStorage<CurrentPosition>,
            specs::WriteStorage<TargetPosition>,
        ) = game_state.world.system_data();

        for (_, current_position, target_position) in
            (&player_c, &mut current_position_c, &mut target_position_c).join()
        {
            // Help linter
            #[cfg(debug_assertions)]
            let current_position = current_position as &mut CurrentPosition;
            #[cfg(debug_assertions)]
            let target_position = target_position as &mut TargetPosition;

            let player_entity = tile_map
                .get_tile_mut(target_position.x, target_position.y)
                .entity
                .take()
                .expect(&format!(
                    "Player entity isn't in tile_map @ [{}, {}]\n{:#?}\n{:#?}",
                    target_position.x, target_position.y, current_position, target_position
                ));

            tile_map.get_tile_mut(position.0, position.1).entity = Some(player_entity);

            current_position.x = position.0 as f32;
            current_position.y = position.1 as f32;
            target_position.from_x = position.0;
            target_position.from_y = position.1;
            target_position.x = position.0;
            target_position.y = position.1;
        }

        Ok(())
    }
}
