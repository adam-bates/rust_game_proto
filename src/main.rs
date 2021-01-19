mod config;
mod error;
mod filesystem;
mod game;
mod utils;

use error::types::Result;

fn main() -> Result {
    // Initialize filesystem
    let mut fs = filesystem::new_filesystem()?;

    // Setup logger
    let log_opts = config::log::setup(&mut fs)?;

    let internal_error_handler = Box::new(|e| error::handle_game_err(e).unwrap());

    // Run game
    game::run_game(fs, internal_error_handler).or_else(error::handle_game_err)?;

    // Clean up logger
    config::log::clean_up(log_opts)
}
