mod context;
mod events;
mod game;
mod game_loop;
mod game_state;
mod settings;
mod render;

use super::{config, error, filesystem};

pub fn run_game(fs: ggez::filesystem::Filesystem) -> ggez::GameResult {
    game::run_game(fs)
}
