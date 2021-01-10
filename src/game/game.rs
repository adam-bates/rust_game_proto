use log::info;

use super::config;
use super::error::types;

pub fn run_game(
    _ctx: &mut ggez::Context,
    _events_loop: &mut ggez::event::EventsLoop,
) -> types::GameResult {
    info!("Running game: {}", config::APPLICATION_NAME);

    Ok(())
}
