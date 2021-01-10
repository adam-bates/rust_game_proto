mod config;
mod error;
mod game;

fn main() -> error::types::Result {

    // Set up logger
    config::log::setup().or_else(error::handle_log_setup_err)?;

    // Run game
    game::run_game().or_else(error::handle_game_err)
}
