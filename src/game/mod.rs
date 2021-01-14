mod game;
mod context;
mod settings;

use super::{error, filesystem};

pub fn run_game(fs: ggez::filesystem::Filesystem) -> ggez::GameResult {
    game::run_game(fs)
}
