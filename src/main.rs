mod config;
mod context;
mod error;
mod game;
mod utils;

use error::types;

fn main() -> types::Result {
    // Create ggez context
    let (ctx, events_loop) = &mut context::new_context();

    ggez::filesystem::print_all(ctx);

    let config_path = ggez::filesystem::user_config_dir(ctx);

    // Set up logger
    let log_output = match config::log::setup(config_path) {
        Err(error) => return error::handle_log_setup_err(error),
        Ok(output) => output,
    };

    // Run game
    game::run_game(ctx, events_loop).or_else(error::handle_game_err)?;

    // Clean up logger
    config::log::clean_up(log_output)
}
