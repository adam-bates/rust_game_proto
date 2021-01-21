mod context;
mod ecs;
mod events;
mod game;
mod game_loop;
mod game_state;
mod input;
mod render;
mod scenes;
mod settings;
mod world;

use super::{config, error, filesystem};

pub fn run_game(
    fs: ggez::filesystem::Filesystem,
    error_handler: Box<dyn Fn(ggez::GameError)>,
) -> ggez::GameResult {
    game::run_game(fs, error_handler)
}
