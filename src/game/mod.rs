mod game;
mod context;
mod settings;

use super::config;
use super::error;

pub fn run_game(fs: ggez::filesystem::Filesystem) -> ggez::GameResult {
    game::run_game(fs)
}
