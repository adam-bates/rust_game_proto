mod config;
mod error;
mod filesystem;
mod game;
mod utils;

use error::types;

fn main() -> types::Result {
    // Initialize filesystem
    let mut fs = filesystem::new_filesystem()?;

    // Setup logger
    let log_opts = config::log::setup(&mut fs)?;

    // Run game
    game::run_game(fs).or_else(error::handle_game_err)?;

    // Clean up logger
    config::log::clean_up(log_opts)
}
