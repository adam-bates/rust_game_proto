pub mod settings;

mod context;
mod ecs;
mod events;
mod game;
mod game_loop;
mod game_state;
mod input;
mod maps;
mod render;
mod save;
mod scenes;
mod world;

use super::{config, error, filesystem, utils};

pub fn run_game(
    fs: ggez::filesystem::Filesystem,
    error_handler: Box<dyn Fn(ggez::GameError)>,
) -> ggez::GameResult {
    game::run_game(fs, error_handler)
}
