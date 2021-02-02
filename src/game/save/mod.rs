use super::{error::types::GameResult, game_state::GameState};

// Here's a draft of my thoughts on saving / loading ...

// When saving / loading, we should specify one of a finite number of save slots, which will have specific file names

// SaveData will hold all of the serializable data, and have functions to convert to and from GameState

// SaveData will have:
// - from_state(GameState) -> Result<Self>
// - to_state(self) -> Result<GameState>
// - save(&self, SaveSlot) -> Result
// - load(SaveSlot) -> Result<Self>

pub enum SaveSlot {
    ONE,
    TWO,
    THREE,
}

pub struct SaveData {}

impl SaveData {
    pub fn from_game_state(game_state: &GameState) -> GameResult<Self> {
        Ok(SaveData {})
    }

    pub fn load(slot: SaveSlot) -> GameResult<Self> {
        Ok(SaveData {})
    }

    pub fn to_game_state(self) -> GameResult<GameState> {
        todo!()
    }

    pub fn save(&self, slot: SaveSlot) -> GameResult {
        Ok(())
    }
}
