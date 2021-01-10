mod config;
mod error;
mod game;

fn main() -> error::types::Result {

    // Set up logger
    config::log::setup();

    // Run game
    game::run_game().or_else(error::handle_game_err)
}
