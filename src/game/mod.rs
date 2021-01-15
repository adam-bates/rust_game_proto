mod game;
mod game_state;
mod game_loop;
mod context;
mod settings;

use super::{config, error, filesystem};

pub fn run_game(fs: ggez::filesystem::Filesystem) -> ggez::GameResult {
    game::run_game(fs)
}
