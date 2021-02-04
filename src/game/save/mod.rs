use super::{
    ecs::components::{Player, TargetPosition},
    error::types::GameResult,
    game_state::GameState,
};
use serde::{Deserialize, Serialize};
use specs::Join;

#[derive(Serialize, Deserialize, Debug)]
pub enum SaveSlot {
    One,
    Two,
    Three,
}

impl SaveSlot {
    fn id(&self) -> usize {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
        }
    }

    fn from_id(id: usize) -> Option<Self> {
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
    pub fn from_game_state(game_state: &GameState) -> GameResult<Self> {
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

    pub fn to_game_state(self) -> GameResult<GameState> {
        todo!()
    }

    pub fn save(&self, ctx: &mut ggez::Context, slot: SaveSlot) -> GameResult {
        let vfs = ctx
            .filesystem
            .find_vfs(&ctx.filesystem.user_data_path)
            .expect("Couldn't find user data vfs");

        let saves_path = std::path::PathBuf::from("/saves");

        if !vfs.exists(&saves_path) {
            vfs.mkdir(&saves_path)?;
        }

        let save_filename = &format!("{}.sav", slot.id());
        let backup_filename = &format!("{}.backup.sav", slot.id());

        let save_file_path = saves_path.join(save_filename);
        let backup_file_path = saves_path.join(backup_filename);

        if vfs.exists(&save_file_path) {
            let save_file = vfs.open(&save_file_path)?;
            let old_save_data: Option<Self> = match bincode::deserialize_from(save_file) {
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
        bincode::serialize_into(save_file, &self).map_err(|e| {
            ggez::GameError::CustomError(format!(
                "Error serializing save data into save file: {:?}\n{}",
                self, e
            ))
        })?;

        let save_file = vfs.open(&save_file_path)?;
        let save_data: Self = bincode::deserialize_from(save_file).map_err(|e| {
            ggez::GameError::CustomError(format!(
                "Failed to read save data: {:?}\n{}",
                save_file_path, e
            ))
        })?;

        if save_data != *self {
            return Err(ggez::GameError::CustomError(format!(
                "Error saving data, save file doesn't match save data. Save data: {:?}",
                save_data
            )));
        }

        // Delete backup save now that main save is confirmed valid
        if vfs.exists(&backup_file_path) {
            vfs.rm(&backup_file_path)?;
        }

        Ok(())
    }

    pub fn load(ctx: &mut ggez::Context, slot: SaveSlot) -> GameResult<Self> {
        todo!()
    }
}
